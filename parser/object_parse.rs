use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir;
use object::{self, Object};
use traits;

use super::die_parse;

impl<'a> Parse<'a> for object::File<'a> {
    type ItemsExtra = ();

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        _extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        // Identify the endianty of the file.
        let endian = if self.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        // Get the contents of the .debug_abbrev section in the file.
        let debug_abbrev_data = self.section_data_by_name(".debug_abbrev").ok_or(
            traits::Error::with_msg("Could not find .debug_abbrev section"),
        )?;
        let debug_abbrev = gimli::DebugAbbrev::new(&debug_abbrev_data, endian);

        // Get the contents of the string table (.debug_str) section in the file.
        let debug_string_data = self
            .section_data_by_name(".debug_str")
            .ok_or(traits::Error::with_msg("Could not find .debug_str section"))?;
        let debug_str = gimli::DebugStr::new(&debug_string_data, endian);

        // Get the contents of the .debug_info section in the file.
        let debug_info_sect_data = self.section_data_by_name(".debug_info").ok_or(
            traits::Error::with_msg("Could not find .debug_info section"),
        )?;
        let debug_info = gimli::DebugInfo::new(&debug_info_sect_data, endian);

        // Parse the items in each compilation unit in the file.
        while let Some((unit_id, unit)) = debug_info.units().enumerate().next()? {
            let extra = (unit_id, debug_abbrev, debug_str);
            unit.parse_items(items, extra)?
        }

        Ok(())
    }

    type EdgesExtra = ();

    fn parse_edges(
        &self,
        _items: &mut ir::ItemsBuilder,
        _extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

impl<'a, R> Parse<'a> for gimli::CompilationUnitHeader<R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (usize, gimli::DebugAbbrev<R>, gimli::DebugStr<R>);

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (unit_id, debug_abbrev, debug_str) = extra;

        // Find the abbreviations associated with this compilation unit.
        let abbrevs = self
            .abbreviations(&debug_abbrev)
            .expect("Could not find abbreviations");

        let mut entry_id = 0;

        // Parse the contained debugging information entries in depth-first order.
        while let Some((depth, entry)) = self.entries(&abbrevs).next_dfs()? {
            // Bail out of the loop when we return to the starting position.
            if depth >= 0 {
                break;
            }

            let id = ir::Id::entry(unit_id, entry_id);
            entry.parse_items(items, (id, &debug_str))?;
            entry_id += 1;
        }

        Ok(())
    }

    type EdgesExtra = ();

    fn parse_edges(
        &self,
        _items: &mut ir::ItemsBuilder,
        _extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (ir::Id, &'unit gimli::DebugStr<R>);

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (id, debug_str) = extra;

        // Calculate the item's name, kind, and size.
        let item_kind: ir::ItemKind = die_parse::item_kind(&self)?;
        let name = self::die_parse::item_name(&self, &item_kind, &debug_str)?;
        let size = self::die_parse::item_size(&self, &item_kind)?;

        // Create a new IR item for this entity, add it to the items builder.
        let new_ir_item = ir::Item::new(id, name, size, item_kind);
        items.add_item(new_ir_item);

        Ok(())
    }

    type EdgesExtra = ();

    fn parse_edges(
        &self,
        _items: &mut ir::ItemsBuilder,
        _extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

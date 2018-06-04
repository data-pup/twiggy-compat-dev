use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir::{self, Id};
use object::{self, Object};
use traits;

impl<'a> Parse<'a> for object::File<'a> {
    type ItemsExtra = ();

    /// Parse `Self` into one or more `ir::Item`s and add them to the builder.
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
        let debug_abbrev_data = self
            .section_data_by_name(".debug_abbrev")
            .ok_or(
                traits::Error::with_msg("Could not find .debug_abbrev section")
            )?;
        let debug_abbrev = gimli::DebugAbbrev::new(&debug_abbrev_data, endian);

        // Get the contents of the string table (.debug_str) section in the file.
        let debug_string_data = self
            .section_data_by_name(".debug_str")
            .ok_or(
                traits::Error::with_msg("Could not find .debug_str section")
            )?;
        let debug_str = gimli::DebugStr::new(&debug_string_data, endian);

        // Get the contents of the .debug_info section in the file.
        let debug_info_sect_data = self
            .section_data_by_name(".debug_info")
            .ok_or(
                traits::Error::with_msg("Could not find .debug_info section")
            )?;
        let debug_info = gimli::DebugInfo::new(&debug_info_sect_data, endian);

        // Iterate through the entries inside of each unit.
        while let Some((unit_id, unit)) = debug_info.units().enumerate().next()? {
            let extra = (unit_id, debug_abbrev, debug_str);
            unit.parse_items(items, extra)?
        }

        Ok(())
    }

    /// Any extra data needed to parse this type's edges.
    type EdgesExtra = ();

    /// Parse edges between items. This is only called *after* we have already
    /// parsed items.
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
    /// Any extra data needed to parse this type's items.
    type ItemsExtra = (usize, gimli::DebugAbbrev<R>, gimli::DebugStr<R>);

    /// Parse `Self` into one or more `ir::Item`s and add them to the builder.
    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        // Destructure the information in `extra`.
        let (unit_id, debug_abbrev, debug_str) = extra;

        // Find the abbreviations associated with this compilation unit.
        let abbrevs = self
            .abbreviations(&debug_abbrev)
            .expect("Could not find abbreviations");

        let mut entry_id = 0;
        while let Some((depth, current)) = self.entries(&abbrevs).next_dfs()? {
            // Bail out of the loop when we return to the starting position.
            if depth >= 0 {
                break;
            }

            let id = Id::entry(unit_id, entry_id);
            current.parse_items(items, (id, &debug_str))?;
            entry_id += 1;
        }

        Ok(())
    }

    /// Any extra data needed to parse this type's edges.
    type EdgesExtra = ();

    /// Parse edges between items. This is only called *after* we have already
    /// parsed items.
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
    /// Any extra data needed to parse this type's items.
    type ItemsExtra = (Id, &'unit gimli::DebugStr<R>);

    /// Parse `Self` into one or more `ir::Item`s and add them to the builder.
    fn parse_items(
        &self,
        _items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (_id, debug_str) = extra;

        let _temp = self.attr(gimli::DW_AT_name)?
            .ok_or(traits::Error::with_msg(
                "Could not find DW_AT_name attribute for debugging information entry"
            ))?
            .string_value(&debug_str)
            .ok_or(traits::Error::with_msg(
                "Could not find entity name in string table",
            ))?;
            // .to_string()? // FIXUP: This causes an error?
            // .to_owned();

        // let size = current
        //     .attr(gimli::DW_AT_byte_size)?
        //     .and_then(|attr| attr.udata_value())
        //     .ok_or(traits::Error::with_msg(
        //         "Could not find DW_AT_byte_size attribute for entry",
        //     ))? as u32; // FIXUP: Should we change the size in ir::Item to u64?

        // let new_ir_item = ir::Item::new(id, name, size, ir::Misc::new());
        // items.add_item(new_ir_item);

        unimplemented!();

        // Ok(())
    }

    /// Any extra data needed to parse this type's edges.
    type EdgesExtra = ();

    /// Parse edges between items. This is only called *after* we have already
    /// parsed items.
    fn parse_edges(
        &self,
        _items: &mut ir::ItemsBuilder,
        _extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

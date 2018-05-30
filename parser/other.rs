use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir::{self, Id};
use object;
use object::Object;
use traits;

impl<'a> Parse<'a> for object::File<'a> {
    type ItemsExtra = ();

    /// Parse `Self` into one or more `ir::Item`s and add them to the builder.
    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let endian = if self.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        let debug_info_sect_data = self
            .section_data_by_name(".debug_info")
            .expect("Could not find .debug_info section");
        let debug_info = gimli::DebugInfo::new(&debug_info_sect_data, endian);

        let debug_abbrev_data = self
            .section_data_by_name(".debug_abbrev")
            .expect("Could not find .debug_abbrev section");
        let debug_abbrev = gimli::DebugAbbrev::new(&debug_abbrev_data, endian);

        // let mut iter = _debug_info.units();
        // while let Some(unit) = iter
        //     .next()
        //     .expect("Could not find next unit in .debug_info")
        // {
        //     // let debug_abbrev = gimli::DebugAbbrev::new(&unit, endian);
        //     // let abbrevs_ = unit.abbreviations(&debug_abbrev).unwrap();
        //     unimplemented!();
        // }

        let compilation_units = debug_info
            .units()
            .collect::<Vec<_>>()
            .expect("Could not collect .debug_info units");

        for unit in compilation_units.iter() {
            let abbrevs = unit
                .abbreviations(&debug_abbrev)
                .expect("Could not find abbreviations");
            let mut entries_cursor = unit.entries(&abbrevs);

            while let Some((delta_depth, current)) = entries_cursor
                .next_dfs()
                .expect("Could not parse next entry")
            {
                unimplemented!();
            }

            unimplemented!();
        }

        unimplemented!();
    }

    /// Any extra data needed to parse this type's edges.
    type EdgesExtra = ();

    /// Parse edges between items. This is only called *after* we have already
    /// parsed items.
    fn parse_edges(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

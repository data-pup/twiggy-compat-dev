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

        // Get the contents of the .debug_info section in the file.
        let debug_info_sect_data = self
            .section_data_by_name(".debug_info")
            .expect("Could not find .debug_info section");
        let debug_info = gimli::DebugInfo::new(&debug_info_sect_data, endian);

        // Get the contents of the .debug_abbrev section in the file.
        let debug_abbrev_data = self
            .section_data_by_name(".debug_abbrev")
            .expect("Could not find .debug_abbrev section");
        let debug_abbrev = gimli::DebugAbbrev::new(&debug_abbrev_data, endian);

        // Get the contents of the string table (.debug_str) section in the file.
        let debug_string_data = self
            .section_data_by_name(".debug_str")
            .expect("Could not find .debug_str section");
        let debug_str = gimli::DebugStr::new(&debug_string_data, endian);

        // Collect the units in .debug_info into a Vec of compilation unit headers.
        let compilation_units = debug_info
            .units()
            .collect::<Vec<_>>()
            .expect("Could not collect .debug_info units");

        // Iterate through the entries inside of each unit.
        for (unit_id, unit) in compilation_units.iter().enumerate() {
            let abbrevs = unit
                .abbreviations(&debug_abbrev)
                .expect("Could not find abbreviations");

            let mut curr_entry_id = 0;
            let mut entries_cursor = unit.entries(&abbrevs);

            // Traverse the entries in the unit in depth-first order.
            while let Some((delta_depth, current)) = entries_cursor
                .next_dfs()
                .expect("Could not parse next entry")
            {
                // Bail out of the loop when we return to the starting position.
                if delta_depth >= 0 {
                    break;
                }

                let id = Id::entry(unit_id, curr_entry_id);

                let name: String = current
                    .attr(gimli::DW_AT_name)?
                    .ok_or(traits::Error::with_msg(
                        "Could not find DW_AT_name attribute for entry",
                    ))?
                    .string_value(&debug_str)
                    .ok_or(traits::Error::with_msg(
                        "Could not find entity name in string table",
                    ))?
                    .to_string()?
                    .to_owned(); // FIXUP: `to_string` -> `to_owned` seems less than ideal?

                let size = current
                    .attr(gimli::DW_AT_byte_size)?
                    .and_then(|attr| attr.udata_value())
                    .expect("Could not find DW_AT_byte_size attribute for entry")
                    as u32; // FIXUP: Should we change the size in ir::Item to u64?

                let new_ir_item = ir::Item::new(id, name, size, ir::Misc::new());
                items.add_item(new_ir_item);

                curr_entry_id += 1;
            }
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

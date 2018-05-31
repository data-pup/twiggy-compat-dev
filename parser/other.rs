use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir::{self, Id};
use object::{self, Object};
use traits;

fn serialized_size<T>(t: T) -> u32 {
    unimplemented!();
}

impl<'a> Parse<'a> for object::File<'a> {
    type ItemsExtra = ();

    /// Parse `Self` into one or more `ir::Item`s and add them to the builder.
    fn parse_items(
        &self,
        _items: &mut ir::ItemsBuilder,
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

                // TODO:
                // *  Create an Id value for the given entry.
                // *  Add the item to the ItemsBuilder.

                let _id = Id::entry(unit_id, curr_entry_id);
                let _size = current.attr_value(gimli::DW_AT_byte_size);
                let _name = current.attr_value(gimli::DW_AT_name);
                // let new_ir_item = ir::Item::new(id, name, size, ir::Misc::new());
                unimplemented!();
                // items.add_item(new_ir_item);

                curr_entry_id += 1;
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
        _items: &mut ir::ItemsBuilder,
        _extra: Self::EdgesExtra,
    ) -> Result<(), traits::Error> {
        unimplemented!();
    }
}

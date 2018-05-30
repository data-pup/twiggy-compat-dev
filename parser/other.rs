use super::Parse;
use gimli;
use ir::{self, Id};
use object;
use object::Object;
use std::fmt::Write;
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

        let debug_sect_data = self
            .section_data_by_name(".debug_info")
            .expect("Could not find .debug_info section");
        let _debug_info = gimli::DebugInfo::new(&debug_sect_data, endian);

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

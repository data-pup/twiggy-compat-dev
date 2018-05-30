use super::Parse;
use ir::{self, Id};
use gimli;
use object;
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

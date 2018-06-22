use gimli;
use ir;
use traits;

use super::Parse;

impl<'input, R> Parse<'input> for gimli::CompilationUnitHeader<R, R::Offset>
where
    R: 'input + gimli::Reader,
{
    type ItemsExtra = (
        usize,
        gimli::DebugAbbrev<R>,
        gimli::DebugStr<R>,
        &'input gimli::RangeLists<R>,
    );

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        println!("Parsing compilation unit..."); // FIXUP: Debug print line.

        let (unit_id, debug_abbrev, debug_str, rnglists) = extra;

        // Get the size of addresses in this type-unit.
        let addr_size: u8 = self.address_size();
        let version: u16 = self.version();

        // Find the abbreviations associated with this compilation unit.
        let abbrevs = self
            .abbreviations(&debug_abbrev)
            .expect("Could not find abbreviations");

        let mut entry_id = 0; // Debugging information entry ID counter.

        // Create an entries cursor, and move it to the root.
        let mut die_cursor = self.entries(&abbrevs);
        assert!(die_cursor.next_dfs().unwrap().is_some());

        // Parse the contained debugging information entries in depth-first order.
        let mut depth = 0;
        while let Some((delta, entry)) = die_cursor.next_dfs()? {
            // Update depth value, and break out of the loop when we
            // return to the original starting position.
            depth += delta;
            assert!(depth >= 0);
            if depth <= 0 {
                break;
            }

            let id = ir::Id::entry(unit_id, entry_id);
            let die_extra = (id, addr_size, version, &debug_str, rnglists);
            entry.parse_items(items, die_extra)?;
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
use gimli;
use ir;
use traits;

use super::die_ir_item_kind::item_kind;
use super::die_ir_item_name::item_name;
use super::die_ir_item_size::compilation_unit_size;
use super::Parse;

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (
        ir::Id,
        u8,
        u16,
        &'unit gimli::DebugStr<R>,
        &'unit gimli::RangeLists<R>,
    );

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        println!("Parsing DIE..."); // FIXUP: Debug print line.

        let (id, addr_size, version, debug_str, rnglists) = extra;

        if let Some(kind) = item_kind(self)? {
            let name_opt = item_name(self, debug_str)?;
            // FIXUP: This will eventually result in a plain `ir::Item` object,
            // returning an Option for now so I can develop incrementally.
            let new_ir_item: Option<ir::Item> = match kind {
                ir::ItemKind::Code(_) => None,
                ir::ItemKind::CompilationUnit(_) => {
                    // FIXUP: This item kind should not end up occurring here.
                    let name = name_opt.unwrap_or(format!("Code[{:?}]", id));
                    let size = compilation_unit_size(self, addr_size, version, rnglists)? as u32;
                    Some(ir::Item::new(id, name, size, kind))
                }
                ir::ItemKind::Data(_) => {
                    // let _location = self.attr_value(gimli::DW_AT_location)?;
                    // unimplemented!();
                    None
                }
                ir::ItemKind::Debug(_) => None,
                ir::ItemKind::Label(_) => None,
                ir::ItemKind::Misc(_) => None,
                ir::ItemKind::Scope(_) => None,
                ir::ItemKind::Subroutine(_) => None,
            };

            // FIXUP: See above note, unwrapping will not always be needed.
            if let Some(item) = new_ir_item {
                items.add_item(item);
            }
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

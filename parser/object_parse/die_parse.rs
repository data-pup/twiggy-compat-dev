use gimli;
use ir;
use traits;

use super::die_ir_item_kind::item_kind;
use super::die_ir_item_name::item_name;
use super::die_ir_item_size::compilation_unit_size;
use super::Parse;

pub struct DIEItemsExtra<'unit, R>
where
    R: gimli::Reader,
    R: 'unit,
{
    pub ir_id: ir::Id,
    pub addr_size: u8,
    pub dwarf_version: u16,
    pub debug_str: &'unit gimli::DebugStr<R>,
    pub rnglists: &'unit gimli::RangeLists<R>,
    pub comp_unit: &'unit gimli::CompilationUnitHeader<R, <R as gimli::Reader>::Offset>,
}

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    // type ItemsExtra = (
    //     ir::Id,
    //     u8,
    //     u16,
    //     &'unit gimli::DebugStr<R>,
    //     &'unit gimli::RangeLists<R>,
    //     &'unit gimli::CompilationUnitHeader<R, <R as gimli::Reader>::Offset>,
    // );
    type ItemsExtra = DIEItemsExtra<'unit, R>;

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        println!("Parsing DIE..."); // FIXUP: Debug print line.

        // let (_id, _addr_size, _version, debug_str, _rnglists, _comp_unit) = extra;
        let Self::ItemsExtra {
            ir_id,
            addr_size,
            dwarf_version,
            debug_str,
            rnglists,
            comp_unit,
        } = extra;

        if let Some(kind) = item_kind(self)? {
            let name_opt = item_name(self, debug_str)?;
            // FIXUP: This will eventually result in a plain `ir::Item` object,
            // returning an Option for now so I can develop incrementally.
            let new_ir_item: Option<ir::Item> = match kind {
                ir::ItemKind::Code(_) => None,
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

use gimli;
use ir;
use traits;

use super::die_ir_item_kind::item_kind;
use super::die_ir_item_name::item_name;
use super::die_is_edge::is_edge;
use super::Parse;

// FIXUP: Not using this function as is.
// use super::die_ir_item_size::compilation_unit_size;

pub struct DIEItemsExtra<'unit, R>
where
    R: 'unit + gimli::Reader,
{
    pub ir_id: ir::Id,
    pub addr_size: u8,
    pub dwarf_version: u16,
    pub debug_str: &'unit gimli::DebugStr<R>,
    pub debug_types: &'unit gimli::DebugTypes<R>,
    pub rnglists: &'unit gimli::RangeLists<R>,
    pub comp_unit: &'unit gimli::CompilationUnitHeader<R, <R as gimli::Reader>::Offset>,
}

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = DIEItemsExtra<'unit, R>;

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let Self::ItemsExtra {
            ir_id,
            addr_size: _,
            dwarf_version: _,
            debug_str,
            debug_types,
            rnglists: _,
            comp_unit,
        } = extra;

        if let Some(kind) = item_kind(self, debug_types, comp_unit)? {
            let name_attr = item_name(self, debug_str)?;
            // FIXUP: This will eventually result in a plain `ir::Item` object,
            // returning an Option for now so I can develop incrementally.
            let new_ir_item: Option<ir::Item> = match kind {
                ir::ItemKind::Code(_) => None,
                ir::ItemKind::Data(_) => {
                    // let ir_name = name_attr.unwrap_or("DATA".to_string());
                    // let ir_size = 1; // FIXUP: Add logic for this.
                    // Some(ir::Item::new(ir_id, ir_name, ir_size, kind))
                    None
                }
                ir::ItemKind::Debug(_) => None,
                ir::ItemKind::Misc(_) => None,
                ir::ItemKind::Scope(_) => {
                    // let ir_name = name_attr.unwrap_or("SCOPE".to_string());
                    // let ir_size = 3; // FIXUP: Add logic for this.
                    // Some(ir::Item::new(ir_id, ir_name, ir_size, kind))
                    None
                }
                ir::ItemKind::Subroutine(_) => {
                    let ir_name = name_attr.unwrap_or("SUBROUTINE".to_string());
                    let ir_size = 2; // FIXUP: Add logic for this.
                    Some(ir::Item::new(ir_id, ir_name, ir_size, kind))
                }
                ir::ItemKind::Type(_) => {
                    unimplemented!();
                }
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
        if is_edge(self)? {}

        Ok(())
    }
}

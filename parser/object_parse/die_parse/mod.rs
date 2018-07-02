use gimli;
use ir;
use traits;

use super::Parse;

mod item_kind;
mod item_name;
mod location_attrs;

use self::item_kind::item_kind;
use self::item_name::item_name;
use self::location_attrs::DieLocationAttributes;

/// This type alias is used to represent an option return value for
/// a procedure that could return an Error.
type FallilbleOption<T> = Result<Option<T>, traits::Error>;

/// This struct represents the extra items required by the Parse trait's
/// `parse_items` method. This is constructed by the compilation unit's
/// own implementation of `parse_items`.
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
            addr_size,
            dwarf_version,
            debug_str,
            debug_types,
            rnglists,
            comp_unit,
        } = extra;

        if let Some(kind) = item_kind(self, debug_types, comp_unit)? {
            let name_attr = item_name(self, debug_str)?;
            let location_attrs = DieLocationAttributes::try_from(self)?;

            // FIXUP: This will eventually result in a plain `ir::Item` object,
            // returning an Option for now so I can develop incrementally.
            let new_ir_item: Option<ir::Item> = match kind {
                ir::ItemKind::Code(_) => None,
                ir::ItemKind::Data(_) => None,
                ir::ItemKind::Debug(_) => None,
                ir::ItemKind::Misc(_) => None,
                ir::ItemKind::Scope(_) => None,
                ir::ItemKind::Subroutine(_) => {
                    let ir_name = name_attr.unwrap_or("Subroutine".to_string());
                    if let Some(ir_size) =
                        location_attrs.entity_size(addr_size, dwarf_version, rnglists)?
                    {
                        Some(ir::Item::new(ir_id, ir_name, ir_size as u32, kind))
                    } else {
                        None
                    }
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
        // TODO: Add edges representing the call graph.
        Ok(())
    }
}

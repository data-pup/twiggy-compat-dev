use super::Parse;
use gimli;
use ir;
use traits;

use die_item_kind::item_kind;
use die_item_size::item_size;

/// Calculate the item's name. For more information about this, refer to Section 2.15 of
/// the DWARF v5 specification: 'Identifier Names'. Any DIE associated representing an
/// entity that has been given a name may have a `DW_AT_name` attribute. If there was
/// not a name assigned to the entity in the source code, the attribute may either not
/// exist, or be a single null byte.
fn item_name<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    item_type: &ir::ItemKind,
    debug_str: &gimli::DebugStr<R>,
) -> Result<String, traits::Error>
where
    R: gimli::Reader,
{
    match die.attr(gimli::DW_AT_name)? {
        Some(dw_at) => {
            let name: String = dw_at.string_value(&debug_str)
                .ok_or(traits::Error::with_msg(
                    "Could not find entity name in string table",
                ))?
                .to_string()? // This `to_string()` returns a Result<Cow<'_, str>, _>
                .to_string();
            Ok(name)
        }
        None => {
            // FIXUP: Assign a name using the tag / entity type?
            match item_type {
                _ => unimplemented!(),
            }
        }
    }
}

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (ir::Id, &'unit gimli::DebugStr<R>);

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (id, debug_str) = extra;

        // Calculate the item's name, kind, and size.
        let item_kind: ir::ItemKind = item_kind(&self)?;
        let name = item_name(&self, &item_kind, &debug_str)?;
        let size = item_size(&self, &item_kind)?;

        // Create a new IR item for this entity, add it to the items builder.
        let new_ir_item = ir::Item::new(id, name, size, item_kind);
        items.add_item(new_ir_item);

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
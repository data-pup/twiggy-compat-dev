use gimli;
use ir;
use traits;

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

/// Calculate the kind of IR item to represent the code or data associated with
/// a given debugging information entry.
pub fn item_kind<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
) -> Result<Option<ir::ItemKind>, traits::Error>
where
    R: gimli::Reader,
{
    let item_kind = match die.tag() {
        gimli::DW_TAG_null => unimplemented!(),

        // Program Scope Entries: (Chapter 3)
        // --------------------------------------------------------------------
        // Compilation units. (Section 3.1)
        gimli::DW_TAG_compile_unit
        | gimli::DW_TAG_partial_unit
        | gimli::DW_TAG_imported_unit
        | gimli::DW_TAG_type_unit => Some(ir::CompilationUnit::new().into()),
        gimli::DW_TAG_skeleton_unit => unimplemented!(),
        // Module, namespace, and imported entries. (Section 3.2)
        gimli::DW_TAG_module
        | gimli::DW_TAG_namespace
        | gimli::DW_TAG_imported_module
        | gimli::DW_TAG_imported_declaration => Some(ir::Scope::new().into()),
        // Subroutine and entry point entries. (Section 3.3)
        gimli::DW_TAG_subprogram | gimli::DW_TAG_inlined_subroutine | gimli::DW_TAG_entry_point => {
            Some(ir::Subroutine::new().into())
        }
        // Label entries. (Section 3.6)
        gimli::DW_TAG_label => Some(ir::Label::new().into()),
        // With statements. (Section 3.7)
        gimli::DW_TAG_with_stmt => unimplemented!(),
        // Data Object and Object List Entries: (Chapter 4)
        // --------------------------------------------------------------------
        // Data object entries. (Section 4.1)
        gimli::DW_TAG_variable | gimli::DW_TAG_formal_parameter | gimli::DW_TAG_constant => {
            // FIXUP: This will return an offset into the current compilation unit.
            // let ty = item_type_name(&die)?;
            // Some(ir::Data::new(ty).into())
            None
        }
        // Common block entries. (Section 4.2)
        gimli::DW_TAG_common_block => None,
        // Namelist entries. (Section 4.3)
        gimli::DW_TAG_namelist => None,
        gimli::DW_TAG_namelist_item => None,
        // Type Entries: (Chapter 5)
        // --------------------------------------------------------------------
        // Base type entries. (Section 5.1)
        gimli::DW_TAG_base_type => None,
        // Unspecified type entries. (Section 5.2)
        gimli::DW_TAG_unspecified_type => None,
        // Type modifier entries. (Section 5.3)
        gimli::DW_TAG_atomic_type => None,
        gimli::DW_TAG_const_type => None,
        gimli::DW_TAG_immutable_type => None,
        gimli::DW_TAG_packed_type => None,
        gimli::DW_TAG_pointer_type => None,
        gimli::DW_TAG_reference_type => None,
        gimli::DW_TAG_restrict_type => None,
        gimli::DW_TAG_rvalue_reference_type => None,
        gimli::DW_TAG_shared_type => None,
        gimli::DW_TAG_volatile_type => None,
        // Typedef entries. (Section 5.4)
        gimli::DW_TAG_typedef => None,
        // Array type entries. (Section 5.5)
        gimli::DW_TAG_array_type => None,
        // Coarray type entries. (Section 5.6)
        gimli::DW_TAG_coarray_type => None,
        // Structure, union, and class type entries. (Section 5.7.1)
        gimli::DW_TAG_class_type => None,
        gimli::DW_TAG_structure_type => None,
        gimli::DW_TAG_union_type => None,
        // Interface type entries. (Section 5.7.2)
        gimli::DW_TAG_interface_type => None,
        // Derived or extended structures, classes, and interfaces. (Section 5.7.3)
        gimli::DW_TAG_inheritance => None,
        // Access declarations. (Section 5.7.4)
        gimli::DW_TAG_access_declaration => None,
        // Friend entries. (Section 5.7.5)
        gimli::DW_TAG_friend => None,
        // Data member entries. (Section 5.7.6)
        gimli::DW_TAG_member => None,
        // Class variable entries. (Section 5.7.7)
        // FIXUP: This also seems to use `DW_TAG_variable`?
        // Member function entries. (Section 5.7.8)
        // FIXUP: This also seems to use `DW_TAG_subprogram`?
        // Class template instantiations. (Section 5.7.9)
        // FIXUP: This also uses `DW_TAG_class_type` `DW_TAG_structure_type`
        // and `DW_TAG_union_type`?
        // Variant entries. (Section 5.7.10)
        gimli::DW_TAG_variant => None,
        gimli::DW_TAG_variant_part => None,
        // Condition entries. (Section 5.8)
        gimli::DW_TAG_condition => None,
        // Enumeration entries. (Section 5.9)
        gimli::DW_TAG_enumeration_type => None,
        // Subroutine type entries. (Section 5.10)
        gimli::DW_TAG_subroutine_type => None,
        // String type entries. (Section 5.11)
        gimli::DW_TAG_string_type => None,
        // Set type entries. (Section 5.12)
        gimli::DW_TAG_set_type => None,
        // Subrange type entries. (Section 5.13)
        gimli::DW_TAG_subrange_type => None,
        // Pointer to member type entries. (Section 5.14)
        gimli::DW_TAG_ptr_to_member_type => None,
        // File type entries. (Section 5.15)
        gimli::DW_TAG_file_type => None,
        // Dynamic type entries. (Section 5.16)
        gimli::DW_TAG_dynamic_type => None,
        // Template alias type entries. (Section 5.17)
        gimli::DW_TAG_template_alias => None,
        // Miscellaneous tags:
        // ------------------------------------------------------------------------
        gimli::DW_TAG_lexical_block => None,
        gimli::DW_TAG_try_block => None,
        gimli::DW_TAG_catch_block => None,
        gimli::DW_TAG_call_site => None,
        gimli::DW_TAG_call_site_parameter => None,
        gimli::DW_TAG_unspecified_parameters => None,
        gimli::DW_TAG_common_inclusion => None,
        gimli::DW_TAG_enumerator => None,
        gimli::DW_TAG_template_value_parameter => None,
        gimli::DW_TAG_thrown_type => None,
        // TODO: Sort these remaining tags out.
        gimli::DW_TAG_dwarf_procedure => None,
        gimli::DW_TAG_template_type_parameter => None,
        gimli::DW_TAG_generic_subrange => None,
        gimli::DW_TAG_lo_user => None,
        gimli::DW_TAG_hi_user => None,
        // Default case.   (FIXUP: Should this return a `ItemKind::Misc`?)
        gimli::DwTag(_) => None,
    };

    Ok(item_kind)
}

/// Find the name of the entry referenced by the `DW_AT_type` attribute for a
/// DIE representing a data object or object list entry. Note that this is
/// referring to the type, not the item kind, and returns the value of that
/// entry's `DW_AT_name` attribute.
///
/// FIXUP: What type(s) is contained in the `DW_AT_type` attribute?
fn item_type_name<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
) -> Result<Option<String>, traits::Error>
where
    R: gimli::Reader,
{
    if let Some(type_attr) = die.attr_value(gimli::DW_AT_type)? {
        match type_attr {
            gimli::AttributeValue::DebugTypesRef(_) => unimplemented!(),
            gimli::AttributeValue::UnitRef(_) => unimplemented!(),
            _ => Err(traits::Error::with_msg(format!(
                "Unexpected type encoding, found type: {:?}",
                type_attr
            ))),
        }
    } else {
        Ok(None)
    }
}
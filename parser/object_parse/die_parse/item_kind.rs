use gimli;
use ir;

use super::FallilbleOption;

/// Calculate the kind of IR item to represent the code or data associated with
/// a given debugging information entry.
pub fn item_kind<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    _debug_types: &gimli::DebugTypes<R>,
    _compilation_unit: &gimli::CompilationUnitHeader<R, <R as gimli::Reader>::Offset>,
) -> FallilbleOption<ir::ItemKind>
where
    R: gimli::Reader,
{
    let item_kind = match die.tag() {
        gimli::DW_TAG_null => unimplemented!(),

        // Program Scope Entries: (Chapter 3)
        // --------------------------------------------------------------------
        // Compilation units. (Section 3.1)
        // These are disregarded, and not represented in the twiggy IR.
        gimli::DW_TAG_compile_unit
        | gimli::DW_TAG_partial_unit
        | gimli::DW_TAG_imported_unit
        | gimli::DW_TAG_type_unit
        | gimli::DW_TAG_skeleton_unit => unimplemented!(),
        // Module, namespace, and imported entries. (Section 3.2)
        gimli::DW_TAG_module
        | gimli::DW_TAG_namespace
        | gimli::DW_TAG_imported_module
        | gimli::DW_TAG_imported_declaration => Some(ir::Scope::new().into()),
        // Subroutine and entry point entries. (Section 3.3)
        gimli::DW_TAG_subprogram => Some(ir::Subroutine::new().into()),
        gimli::DW_TAG_inlined_subroutine | gimli::DW_TAG_entry_point => None,
        // Label entries. (Section 3.6)
        gimli::DW_TAG_label => None,
        // With statements. (Section 3.7)
        gimli::DW_TAG_with_stmt => unimplemented!(),
        // Data Object and Object List Entries: (Chapter 4)
        // --------------------------------------------------------------------
        // Data object entries. (Section 4.1)
        gimli::DW_TAG_variable | gimli::DW_TAG_formal_parameter | gimli::DW_TAG_constant => {
            // let ty = type_name(&die, debug_types, compilation_unit)?;
            let ty = Some("VARIABLE".to_string()); // FIXUP.
            Some(ir::Data::new(ty).into())
        }
        // Common block entries. (Section 4.2)
        gimli::DW_TAG_common_block => unimplemented!(),
        // Namelist entries. (Section 4.3)
        gimli::DW_TAG_namelist => unimplemented!(),
        gimli::DW_TAG_namelist_item => unimplemented!(),
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

use gimli;
use ir;
use traits;

/// Find the `ItemKind` type for an entry with the given tag.
pub fn item_kind<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
) -> Result<ir::ItemKind, traits::Error>
where
    R: gimli::Reader,
{
    let tag = die.tag();
    let kind: ir::ItemKind = match tag {
        gimli::DW_TAG_null => unimplemented!(),

        // Program Scope Entries: (Chapter 3)
        // --------------------------------------------------------------------

        // Compilation units. (Section 3.1)
        gimli::DW_TAG_compile_unit
        | gimli::DW_TAG_partial_unit
        | gimli::DW_TAG_imported_unit
        | gimli::DW_TAG_skeleton_unit => unimplemented!(),

        // Module, namespace, and imported entries. (Section 3.2)
        gimli::DW_TAG_module => unimplemented!(),
        gimli::DW_TAG_namespace => unimplemented!(),
        gimli::DW_TAG_imported_module => unimplemented!(),
        gimli::DW_TAG_imported_declaration => unimplemented!(),

        // Subroutine and entry point entries. (Section 3.3)
        gimli::DW_TAG_subprogram => unimplemented!(),
        gimli::DW_TAG_inlined_subroutine => unimplemented!(),
        gimli::DW_TAG_entry_point => unimplemented!(),

        // Label entries. (Section 3.6)
        gimli::DW_TAG_label => unimplemented!(),

        // With statements. (Section 3.7)
        gimli::DW_TAG_with_stmt => unimplemented!(),

        // Data Object and Object List Entries: (Chapter 4)
        // --------------------------------------------------------------------

        // Data object entries. (Section 4.1)
        gimli::DW_TAG_variable => unimplemented!(),
        gimli::DW_TAG_formal_parameter => unimplemented!(),
        gimli::DW_TAG_constant => unimplemented!(),

        // Common block entries. (Section 4.2)
        gimli::DW_TAG_common_block => unimplemented!(),

        // Namelist entries. (Section 4.3)
        gimli::DW_TAG_namelist => unimplemented!(),
        gimli::DW_TAG_namelist_item => unimplemented!(),

        // Type Entries: (Chapter 5)
        // --------------------------------------------------------------------

        // Base type entries. (Section 5.1)
        gimli::DW_TAG_base_type => unimplemented!(),

        // Unspecified type entries. (Section 5.2)
        gimli::DW_TAG_unspecified_type => unimplemented!(),

        // Type modifier entries. (Section 5.3)
        gimli::DW_TAG_atomic_type => unimplemented!(),
        gimli::DW_TAG_const_type => unimplemented!(),
        gimli::DW_TAG_immutable_type => unimplemented!(),
        gimli::DW_TAG_packed_type => unimplemented!(),
        gimli::DW_TAG_pointer_type => unimplemented!(),
        gimli::DW_TAG_reference_type => unimplemented!(),
        gimli::DW_TAG_restrict_type => unimplemented!(),
        gimli::DW_TAG_rvalue_reference_type => unimplemented!(),
        gimli::DW_TAG_shared_type => unimplemented!(),
        gimli::DW_TAG_volatile_type => unimplemented!(),

        // Typedef entries. (Section 5.4)
        gimli::DW_TAG_typedef => unimplemented!(),

        // Array type entries. (Section 5.5)
        gimli::DW_TAG_array_type => unimplemented!(),

        // Coarray type entries. (Section 5.6)
        gimli::DW_TAG_coarray_type => unimplemented!(),

        // Structure, union, and class type entries. (Section 5.7.1)
        gimli::DW_TAG_class_type => unimplemented!(),
        gimli::DW_TAG_structure_type => unimplemented!(),
        gimli::DW_TAG_union_type => unimplemented!(),

        // Interface type entries. (Section 5.7.2)
        gimli::DW_TAG_interface_type => unimplemented!(),

        // Derived or extended structures, classes, and interfaces. (Section 5.7.3)
        gimli::DW_TAG_inheritance => unimplemented!(),

        // Access declarations. (Section 5.7.4)
        gimli::DW_TAG_access_declaration => unimplemented!(),

        // Friend entries. (Section 5.7.5)
        gimli::DW_TAG_friend => unimplemented!(),

        // Data member entries. (Section 5.7.6)
        gimli::DW_TAG_member => unimplemented!(),

        // Class variable entries. (Section 5.7.7)
        // FIXUP: This also seems to use `DW_TAG_variable`?

        // Member function entries. (Section 5.7.8)
        // FIXUP: This also seems to use `DW_TAG_subprogram`?

        // Class template instantiations. (Section 5.7.9)
        // FIXUP: This also uses `DW_TAG_class_type` `DW_TAG_structure_type`
        // and `DW_TAG_union_type`?

        // Variant entries. (Section 5.7.10)
        gimli::DW_TAG_variant => unimplemented!(),
        gimli::DW_TAG_variant_part => unimplemented!(),

        // Condition entries. (Section 5.8)
        gimli::DW_TAG_condition => unimplemented!(),

        // Enumeration entries. (Section 5.9)
        gimli::DW_TAG_enumeration_type => unimplemented!(),

        // Subroutine type entries. (Section 5.10)
        gimli::DW_TAG_subroutine_type => unimplemented!(),

        // String type entries. (Section 5.11)
        gimli::DW_TAG_string_type => unimplemented!(),

        // Set type entries. (Section 5.12)
        gimli::DW_TAG_set_type => unimplemented!(),

        // Subrange type entries. (Section 5.13)
        gimli::DW_TAG_subrange_type => unimplemented!(),

        // Pointer to member type entries. (Section 5.14)
        gimli::DW_TAG_ptr_to_member_type => unimplemented!(),

        // File type entries. (Section 5.15)
        gimli::DW_TAG_file_type => unimplemented!(),

        // Dynamic type entries. (Section 5.16)
        gimli::DW_TAG_dynamic_type => unimplemented!(),

        // Template alias type entries. (Section 5.17)
        gimli::DW_TAG_template_alias => unimplemented!(),

        // Miscellaneous tags:
        // ------------------------------------------------------------------------
        gimli::DW_TAG_lexical_block => unimplemented!(),

        gimli::DW_TAG_try_block => unimplemented!(),
        gimli::DW_TAG_catch_block => unimplemented!(),

        gimli::DW_TAG_call_site => unimplemented!(),
        gimli::DW_TAG_call_site_parameter => unimplemented!(),

        gimli::DW_TAG_unspecified_parameters => unimplemented!(),
        gimli::DW_TAG_common_inclusion => unimplemented!(),
        gimli::DW_TAG_enumerator => unimplemented!(),
        gimli::DW_TAG_template_value_parameter => unimplemented!(),
        gimli::DW_TAG_thrown_type => unimplemented!(),

        // TODO: Sort these remaining tags out.
        gimli::DW_TAG_dwarf_procedure => unimplemented!(),
        gimli::DW_TAG_template_type_parameter => unimplemented!(),
        gimli::DW_TAG_type_unit => unimplemented!(),
        gimli::DW_TAG_generic_subrange => unimplemented!(),
        gimli::DW_TAG_lo_user => unimplemented!(),
        gimli::DW_TAG_hi_user => unimplemented!(),

        // Default case.
        gimli::DwTag(_) => return Err(traits::Error::with_msg("Unrecognized DwTag value")),
    };

    Ok(kind)
}

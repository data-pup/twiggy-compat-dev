use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir;
use object::{self, Object};
use traits;

impl<'a> Parse<'a> for object::File<'a> {
    type ItemsExtra = ();

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        _extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        // Identify the endianty of the file.
        let endian = if self.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };

        // Get the contents of the .debug_abbrev section in the file.
        let debug_abbrev_data = self.section_data_by_name(".debug_abbrev").ok_or(
            traits::Error::with_msg("Could not find .debug_abbrev section"),
        )?;
        let debug_abbrev = gimli::DebugAbbrev::new(&debug_abbrev_data, endian);

        // Get the contents of the string table (.debug_str) section in the file.
        let debug_string_data = self
            .section_data_by_name(".debug_str")
            .ok_or(traits::Error::with_msg("Could not find .debug_str section"))?;
        let debug_str = gimli::DebugStr::new(&debug_string_data, endian);

        // Get the contents of the .debug_info section in the file.
        let debug_info_sect_data = self.section_data_by_name(".debug_info").ok_or(
            traits::Error::with_msg("Could not find .debug_info section"),
        )?;
        let debug_info = gimli::DebugInfo::new(&debug_info_sect_data, endian);

        // Parse the items in each compilation unit in the file.
        while let Some((unit_id, unit)) = debug_info.units().enumerate().next()? {
            let extra = (unit_id, debug_abbrev, debug_str);
            unit.parse_items(items, extra)?
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

impl<'a, R> Parse<'a> for gimli::CompilationUnitHeader<R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (usize, gimli::DebugAbbrev<R>, gimli::DebugStr<R>);

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (unit_id, debug_abbrev, debug_str) = extra;

        // Get the size of addresses in this type-unit.
        let addr_size = self.address_size();

        // Find the abbreviations associated with this compilation unit.
        let abbrevs = self
            .abbreviations(&debug_abbrev)
            .expect("Could not find abbreviations");

        let mut entry_id = 0;

        // Parse the contained debugging information entries in depth-first order.
        while let Some((depth, entry)) = self.entries(&abbrevs).next_dfs()? {
            // Bail out of the loop when we return to the starting position.
            if depth >= 0 {
                break;
            }

            let id = ir::Id::entry(unit_id, entry_id);
            entry.parse_items(items, (id, addr_size, &debug_str))?;
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

impl<'abbrev, 'unit, R> Parse<'unit>
    for gimli::DebuggingInformationEntry<'abbrev, 'unit, R, R::Offset>
where
    R: gimli::Reader,
{
    type ItemsExtra = (ir::Id, u8, &'unit gimli::DebugStr<R>);

    fn parse_items(
        &self,
        items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (_id, _addr_size, debug_str) = extra;

        if let Some(new_item_kind) = item_kind(self) {
            // Attributes that we will use to create an IR item.
            let _name = item_name(self, debug_str)?;
            let _location = self.attr_value(gimli::DW_AT_location)?;
            let _low_pc = self.attr_value(gimli::DW_AT_low_pc)?;
            let _high_pc = self.attr_value(gimli::DW_AT_high_pc)?;
            let _ranges = self.attr_value(gimli::DW_AT_ranges)?;

            let new_ir_item = match new_item_kind {
                ir::ItemKind::Code(_) => {
                    unimplemented!();
                }
                ir::ItemKind::Data(_) => unimplemented!(),
                ir::ItemKind::Debug(_) => unimplemented!(),
                ir::ItemKind::Misc(_) => unimplemented!(),
            };

            items.add_item(new_ir_item);
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

/// Calculate the item's name. For more information about this, refer to Section 2.15 of
/// the DWARF v5 specification: 'Identifier Names'. Any DIE associated representing an
/// entity that has been given a name may have a `DW_AT_name` attribute. If there was
/// not a name assigned to the entity in the source code, the attribute may either not
/// exist, or be a single null byte.
pub fn item_name<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    debug_str: &gimli::DebugStr<R>,
) -> Result<Option<String>, traits::Error>
where
    R: gimli::Reader,
{
    if let Some(s) = die
        .attr(gimli::DW_AT_name)?
        .and_then(|attr| attr.string_value(&debug_str))
    {
        let name = Some(
            s
                .to_string()? // This `to_string()` returns a `Result<Cow<'_, str>, _>`.
                .to_string(), // This `to_string()` returns a String.
        );
        Ok(name)
    } else {
        Ok(None)
    }
}

/// Calculate the kind of IR item to represent the code or data associated with
/// a given debugging information entry.
pub fn item_kind<R>(die: &gimli::DebuggingInformationEntry<R, R::Offset>) -> Option<ir::ItemKind>
where
    R: gimli::Reader,
{
    match die.tag() {
        gimli::DW_TAG_null => unimplemented!(),

        // Program Scope Entries: (Chapter 3)
        // --------------------------------------------------------------------

        // Compilation units. (Section 3.1)
        gimli::DW_TAG_compile_unit | gimli::DW_TAG_partial_unit | gimli::DW_TAG_imported_unit => {
            unimplemented!()
        }

        gimli::DW_TAG_skeleton_unit => unimplemented!(),

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

        // Default case.   (FIXUP: Should this return a `ItemKind::Misc`?)
        gimli::DwTag(_) => None,
    }
}

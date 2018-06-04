use super::Parse;
use fallible_iterator::FallibleIterator;
use gimli;
use ir;
use object::{self, Object};
use traits;

/// Find the `ItemKind` type for an entry with the given tag.
/// FIXUP: Must this match be exhaustive? Is this even a good approach?
fn ir_item_kind(tag: gimli::DwTag) -> Result<ir::ItemKind, traits::Error> {
    match tag {
        gimli::DW_TAG_null => unimplemented!(),

        gimli::DW_TAG_array_type => unimplemented!(),
        gimli::DW_TAG_class_type => unimplemented!(),
        gimli::DW_TAG_entry_point => unimplemented!(),
        gimli::DW_TAG_enumeration_type => unimplemented!(),
        gimli::DW_TAG_formal_parameter => unimplemented!(),
        gimli::DW_TAG_imported_declaration => unimplemented!(),
        gimli::DW_TAG_label => unimplemented!(),
        gimli::DW_TAG_lexical_block => unimplemented!(),
        gimli::DW_TAG_member => unimplemented!(),
        gimli::DW_TAG_pointer_type => unimplemented!(),
        gimli::DW_TAG_reference_type => unimplemented!(),
        gimli::DW_TAG_compile_unit => unimplemented!(),
        gimli::DW_TAG_string_type => unimplemented!(),
        gimli::DW_TAG_structure_type => unimplemented!(),
        gimli::DW_TAG_subroutine_type => unimplemented!(),
        gimli::DW_TAG_typedef => unimplemented!(),
        gimli::DW_TAG_union_type => unimplemented!(),
        gimli::DW_TAG_unspecified_parameters => unimplemented!(),
        gimli::DW_TAG_variant => unimplemented!(),
        gimli::DW_TAG_common_block => unimplemented!(),
        gimli::DW_TAG_common_inclusion => unimplemented!(),
        gimli::DW_TAG_inheritance => unimplemented!(),
        gimli::DW_TAG_inlined_subroutine => unimplemented!(),
        gimli::DW_TAG_module => unimplemented!(),
        gimli::DW_TAG_ptr_to_member_type => unimplemented!(),
        gimli::DW_TAG_set_type => unimplemented!(),
        gimli::DW_TAG_subrange_type => unimplemented!(),
        gimli::DW_TAG_with_stmt => unimplemented!(),
        gimli::DW_TAG_access_declaration => unimplemented!(),
        gimli::DW_TAG_base_type => unimplemented!(),
        gimli::DW_TAG_catch_block => unimplemented!(),
        gimli::DW_TAG_const_type => unimplemented!(),
        gimli::DW_TAG_constant => unimplemented!(),
        gimli::DW_TAG_enumerator => unimplemented!(),
        gimli::DW_TAG_file_type => unimplemented!(),
        gimli::DW_TAG_friend => unimplemented!(),
        gimli::DW_TAG_namelist => unimplemented!(),
        gimli::DW_TAG_namelist_item => unimplemented!(),
        gimli::DW_TAG_packed_type => unimplemented!(),
        gimli::DW_TAG_subprogram => unimplemented!(),
        gimli::DW_TAG_template_type_parameter => unimplemented!(),
        gimli::DW_TAG_template_value_parameter => unimplemented!(),
        gimli::DW_TAG_thrown_type => unimplemented!(),
        gimli::DW_TAG_try_block => unimplemented!(),
        gimli::DW_TAG_variant_part => unimplemented!(),
        gimli::DW_TAG_variable => unimplemented!(),
        gimli::DW_TAG_volatile_type => unimplemented!(),

        // DWARF 3.
        gimli::DW_TAG_dwarf_procedure => unimplemented!(),
        gimli::DW_TAG_restrict_type => unimplemented!(),
        gimli::DW_TAG_interface_type => unimplemented!(),
        gimli::DW_TAG_namespace => unimplemented!(),
        gimli::DW_TAG_imported_module => unimplemented!(),
        gimli::DW_TAG_unspecified_type => unimplemented!(),
        gimli::DW_TAG_partial_unit => unimplemented!(),
        gimli::DW_TAG_imported_unit => unimplemented!(),
        gimli::DW_TAG_condition => unimplemented!(),
        gimli::DW_TAG_shared_type => unimplemented!(),

        // DWARF 4.
        gimli::DW_TAG_type_unit => unimplemented!(),
        gimli::DW_TAG_rvalue_reference_type => unimplemented!(),
        gimli::DW_TAG_template_alias => unimplemented!(),

        // DWARF 5.
        gimli::DW_TAG_coarray_type => unimplemented!(),
        gimli::DW_TAG_generic_subrange => unimplemented!(),
        gimli::DW_TAG_dynamic_type => unimplemented!(),
        gimli::DW_TAG_atomic_type => unimplemented!(),
        gimli::DW_TAG_call_site => unimplemented!(),
        gimli::DW_TAG_call_site_parameter => unimplemented!(),
        gimli::DW_TAG_skeleton_unit => unimplemented!(),
        gimli::DW_TAG_immutable_type => unimplemented!(),

        gimli::DW_TAG_lo_user => unimplemented!(),
        gimli::DW_TAG_hi_user => unimplemented!(),

        // SGI/MIPS extensions.
        gimli::DW_TAG_MIPS_loop => unimplemented!(),

        // HP extensions.
        gimli::DW_TAG_HP_array_descriptor => unimplemented!(),
        gimli::DW_TAG_HP_Bliss_field => unimplemented!(),
        gimli::DW_TAG_HP_Bliss_field_set => unimplemented!(),

        // GNU extensions.
        gimli::DW_TAG_format_label => unimplemented!(),
        gimli::DW_TAG_function_template => unimplemented!(),
        gimli::DW_TAG_class_template => unimplemented!(),
        gimli::DW_TAG_GNU_BINCL => unimplemented!(),
        gimli::DW_TAG_GNU_EINCL => unimplemented!(),
        gimli::DW_TAG_GNU_template_template_param => unimplemented!(),
        gimli::DW_TAG_GNU_template_parameter_pack => unimplemented!(),
        gimli::DW_TAG_GNU_formal_parameter_pack => unimplemented!(),
        gimli::DW_TAG_GNU_call_site => unimplemented!(),
        gimli::DW_TAG_GNU_call_site_parameter => unimplemented!(),

        gimli::DW_TAG_APPLE_property => unimplemented!(),

        // SUN extensions.
        gimli::DW_TAG_SUN_function_template => unimplemented!(),
        gimli::DW_TAG_SUN_class_template => unimplemented!(),
        gimli::DW_TAG_SUN_struct_template => unimplemented!(),
        gimli::DW_TAG_SUN_union_template => unimplemented!(),
        gimli::DW_TAG_SUN_indirect_inheritance => unimplemented!(),
        gimli::DW_TAG_SUN_codeflags => unimplemented!(),
        gimli::DW_TAG_SUN_memop_info => unimplemented!(),
        gimli::DW_TAG_SUN_omp_child_func => unimplemented!(),
        gimli::DW_TAG_SUN_rtti_descriptor => unimplemented!(),
        gimli::DW_TAG_SUN_dtor_info => unimplemented!(),
        gimli::DW_TAG_SUN_dtor => unimplemented!(),
        gimli::DW_TAG_SUN_f90_interface => unimplemented!(),
        gimli::DW_TAG_SUN_fortran_vax_structure => unimplemented!(),

        // ALTIUM extensions.
        gimli::DW_TAG_ALTIUM_circ_type => unimplemented!(),
        gimli::DW_TAG_ALTIUM_mwa_circ_type => unimplemented!(),
        gimli::DW_TAG_ALTIUM_rev_carry_type => unimplemented!(),
        gimli::DW_TAG_ALTIUM_rom => unimplemented!(),

        // Extensions for UPC.
        gimli::DW_TAG_upc_shared_type => unimplemented!(),
        gimli::DW_TAG_upc_strict_type => unimplemented!(),
        gimli::DW_TAG_upc_relaxed_type => unimplemented!(),

        // PGI (STMicroelectronics) extensions.
        gimli::DW_TAG_PGI_kanji_type => unimplemented!(),
        gimli::DW_TAG_PGI_interface_block => unimplemented!(),

        // Borland extensions.
        gimli::DW_TAG_BORLAND_property => unimplemented!(),
        gimli::DW_TAG_BORLAND_Delphi_string => unimplemented!(),
        gimli::DW_TAG_BORLAND_Delphi_dynamic_array => unimplemented!(),
        gimli::DW_TAG_BORLAND_Delphi_set => unimplemented!(),
        gimli::DW_TAG_BORLAND_Delphi_variant => unimplemented!(),

        // Default case.
        gimli::DwTag(_) => Err(traits::Error::with_msg("Unrecognized DwTag value")),
    }
}

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
            entry.parse_items(items, (id, &debug_str))?;
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
    type ItemsExtra = (ir::Id, &'unit gimli::DebugStr<R>);

    fn parse_items(
        &self,
        _items: &mut ir::ItemsBuilder,
        extra: Self::ItemsExtra,
    ) -> Result<(), traits::Error> {
        let (_id, debug_str) = extra;

        let item_kind: ir::ItemKind = ir_item_kind(self.tag())?;

        let name: String = self
            .attr(gimli::DW_AT_name)?
            .ok_or(traits::Error::with_msg(
                "Could not find DW_AT_name attribute for debugging information entry",
            ))?
            .string_value(&debug_str)
            .ok_or(traits::Error::with_msg(
                "Could not find entity name in string table",
            ))?
            .to_string()? // This `to_string()` returns a Result<Cow<'_, str>, _>
            .to_string();

        // Data objects (variables, types) may have a `DW_AT_location` object.

        let size = match item_kind {
            ir::ItemKind::Code(_) => {
                // (Section 2.17) Check if entity has single DW_AT_low_pc,
                // a (DW_AT_low_pc, DW_AT_high_pc) pair, or a `DW_AT_ranges`
                // value to represent the associated addresses.
                unimplemented!();
            }
            ir::ItemKind::Data(_) => {
                // (Section 2.16) Any DIE representing a data object, such as
                // variables or parameters, may have a `DW_AT_location` attribute.
                unimplemented!();
            }
            ir::ItemKind::Debug(_) => {
                // TODO: According to `ir.rs`, this can include DWARF sections?
                unimplemented!();
            }
            ir::ItemKind::Misc(_) => {
                // TODO: How should miscellaneous items be handled?
                unimplemented!();
            }
        };

        // let size = current
        //     .attr(gimli::DW_AT_byte_size)?
        //     .and_then(|attr| attr.udata_value())
        //     .ok_or(traits::Error::with_msg(
        //         "Could not find DW_AT_byte_size attribute for entry",
        //     ))? as u32; // FIXUP: Should we change the size in ir::Item to u64?

        // let new_ir_item = ir::Item::new(id, name, size, ir::Misc::new());
        // items.add_item(new_ir_item);

        unimplemented!();

        // Ok(())
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

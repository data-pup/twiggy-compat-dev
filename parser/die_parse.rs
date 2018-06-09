use gimli;
use ir;
use traits;

// TODO: Move the size calculation into the object_parse module.

/// Calculate the size of an entity associated with a debugging information
/// entry (DIE). For more information about this, refer to Section 2.17 of
/// the DWARF v5 specification: 'Code Addresses, Ranges, and Base Addresses'
/// FIXUP: Will we need to implement a separate function for other entry types?
pub fn _item_size<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    item_kind: &ir::ItemKind,
) -> Result<u32, traits::Error>
where
    R: gimli::Reader,
{
    match item_kind {
        // (Section 2.17) This includes any entities associated with executable machine code
        // including compilation units, module initialization, subroutines, lexical blocks,
        // try/catch blocks, labels, etc.
        //
        // Check if entity has single DW_AT_low_pc, a (DW_AT_low_pc, DW_AT_high_pc) pair, or
        // a `DW_AT_ranges` value to represent the associated addresses. If only
        // `DW_AT_low_pc` exists, then the item only occupies a single address.
        //
        // FIXUP: `DW_AT_high_pc` is sometimes the size, and sometimes an address?
        ir::ItemKind::Code(_) => {
            let low_pc: Option<gimli::AttributeValue<R>> =
                die.attr(gimli::DW_AT_low_pc)?.map(|attr| attr.value());
            let high_pc: Option<gimli::AttributeValue<R>> =
                die.attr(gimli::DW_AT_high_pc)?.map(|attr| attr.value());
            let ranges: Option<gimli::AttributeValue<R>> =
                die.attr(gimli::DW_AT_ranges)?.map(|attr| attr.value());
            match (low_pc, high_pc, ranges) {
                // The associated entity occupies a single address.
                (Some(_low_pc), None, _) => unimplemented!(),
                // The associated entity occupies contiguous space in memory.
                (Some(low_val), Some(high_val), _) => {
                    let size: u64 = match high_val {
                        gimli::AttributeValue::Addr(end_addr) => {
                            let start_addr: u64 = match low_val {
                                gimli::AttributeValue::Addr(a) => a,
                                _ => {
                                    return Err(traits::Error::with_msg(
                                        "Could not identify low address",
                                    ))
                                }
                            };
                            end_addr - start_addr
                        }
                        // TODO: Handle 1, 2, 4, 8 byte cases.
                        gimli::AttributeValue::Data1(_) => unimplemented!(),
                        gimli::AttributeValue::Data2(_) => unimplemented!(),
                        gimli::AttributeValue::Data4(_) => unimplemented!(),
                        gimli::AttributeValue::Data8(_) => unimplemented!(),
                        gimli::AttributeValue::Udata(offset) => offset,
                        _ => {
                            return Err(traits::Error::with_msg("Unexpected DW_AT_high_pc encoding"))
                        }
                    };
                    Ok(size as u32)
                }
                // Find the `DW_AT_ranges` attribute.
                (_, _, Some(range_val)) => {
                    let _ranges_ref: gimli::DebugRangesOffset<R::Offset> = match range_val {
                        gimli::AttributeValue::DebugRangesRef(r) => r,
                        _ => {
                            return Err(traits::Error::with_msg("Unexpected DW_AT_ranges encoding"))
                        }
                    };
                    unimplemented!();
                }
                // Return an error if no location attributes could be found.
                _ => Err(traits::Error::with_msg("Could not calculate size of item")),
            }
        }
        ir::ItemKind::CompUnit => unimplemented!(),
        // (Section 2.16) Any DIE representing a data object, such as variables or parameters,
        // may have a `DW_AT_location` attribute.
        // TODO: This will either be 4 or 8? This is found in the compilation unit header?
        ir::ItemKind::Data(_) => {
            unimplemented!();
        }
        // TODO: According to `ir.rs`, this can include DWARF sections?
        ir::ItemKind::Debug(_) => {
            unimplemented!();
        }
        ir::ItemKind::Misc(_) => {
            unimplemented!();
        }
    }
}

use gimli;
use traits;

use super::DieLocationAttributes;

/// Find the size of an entity that has a machine code address, or a range of
/// machine code addresses. This includes compilation units, module
/// initialization, subroutines, lexical blocks, try/catch blocks (see Section
/// 3.8 on page 93), labels, etc.
///
/// For more information about this, refer to Chapter 2.17 'Code Addresses,
/// Ranges, and Base Addresses' (pg. 51) in the DWARF5 specification.
pub fn subroutine_size<R>(
    location_attrs: DieLocationAttributes<R>,
    addr_size: u8,
    version: u16,
    rnglists: &gimli::RangeLists<R>,
) -> Result<u64, traits::Error>
where
    R: gimli::Reader,
{
    let low_pc_value: Option<u64> = location_attrs.dw_at_low_pc()?;

    if let Some(base_addr) = low_pc_value {
        let high_pc_val = location_attrs.dw_at_high_pc();
        get_contiguous_item_size(base_addr, high_pc_val, addr_size)
    } else {
        let base_addr = location_attrs.dw_at_entry_pc();
        if base_addr.is_none() {
            // FIXUP: A subroutine entry representing a subroutine declaration that
            // is not also a definition does not have code address or range attributes.
            return Ok(0);
        }
        let ranges_attr = location_attrs.dw_at_ranges().unwrap(); // FIXUP
        get_ranges_item_size(
            ranges_attr,
            base_addr.unwrap(),
            addr_size,
            version,
            rnglists,
        )
    }
}

/// FIXUP FIXUP FIXUP: UPDATE THESE COMMENTS.
/// Find the value of the `DW_AT_low_pc` for a DIE representing an entity with
/// a contiguous range of machine code addresses. If there is not a
/// `DW_AT_low_pc` value, then the addresses are not contiguous, and
/// `DW_AT_ranges` should be used instead.
fn get_contiguous_item_size<R>(
    low_pc_val: u64,
    high_pc_val: Option<&gimli::AttributeValue<R>>,
    addr_size: u8,
) -> Result<u64, traits::Error>
where
    R: gimli::Reader,
{
    if let Some(high_pc_attr) = high_pc_val {
        match high_pc_attr {
            gimli::AttributeValue::Addr(end_addr) => Ok(end_addr - low_pc_val),
            gimli::AttributeValue::Udata(offset) => Ok(*offset),
            _ => Err(traits::Error::with_msg(
                "Unexpected DW_AT_high_pc attribute value",
            )),
        }
    } else {
        Ok(addr_size as u64)
    }
}

/// FIXUP: UPDATE THESE COMMENTS.
///
/// Get the size of an entity that occupies non-contiguous address ranges.
fn get_ranges_item_size<R>(
    _ranges_attr: &gimli::AttributeValue<R>,
    _base_addr: &gimli::AttributeValue<R>,
    _addr_size: u8,
    _version: u16,
    _rnglists: &gimli::RangeLists<R>,
) -> Result<u64, traits::Error>
where
    R: gimli::Reader,
{
    unimplemented!();
    // } else if let Some(ranges_attr) = die.attr_value(gimli::DW_AT_ranges)? {
    //     match ranges_attr {
    //         gimli::AttributeValue::RangeListsRef(offset) => {
    //             let size: u64 = rnglists
    //                 .ranges(offset, version, addr_size, base_addr)?
    //                 .map(|r| r.end - r.begin)
    //                 .fold(0, |res, size| res + size)?;
    //             Ok(size)
    //         }
    //         _ => Err(traits::Error::with_msg("Unexpected DW_AT_ranges value")),
    //     }
}

use fallible_iterator::FallibleIterator;
use gimli;
use traits;

/// Find the size of an entity that has a machine code address, or a range of
/// machine code addresses. This includes compilation units, module
/// initialization, subroutines, lexical blocks, try/catch blocks (see Section
/// 3.8 on page 93), labels, etc.
///
/// For more information about this, refer to Chapter 2.17 'Code Addresses,
/// Ranges, and Base Addresses' (pg. 51) in the DWARF5 specification.
pub fn compilation_unit_size<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    addr_size: u8,
    version: u16,
    rnglists: &gimli::RangeLists<R>,
) -> Result<u64, traits::Error>
where
    R: gimli::Reader,
{
    let base_addr: u64 = die_low_pc_value(die)?.ok_or(traits::Error::with_msg(
        "Compilation unit missing DW_AT_low_pc attribute",
    ))?;

    if let Some(high_pc_attr) = die.attr_value(gimli::DW_AT_high_pc)? {
        match high_pc_attr {
            gimli::AttributeValue::Addr(end_addr) => Ok(end_addr - base_addr),
            gimli::AttributeValue::Udata(offset) => Ok(offset),
            _ => Err(traits::Error::with_msg(
                "Unexpected DW_AT_high_pc attribute value",
            )),
        }
    } else if let Some(ranges_attr) = die.attr_value(gimli::DW_AT_ranges)? {
        match ranges_attr {
            gimli::AttributeValue::RangeListsRef(offset) => {
                let size: u64 = rnglists
                    .ranges(offset, version, addr_size, base_addr)?
                    .map(|r| r.end - r.begin)
                    .fold(0, |res, size| res + size)?;

                Ok(size)
            }
            _ => Err(traits::Error::with_msg("Unexpected DW_AT_ranges value")),
        }
    } else {
        Err(traits::Error::with_msg(
            "Error calculating compilation unit size",
        ))
    }
}

/// Find the value of the `DW_AT_low_pc` for a DIE representing an entity with
/// a contiguous range of machine code addresses. If there is not a
/// `DW_AT_low_pc` value, then the addresses are not contiguous, and
/// `DW_AT_ranges` should be used instead.
fn die_low_pc_value<R>(
    die: &gimli::DebuggingInformationEntry<R, R::Offset>,
) -> Result<Option<u64>, traits::Error>
where
    R: gimli::Reader,
{
    match die.attr_value(gimli::DW_AT_low_pc)? {
        Some(gimli::AttributeValue::Addr(address)) => Ok(Some(address)),
        Some(_) => Err(traits::Error::with_msg("Unexpected DW_AT_low_pc value")),
        None => Ok(None),
    }
}

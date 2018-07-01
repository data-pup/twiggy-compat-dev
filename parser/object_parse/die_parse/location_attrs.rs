use fallible_iterator::FallibleIterator;

use gimli;
use traits;

/// This struct holds the values for DWARF attributes related to an object's
/// location in a binary. This is intended to help consolidate the error
/// checking involved in reading attributes, and simplify the process of
/// size calculations for the entity that a debugging information entry (DIE)
/// describes.
///
/// For more information about these attributes, refer to Chapter 2.17 'Code
/// Addresses, Ranges, and Base Addresses' (pg. 51) in the DWARF5 specification.
pub struct DieLocationAttributes<R: gimli::Reader> {
    dw_at_low_pc: Option<gimli::AttributeValue<R>>,
    dw_at_high_pc: Option<gimli::AttributeValue<R>>,
    dw_at_entry_pc: Option<gimli::AttributeValue<R>>,
    dw_at_ranges: Option<gimli::AttributeValue<R>>,
}

impl<R: gimli::Reader> DieLocationAttributes<R> {
    /// Try to create a new location attributes instance using the given
    /// debugging information entry (DIE). Reading these attributes may fail,
    /// so this will return a Result rather than a plain `Self`.
    /// FIXUP: Is using the TryFrom trait acceptable?
    pub fn try_from(
        die: &gimli::DebuggingInformationEntry<R, R::Offset>,
    ) -> Result<Self, traits::Error> {
        Ok(Self {
            dw_at_low_pc: die.attr_value(gimli::DW_AT_low_pc)?,
            dw_at_high_pc: die.attr_value(gimli::DW_AT_high_pc)?,
            dw_at_entry_pc: die.attr_value(gimli::DW_AT_entry_pc)?,
            dw_at_ranges: die.attr_value(gimli::DW_AT_ranges)?,
        })
    }

    /// Compute the size of a subprogram described by this DIE.
    pub fn entity_size(
        &self,
        addr_size: u8,
        version: u16,
        rnglists: &gimli::RangeLists<R>,
    ) -> Result<Option<u64>, traits::Error> {
        if let Some(base_addr) = self.base_addr()? {
            if let Some(size) = self.contiguous_entity_size(base_addr)? {
                Ok(Some(size))
            } else if let Some(size) =
                self.noncontiguous_entity_size(base_addr, addr_size, version, rnglists)?
            {
                Ok(Some(size))
            } else {
                Ok(None)
            }
        } else {
            // If no base address attribute exists, this DIE does not represent
            // a subroutine definition and a size should not be computed.
            Ok(None)
        }
    }

    /// Return the base address, which will be the value of `DW_AT_low_pc`,
    /// or `DW_AT_entry_pc` if the former attribute does not exist.
    /// FIXUP: Should this refer to the DWARF version for prioritizing which
    /// attribute to consider first?
    fn base_addr(&self) -> Result<Option<u64>, traits::Error> {
        let base_attr = if self.dw_at_low_pc.is_some() {
            self.dw_at_low_pc.as_ref()
        } else if self.dw_at_entry_pc.is_some() {
            self.dw_at_entry_pc.as_ref()
        } else {
            None
        };

        match base_attr {
            Some(gimli::AttributeValue::Addr(address)) => Ok(Some(*address)),
            Some(_) => Err(traits::Error::with_msg(
                "Unexpected base address attribute value",
            )),
            None => Ok(None),
        }
    }

    /// Compute the size of an entity occupying a contiguous range of machine
    /// code addresses in the binary.
    fn contiguous_entity_size(&self, base_addr: u64) -> Result<Option<u64>, traits::Error> {
        if let Some(high_pc_attr) = &self.dw_at_high_pc {
            match high_pc_attr {
                gimli::AttributeValue::Addr(end_addr) => Ok(Some(end_addr - base_addr)),
                gimli::AttributeValue::Udata(offset) => Ok(Some(*offset)),
                _ => Err(traits::Error::with_msg(
                    "Unexpected DW_AT_high_pc attribute value",
                )),
            }
        } else {
            Ok(None)
        }
    }

    /// Compute the size of an entity occupying a series of non-contigous
    /// ranges of machine code addresses in the binary.
    fn noncontiguous_entity_size(
        &self,
        base_addr: u64,
        addr_size: u8,
        version: u16,
        rnglists: &gimli::RangeLists<R>,
    ) -> Result<Option<u64>, traits::Error> {
        if let Some(ranges_attr) = &self.dw_at_ranges {
            match ranges_attr {
                gimli::AttributeValue::RangeListsRef(offset) => {
                    let size: u64 = rnglists
                        .ranges(*offset, version, addr_size, base_addr)?
                        .map(|r| r.end - r.begin)
                        .fold(0, |res, size| res + size)?;
                    Ok(Some(size))
                }
                _ => Err(traits::Error::with_msg("Unexpected DW_AT_ranges value")),
            }
        } else {
            Ok(None)
        }
    }
}

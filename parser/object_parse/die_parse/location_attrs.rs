use gimli;
use traits;

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
        let res = Self {
            dw_at_low_pc: die.attr_value(gimli::DW_AT_low_pc)?,
            dw_at_high_pc: die.attr_value(gimli::DW_AT_high_pc)?,
            dw_at_entry_pc: die.attr_value(gimli::DW_AT_entry_pc)?,
            dw_at_ranges: die.attr_value(gimli::DW_AT_ranges)?,
        };

        Ok(res)
    }

    /// Get the value of the DW_AT_low_pc attribute in the form of a u64.
    pub fn get_low_pc_value(&self) -> Result<Option<u64>, traits::Error> {
        match self.dw_at_low_pc {
            Some(gimli::AttributeValue::Addr(address)) => Ok(Some(address)),
            Some(_) => Err(traits::Error::with_msg("Unexpected DW_AT_low_pc value")),
            None => Ok(None),
        }
    }

    /// FIXUP: ADD COMMENT.
    pub fn get_high_pc(&self) -> Option<&gimli::AttributeValue<R>> {
        self.dw_at_high_pc.as_ref()
    }

    /// FIXUP: ADD COMMENT.
    pub fn get_entry_pc(&self) -> Option<&gimli::AttributeValue<R>> {
        self.dw_at_entry_pc.as_ref()
    }

    /// FIXUP: ADD COMMENT.
    pub fn get_ranges(&self) -> Option<&gimli::AttributeValue<R>> {
        self.dw_at_ranges.as_ref()
    }

    /// Return a boolean value specifying whether or not this DIE represents
    /// the definition of a subroutine. DIEs without any location attributes
    /// represent a declaration.
    fn _is_definition(&self) -> bool {
        self.dw_at_low_pc.is_some() && self.dw_at_entry_pc.is_some()
    }

    /// Return a boolean value specifying whether or not this DIE occupies
    /// a contiguous range of machine code addresses.
    fn _is_contiguous(&self) -> bool {
        self.dw_at_low_pc.is_some() && self.dw_at_high_pc.is_some()
    }

    /// Return the base address, which will be the value of `DW_AT_low_pc`,
    /// or `DW_AT_entry_pc` if the former attribute does not exist.
    fn _base_addr(&self) -> Option<&gimli::AttributeValue<R>> {
        if let Some(dw_at_low_pc) = &self.dw_at_low_pc {
            Some(&dw_at_low_pc)
        } else if let Some(dw_at_entry_pc) = &self.dw_at_entry_pc {
            Some(&dw_at_entry_pc)
        } else {
            None
        }
    }
}
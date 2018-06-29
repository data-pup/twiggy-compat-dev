use gimli;
use traits;

pub struct DieLocationAttributes<R: gimli::Reader> {
    dw_tag: gimli::DwTag,
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
            dw_tag: die.tag(),
            dw_at_low_pc: die.attr_value(gimli::DW_AT_low_pc)?,
            dw_at_high_pc: die.attr_value(gimli::DW_AT_high_pc)?,
            dw_at_entry_pc: die.attr_value(gimli::DW_AT_entry_pc)?,
            dw_at_ranges: die.attr_value(gimli::DW_AT_ranges)?,
        };

        Ok(res)
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

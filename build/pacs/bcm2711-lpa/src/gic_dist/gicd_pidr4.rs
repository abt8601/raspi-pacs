#[doc = "Register `GICD_PIDR4` reader"]
pub type R = crate::R<GICD_PIDR4_SPEC>;
#[doc = "Field `GICD_PIDR4` reader - Peripheral ID 4"]
pub type GICD_PIDR4_R = crate::FieldReader<GICD_PIDR4_A>;
#[doc = "Peripheral ID 4\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR4_A {
    #[doc = "4: Valid"]
    VALID = 4,
    #[doc = "0: Invalid"]
    INVALID = 0,
}
impl From<GICD_PIDR4_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR4_A {
    type Ux = u32;
}
impl GICD_PIDR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_PIDR4_A {
        match self.bits {
            4 => GICD_PIDR4_A::VALID,
            _ => GICD_PIDR4_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR4_A::VALID
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_PIDR4_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 4"]
    #[inline(always)]
    pub fn gicd_pidr4(&self) -> GICD_PIDR4_R {
        GICD_PIDR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR4")
            .field("gicd_pidr4", &format_args!("{}", self.gicd_pidr4().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Peripheral ID 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR4_SPEC;
impl crate::RegisterSpec for GICD_PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr4::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR4_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR4 to value 0x04"]
impl crate::Resettable for GICD_PIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}

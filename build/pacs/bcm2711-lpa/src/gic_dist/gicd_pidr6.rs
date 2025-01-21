#[doc = "Register `GICD_PIDR6` reader"]
pub type R = crate::R<GICD_PIDR6_SPEC>;
#[doc = "Field `GICD_PIDR6` reader - Peripheral ID 6"]
pub type GICD_PIDR6_R = crate::FieldReader<GICD_PIDR6_A>;
#[doc = "Peripheral ID 6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR6_A {
    #[doc = "0: Valid"]
    VALID = 0,
    #[doc = "1: Invalid"]
    INVALID = 1,
}
impl From<GICD_PIDR6_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR6_A {
    type Ux = u32;
}
impl GICD_PIDR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_PIDR6_A {
        match self.bits {
            0 => GICD_PIDR6_A::VALID,
            _ => GICD_PIDR6_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR6_A::VALID
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_PIDR6_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 6"]
    #[inline(always)]
    pub fn gicd_pidr6(&self) -> GICD_PIDR6_R {
        GICD_PIDR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR6")
            .field("gicd_pidr6", &format_args!("{}", self.gicd_pidr6().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Peripheral ID 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR6_SPEC;
impl crate::RegisterSpec for GICD_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr6::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR6_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR6 to value 0"]
impl crate::Resettable for GICD_PIDR6_SPEC {
    const RESET_VALUE: u32 = 0;
}

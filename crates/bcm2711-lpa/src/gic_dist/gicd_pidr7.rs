#[doc = "Register `GICD_PIDR7` reader"]
pub type R = crate::R<GICD_PIDR7_SPEC>;
#[doc = "Field `GICD_PIDR7` reader - Peripheral ID 7"]
pub type GICD_PIDR7_R = crate::FieldReader<GICD_PIDR7_A>;
#[doc = "Peripheral ID 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR7_A {
    #[doc = "0: Valid"]
    VALID = 0,
    #[doc = "1: Invalid"]
    INVALID = 1,
}
impl From<GICD_PIDR7_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR7_A {
    type Ux = u32;
}
impl GICD_PIDR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_PIDR7_A {
        match self.bits {
            0 => GICD_PIDR7_A::VALID,
            _ => GICD_PIDR7_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR7_A::VALID
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_PIDR7_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 7"]
    #[inline(always)]
    pub fn gicd_pidr7(&self) -> GICD_PIDR7_R {
        GICD_PIDR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR7")
            .field("gicd_pidr7", &format_args!("{}", self.gicd_pidr7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Peripheral ID 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR7_SPEC;
impl crate::RegisterSpec for GICD_PIDR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr7::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR7_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR7 to value 0"]
impl crate::Resettable for GICD_PIDR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

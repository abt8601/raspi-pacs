#[doc = "Register `GICD_PIDR5` reader"]
pub type R = crate::R<GICD_PIDR5_SPEC>;
#[doc = "Field `GICD_PIDR5` reader - Peripheral ID 5"]
pub type GICD_PIDR5_R = crate::FieldReader<GICD_PIDR5_A>;
#[doc = "Peripheral ID 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR5_A {
    #[doc = "0: Valid"]
    VALID = 0,
    #[doc = "1: Invalid"]
    INVALID = 1,
}
impl From<GICD_PIDR5_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR5_A {
    type Ux = u32;
}
impl GICD_PIDR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_PIDR5_A {
        match self.bits {
            0 => GICD_PIDR5_A::VALID,
            _ => GICD_PIDR5_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR5_A::VALID
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_PIDR5_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 5"]
    #[inline(always)]
    pub fn gicd_pidr5(&self) -> GICD_PIDR5_R {
        GICD_PIDR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR5")
            .field("gicd_pidr5", &format_args!("{}", self.gicd_pidr5().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Peripheral ID 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR5_SPEC;
impl crate::RegisterSpec for GICD_PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr5::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR5_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR5 to value 0"]
impl crate::Resettable for GICD_PIDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

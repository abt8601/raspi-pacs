#[doc = "Register `GICD_PIDR0` reader"]
pub type R = crate::R<GICD_PIDR0_SPEC>;
#[doc = "Field `GICD_PIDR0` reader - Peripheral ID 0"]
pub type GICD_PIDR0_R = crate::FieldReader<GICD_PIDR0_A>;
#[doc = "Peripheral ID 0\n\nValue on reset: 144"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR0_A {
    #[doc = "144: Valid"]
    VALID = 144,
    #[doc = "0: Invalid"]
    INVALID = 0,
}
impl From<GICD_PIDR0_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR0_A {
    type Ux = u32;
}
impl GICD_PIDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_PIDR0_A {
        match self.bits {
            144 => GICD_PIDR0_A::VALID,
            _ => GICD_PIDR0_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR0_A::VALID
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_PIDR0_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 0"]
    #[inline(always)]
    pub fn gicd_pidr0(&self) -> GICD_PIDR0_R {
        GICD_PIDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR0")
            .field("gicd_pidr0", &format_args!("{}", self.gicd_pidr0().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Peripheral ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR0_SPEC;
impl crate::RegisterSpec for GICD_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr0::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR0_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GICD_PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x90;
}

#[doc = "Register `GICD_CIDR0` reader"]
pub type R = crate::R<GICD_CIDR0_SPEC>;
#[doc = "Field `GICD_CIDR0` reader - Component ID 0"]
pub type GICD_CIDR0_R = crate::FieldReader<GICD_CIDR0_A>;
#[doc = "Component ID 0\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR0_A {
    #[doc = "13: Valid"]
    VALID = 13,
    #[doc = "0: Invalid ID"]
    INVALID = 0,
}
impl From<GICD_CIDR0_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_CIDR0_A {
    type Ux = u32;
}
impl GICD_CIDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_CIDR0_A {
        match self.bits {
            13 => GICD_CIDR0_A::VALID,
            _ => GICD_CIDR0_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR0_A::VALID
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_CIDR0_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 0"]
    #[inline(always)]
    pub fn gicd_cidr0(&self) -> GICD_CIDR0_R {
        GICD_CIDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR0")
            .field("gicd_cidr0", &format_args!("{}", self.gicd_cidr0().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_CIDR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Component ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR0_SPEC;
impl crate::RegisterSpec for GICD_CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr0::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR0_SPEC {}
#[doc = "`reset()` method sets GICD_CIDR0 to value 0x0d"]
impl crate::Resettable for GICD_CIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}

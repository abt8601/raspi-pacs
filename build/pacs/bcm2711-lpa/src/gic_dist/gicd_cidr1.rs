#[doc = "Register `GICD_CIDR1` reader"]
pub type R = crate::R<GICD_CIDR1_SPEC>;
#[doc = "Field `GICD_CIDR1` reader - Component ID 1"]
pub type GICD_CIDR1_R = crate::FieldReader<GICD_CIDR1_A>;
#[doc = "Component ID 1\n\nValue on reset: 240"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR1_A {
    #[doc = "240: Valid"]
    VALID = 240,
    #[doc = "0: Invalid ID"]
    INVALID = 0,
}
impl From<GICD_CIDR1_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_CIDR1_A {
    type Ux = u32;
}
impl GICD_CIDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_CIDR1_A {
        match self.bits {
            240 => GICD_CIDR1_A::VALID,
            _ => GICD_CIDR1_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR1_A::VALID
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_CIDR1_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 1"]
    #[inline(always)]
    pub fn gicd_cidr1(&self) -> GICD_CIDR1_R {
        GICD_CIDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR1")
            .field("gicd_cidr1", &format_args!("{}", self.gicd_cidr1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_CIDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Component ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR1_SPEC;
impl crate::RegisterSpec for GICD_CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr1::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR1_SPEC {}
#[doc = "`reset()` method sets GICD_CIDR1 to value 0xf0"]
impl crate::Resettable for GICD_CIDR1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}

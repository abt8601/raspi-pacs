#[doc = "Register `GICD_CIDR0` reader"]
pub struct R(crate::R<GICD_CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_CIDR0` reader - Component ID 0"]
pub type GICD_CIDR0_R = crate::FieldReader<u32, GICD_CIDR0_A>;
#[doc = "Component ID 0\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR0_A {
    #[doc = "13: Valid"]
    VALID = 13,
}
impl From<GICD_CIDR0_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR0_A) -> Self {
        variant as _
    }
}
impl GICD_CIDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_CIDR0_A> {
        match self.bits {
            13 => Some(GICD_CIDR0_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR0_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 0"]
    #[inline(always)]
    pub fn gicd_cidr0(&self) -> GICD_CIDR0_R {
        GICD_CIDR0_R::new(self.bits)
    }
}
#[doc = "Component ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr0](index.html) module"]
pub struct GICD_CIDR0_SPEC;
impl crate::RegisterSpec for GICD_CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr0::R](R) reader structure"]
impl crate::Readable for GICD_CIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR0 to value 0x0d"]
impl crate::Resettable for GICD_CIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}

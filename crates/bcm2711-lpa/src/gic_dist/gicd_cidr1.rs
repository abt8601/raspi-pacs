#[doc = "Register `GICD_CIDR1` reader"]
pub struct R(crate::R<GICD_CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_CIDR1` reader - Component ID 1"]
pub type GICD_CIDR1_R = crate::FieldReader<u32, GICD_CIDR1_A>;
#[doc = "Component ID 1\n\nValue on reset: 240"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR1_A {
    #[doc = "240: Valid"]
    VALID = 240,
}
impl From<GICD_CIDR1_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR1_A) -> Self {
        variant as _
    }
}
impl GICD_CIDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_CIDR1_A> {
        match self.bits {
            240 => Some(GICD_CIDR1_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR1_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 1"]
    #[inline(always)]
    pub fn gicd_cidr1(&self) -> GICD_CIDR1_R {
        GICD_CIDR1_R::new(self.bits)
    }
}
#[doc = "Component ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr1](index.html) module"]
pub struct GICD_CIDR1_SPEC;
impl crate::RegisterSpec for GICD_CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr1::R](R) reader structure"]
impl crate::Readable for GICD_CIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR1 to value 0xf0"]
impl crate::Resettable for GICD_CIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}

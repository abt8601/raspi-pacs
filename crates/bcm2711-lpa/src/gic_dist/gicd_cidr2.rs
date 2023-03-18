#[doc = "Register `GICD_CIDR2` reader"]
pub struct R(crate::R<GICD_CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_CIDR2` reader - Component ID 2"]
pub type GICD_CIDR2_R = crate::FieldReader<u32, GICD_CIDR2_A>;
#[doc = "Component ID 2\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR2_A {
    #[doc = "5: Valid"]
    VALID = 5,
}
impl From<GICD_CIDR2_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR2_A) -> Self {
        variant as _
    }
}
impl GICD_CIDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_CIDR2_A> {
        match self.bits {
            5 => Some(GICD_CIDR2_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR2_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 2"]
    #[inline(always)]
    pub fn gicd_cidr2(&self) -> GICD_CIDR2_R {
        GICD_CIDR2_R::new(self.bits)
    }
}
#[doc = "Component ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr2](index.html) module"]
pub struct GICD_CIDR2_SPEC;
impl crate::RegisterSpec for GICD_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr2::R](R) reader structure"]
impl crate::Readable for GICD_CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR2 to value 0x05"]
impl crate::Resettable for GICD_CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}

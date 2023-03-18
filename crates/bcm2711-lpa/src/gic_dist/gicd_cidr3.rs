#[doc = "Register `GICD_CIDR3` reader"]
pub struct R(crate::R<GICD_CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_CIDR3` reader - Component ID 3"]
pub type GICD_CIDR3_R = crate::FieldReader<u32, GICD_CIDR3_A>;
#[doc = "Component ID 3\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR3_A {
    #[doc = "177: Valid"]
    VALID = 177,
}
impl From<GICD_CIDR3_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR3_A) -> Self {
        variant as _
    }
}
impl GICD_CIDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_CIDR3_A> {
        match self.bits {
            177 => Some(GICD_CIDR3_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR3_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 3"]
    #[inline(always)]
    pub fn gicd_cidr3(&self) -> GICD_CIDR3_R {
        GICD_CIDR3_R::new(self.bits)
    }
}
#[doc = "Component ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr3](index.html) module"]
pub struct GICD_CIDR3_SPEC;
impl crate::RegisterSpec for GICD_CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr3::R](R) reader structure"]
impl crate::Readable for GICD_CIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR3 to value 0xb1"]
impl crate::Resettable for GICD_CIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}

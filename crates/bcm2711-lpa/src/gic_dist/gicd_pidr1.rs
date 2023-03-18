#[doc = "Register `GICD_PIDR1` reader"]
pub struct R(crate::R<GICD_PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR1` reader - Peripheral ID 1"]
pub type GICD_PIDR1_R = crate::FieldReader<u32, GICD_PIDR1_A>;
#[doc = "Peripheral ID 1\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR1_A {
    #[doc = "180: Valid"]
    VALID = 180,
}
impl From<GICD_PIDR1_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR1_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR1_A> {
        match self.bits {
            180 => Some(GICD_PIDR1_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR1_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 1"]
    #[inline(always)]
    pub fn gicd_pidr1(&self) -> GICD_PIDR1_R {
        GICD_PIDR1_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr1](index.html) module"]
pub struct GICD_PIDR1_SPEC;
impl crate::RegisterSpec for GICD_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr1::R](R) reader structure"]
impl crate::Readable for GICD_PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR1 to value 0xb4"]
impl crate::Resettable for GICD_PIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}

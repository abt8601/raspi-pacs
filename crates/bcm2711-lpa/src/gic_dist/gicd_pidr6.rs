#[doc = "Register `GICD_PIDR6` reader"]
pub struct R(crate::R<GICD_PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR6` reader - Peripheral ID 6"]
pub type GICD_PIDR6_R = crate::FieldReader<u32, GICD_PIDR6_A>;
#[doc = "Peripheral ID 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR6_A {
    #[doc = "0: Valid"]
    VALID = 0,
}
impl From<GICD_PIDR6_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR6_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR6_A> {
        match self.bits {
            0 => Some(GICD_PIDR6_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR6_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 6"]
    #[inline(always)]
    pub fn gicd_pidr6(&self) -> GICD_PIDR6_R {
        GICD_PIDR6_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr6](index.html) module"]
pub struct GICD_PIDR6_SPEC;
impl crate::RegisterSpec for GICD_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr6::R](R) reader structure"]
impl crate::Readable for GICD_PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR6 to value 0"]
impl crate::Resettable for GICD_PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_PIDR5` reader"]
pub struct R(crate::R<GICD_PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR5` reader - Peripheral ID 5"]
pub type GICD_PIDR5_R = crate::FieldReader<u32, GICD_PIDR5_A>;
#[doc = "Peripheral ID 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR5_A {
    #[doc = "0: Valid"]
    VALID = 0,
}
impl From<GICD_PIDR5_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR5_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR5_A> {
        match self.bits {
            0 => Some(GICD_PIDR5_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR5_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 5"]
    #[inline(always)]
    pub fn gicd_pidr5(&self) -> GICD_PIDR5_R {
        GICD_PIDR5_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr5](index.html) module"]
pub struct GICD_PIDR5_SPEC;
impl crate::RegisterSpec for GICD_PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr5::R](R) reader structure"]
impl crate::Readable for GICD_PIDR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR5 to value 0"]
impl crate::Resettable for GICD_PIDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

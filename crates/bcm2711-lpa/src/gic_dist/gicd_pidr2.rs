#[doc = "Register `GICD_PIDR2` reader"]
pub struct R(crate::R<GICD_PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR2` reader - Peripheral ID 2"]
pub type GICD_PIDR2_R = crate::FieldReader<u32, GICD_PIDR2_A>;
#[doc = "Peripheral ID 2\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR2_A {
    #[doc = "43: Valid"]
    VALID = 43,
}
impl From<GICD_PIDR2_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR2_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR2_A> {
        match self.bits {
            43 => Some(GICD_PIDR2_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR2_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 2"]
    #[inline(always)]
    pub fn gicd_pidr2(&self) -> GICD_PIDR2_R {
        GICD_PIDR2_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr2](index.html) module"]
pub struct GICD_PIDR2_SPEC;
impl crate::RegisterSpec for GICD_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr2::R](R) reader structure"]
impl crate::Readable for GICD_PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR2 to value 0x2b"]
impl crate::Resettable for GICD_PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b;
}

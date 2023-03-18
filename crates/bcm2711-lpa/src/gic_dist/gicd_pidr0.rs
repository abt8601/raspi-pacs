#[doc = "Register `GICD_PIDR0` reader"]
pub struct R(crate::R<GICD_PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR0` reader - Peripheral ID 0"]
pub type GICD_PIDR0_R = crate::FieldReader<u32, GICD_PIDR0_A>;
#[doc = "Peripheral ID 0\n\nValue on reset: 144"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR0_A {
    #[doc = "144: Valid"]
    VALID = 144,
}
impl From<GICD_PIDR0_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR0_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR0_A> {
        match self.bits {
            144 => Some(GICD_PIDR0_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR0_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 0"]
    #[inline(always)]
    pub fn gicd_pidr0(&self) -> GICD_PIDR0_R {
        GICD_PIDR0_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr0](index.html) module"]
pub struct GICD_PIDR0_SPEC;
impl crate::RegisterSpec for GICD_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr0::R](R) reader structure"]
impl crate::Readable for GICD_PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GICD_PIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x90;
}

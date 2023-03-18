#[doc = "Register `GICD_PIDR4` reader"]
pub struct R(crate::R<GICD_PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GICD_PIDR4` reader - Peripheral ID 4"]
pub type GICD_PIDR4_R = crate::FieldReader<u32, GICD_PIDR4_A>;
#[doc = "Peripheral ID 4\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR4_A {
    #[doc = "4: Valid"]
    VALID = 4,
}
impl From<GICD_PIDR4_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR4_A) -> Self {
        variant as _
    }
}
impl GICD_PIDR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GICD_PIDR4_A> {
        match self.bits {
            4 => Some(GICD_PIDR4_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR4_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 4"]
    #[inline(always)]
    pub fn gicd_pidr4(&self) -> GICD_PIDR4_R {
        GICD_PIDR4_R::new(self.bits)
    }
}
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr4](index.html) module"]
pub struct GICD_PIDR4_SPEC;
impl crate::RegisterSpec for GICD_PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr4::R](R) reader structure"]
impl crate::Readable for GICD_PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR4 to value 0x04"]
impl crate::Resettable for GICD_PIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

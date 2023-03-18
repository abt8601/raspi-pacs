#[doc = "Register `GICD_IIDR` reader"]
pub struct R(crate::R<GICD_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMPLEMENTER` reader - Implementer"]
pub type IMPLEMENTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REVISION` reader - Revision"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VARIANT` reader - Variant"]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRODUCT_ID` reader - Product ID"]
pub type PRODUCT_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Implementer"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Variant"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Product ID"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Distributor Implementer Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_iidr](index.html) module"]
pub struct GICD_IIDR_SPEC;
impl crate::RegisterSpec for GICD_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_iidr::R](R) reader structure"]
impl crate::Readable for GICD_IIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_IIDR to value 0x0200_143b"]
impl crate::Resettable for GICD_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_143b;
}

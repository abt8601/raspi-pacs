#[doc = "Register `GICD_IIDR` reader"]
pub type R = crate::R<GICD_IIDR_SPEC>;
#[doc = "Field `IMPLEMENTER` reader - Implementer"]
pub type IMPLEMENTER_R = crate::FieldReader<u16>;
#[doc = "Field `REVISION` reader - Revision"]
pub type REVISION_R = crate::FieldReader;
#[doc = "Field `VARIANT` reader - Variant"]
pub type VARIANT_R = crate::FieldReader;
#[doc = "Field `PRODUCT_ID` reader - Product ID"]
pub type PRODUCT_ID_R = crate::FieldReader;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IIDR")
            .field("product_id", &format_args!("{}", self.product_id().bits()))
            .field("variant", &format_args!("{}", self.variant().bits()))
            .field("revision", &format_args!("{}", self.revision().bits()))
            .field(
                "implementer",
                &format_args!("{}", self.implementer().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IIDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Distributor Implementer Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_iidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IIDR_SPEC;
impl crate::RegisterSpec for GICD_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_iidr::R`](R) reader structure"]
impl crate::Readable for GICD_IIDR_SPEC {}
#[doc = "`reset()` method sets GICD_IIDR to value 0x0200_143b"]
impl crate::Resettable for GICD_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_143b;
}

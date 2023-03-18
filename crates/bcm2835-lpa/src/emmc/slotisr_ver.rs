#[doc = "Register `SLOTISR_VER` reader"]
pub struct R(crate::R<SLOTISR_VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOTISR_VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOTISR_VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOTISR_VER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOTISR_VER` writer"]
pub struct W(crate::W<SLOTISR_VER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOTISR_VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SLOTISR_VER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOTISR_VER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_STATUS` reader - OR of interrupt and wakeup signals for each slot"]
pub type SLOT_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOT_STATUS` writer - OR of interrupt and wakeup signals for each slot"]
pub type SLOT_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLOTISR_VER_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDVERSION` reader - Host controller specification version"]
pub type SDVERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDVERSION` writer - Host controller specification version"]
pub type SDVERSION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTISR_VER_SPEC, u8, u8, 8, O>;
#[doc = "Field `VENDOR` reader - Vendor version number"]
pub type VENDOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VENDOR` writer - Vendor version number"]
pub type VENDOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTISR_VER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    pub fn slot_status(&self) -> SLOT_STATUS_R {
        SLOT_STATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    pub fn sdversion(&self) -> SDVERSION_R {
        SDVERSION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    pub fn vendor(&self) -> VENDOR_R {
        VENDOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot_status(&mut self) -> SLOT_STATUS_W<0> {
        SLOT_STATUS_W::new(self)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    #[must_use]
    pub fn sdversion(&mut self) -> SDVERSION_W<16> {
        SDVERSION_W::new(self)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    #[must_use]
    pub fn vendor(&mut self) -> VENDOR_W<24> {
        VENDOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Version information and slot interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slotisr_ver](index.html) module"]
pub struct SLOTISR_VER_SPEC;
impl crate::RegisterSpec for SLOTISR_VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slotisr_ver::R](R) reader structure"]
impl crate::Readable for SLOTISR_VER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slotisr_ver::W](W) writer structure"]
impl crate::Writable for SLOTISR_VER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOTISR_VER to value 0"]
impl crate::Resettable for SLOTISR_VER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_ITARGETSR26` reader"]
pub struct R(crate::R<GICD_ITARGETSR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR26` writer"]
pub struct W(crate::W<GICD_ITARGETSR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR26_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISP` reader - ISP"]
pub type ISP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISP` writer - ISP"]
pub type ISP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB` writer - USB"]
pub type USB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `V3D` writer - V3D"]
pub type V3D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR26_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TRANSPOSER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TRANSPOSER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR26_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> ISP_R {
        ISP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TRANSPOSER_R {
        TRANSPOSER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> ISP_W<0> {
        ISP_W::new(self)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<8> {
        USB_W::new(self)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3D_W<16> {
        V3D_W::new(self)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TRANSPOSER_W<24> {
        TRANSPOSER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 104 - 107\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr26](index.html) module"]
pub struct GICD_ITARGETSR26_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr26::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr26::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR26_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR26 to value 0"]
impl crate::Resettable for GICD_ITARGETSR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_ITARGETSR25` reader"]
pub struct R(crate::R<GICD_ITARGETSR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR25` writer"]
pub struct W(crate::W<GICD_ITARGETSR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR25_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR25_SPEC, u8, u8, 8, O>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR25_SPEC, u8, u8, 8, O>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR25_SPEC, u8, u8, 8, O>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR25_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0_R {
        H264_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1_R {
        H264_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2_R {
        H264_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0_W<0> {
        H264_0_W::new(self)
    }
    #[doc = "Bits 8:15 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1_W<8> {
        H264_1_W::new(self)
    }
    #[doc = "Bits 16:23 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2_W<16> {
        H264_2_W::new(self)
    }
    #[doc = "Bits 24:31 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<24> {
        JPEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 100 - 103\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr25](index.html) module"]
pub struct GICD_ITARGETSR25_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr25::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr25::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR25_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR25 to value 0"]
impl crate::Resettable for GICD_ITARGETSR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

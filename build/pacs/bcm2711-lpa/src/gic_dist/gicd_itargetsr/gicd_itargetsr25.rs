#[doc = "Register `GICD_ITARGETSR25` reader"]
pub type R = crate::R<GICD_ITARGETSR25_SPEC>;
#[doc = "Register `GICD_ITARGETSR25` writer"]
pub type W = crate::W<GICD_ITARGETSR25_SPEC>;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0_R = crate::FieldReader;
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1_R = crate::FieldReader;
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2_R = crate::FieldReader;
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::FieldReader;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR25")
            .field("h264_0", &format_args!("{}", self.h264_0().bits()))
            .field("h264_1", &format_args!("{}", self.h264_1().bits()))
            .field("h264_2", &format_args!("{}", self.h264_2().bits()))
            .field("jpeg", &format_args!("{}", self.jpeg().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR25_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0_W<GICD_ITARGETSR25_SPEC> {
        H264_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1_W<GICD_ITARGETSR25_SPEC> {
        H264_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2_W<GICD_ITARGETSR25_SPEC> {
        H264_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<GICD_ITARGETSR25_SPEC> {
        JPEG_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Processor Target 100 - 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR25_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr25::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR25_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr25::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR25_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR25 to value 0"]
impl crate::Resettable for GICD_ITARGETSR25_SPEC {
    const RESET_VALUE: u32 = 0;
}

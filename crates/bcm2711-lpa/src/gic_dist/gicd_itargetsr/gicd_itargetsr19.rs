#[doc = "Register `GICD_ITARGETSR19` reader"]
pub struct R(crate::R<GICD_ITARGETSR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR19` writer"]
pub struct W(crate::W<GICD_ITARGETSR19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR19_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR19_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR19_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR19_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR19_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<0> {
        SWI4_W::new(self)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<8> {
        SWI5_W::new(self)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<16> {
        SWI6_W::new(self)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<24> {
        SWI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 76 - 79\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr19](index.html) module"]
pub struct GICD_ITARGETSR19_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr19::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr19::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR19 to value 0"]
impl crate::Resettable for GICD_ITARGETSR19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

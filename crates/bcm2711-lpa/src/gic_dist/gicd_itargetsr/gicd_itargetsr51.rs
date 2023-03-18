#[doc = "Register `GICD_ITARGETSR51` reader"]
pub struct R(crate::R<GICD_ITARGETSR51_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR51_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR51_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR51_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR51` writer"]
pub struct W(crate::W<GICD_ITARGETSR51_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR51_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR51_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR51_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type INT204_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type INT204_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR51_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type INT205_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type INT205_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR51_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type INT206_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type INT206_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR51_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type INT207_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type INT207_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR51_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> INT204_R {
        INT204_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> INT205_R {
        INT205_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> INT206_R {
        INT206_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> INT207_R {
        INT207_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> INT204_W<0> {
        INT204_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> INT205_W<8> {
        INT205_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> INT206_W<16> {
        INT206_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> INT207_W<24> {
        INT207_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 204 - 207\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr51](index.html) module"]
pub struct GICD_ITARGETSR51_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr51::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR51_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr51::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR51_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR51 to value 0"]
impl crate::Resettable for GICD_ITARGETSR51_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

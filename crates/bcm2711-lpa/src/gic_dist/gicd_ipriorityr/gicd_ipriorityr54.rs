#[doc = "Register `GICD_IPRIORITYR54` reader"]
pub struct R(crate::R<GICD_IPRIORITYR54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR54` writer"]
pub struct W(crate::W<GICD_IPRIORITYR54_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR54_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR54_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR54_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type INT216_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type INT216_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR54_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type INT217_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type INT217_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR54_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type INT218_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type INT218_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR54_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type INT219_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type INT219_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR54_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> INT216_R {
        INT216_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> INT217_R {
        INT217_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> INT218_R {
        INT218_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> INT219_R {
        INT219_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> INT216_W<0> {
        INT216_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> INT217_W<8> {
        INT217_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> INT218_W<16> {
        INT218_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> INT219_W<24> {
        INT219_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 216 - 219 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr54](index.html) module"]
pub struct GICD_IPRIORITYR54_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR54_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr54::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR54_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr54::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR54_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR54 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR54_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

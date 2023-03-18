#[doc = "Register `DC` reader"]
pub struct R(crate::R<DC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DC` writer"]
pub struct W(crate::W<DC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DC_SPEC>;
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
impl From<crate::W<DC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDREQ` reader - DMA Write request threshold"]
pub type TDREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDREQ` writer - DMA Write request threshold"]
pub type TDREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_SPEC, u8, u8, 8, O>;
#[doc = "Field `TPANIC` reader - DMA write panic threshold"]
pub type TPANIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPANIC` writer - DMA write panic threshold"]
pub type TPANIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDREQ` reader - DMA read request threshold"]
pub type RDREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDREQ` writer - DMA read request threshold"]
pub type RDREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_SPEC, u8, u8, 8, O>;
#[doc = "Field `RPANIC` reader - DMA read panic threshold"]
pub type RPANIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPANIC` writer - DMA read panic threshold"]
pub type RPANIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    pub fn tdreq(&self) -> TDREQ_R {
        TDREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    pub fn tpanic(&self) -> TPANIC_R {
        TPANIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    pub fn rdreq(&self) -> RDREQ_R {
        RDREQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    pub fn rpanic(&self) -> RPANIC_R {
        RPANIC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tdreq(&mut self) -> TDREQ_W<0> {
        TDREQ_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tpanic(&mut self) -> TPANIC_W<8> {
        TPANIC_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rdreq(&mut self) -> RDREQ_W<16> {
        RDREQ_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rpanic(&mut self) -> RPANIC_W<24> {
        RPANIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc](index.html) module"]
pub struct DC_SPEC;
impl crate::RegisterSpec for DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc::R](R) reader structure"]
impl crate::Readable for DC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dc::W](W) writer structure"]
impl crate::Writable for DC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC to value 0x3020_1050"]
impl crate::Resettable for DC_SPEC {
    const RESET_VALUE: Self::Ux = 0x3020_1050;
}

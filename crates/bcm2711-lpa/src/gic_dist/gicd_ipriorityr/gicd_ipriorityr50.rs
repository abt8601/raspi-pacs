#[doc = "Register `GICD_IPRIORITYR50` reader"]
pub struct R(crate::R<GICD_IPRIORITYR50_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR50_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR50_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR50_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR50` writer"]
pub struct W(crate::W<GICD_IPRIORITYR50_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR50_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR50_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR50_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type INT200_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type INT200_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR50_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type INT201_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type INT201_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR50_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type INT202_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type INT202_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR50_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type INT203_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type INT203_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR50_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> INT200_R {
        INT200_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> INT201_R {
        INT201_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> INT202_R {
        INT202_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> INT203_R {
        INT203_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> INT200_W<0> {
        INT200_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> INT201_W<8> {
        INT201_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> INT202_W<16> {
        INT202_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> INT203_W<24> {
        INT203_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 200 - 203 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr50](index.html) module"]
pub struct GICD_IPRIORITYR50_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR50_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr50::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR50_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr50::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR50_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR50 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR50_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

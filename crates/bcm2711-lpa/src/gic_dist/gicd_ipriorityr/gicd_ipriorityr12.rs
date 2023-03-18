#[doc = "Register `GICD_IPRIORITYR12` reader"]
pub struct R(crate::R<GICD_IPRIORITYR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR12` writer"]
pub struct W(crate::W<GICD_IPRIORITYR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR12_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type INT48_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT48` writer - Interrupt 48"]
pub type INT48_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR12_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type INT49_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT49` writer - Interrupt 49"]
pub type INT49_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR12_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type INT50_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT50` writer - Interrupt 50"]
pub type INT50_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR12_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type INT51_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT51` writer - Interrupt 51"]
pub type INT51_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR12_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> INT48_R {
        INT48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> INT49_R {
        INT49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> INT50_R {
        INT50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> INT51_R {
        INT51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn int48(&mut self) -> INT48_W<0> {
        INT48_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn int49(&mut self) -> INT49_W<8> {
        INT49_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn int50(&mut self) -> INT50_W<16> {
        INT50_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn int51(&mut self) -> INT51_W<24> {
        INT51_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 48 - 51 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr12](index.html) module"]
pub struct GICD_IPRIORITYR12_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr12::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr12::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR12 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

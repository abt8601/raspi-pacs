#[doc = "Register `GICD_IPRIORITYR0` reader"]
pub struct R(crate::R<GICD_IPRIORITYR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR0` writer"]
pub struct W(crate::W<GICD_IPRIORITYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR0_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type INT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT0` writer - Interrupt 0"]
pub type INT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type INT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT1` writer - Interrupt 1"]
pub type INT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type INT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT2` writer - Interrupt 2"]
pub type INT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type INT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT3` writer - Interrupt 3"]
pub type INT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<0> {
        INT0_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<8> {
        INT1_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<16> {
        INT2_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<24> {
        INT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 0 - 3 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr0](index.html) module"]
pub struct GICD_IPRIORITYR0_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr0::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr0::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR0 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

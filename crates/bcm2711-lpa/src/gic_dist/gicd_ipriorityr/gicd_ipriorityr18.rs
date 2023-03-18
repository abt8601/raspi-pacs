#[doc = "Register `GICD_IPRIORITYR18` reader"]
pub struct R(crate::R<GICD_IPRIORITYR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR18` writer"]
pub struct W(crate::W<GICD_IPRIORITYR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR18_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR18_SPEC, u8, u8, 8, O>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR18_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<8> {
        SWI1_W::new(self)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<16> {
        SWI2_W::new(self)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<24> {
        SWI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 72 - 75 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr18](index.html) module"]
pub struct GICD_IPRIORITYR18_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr18::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr18::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR18 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `LTOH` reader"]
pub struct R(crate::R<LTOH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTOH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTOH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTOH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTOH` writer"]
pub struct W(crate::W<LTOH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTOH_SPEC>;
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
impl From<crate::W<LTOH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTOH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOH` reader - Output hold delay"]
pub type TOH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOH` writer - Output hold delay"]
pub type TOH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTOH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    pub fn toh(&self) -> TOH_R {
        TOH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    #[must_use]
    pub fn toh(&mut self) -> TOH_W<0> {
        TOH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LoSSI output hold delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltoh](index.html) module"]
pub struct LTOH_SPEC;
impl crate::RegisterSpec for LTOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltoh::R](R) reader structure"]
impl crate::Readable for LTOH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltoh::W](W) writer structure"]
impl crate::Writable for LTOH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTOH to value 0x01"]
impl crate::Resettable for LTOH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

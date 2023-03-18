#[doc = "Register `CLKT` reader"]
pub struct R(crate::R<CLKT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKT` writer"]
pub struct W(crate::W<CLKT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKT_SPEC>;
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
impl From<crate::W<CLKT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUT` reader - Number of SCL clock cycles to wait"]
pub type TOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUT` writer - Number of SCL clock cycles to wait"]
pub type TOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<0> {
        TOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock stretch timeout (broken on 283x)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkt](index.html) module"]
pub struct CLKT_SPEC;
impl crate::RegisterSpec for CLKT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkt::R](R) reader structure"]
impl crate::Readable for CLKT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkt::W](W) writer structure"]
impl crate::Writable for CLKT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKT to value 0"]
impl crate::Resettable for CLKT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `EXRDFIFO_CFG` reader"]
pub struct R(crate::R<EXRDFIFO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXRDFIFO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXRDFIFO_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXRDFIFO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXRDFIFO_CFG` writer"]
pub struct W(crate::W<EXRDFIFO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXRDFIFO_CFG_SPEC>;
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
impl From<crate::W<EXRDFIFO_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXRDFIFO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_THRSH` reader - Read threshold in 32 bit words"]
pub type RD_THRSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_THRSH` writer - Read threshold in 32 bit words"]
pub type RD_THRSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXRDFIFO_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    pub fn rd_thrsh(&self) -> RD_THRSH_R {
        RD_THRSH_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    #[must_use]
    pub fn rd_thrsh(&mut self) -> RD_THRSH_W<0> {
        RD_THRSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fine tune DMA request generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exrdfifo_cfg](index.html) module"]
pub struct EXRDFIFO_CFG_SPEC;
impl crate::RegisterSpec for EXRDFIFO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exrdfifo_cfg::R](R) reader structure"]
impl crate::Readable for EXRDFIFO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exrdfifo_cfg::W](W) writer structure"]
impl crate::Writable for EXRDFIFO_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_CFG to value 0"]
impl crate::Resettable for EXRDFIFO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

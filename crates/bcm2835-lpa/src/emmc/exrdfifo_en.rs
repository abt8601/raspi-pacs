#[doc = "Register `EXRDFIFO_EN` reader"]
pub struct R(crate::R<EXRDFIFO_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXRDFIFO_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXRDFIFO_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXRDFIFO_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXRDFIFO_EN` writer"]
pub struct W(crate::W<EXRDFIFO_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXRDFIFO_EN_SPEC>;
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
impl From<crate::W<EXRDFIFO_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXRDFIFO_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable the extension FIFO"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable the extension FIFO"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXRDFIFO_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable the extension data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exrdfifo_en](index.html) module"]
pub struct EXRDFIFO_EN_SPEC;
impl crate::RegisterSpec for EXRDFIFO_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exrdfifo_en::R](R) reader structure"]
impl crate::Readable for EXRDFIFO_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exrdfifo_en::W](W) writer structure"]
impl crate::Writable for EXRDFIFO_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_EN to value 0"]
impl crate::Resettable for EXRDFIFO_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

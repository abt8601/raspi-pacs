#[doc = "Register `SPI_INT_SPT` reader"]
pub struct R(crate::R<SPI_INT_SPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_INT_SPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_INT_SPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_INT_SPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_INT_SPT` writer"]
pub struct W(crate::W<SPI_INT_SPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_INT_SPT_SPEC>;
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
impl From<crate::W<SPI_INT_SPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_INT_SPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECT` reader - "]
pub type SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT` writer - "]
pub type SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_INT_SPT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupts in SPI mode depend on CS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_int_spt](index.html) module"]
pub struct SPI_INT_SPT_SPEC;
impl crate::RegisterSpec for SPI_INT_SPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_int_spt::R](R) reader structure"]
impl crate::Readable for SPI_INT_SPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_int_spt::W](W) writer structure"]
impl crate::Writable for SPI_INT_SPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_INT_SPT to value 0"]
impl crate::Resettable for SPI_INT_SPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

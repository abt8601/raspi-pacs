#[doc = "Register `ENABLES` reader"]
pub struct R(crate::R<ENABLES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLES` writer"]
pub struct W(crate::W<ENABLES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLES_SPEC>;
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
impl From<crate::W<ENABLES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_1` reader - UART1 enabled"]
pub type UART_1_R = crate::BitReader<bool>;
#[doc = "Field `UART_1` writer - UART1 enabled"]
pub type UART_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLES_SPEC, bool, O>;
#[doc = "Field `SPI_1` reader - SPI1 enabled"]
pub type SPI_1_R = crate::BitReader<bool>;
#[doc = "Field `SPI_1` writer - SPI1 enabled"]
pub type SPI_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLES_SPEC, bool, O>;
#[doc = "Field `SPI_2` reader - SPI2 enabled"]
pub type SPI_2_R = crate::BitReader<bool>;
#[doc = "Field `SPI_2` writer - SPI2 enabled"]
pub type SPI_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLES_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART1 enabled"]
    #[inline(always)]
    pub fn uart_1(&self) -> UART_1_R {
        UART_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 enabled"]
    #[inline(always)]
    pub fn spi_1(&self) -> SPI_1_R {
        SPI_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI2 enabled"]
    #[inline(always)]
    pub fn spi_2(&self) -> SPI_2_R {
        SPI_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART1 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn uart_1(&mut self) -> UART_1_W<0> {
        UART_1_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> SPI_1_W<1> {
        SPI_1_W::new(self)
    }
    #[doc = "Bit 2 - SPI2 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> SPI_2_W<2> {
        SPI_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable sub-peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enables](index.html) module"]
pub struct ENABLES_SPEC;
impl crate::RegisterSpec for ENABLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enables::R](R) reader structure"]
impl crate::Readable for ENABLES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enables::W](W) writer structure"]
impl crate::Writable for ENABLES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLES to value 0"]
impl crate::Resettable for ENABLES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

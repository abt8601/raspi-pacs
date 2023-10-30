#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IRQ_SPEC>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IRQ_SPEC>;
#[doc = "Field `UART_1` reader - UART1 interrupt active"]
pub type UART_1_R = crate::BitReader;
#[doc = "Field `UART_1` writer - UART1 interrupt active"]
pub type UART_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type SPI_1_R = crate::BitReader;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type SPI_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type SPI_2_R = crate::BitReader;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type SPI_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    pub fn uart_1(&self) -> UART_1_R {
        UART_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(&self) -> SPI_1_R {
        SPI_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(&self) -> SPI_2_R {
        SPI_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("spi_2", &format_args!("{}", self.spi_2().bit()))
            .field("spi_1", &format_args!("{}", self.spi_1().bit()))
            .field("uart_1", &format_args!("{}", self.uart_1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IRQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_1(&mut self) -> UART_1_W<IRQ_SPEC, 0> {
        UART_1_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> SPI_1_W<IRQ_SPEC, 1> {
        SPI_1_W::new(self)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> SPI_2_W<IRQ_SPEC, 2> {
        SPI_2_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

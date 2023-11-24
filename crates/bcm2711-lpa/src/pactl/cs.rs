#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `SPI_0` reader - SPI0 interrupt active"]
pub type SPI_0_R = crate::BitReader;
#[doc = "Field `SPI_0` writer - SPI0 interrupt active"]
pub type SPI_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type SPI_1_R = crate::BitReader;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type SPI_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type SPI_2_R = crate::BitReader;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type SPI_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_3` reader - SPI3 interrupt active"]
pub type SPI_3_R = crate::BitReader;
#[doc = "Field `SPI_3` writer - SPI3 interrupt active"]
pub type SPI_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_4` reader - SPI4 interrupt active"]
pub type SPI_4_R = crate::BitReader;
#[doc = "Field `SPI_4` writer - SPI4 interrupt active"]
pub type SPI_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_5` reader - SPI5 interrupt active"]
pub type SPI_5_R = crate::BitReader;
#[doc = "Field `SPI_5` writer - SPI5 interrupt active"]
pub type SPI_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_6` reader - SPI6 interrupt active"]
pub type SPI_6_R = crate::BitReader;
#[doc = "Field `SPI_6` writer - SPI6 interrupt active"]
pub type SPI_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_0` reader - I2C0 interrupt active"]
pub type I2C_0_R = crate::BitReader;
#[doc = "Field `I2C_0` writer - I2C0 interrupt active"]
pub type I2C_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_1` reader - I2C1 interrupt active"]
pub type I2C_1_R = crate::BitReader;
#[doc = "Field `I2C_1` writer - I2C1 interrupt active"]
pub type I2C_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_2` reader - I2C2 interrupt active"]
pub type I2C_2_R = crate::BitReader;
#[doc = "Field `I2C_2` writer - I2C2 interrupt active"]
pub type I2C_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_3` reader - I2C3 interrupt active"]
pub type I2C_3_R = crate::BitReader;
#[doc = "Field `I2C_3` writer - I2C3 interrupt active"]
pub type I2C_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_4` reader - I2C4 interrupt active"]
pub type I2C_4_R = crate::BitReader;
#[doc = "Field `I2C_4` writer - I2C4 interrupt active"]
pub type I2C_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_5` reader - I2C5 interrupt active"]
pub type I2C_5_R = crate::BitReader;
#[doc = "Field `I2C_5` writer - I2C5 interrupt active"]
pub type I2C_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_6` reader - I2C6 interrupt active"]
pub type I2C_6_R = crate::BitReader;
#[doc = "Field `I2C_6` writer - I2C6 interrupt active"]
pub type I2C_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_7` reader - I2C7 interrupt active"]
pub type I2C_7_R = crate::BitReader;
#[doc = "Field `I2C_7` writer - I2C7 interrupt active"]
pub type I2C_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_5` reader - UART5 interrupt active"]
pub type UART_5_R = crate::BitReader;
#[doc = "Field `UART_5` writer - UART5 interrupt active"]
pub type UART_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_4` reader - UART4 interrupt active"]
pub type UART_4_R = crate::BitReader;
#[doc = "Field `UART_4` writer - UART4 interrupt active"]
pub type UART_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_3` reader - UART3 interrupt active"]
pub type UART_3_R = crate::BitReader;
#[doc = "Field `UART_3` writer - UART3 interrupt active"]
pub type UART_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_2` reader - UART2 interrupt active"]
pub type UART_2_R = crate::BitReader;
#[doc = "Field `UART_2` writer - UART2 interrupt active"]
pub type UART_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_0` reader - UART0 interrupt active"]
pub type UART_0_R = crate::BitReader;
#[doc = "Field `UART_0` writer - UART0 interrupt active"]
pub type UART_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI0 interrupt active"]
    #[inline(always)]
    pub fn spi_0(&self) -> SPI_0_R {
        SPI_0_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - SPI3 interrupt active"]
    #[inline(always)]
    pub fn spi_3(&self) -> SPI_3_R {
        SPI_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI4 interrupt active"]
    #[inline(always)]
    pub fn spi_4(&self) -> SPI_4_R {
        SPI_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 interrupt active"]
    #[inline(always)]
    pub fn spi_5(&self) -> SPI_5_R {
        SPI_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI6 interrupt active"]
    #[inline(always)]
    pub fn spi_6(&self) -> SPI_6_R {
        SPI_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C0 interrupt active"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2C_0_R {
        I2C_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C1 interrupt active"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2C_1_R {
        I2C_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C2 interrupt active"]
    #[inline(always)]
    pub fn i2c_2(&self) -> I2C_2_R {
        I2C_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C3 interrupt active"]
    #[inline(always)]
    pub fn i2c_3(&self) -> I2C_3_R {
        I2C_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C4 interrupt active"]
    #[inline(always)]
    pub fn i2c_4(&self) -> I2C_4_R {
        I2C_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C5 interrupt active"]
    #[inline(always)]
    pub fn i2c_5(&self) -> I2C_5_R {
        I2C_5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C6 interrupt active"]
    #[inline(always)]
    pub fn i2c_6(&self) -> I2C_6_R {
        I2C_6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C7 interrupt active"]
    #[inline(always)]
    pub fn i2c_7(&self) -> I2C_7_R {
        I2C_7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - UART5 interrupt active"]
    #[inline(always)]
    pub fn uart_5(&self) -> UART_5_R {
        UART_5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART4 interrupt active"]
    #[inline(always)]
    pub fn uart_4(&self) -> UART_4_R {
        UART_4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART3 interrupt active"]
    #[inline(always)]
    pub fn uart_3(&self) -> UART_3_R {
        UART_3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART2 interrupt active"]
    #[inline(always)]
    pub fn uart_2(&self) -> UART_2_R {
        UART_2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART0 interrupt active"]
    #[inline(always)]
    pub fn uart_0(&self) -> UART_0_R {
        UART_0_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("spi_0", &format_args!("{}", self.spi_0().bit()))
            .field("spi_1", &format_args!("{}", self.spi_1().bit()))
            .field("spi_2", &format_args!("{}", self.spi_2().bit()))
            .field("spi_3", &format_args!("{}", self.spi_3().bit()))
            .field("spi_4", &format_args!("{}", self.spi_4().bit()))
            .field("spi_5", &format_args!("{}", self.spi_5().bit()))
            .field("spi_6", &format_args!("{}", self.spi_6().bit()))
            .field("i2c_0", &format_args!("{}", self.i2c_0().bit()))
            .field("i2c_1", &format_args!("{}", self.i2c_1().bit()))
            .field("i2c_2", &format_args!("{}", self.i2c_2().bit()))
            .field("i2c_3", &format_args!("{}", self.i2c_3().bit()))
            .field("i2c_4", &format_args!("{}", self.i2c_4().bit()))
            .field("i2c_5", &format_args!("{}", self.i2c_5().bit()))
            .field("i2c_6", &format_args!("{}", self.i2c_6().bit()))
            .field("i2c_7", &format_args!("{}", self.i2c_7().bit()))
            .field("uart_5", &format_args!("{}", self.uart_5().bit()))
            .field("uart_4", &format_args!("{}", self.uart_4().bit()))
            .field("uart_3", &format_args!("{}", self.uart_3().bit()))
            .field("uart_2", &format_args!("{}", self.uart_2().bit()))
            .field("uart_0", &format_args!("{}", self.uart_0().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_0(&mut self) -> SPI_0_W<CS_SPEC> {
        SPI_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> SPI_1_W<CS_SPEC> {
        SPI_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> SPI_2_W<CS_SPEC> {
        SPI_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_3(&mut self) -> SPI_3_W<CS_SPEC> {
        SPI_3_W::new(self, 3)
    }
    #[doc = "Bit 4 - SPI4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_4(&mut self) -> SPI_4_W<CS_SPEC> {
        SPI_4_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPI5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_5(&mut self) -> SPI_5_W<CS_SPEC> {
        SPI_5_W::new(self, 5)
    }
    #[doc = "Bit 6 - SPI6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_6(&mut self) -> SPI_6_W<CS_SPEC> {
        SPI_6_W::new(self, 6)
    }
    #[doc = "Bit 8 - I2C0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_0(&mut self) -> I2C_0_W<CS_SPEC> {
        I2C_0_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_1(&mut self) -> I2C_1_W<CS_SPEC> {
        I2C_1_W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_2(&mut self) -> I2C_2_W<CS_SPEC> {
        I2C_2_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_3(&mut self) -> I2C_3_W<CS_SPEC> {
        I2C_3_W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_4(&mut self) -> I2C_4_W<CS_SPEC> {
        I2C_4_W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_5(&mut self) -> I2C_5_W<CS_SPEC> {
        I2C_5_W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_6(&mut self) -> I2C_6_W<CS_SPEC> {
        I2C_6_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C7 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_7(&mut self) -> I2C_7_W<CS_SPEC> {
        I2C_7_W::new(self, 15)
    }
    #[doc = "Bit 16 - UART5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_5(&mut self) -> UART_5_W<CS_SPEC> {
        UART_5_W::new(self, 16)
    }
    #[doc = "Bit 17 - UART4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_4(&mut self) -> UART_4_W<CS_SPEC> {
        UART_4_W::new(self, 17)
    }
    #[doc = "Bit 18 - UART3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_3(&mut self) -> UART_3_W<CS_SPEC> {
        UART_3_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_2(&mut self) -> UART_2_W<CS_SPEC> {
        UART_2_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_0(&mut self) -> UART_0_W<CS_SPEC> {
        UART_0_W::new(self, 20)
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
#[doc = "Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

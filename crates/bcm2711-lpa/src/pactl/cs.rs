#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_0` reader - SPI0 interrupt active"]
pub type SPI_0_R = crate::BitReader<bool>;
#[doc = "Field `SPI_0` writer - SPI0 interrupt active"]
pub type SPI_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type SPI_1_R = crate::BitReader<bool>;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type SPI_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type SPI_2_R = crate::BitReader<bool>;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type SPI_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_3` reader - SPI3 interrupt active"]
pub type SPI_3_R = crate::BitReader<bool>;
#[doc = "Field `SPI_3` writer - SPI3 interrupt active"]
pub type SPI_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_4` reader - SPI4 interrupt active"]
pub type SPI_4_R = crate::BitReader<bool>;
#[doc = "Field `SPI_4` writer - SPI4 interrupt active"]
pub type SPI_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_5` reader - SPI5 interrupt active"]
pub type SPI_5_R = crate::BitReader<bool>;
#[doc = "Field `SPI_5` writer - SPI5 interrupt active"]
pub type SPI_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `SPI_6` reader - SPI6 interrupt active"]
pub type SPI_6_R = crate::BitReader<bool>;
#[doc = "Field `SPI_6` writer - SPI6 interrupt active"]
pub type SPI_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_0` reader - I2C0 interrupt active"]
pub type I2C_0_R = crate::BitReader<bool>;
#[doc = "Field `I2C_0` writer - I2C0 interrupt active"]
pub type I2C_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_1` reader - I2C1 interrupt active"]
pub type I2C_1_R = crate::BitReader<bool>;
#[doc = "Field `I2C_1` writer - I2C1 interrupt active"]
pub type I2C_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_2` reader - I2C2 interrupt active"]
pub type I2C_2_R = crate::BitReader<bool>;
#[doc = "Field `I2C_2` writer - I2C2 interrupt active"]
pub type I2C_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_3` reader - I2C3 interrupt active"]
pub type I2C_3_R = crate::BitReader<bool>;
#[doc = "Field `I2C_3` writer - I2C3 interrupt active"]
pub type I2C_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_4` reader - I2C4 interrupt active"]
pub type I2C_4_R = crate::BitReader<bool>;
#[doc = "Field `I2C_4` writer - I2C4 interrupt active"]
pub type I2C_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_5` reader - I2C5 interrupt active"]
pub type I2C_5_R = crate::BitReader<bool>;
#[doc = "Field `I2C_5` writer - I2C5 interrupt active"]
pub type I2C_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_6` reader - I2C6 interrupt active"]
pub type I2C_6_R = crate::BitReader<bool>;
#[doc = "Field `I2C_6` writer - I2C6 interrupt active"]
pub type I2C_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `I2C_7` reader - I2C7 interrupt active"]
pub type I2C_7_R = crate::BitReader<bool>;
#[doc = "Field `I2C_7` writer - I2C7 interrupt active"]
pub type I2C_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `UART_5` reader - UART5 interrupt active"]
pub type UART_5_R = crate::BitReader<bool>;
#[doc = "Field `UART_5` writer - UART5 interrupt active"]
pub type UART_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `UART_4` reader - UART4 interrupt active"]
pub type UART_4_R = crate::BitReader<bool>;
#[doc = "Field `UART_4` writer - UART4 interrupt active"]
pub type UART_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `UART_3` reader - UART3 interrupt active"]
pub type UART_3_R = crate::BitReader<bool>;
#[doc = "Field `UART_3` writer - UART3 interrupt active"]
pub type UART_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `UART_2` reader - UART2 interrupt active"]
pub type UART_2_R = crate::BitReader<bool>;
#[doc = "Field `UART_2` writer - UART2 interrupt active"]
pub type UART_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `UART_0` reader - UART0 interrupt active"]
pub type UART_0_R = crate::BitReader<bool>;
#[doc = "Field `UART_0` writer - UART0 interrupt active"]
pub type UART_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - SPI0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_0(&mut self) -> SPI_0_W<0> {
        SPI_0_W::new(self)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> SPI_1_W<1> {
        SPI_1_W::new(self)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> SPI_2_W<2> {
        SPI_2_W::new(self)
    }
    #[doc = "Bit 3 - SPI3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_3(&mut self) -> SPI_3_W<3> {
        SPI_3_W::new(self)
    }
    #[doc = "Bit 4 - SPI4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_4(&mut self) -> SPI_4_W<4> {
        SPI_4_W::new(self)
    }
    #[doc = "Bit 5 - SPI5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_5(&mut self) -> SPI_5_W<5> {
        SPI_5_W::new(self)
    }
    #[doc = "Bit 6 - SPI6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_6(&mut self) -> SPI_6_W<6> {
        SPI_6_W::new(self)
    }
    #[doc = "Bit 8 - I2C0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_0(&mut self) -> I2C_0_W<8> {
        I2C_0_W::new(self)
    }
    #[doc = "Bit 9 - I2C1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_1(&mut self) -> I2C_1_W<9> {
        I2C_1_W::new(self)
    }
    #[doc = "Bit 10 - I2C2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_2(&mut self) -> I2C_2_W<10> {
        I2C_2_W::new(self)
    }
    #[doc = "Bit 11 - I2C3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_3(&mut self) -> I2C_3_W<11> {
        I2C_3_W::new(self)
    }
    #[doc = "Bit 12 - I2C4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_4(&mut self) -> I2C_4_W<12> {
        I2C_4_W::new(self)
    }
    #[doc = "Bit 13 - I2C5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_5(&mut self) -> I2C_5_W<13> {
        I2C_5_W::new(self)
    }
    #[doc = "Bit 14 - I2C6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_6(&mut self) -> I2C_6_W<14> {
        I2C_6_W::new(self)
    }
    #[doc = "Bit 15 - I2C7 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_7(&mut self) -> I2C_7_W<15> {
        I2C_7_W::new(self)
    }
    #[doc = "Bit 16 - UART5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_5(&mut self) -> UART_5_W<16> {
        UART_5_W::new(self)
    }
    #[doc = "Bit 17 - UART4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_4(&mut self) -> UART_4_W<17> {
        UART_4_W::new(self)
    }
    #[doc = "Bit 18 - UART3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_3(&mut self) -> UART_3_W<18> {
        UART_3_W::new(self)
    }
    #[doc = "Bit 19 - UART2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_2(&mut self) -> UART_2_W<19> {
        UART_2_W::new(self)
    }
    #[doc = "Bit 20 - UART0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_0(&mut self) -> UART_0_W<20> {
        UART_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

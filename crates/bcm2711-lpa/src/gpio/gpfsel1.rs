#[doc = "Register `GPFSEL1` reader"]
pub struct R(crate::R<GPFSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPFSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPFSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPFSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPFSEL1` writer"]
pub struct W(crate::W<GPFSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPFSEL1_SPEC>;
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
impl From<crate::W<GPFSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPFSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSEL10` reader - Function Select 10"]
pub type FSEL10_R = crate::FieldReader<u8, FSEL10_A>;
#[doc = "Function Select 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL10_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_MOSI"]
    SPI0_MOSI = 4,
    #[doc = "5: Pin is connected to SD2"]
    SD2 = 5,
    #[doc = "6: Pin is connected to DPI_D6"]
    DPI_D6 = 6,
    #[doc = "7: Pin is connected to BSCSL_MOSI"]
    BSCSL_MOSI = 7,
    #[doc = "3: Pin is connected to CTS4"]
    CTS4 = 3,
    #[doc = "2: Pin is connected to SDA5"]
    SDA5 = 2,
}
impl From<FSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL10_A) -> Self {
        variant as _
    }
}
impl FSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL10_A {
        match self.bits {
            0 => FSEL10_A::INPUT,
            1 => FSEL10_A::OUTPUT,
            4 => FSEL10_A::SPI0_MOSI,
            5 => FSEL10_A::SD2,
            6 => FSEL10_A::DPI_D6,
            7 => FSEL10_A::BSCSL_MOSI,
            3 => FSEL10_A::CTS4,
            2 => FSEL10_A::SDA5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL10_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_MOSI`"]
    #[inline(always)]
    pub fn is_spi0_mosi(&self) -> bool {
        *self == FSEL10_A::SPI0_MOSI
    }
    #[doc = "Checks if the value of the field is `SD2`"]
    #[inline(always)]
    pub fn is_sd2(&self) -> bool {
        *self == FSEL10_A::SD2
    }
    #[doc = "Checks if the value of the field is `DPI_D6`"]
    #[inline(always)]
    pub fn is_dpi_d6(&self) -> bool {
        *self == FSEL10_A::DPI_D6
    }
    #[doc = "Checks if the value of the field is `BSCSL_MOSI`"]
    #[inline(always)]
    pub fn is_bscsl_mosi(&self) -> bool {
        *self == FSEL10_A::BSCSL_MOSI
    }
    #[doc = "Checks if the value of the field is `CTS4`"]
    #[inline(always)]
    pub fn is_cts4(&self) -> bool {
        *self == FSEL10_A::CTS4
    }
    #[doc = "Checks if the value of the field is `SDA5`"]
    #[inline(always)]
    pub fn is_sda5(&self) -> bool {
        *self == FSEL10_A::SDA5
    }
}
#[doc = "Field `FSEL10` writer - Function Select 10"]
pub type FSEL10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL10_A, 3, O>;
impl<'a, const O: u8> FSEL10_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL10_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL10_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn spi0_mosi(self) -> &'a mut W {
        self.variant(FSEL10_A::SPI0_MOSI)
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn sd2(self) -> &'a mut W {
        self.variant(FSEL10_A::SD2)
    }
    #[doc = "Pin is connected to DPI_D6"]
    #[inline(always)]
    pub fn dpi_d6(self) -> &'a mut W {
        self.variant(FSEL10_A::DPI_D6)
    }
    #[doc = "Pin is connected to BSCSL_MOSI"]
    #[inline(always)]
    pub fn bscsl_mosi(self) -> &'a mut W {
        self.variant(FSEL10_A::BSCSL_MOSI)
    }
    #[doc = "Pin is connected to CTS4"]
    #[inline(always)]
    pub fn cts4(self) -> &'a mut W {
        self.variant(FSEL10_A::CTS4)
    }
    #[doc = "Pin is connected to SDA5"]
    #[inline(always)]
    pub fn sda5(self) -> &'a mut W {
        self.variant(FSEL10_A::SDA5)
    }
}
#[doc = "Field `FSEL11` reader - Function Select 11"]
pub type FSEL11_R = crate::FieldReader<u8, FSEL11_A>;
#[doc = "Function Select 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL11_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_SCLK"]
    SPI0_SCLK = 4,
    #[doc = "5: Pin is connected to SD3"]
    SD3 = 5,
    #[doc = "6: Pin is connected to DPI_D7"]
    DPI_D7 = 6,
    #[doc = "7: Pin is connected to BSCSL_SCLK"]
    BSCSL_SCLK = 7,
    #[doc = "3: Pin is connected to RTS4"]
    RTS4 = 3,
    #[doc = "2: Pin is connected to SCL5"]
    SCL5 = 2,
}
impl From<FSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL11_A) -> Self {
        variant as _
    }
}
impl FSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL11_A {
        match self.bits {
            0 => FSEL11_A::INPUT,
            1 => FSEL11_A::OUTPUT,
            4 => FSEL11_A::SPI0_SCLK,
            5 => FSEL11_A::SD3,
            6 => FSEL11_A::DPI_D7,
            7 => FSEL11_A::BSCSL_SCLK,
            3 => FSEL11_A::RTS4,
            2 => FSEL11_A::SCL5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL11_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `SPI0_SCLK`"]
    #[inline(always)]
    pub fn is_spi0_sclk(&self) -> bool {
        *self == FSEL11_A::SPI0_SCLK
    }
    #[doc = "Checks if the value of the field is `SD3`"]
    #[inline(always)]
    pub fn is_sd3(&self) -> bool {
        *self == FSEL11_A::SD3
    }
    #[doc = "Checks if the value of the field is `DPI_D7`"]
    #[inline(always)]
    pub fn is_dpi_d7(&self) -> bool {
        *self == FSEL11_A::DPI_D7
    }
    #[doc = "Checks if the value of the field is `BSCSL_SCLK`"]
    #[inline(always)]
    pub fn is_bscsl_sclk(&self) -> bool {
        *self == FSEL11_A::BSCSL_SCLK
    }
    #[doc = "Checks if the value of the field is `RTS4`"]
    #[inline(always)]
    pub fn is_rts4(&self) -> bool {
        *self == FSEL11_A::RTS4
    }
    #[doc = "Checks if the value of the field is `SCL5`"]
    #[inline(always)]
    pub fn is_scl5(&self) -> bool {
        *self == FSEL11_A::SCL5
    }
}
#[doc = "Field `FSEL11` writer - Function Select 11"]
pub type FSEL11_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL11_A, 3, O>;
impl<'a, const O: u8> FSEL11_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL11_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL11_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn spi0_sclk(self) -> &'a mut W {
        self.variant(FSEL11_A::SPI0_SCLK)
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn sd3(self) -> &'a mut W {
        self.variant(FSEL11_A::SD3)
    }
    #[doc = "Pin is connected to DPI_D7"]
    #[inline(always)]
    pub fn dpi_d7(self) -> &'a mut W {
        self.variant(FSEL11_A::DPI_D7)
    }
    #[doc = "Pin is connected to BSCSL_SCLK"]
    #[inline(always)]
    pub fn bscsl_sclk(self) -> &'a mut W {
        self.variant(FSEL11_A::BSCSL_SCLK)
    }
    #[doc = "Pin is connected to RTS4"]
    #[inline(always)]
    pub fn rts4(self) -> &'a mut W {
        self.variant(FSEL11_A::RTS4)
    }
    #[doc = "Pin is connected to SCL5"]
    #[inline(always)]
    pub fn scl5(self) -> &'a mut W {
        self.variant(FSEL11_A::SCL5)
    }
}
#[doc = "Field `FSEL12` reader - Function Select 12"]
pub type FSEL12_R = crate::FieldReader<u8, FSEL12_A>;
#[doc = "Function Select 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL12_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PWM0_0"]
    PWM0_0 = 4,
    #[doc = "5: Pin is connected to SD4"]
    SD4 = 5,
    #[doc = "6: Pin is connected to DPI_D8"]
    DPI_D8 = 6,
    #[doc = "7: Pin is connected to SPI5_CE0_N"]
    SPI5_CE0_N = 7,
    #[doc = "3: Pin is connected to TXD5"]
    TXD5 = 3,
    #[doc = "2: Pin is connected to SDA5"]
    SDA5 = 2,
}
impl From<FSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL12_A) -> Self {
        variant as _
    }
}
impl FSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL12_A {
        match self.bits {
            0 => FSEL12_A::INPUT,
            1 => FSEL12_A::OUTPUT,
            4 => FSEL12_A::PWM0_0,
            5 => FSEL12_A::SD4,
            6 => FSEL12_A::DPI_D8,
            7 => FSEL12_A::SPI5_CE0_N,
            3 => FSEL12_A::TXD5,
            2 => FSEL12_A::SDA5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL12_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PWM0_0`"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == FSEL12_A::PWM0_0
    }
    #[doc = "Checks if the value of the field is `SD4`"]
    #[inline(always)]
    pub fn is_sd4(&self) -> bool {
        *self == FSEL12_A::SD4
    }
    #[doc = "Checks if the value of the field is `DPI_D8`"]
    #[inline(always)]
    pub fn is_dpi_d8(&self) -> bool {
        *self == FSEL12_A::DPI_D8
    }
    #[doc = "Checks if the value of the field is `SPI5_CE0_N`"]
    #[inline(always)]
    pub fn is_spi5_ce0_n(&self) -> bool {
        *self == FSEL12_A::SPI5_CE0_N
    }
    #[doc = "Checks if the value of the field is `TXD5`"]
    #[inline(always)]
    pub fn is_txd5(&self) -> bool {
        *self == FSEL12_A::TXD5
    }
    #[doc = "Checks if the value of the field is `SDA5`"]
    #[inline(always)]
    pub fn is_sda5(&self) -> bool {
        *self == FSEL12_A::SDA5
    }
}
#[doc = "Field `FSEL12` writer - Function Select 12"]
pub type FSEL12_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL12_A, 3, O>;
impl<'a, const O: u8> FSEL12_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL12_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL12_A::OUTPUT)
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut W {
        self.variant(FSEL12_A::PWM0_0)
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn sd4(self) -> &'a mut W {
        self.variant(FSEL12_A::SD4)
    }
    #[doc = "Pin is connected to DPI_D8"]
    #[inline(always)]
    pub fn dpi_d8(self) -> &'a mut W {
        self.variant(FSEL12_A::DPI_D8)
    }
    #[doc = "Pin is connected to SPI5_CE0_N"]
    #[inline(always)]
    pub fn spi5_ce0_n(self) -> &'a mut W {
        self.variant(FSEL12_A::SPI5_CE0_N)
    }
    #[doc = "Pin is connected to TXD5"]
    #[inline(always)]
    pub fn txd5(self) -> &'a mut W {
        self.variant(FSEL12_A::TXD5)
    }
    #[doc = "Pin is connected to SDA5"]
    #[inline(always)]
    pub fn sda5(self) -> &'a mut W {
        self.variant(FSEL12_A::SDA5)
    }
}
#[doc = "Field `FSEL13` reader - Function Select 13"]
pub type FSEL13_R = crate::FieldReader<u8, FSEL13_A>;
#[doc = "Function Select 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL13_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PWM0_1"]
    PWM0_1 = 4,
    #[doc = "5: Pin is connected to SD5"]
    SD5 = 5,
    #[doc = "6: Pin is connected to DPI_D9"]
    DPI_D9 = 6,
    #[doc = "7: Pin is connected to SPI5_MISO"]
    SPI5_MISO = 7,
    #[doc = "3: Pin is connected to RXD5"]
    RXD5 = 3,
    #[doc = "2: Pin is connected to SCL5"]
    SCL5 = 2,
}
impl From<FSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL13_A) -> Self {
        variant as _
    }
}
impl FSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL13_A {
        match self.bits {
            0 => FSEL13_A::INPUT,
            1 => FSEL13_A::OUTPUT,
            4 => FSEL13_A::PWM0_1,
            5 => FSEL13_A::SD5,
            6 => FSEL13_A::DPI_D9,
            7 => FSEL13_A::SPI5_MISO,
            3 => FSEL13_A::RXD5,
            2 => FSEL13_A::SCL5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL13_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PWM0_1`"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == FSEL13_A::PWM0_1
    }
    #[doc = "Checks if the value of the field is `SD5`"]
    #[inline(always)]
    pub fn is_sd5(&self) -> bool {
        *self == FSEL13_A::SD5
    }
    #[doc = "Checks if the value of the field is `DPI_D9`"]
    #[inline(always)]
    pub fn is_dpi_d9(&self) -> bool {
        *self == FSEL13_A::DPI_D9
    }
    #[doc = "Checks if the value of the field is `SPI5_MISO`"]
    #[inline(always)]
    pub fn is_spi5_miso(&self) -> bool {
        *self == FSEL13_A::SPI5_MISO
    }
    #[doc = "Checks if the value of the field is `RXD5`"]
    #[inline(always)]
    pub fn is_rxd5(&self) -> bool {
        *self == FSEL13_A::RXD5
    }
    #[doc = "Checks if the value of the field is `SCL5`"]
    #[inline(always)]
    pub fn is_scl5(&self) -> bool {
        *self == FSEL13_A::SCL5
    }
}
#[doc = "Field `FSEL13` writer - Function Select 13"]
pub type FSEL13_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL13_A, 3, O>;
impl<'a, const O: u8> FSEL13_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL13_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL13_A::OUTPUT)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut W {
        self.variant(FSEL13_A::PWM0_1)
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn sd5(self) -> &'a mut W {
        self.variant(FSEL13_A::SD5)
    }
    #[doc = "Pin is connected to DPI_D9"]
    #[inline(always)]
    pub fn dpi_d9(self) -> &'a mut W {
        self.variant(FSEL13_A::DPI_D9)
    }
    #[doc = "Pin is connected to SPI5_MISO"]
    #[inline(always)]
    pub fn spi5_miso(self) -> &'a mut W {
        self.variant(FSEL13_A::SPI5_MISO)
    }
    #[doc = "Pin is connected to RXD5"]
    #[inline(always)]
    pub fn rxd5(self) -> &'a mut W {
        self.variant(FSEL13_A::RXD5)
    }
    #[doc = "Pin is connected to SCL5"]
    #[inline(always)]
    pub fn scl5(self) -> &'a mut W {
        self.variant(FSEL13_A::SCL5)
    }
}
#[doc = "Field `FSEL14` reader - Function Select 14"]
pub type FSEL14_R = crate::FieldReader<u8, FSEL14_A>;
#[doc = "Function Select 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL14_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to TXD0"]
    TXD0 = 4,
    #[doc = "5: Pin is connected to SD6"]
    SD6 = 5,
    #[doc = "6: Pin is connected to DPI_D10"]
    DPI_D10 = 6,
    #[doc = "7: Pin is connected to SPI5_MOSI"]
    SPI5_MOSI = 7,
    #[doc = "3: Pin is connected to CTS5"]
    CTS5 = 3,
    #[doc = "2: Pin is connected to TXD1"]
    TXD1 = 2,
}
impl From<FSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL14_A) -> Self {
        variant as _
    }
}
impl FSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL14_A {
        match self.bits {
            0 => FSEL14_A::INPUT,
            1 => FSEL14_A::OUTPUT,
            4 => FSEL14_A::TXD0,
            5 => FSEL14_A::SD6,
            6 => FSEL14_A::DPI_D10,
            7 => FSEL14_A::SPI5_MOSI,
            3 => FSEL14_A::CTS5,
            2 => FSEL14_A::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL14_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TXD0`"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == FSEL14_A::TXD0
    }
    #[doc = "Checks if the value of the field is `SD6`"]
    #[inline(always)]
    pub fn is_sd6(&self) -> bool {
        *self == FSEL14_A::SD6
    }
    #[doc = "Checks if the value of the field is `DPI_D10`"]
    #[inline(always)]
    pub fn is_dpi_d10(&self) -> bool {
        *self == FSEL14_A::DPI_D10
    }
    #[doc = "Checks if the value of the field is `SPI5_MOSI`"]
    #[inline(always)]
    pub fn is_spi5_mosi(&self) -> bool {
        *self == FSEL14_A::SPI5_MOSI
    }
    #[doc = "Checks if the value of the field is `CTS5`"]
    #[inline(always)]
    pub fn is_cts5(&self) -> bool {
        *self == FSEL14_A::CTS5
    }
    #[doc = "Checks if the value of the field is `TXD1`"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == FSEL14_A::TXD1
    }
}
#[doc = "Field `FSEL14` writer - Function Select 14"]
pub type FSEL14_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL14_A, 3, O>;
impl<'a, const O: u8> FSEL14_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL14_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL14_A::OUTPUT)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut W {
        self.variant(FSEL14_A::TXD0)
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn sd6(self) -> &'a mut W {
        self.variant(FSEL14_A::SD6)
    }
    #[doc = "Pin is connected to DPI_D10"]
    #[inline(always)]
    pub fn dpi_d10(self) -> &'a mut W {
        self.variant(FSEL14_A::DPI_D10)
    }
    #[doc = "Pin is connected to SPI5_MOSI"]
    #[inline(always)]
    pub fn spi5_mosi(self) -> &'a mut W {
        self.variant(FSEL14_A::SPI5_MOSI)
    }
    #[doc = "Pin is connected to CTS5"]
    #[inline(always)]
    pub fn cts5(self) -> &'a mut W {
        self.variant(FSEL14_A::CTS5)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut W {
        self.variant(FSEL14_A::TXD1)
    }
}
#[doc = "Field `FSEL15` reader - Function Select 15"]
pub type FSEL15_R = crate::FieldReader<u8, FSEL15_A>;
#[doc = "Function Select 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL15_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to RXD0"]
    RXD0 = 4,
    #[doc = "5: Pin is connected to SD7"]
    SD7 = 5,
    #[doc = "6: Pin is connected to DPI_D11"]
    DPI_D11 = 6,
    #[doc = "7: Pin is connected to SPI5_SCLK"]
    SPI5_SCLK = 7,
    #[doc = "3: Pin is connected to RTS5"]
    RTS5 = 3,
    #[doc = "2: Pin is connected to RXD1"]
    RXD1 = 2,
}
impl From<FSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL15_A) -> Self {
        variant as _
    }
}
impl FSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL15_A {
        match self.bits {
            0 => FSEL15_A::INPUT,
            1 => FSEL15_A::OUTPUT,
            4 => FSEL15_A::RXD0,
            5 => FSEL15_A::SD7,
            6 => FSEL15_A::DPI_D11,
            7 => FSEL15_A::SPI5_SCLK,
            3 => FSEL15_A::RTS5,
            2 => FSEL15_A::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL15_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RXD0`"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == FSEL15_A::RXD0
    }
    #[doc = "Checks if the value of the field is `SD7`"]
    #[inline(always)]
    pub fn is_sd7(&self) -> bool {
        *self == FSEL15_A::SD7
    }
    #[doc = "Checks if the value of the field is `DPI_D11`"]
    #[inline(always)]
    pub fn is_dpi_d11(&self) -> bool {
        *self == FSEL15_A::DPI_D11
    }
    #[doc = "Checks if the value of the field is `SPI5_SCLK`"]
    #[inline(always)]
    pub fn is_spi5_sclk(&self) -> bool {
        *self == FSEL15_A::SPI5_SCLK
    }
    #[doc = "Checks if the value of the field is `RTS5`"]
    #[inline(always)]
    pub fn is_rts5(&self) -> bool {
        *self == FSEL15_A::RTS5
    }
    #[doc = "Checks if the value of the field is `RXD1`"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == FSEL15_A::RXD1
    }
}
#[doc = "Field `FSEL15` writer - Function Select 15"]
pub type FSEL15_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL15_A, 3, O>;
impl<'a, const O: u8> FSEL15_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL15_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL15_A::OUTPUT)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut W {
        self.variant(FSEL15_A::RXD0)
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn sd7(self) -> &'a mut W {
        self.variant(FSEL15_A::SD7)
    }
    #[doc = "Pin is connected to DPI_D11"]
    #[inline(always)]
    pub fn dpi_d11(self) -> &'a mut W {
        self.variant(FSEL15_A::DPI_D11)
    }
    #[doc = "Pin is connected to SPI5_SCLK"]
    #[inline(always)]
    pub fn spi5_sclk(self) -> &'a mut W {
        self.variant(FSEL15_A::SPI5_SCLK)
    }
    #[doc = "Pin is connected to RTS5"]
    #[inline(always)]
    pub fn rts5(self) -> &'a mut W {
        self.variant(FSEL15_A::RTS5)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut W {
        self.variant(FSEL15_A::RXD1)
    }
}
#[doc = "Field `FSEL16` reader - Function Select 16"]
pub type FSEL16_R = crate::FieldReader<u8, FSEL16_A>;
#[doc = "Function Select 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL16_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD8"]
    SD8 = 5,
    #[doc = "6: Pin is connected to DPI_D12"]
    DPI_D12 = 6,
    #[doc = "7: Pin is connected to CTS0"]
    CTS0 = 7,
    #[doc = "3: Pin is connected to SPI1_CE2_N"]
    SPI1_CE2_N = 3,
    #[doc = "2: Pin is connected to CTS1"]
    CTS1 = 2,
}
impl From<FSEL16_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL16_A) -> Self {
        variant as _
    }
}
impl FSEL16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL16_A {
        match self.bits {
            0 => FSEL16_A::INPUT,
            1 => FSEL16_A::OUTPUT,
            4 => FSEL16_A::RESERVED0,
            5 => FSEL16_A::SD8,
            6 => FSEL16_A::DPI_D12,
            7 => FSEL16_A::CTS0,
            3 => FSEL16_A::SPI1_CE2_N,
            2 => FSEL16_A::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL16_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL16_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL16_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD8`"]
    #[inline(always)]
    pub fn is_sd8(&self) -> bool {
        *self == FSEL16_A::SD8
    }
    #[doc = "Checks if the value of the field is `DPI_D12`"]
    #[inline(always)]
    pub fn is_dpi_d12(&self) -> bool {
        *self == FSEL16_A::DPI_D12
    }
    #[doc = "Checks if the value of the field is `CTS0`"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == FSEL16_A::CTS0
    }
    #[doc = "Checks if the value of the field is `SPI1_CE2_N`"]
    #[inline(always)]
    pub fn is_spi1_ce2_n(&self) -> bool {
        *self == FSEL16_A::SPI1_CE2_N
    }
    #[doc = "Checks if the value of the field is `CTS1`"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == FSEL16_A::CTS1
    }
}
#[doc = "Field `FSEL16` writer - Function Select 16"]
pub type FSEL16_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL16_A, 3, O>;
impl<'a, const O: u8> FSEL16_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL16_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL16_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL16_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD8"]
    #[inline(always)]
    pub fn sd8(self) -> &'a mut W {
        self.variant(FSEL16_A::SD8)
    }
    #[doc = "Pin is connected to DPI_D12"]
    #[inline(always)]
    pub fn dpi_d12(self) -> &'a mut W {
        self.variant(FSEL16_A::DPI_D12)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut W {
        self.variant(FSEL16_A::CTS0)
    }
    #[doc = "Pin is connected to SPI1_CE2_N"]
    #[inline(always)]
    pub fn spi1_ce2_n(self) -> &'a mut W {
        self.variant(FSEL16_A::SPI1_CE2_N)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut W {
        self.variant(FSEL16_A::CTS1)
    }
}
#[doc = "Field `FSEL17` reader - Function Select 17"]
pub type FSEL17_R = crate::FieldReader<u8, FSEL17_A>;
#[doc = "Function Select 17"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL17_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SD9"]
    SD9 = 5,
    #[doc = "6: Pin is connected to DPI_D13"]
    DPI_D13 = 6,
    #[doc = "7: Pin is connected to RTS0"]
    RTS0 = 7,
    #[doc = "3: Pin is connected to SPI1_CE1_N"]
    SPI1_CE1_N = 3,
    #[doc = "2: Pin is connected to RTS1"]
    RTS1 = 2,
}
impl From<FSEL17_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL17_A) -> Self {
        variant as _
    }
}
impl FSEL17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL17_A {
        match self.bits {
            0 => FSEL17_A::INPUT,
            1 => FSEL17_A::OUTPUT,
            4 => FSEL17_A::RESERVED0,
            5 => FSEL17_A::SD9,
            6 => FSEL17_A::DPI_D13,
            7 => FSEL17_A::RTS0,
            3 => FSEL17_A::SPI1_CE1_N,
            2 => FSEL17_A::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL17_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL17_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL17_A::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SD9`"]
    #[inline(always)]
    pub fn is_sd9(&self) -> bool {
        *self == FSEL17_A::SD9
    }
    #[doc = "Checks if the value of the field is `DPI_D13`"]
    #[inline(always)]
    pub fn is_dpi_d13(&self) -> bool {
        *self == FSEL17_A::DPI_D13
    }
    #[doc = "Checks if the value of the field is `RTS0`"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == FSEL17_A::RTS0
    }
    #[doc = "Checks if the value of the field is `SPI1_CE1_N`"]
    #[inline(always)]
    pub fn is_spi1_ce1_n(&self) -> bool {
        *self == FSEL17_A::SPI1_CE1_N
    }
    #[doc = "Checks if the value of the field is `RTS1`"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == FSEL17_A::RTS1
    }
}
#[doc = "Field `FSEL17` writer - Function Select 17"]
pub type FSEL17_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL17_A, 3, O>;
impl<'a, const O: u8> FSEL17_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL17_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL17_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(FSEL17_A::RESERVED0)
    }
    #[doc = "Pin is connected to SD9"]
    #[inline(always)]
    pub fn sd9(self) -> &'a mut W {
        self.variant(FSEL17_A::SD9)
    }
    #[doc = "Pin is connected to DPI_D13"]
    #[inline(always)]
    pub fn dpi_d13(self) -> &'a mut W {
        self.variant(FSEL17_A::DPI_D13)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut W {
        self.variant(FSEL17_A::RTS0)
    }
    #[doc = "Pin is connected to SPI1_CE1_N"]
    #[inline(always)]
    pub fn spi1_ce1_n(self) -> &'a mut W {
        self.variant(FSEL17_A::SPI1_CE1_N)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut W {
        self.variant(FSEL17_A::RTS1)
    }
}
#[doc = "Field `FSEL18` reader - Function Select 18"]
pub type FSEL18_R = crate::FieldReader<u8, FSEL18_A>;
#[doc = "Function Select 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL18_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PCM_CLK"]
    PCM_CLK = 4,
    #[doc = "5: Pin is connected to SD10"]
    SD10 = 5,
    #[doc = "6: Pin is connected to DPI_D14"]
    DPI_D14 = 6,
    #[doc = "7: Pin is connected to SPI6_CE0_N"]
    SPI6_CE0_N = 7,
    #[doc = "3: Pin is connected to SPI1_CE0_N"]
    SPI1_CE0_N = 3,
    #[doc = "2: Pin is connected to PWM0_0"]
    PWM0_0 = 2,
}
impl From<FSEL18_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL18_A) -> Self {
        variant as _
    }
}
impl FSEL18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL18_A {
        match self.bits {
            0 => FSEL18_A::INPUT,
            1 => FSEL18_A::OUTPUT,
            4 => FSEL18_A::PCM_CLK,
            5 => FSEL18_A::SD10,
            6 => FSEL18_A::DPI_D14,
            7 => FSEL18_A::SPI6_CE0_N,
            3 => FSEL18_A::SPI1_CE0_N,
            2 => FSEL18_A::PWM0_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL18_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL18_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PCM_CLK`"]
    #[inline(always)]
    pub fn is_pcm_clk(&self) -> bool {
        *self == FSEL18_A::PCM_CLK
    }
    #[doc = "Checks if the value of the field is `SD10`"]
    #[inline(always)]
    pub fn is_sd10(&self) -> bool {
        *self == FSEL18_A::SD10
    }
    #[doc = "Checks if the value of the field is `DPI_D14`"]
    #[inline(always)]
    pub fn is_dpi_d14(&self) -> bool {
        *self == FSEL18_A::DPI_D14
    }
    #[doc = "Checks if the value of the field is `SPI6_CE0_N`"]
    #[inline(always)]
    pub fn is_spi6_ce0_n(&self) -> bool {
        *self == FSEL18_A::SPI6_CE0_N
    }
    #[doc = "Checks if the value of the field is `SPI1_CE0_N`"]
    #[inline(always)]
    pub fn is_spi1_ce0_n(&self) -> bool {
        *self == FSEL18_A::SPI1_CE0_N
    }
    #[doc = "Checks if the value of the field is `PWM0_0`"]
    #[inline(always)]
    pub fn is_pwm0_0(&self) -> bool {
        *self == FSEL18_A::PWM0_0
    }
}
#[doc = "Field `FSEL18` writer - Function Select 18"]
pub type FSEL18_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL18_A, 3, O>;
impl<'a, const O: u8> FSEL18_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL18_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL18_A::OUTPUT)
    }
    #[doc = "Pin is connected to PCM_CLK"]
    #[inline(always)]
    pub fn pcm_clk(self) -> &'a mut W {
        self.variant(FSEL18_A::PCM_CLK)
    }
    #[doc = "Pin is connected to SD10"]
    #[inline(always)]
    pub fn sd10(self) -> &'a mut W {
        self.variant(FSEL18_A::SD10)
    }
    #[doc = "Pin is connected to DPI_D14"]
    #[inline(always)]
    pub fn dpi_d14(self) -> &'a mut W {
        self.variant(FSEL18_A::DPI_D14)
    }
    #[doc = "Pin is connected to SPI6_CE0_N"]
    #[inline(always)]
    pub fn spi6_ce0_n(self) -> &'a mut W {
        self.variant(FSEL18_A::SPI6_CE0_N)
    }
    #[doc = "Pin is connected to SPI1_CE0_N"]
    #[inline(always)]
    pub fn spi1_ce0_n(self) -> &'a mut W {
        self.variant(FSEL18_A::SPI1_CE0_N)
    }
    #[doc = "Pin is connected to PWM0_0"]
    #[inline(always)]
    pub fn pwm0_0(self) -> &'a mut W {
        self.variant(FSEL18_A::PWM0_0)
    }
}
#[doc = "Field `FSEL19` reader - Function Select 19"]
pub type FSEL19_R = crate::FieldReader<u8, FSEL19_A>;
#[doc = "Function Select 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL19_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PCM_FS"]
    PCM_FS = 4,
    #[doc = "5: Pin is connected to SD11"]
    SD11 = 5,
    #[doc = "6: Pin is connected to DPI_D15"]
    DPI_D15 = 6,
    #[doc = "7: Pin is connected to SPI6_MISO"]
    SPI6_MISO = 7,
    #[doc = "3: Pin is connected to SPI1_MISO"]
    SPI1_MISO = 3,
    #[doc = "2: Pin is connected to PWM0_1"]
    PWM0_1 = 2,
}
impl From<FSEL19_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL19_A) -> Self {
        variant as _
    }
}
impl FSEL19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL19_A {
        match self.bits {
            0 => FSEL19_A::INPUT,
            1 => FSEL19_A::OUTPUT,
            4 => FSEL19_A::PCM_FS,
            5 => FSEL19_A::SD11,
            6 => FSEL19_A::DPI_D15,
            7 => FSEL19_A::SPI6_MISO,
            3 => FSEL19_A::SPI1_MISO,
            2 => FSEL19_A::PWM0_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL19_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL19_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `PCM_FS`"]
    #[inline(always)]
    pub fn is_pcm_fs(&self) -> bool {
        *self == FSEL19_A::PCM_FS
    }
    #[doc = "Checks if the value of the field is `SD11`"]
    #[inline(always)]
    pub fn is_sd11(&self) -> bool {
        *self == FSEL19_A::SD11
    }
    #[doc = "Checks if the value of the field is `DPI_D15`"]
    #[inline(always)]
    pub fn is_dpi_d15(&self) -> bool {
        *self == FSEL19_A::DPI_D15
    }
    #[doc = "Checks if the value of the field is `SPI6_MISO`"]
    #[inline(always)]
    pub fn is_spi6_miso(&self) -> bool {
        *self == FSEL19_A::SPI6_MISO
    }
    #[doc = "Checks if the value of the field is `SPI1_MISO`"]
    #[inline(always)]
    pub fn is_spi1_miso(&self) -> bool {
        *self == FSEL19_A::SPI1_MISO
    }
    #[doc = "Checks if the value of the field is `PWM0_1`"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == FSEL19_A::PWM0_1
    }
}
#[doc = "Field `FSEL19` writer - Function Select 19"]
pub type FSEL19_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPFSEL1_SPEC, u8, FSEL19_A, 3, O>;
impl<'a, const O: u8> FSEL19_W<'a, O> {
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FSEL19_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FSEL19_A::OUTPUT)
    }
    #[doc = "Pin is connected to PCM_FS"]
    #[inline(always)]
    pub fn pcm_fs(self) -> &'a mut W {
        self.variant(FSEL19_A::PCM_FS)
    }
    #[doc = "Pin is connected to SD11"]
    #[inline(always)]
    pub fn sd11(self) -> &'a mut W {
        self.variant(FSEL19_A::SD11)
    }
    #[doc = "Pin is connected to DPI_D15"]
    #[inline(always)]
    pub fn dpi_d15(self) -> &'a mut W {
        self.variant(FSEL19_A::DPI_D15)
    }
    #[doc = "Pin is connected to SPI6_MISO"]
    #[inline(always)]
    pub fn spi6_miso(self) -> &'a mut W {
        self.variant(FSEL19_A::SPI6_MISO)
    }
    #[doc = "Pin is connected to SPI1_MISO"]
    #[inline(always)]
    pub fn spi1_miso(self) -> &'a mut W {
        self.variant(FSEL19_A::SPI1_MISO)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut W {
        self.variant(FSEL19_A::PWM0_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 10"]
    #[inline(always)]
    pub fn fsel10(&self) -> FSEL10_R {
        FSEL10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 11"]
    #[inline(always)]
    pub fn fsel11(&self) -> FSEL11_R {
        FSEL11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 12"]
    #[inline(always)]
    pub fn fsel12(&self) -> FSEL12_R {
        FSEL12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 13"]
    #[inline(always)]
    pub fn fsel13(&self) -> FSEL13_R {
        FSEL13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 14"]
    #[inline(always)]
    pub fn fsel14(&self) -> FSEL14_R {
        FSEL14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 15"]
    #[inline(always)]
    pub fn fsel15(&self) -> FSEL15_R {
        FSEL15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 16"]
    #[inline(always)]
    pub fn fsel16(&self) -> FSEL16_R {
        FSEL16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 17"]
    #[inline(always)]
    pub fn fsel17(&self) -> FSEL17_R {
        FSEL17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 18"]
    #[inline(always)]
    pub fn fsel18(&self) -> FSEL18_R {
        FSEL18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 19"]
    #[inline(always)]
    pub fn fsel19(&self) -> FSEL19_R {
        FSEL19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 10"]
    #[inline(always)]
    #[must_use]
    pub fn fsel10(&mut self) -> FSEL10_W<0> {
        FSEL10_W::new(self)
    }
    #[doc = "Bits 3:5 - Function Select 11"]
    #[inline(always)]
    #[must_use]
    pub fn fsel11(&mut self) -> FSEL11_W<3> {
        FSEL11_W::new(self)
    }
    #[doc = "Bits 6:8 - Function Select 12"]
    #[inline(always)]
    #[must_use]
    pub fn fsel12(&mut self) -> FSEL12_W<6> {
        FSEL12_W::new(self)
    }
    #[doc = "Bits 9:11 - Function Select 13"]
    #[inline(always)]
    #[must_use]
    pub fn fsel13(&mut self) -> FSEL13_W<9> {
        FSEL13_W::new(self)
    }
    #[doc = "Bits 12:14 - Function Select 14"]
    #[inline(always)]
    #[must_use]
    pub fn fsel14(&mut self) -> FSEL14_W<12> {
        FSEL14_W::new(self)
    }
    #[doc = "Bits 15:17 - Function Select 15"]
    #[inline(always)]
    #[must_use]
    pub fn fsel15(&mut self) -> FSEL15_W<15> {
        FSEL15_W::new(self)
    }
    #[doc = "Bits 18:20 - Function Select 16"]
    #[inline(always)]
    #[must_use]
    pub fn fsel16(&mut self) -> FSEL16_W<18> {
        FSEL16_W::new(self)
    }
    #[doc = "Bits 21:23 - Function Select 17"]
    #[inline(always)]
    #[must_use]
    pub fn fsel17(&mut self) -> FSEL17_W<21> {
        FSEL17_W::new(self)
    }
    #[doc = "Bits 24:26 - Function Select 18"]
    #[inline(always)]
    #[must_use]
    pub fn fsel18(&mut self) -> FSEL18_W<24> {
        FSEL18_W::new(self)
    }
    #[doc = "Bits 27:29 - Function Select 19"]
    #[inline(always)]
    #[must_use]
    pub fn fsel19(&mut self) -> FSEL19_W<27> {
        FSEL19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Function Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpfsel1](index.html) module"]
pub struct GPFSEL1_SPEC;
impl crate::RegisterSpec for GPFSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpfsel1::R](R) reader structure"]
impl crate::Readable for GPFSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpfsel1::W](W) writer structure"]
impl crate::Writable for GPFSEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

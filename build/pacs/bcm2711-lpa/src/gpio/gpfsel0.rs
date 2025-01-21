#[doc = "Register `GPFSEL0` reader"]
pub type R = crate::R<GPFSEL0_SPEC>;
#[doc = "Register `GPFSEL0` writer"]
pub type W = crate::W<GPFSEL0_SPEC>;
#[doc = "Field `FSEL0` reader - Function Select 0"]
pub type FSEL0_R = crate::FieldReader<FSEL0_A>;
#[doc = "Function Select 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL0_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SDA0"]
    SDA0 = 4,
    #[doc = "5: Pin is connected to SA5"]
    SA5 = 5,
    #[doc = "6: Pin is connected to PCLK"]
    PCLK = 6,
    #[doc = "7: Pin is connected to SPI3_CE0_N"]
    SPI3_CE0_N = 7,
    #[doc = "3: Pin is connected to TXD2"]
    TXD2 = 3,
    #[doc = "2: Pin is connected to SDA6"]
    SDA6 = 2,
}
impl From<FSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL0_A {
    type Ux = u8;
}
impl FSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL0_A {
        match self.bits {
            0 => FSEL0_A::INPUT,
            1 => FSEL0_A::OUTPUT,
            4 => FSEL0_A::SDA0,
            5 => FSEL0_A::SA5,
            6 => FSEL0_A::PCLK,
            7 => FSEL0_A::SPI3_CE0_N,
            3 => FSEL0_A::TXD2,
            2 => FSEL0_A::SDA6,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL0_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL0_A::OUTPUT
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == FSEL0_A::SDA0
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn is_sa5(&self) -> bool {
        *self == FSEL0_A::SA5
    }
    #[doc = "Pin is connected to PCLK"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == FSEL0_A::PCLK
    }
    #[doc = "Pin is connected to SPI3_CE0_N"]
    #[inline(always)]
    pub fn is_spi3_ce0_n(&self) -> bool {
        *self == FSEL0_A::SPI3_CE0_N
    }
    #[doc = "Pin is connected to TXD2"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == FSEL0_A::TXD2
    }
    #[doc = "Pin is connected to SDA6"]
    #[inline(always)]
    pub fn is_sda6(&self) -> bool {
        *self == FSEL0_A::SDA6
    }
}
#[doc = "Field `FSEL0` writer - Function Select 0"]
pub type FSEL0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL0_A>;
impl<'a, REG> FSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::OUTPUT)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::SDA0)
    }
    #[doc = "Pin is connected to SA5"]
    #[inline(always)]
    pub fn sa5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::SA5)
    }
    #[doc = "Pin is connected to PCLK"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::PCLK)
    }
    #[doc = "Pin is connected to SPI3_CE0_N"]
    #[inline(always)]
    pub fn spi3_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::SPI3_CE0_N)
    }
    #[doc = "Pin is connected to TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::TXD2)
    }
    #[doc = "Pin is connected to SDA6"]
    #[inline(always)]
    pub fn sda6(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::SDA6)
    }
}
#[doc = "Field `FSEL1` reader - Function Select 1"]
pub type FSEL1_R = crate::FieldReader<FSEL1_A>;
#[doc = "Function Select 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL1_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SCL0"]
    SCL0 = 4,
    #[doc = "5: Pin is connected to SA4"]
    SA4 = 5,
    #[doc = "6: Pin is connected to DE"]
    DE = 6,
    #[doc = "7: Pin is connected to SPI3_MISO"]
    SPI3_MISO = 7,
    #[doc = "3: Pin is connected to RXD2"]
    RXD2 = 3,
    #[doc = "2: Pin is connected to SCL6"]
    SCL6 = 2,
}
impl From<FSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL1_A {
    type Ux = u8;
}
impl FSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL1_A {
        match self.bits {
            0 => FSEL1_A::INPUT,
            1 => FSEL1_A::OUTPUT,
            4 => FSEL1_A::SCL0,
            5 => FSEL1_A::SA4,
            6 => FSEL1_A::DE,
            7 => FSEL1_A::SPI3_MISO,
            3 => FSEL1_A::RXD2,
            2 => FSEL1_A::SCL6,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL1_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL1_A::OUTPUT
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == FSEL1_A::SCL0
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn is_sa4(&self) -> bool {
        *self == FSEL1_A::SA4
    }
    #[doc = "Pin is connected to DE"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == FSEL1_A::DE
    }
    #[doc = "Pin is connected to SPI3_MISO"]
    #[inline(always)]
    pub fn is_spi3_miso(&self) -> bool {
        *self == FSEL1_A::SPI3_MISO
    }
    #[doc = "Pin is connected to RXD2"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == FSEL1_A::RXD2
    }
    #[doc = "Pin is connected to SCL6"]
    #[inline(always)]
    pub fn is_scl6(&self) -> bool {
        *self == FSEL1_A::SCL6
    }
}
#[doc = "Field `FSEL1` writer - Function Select 1"]
pub type FSEL1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL1_A>;
impl<'a, REG> FSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::OUTPUT)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::SCL0)
    }
    #[doc = "Pin is connected to SA4"]
    #[inline(always)]
    pub fn sa4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::SA4)
    }
    #[doc = "Pin is connected to DE"]
    #[inline(always)]
    pub fn de(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::DE)
    }
    #[doc = "Pin is connected to SPI3_MISO"]
    #[inline(always)]
    pub fn spi3_miso(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::SPI3_MISO)
    }
    #[doc = "Pin is connected to RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::RXD2)
    }
    #[doc = "Pin is connected to SCL6"]
    #[inline(always)]
    pub fn scl6(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::SCL6)
    }
}
#[doc = "Field `FSEL2` reader - Function Select 2"]
pub type FSEL2_R = crate::FieldReader<FSEL2_A>;
#[doc = "Function Select 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL2_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SDA1"]
    SDA1 = 4,
    #[doc = "5: Pin is connected to SA3"]
    SA3 = 5,
    #[doc = "6: Pin is connected to LCD_VSYNC"]
    LCD_VSYNC = 6,
    #[doc = "7: Pin is connected to SPI3_MOSI"]
    SPI3_MOSI = 7,
    #[doc = "3: Pin is connected to CTS2"]
    CTS2 = 3,
    #[doc = "2: Pin is connected to SDA3"]
    SDA3 = 2,
}
impl From<FSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL2_A {
    type Ux = u8;
}
impl FSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL2_A {
        match self.bits {
            0 => FSEL2_A::INPUT,
            1 => FSEL2_A::OUTPUT,
            4 => FSEL2_A::SDA1,
            5 => FSEL2_A::SA3,
            6 => FSEL2_A::LCD_VSYNC,
            7 => FSEL2_A::SPI3_MOSI,
            3 => FSEL2_A::CTS2,
            2 => FSEL2_A::SDA3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL2_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL2_A::OUTPUT
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == FSEL2_A::SDA1
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn is_sa3(&self) -> bool {
        *self == FSEL2_A::SA3
    }
    #[doc = "Pin is connected to LCD_VSYNC"]
    #[inline(always)]
    pub fn is_lcd_vsync(&self) -> bool {
        *self == FSEL2_A::LCD_VSYNC
    }
    #[doc = "Pin is connected to SPI3_MOSI"]
    #[inline(always)]
    pub fn is_spi3_mosi(&self) -> bool {
        *self == FSEL2_A::SPI3_MOSI
    }
    #[doc = "Pin is connected to CTS2"]
    #[inline(always)]
    pub fn is_cts2(&self) -> bool {
        *self == FSEL2_A::CTS2
    }
    #[doc = "Pin is connected to SDA3"]
    #[inline(always)]
    pub fn is_sda3(&self) -> bool {
        *self == FSEL2_A::SDA3
    }
}
#[doc = "Field `FSEL2` writer - Function Select 2"]
pub type FSEL2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL2_A>;
impl<'a, REG> FSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::OUTPUT)
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::SDA1)
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn sa3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::SA3)
    }
    #[doc = "Pin is connected to LCD_VSYNC"]
    #[inline(always)]
    pub fn lcd_vsync(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::LCD_VSYNC)
    }
    #[doc = "Pin is connected to SPI3_MOSI"]
    #[inline(always)]
    pub fn spi3_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::SPI3_MOSI)
    }
    #[doc = "Pin is connected to CTS2"]
    #[inline(always)]
    pub fn cts2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::CTS2)
    }
    #[doc = "Pin is connected to SDA3"]
    #[inline(always)]
    pub fn sda3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::SDA3)
    }
}
#[doc = "Field `FSEL3` reader - Function Select 3"]
pub type FSEL3_R = crate::FieldReader<FSEL3_A>;
#[doc = "Function Select 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL3_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SCL1"]
    SCL1 = 4,
    #[doc = "5: Pin is connected to SA2"]
    SA2 = 5,
    #[doc = "6: Pin is connected to LCD_HSYNC"]
    LCD_HSYNC = 6,
    #[doc = "7: Pin is connected to SPI3_SCLK"]
    SPI3_SCLK = 7,
    #[doc = "3: Pin is connected to RTS2"]
    RTS2 = 3,
    #[doc = "2: Pin is connected to SCL3"]
    SCL3 = 2,
}
impl From<FSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL3_A {
    type Ux = u8;
}
impl FSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL3_A {
        match self.bits {
            0 => FSEL3_A::INPUT,
            1 => FSEL3_A::OUTPUT,
            4 => FSEL3_A::SCL1,
            5 => FSEL3_A::SA2,
            6 => FSEL3_A::LCD_HSYNC,
            7 => FSEL3_A::SPI3_SCLK,
            3 => FSEL3_A::RTS2,
            2 => FSEL3_A::SCL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL3_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL3_A::OUTPUT
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == FSEL3_A::SCL1
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn is_sa2(&self) -> bool {
        *self == FSEL3_A::SA2
    }
    #[doc = "Pin is connected to LCD_HSYNC"]
    #[inline(always)]
    pub fn is_lcd_hsync(&self) -> bool {
        *self == FSEL3_A::LCD_HSYNC
    }
    #[doc = "Pin is connected to SPI3_SCLK"]
    #[inline(always)]
    pub fn is_spi3_sclk(&self) -> bool {
        *self == FSEL3_A::SPI3_SCLK
    }
    #[doc = "Pin is connected to RTS2"]
    #[inline(always)]
    pub fn is_rts2(&self) -> bool {
        *self == FSEL3_A::RTS2
    }
    #[doc = "Pin is connected to SCL3"]
    #[inline(always)]
    pub fn is_scl3(&self) -> bool {
        *self == FSEL3_A::SCL3
    }
}
#[doc = "Field `FSEL3` writer - Function Select 3"]
pub type FSEL3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL3_A>;
impl<'a, REG> FSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::OUTPUT)
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::SCL1)
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn sa2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::SA2)
    }
    #[doc = "Pin is connected to LCD_HSYNC"]
    #[inline(always)]
    pub fn lcd_hsync(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::LCD_HSYNC)
    }
    #[doc = "Pin is connected to SPI3_SCLK"]
    #[inline(always)]
    pub fn spi3_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::SPI3_SCLK)
    }
    #[doc = "Pin is connected to RTS2"]
    #[inline(always)]
    pub fn rts2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::RTS2)
    }
    #[doc = "Pin is connected to SCL3"]
    #[inline(always)]
    pub fn scl3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::SCL3)
    }
}
#[doc = "Field `FSEL4` reader - Function Select 4"]
pub type FSEL4_R = crate::FieldReader<FSEL4_A>;
#[doc = "Function Select 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL4_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    GPCLK0 = 4,
    #[doc = "5: Pin is connected to SA1"]
    SA1 = 5,
    #[doc = "6: Pin is connected to DPI_D0"]
    DPI_D0 = 6,
    #[doc = "7: Pin is connected to SPI4_CE0_N"]
    SPI4_CE0_N = 7,
    #[doc = "3: Pin is connected to TXD3"]
    TXD3 = 3,
    #[doc = "2: Pin is connected to SDA3"]
    SDA3 = 2,
}
impl From<FSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL4_A {
    type Ux = u8;
}
impl FSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL4_A {
        match self.bits {
            0 => FSEL4_A::INPUT,
            1 => FSEL4_A::OUTPUT,
            4 => FSEL4_A::GPCLK0,
            5 => FSEL4_A::SA1,
            6 => FSEL4_A::DPI_D0,
            7 => FSEL4_A::SPI4_CE0_N,
            3 => FSEL4_A::TXD3,
            2 => FSEL4_A::SDA3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL4_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL4_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == FSEL4_A::GPCLK0
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn is_sa1(&self) -> bool {
        *self == FSEL4_A::SA1
    }
    #[doc = "Pin is connected to DPI_D0"]
    #[inline(always)]
    pub fn is_dpi_d0(&self) -> bool {
        *self == FSEL4_A::DPI_D0
    }
    #[doc = "Pin is connected to SPI4_CE0_N"]
    #[inline(always)]
    pub fn is_spi4_ce0_n(&self) -> bool {
        *self == FSEL4_A::SPI4_CE0_N
    }
    #[doc = "Pin is connected to TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == FSEL4_A::TXD3
    }
    #[doc = "Pin is connected to SDA3"]
    #[inline(always)]
    pub fn is_sda3(&self) -> bool {
        *self == FSEL4_A::SDA3
    }
}
#[doc = "Field `FSEL4` writer - Function Select 4"]
pub type FSEL4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL4_A>;
impl<'a, REG> FSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::GPCLK0)
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn sa1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::SA1)
    }
    #[doc = "Pin is connected to DPI_D0"]
    #[inline(always)]
    pub fn dpi_d0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::DPI_D0)
    }
    #[doc = "Pin is connected to SPI4_CE0_N"]
    #[inline(always)]
    pub fn spi4_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::SPI4_CE0_N)
    }
    #[doc = "Pin is connected to TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::TXD3)
    }
    #[doc = "Pin is connected to SDA3"]
    #[inline(always)]
    pub fn sda3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::SDA3)
    }
}
#[doc = "Field `FSEL5` reader - Function Select 5"]
pub type FSEL5_R = crate::FieldReader<FSEL5_A>;
#[doc = "Function Select 5"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL5_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    GPCLK1 = 4,
    #[doc = "5: Pin is connected to SA0"]
    SA0 = 5,
    #[doc = "6: Pin is connected to DPI_D1"]
    DPI_D1 = 6,
    #[doc = "7: Pin is connected to SPI4_MISO"]
    SPI4_MISO = 7,
    #[doc = "3: Pin is connected to RXD3"]
    RXD3 = 3,
    #[doc = "2: Pin is connected to SCL3"]
    SCL3 = 2,
}
impl From<FSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL5_A {
    type Ux = u8;
}
impl FSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL5_A {
        match self.bits {
            0 => FSEL5_A::INPUT,
            1 => FSEL5_A::OUTPUT,
            4 => FSEL5_A::GPCLK1,
            5 => FSEL5_A::SA0,
            6 => FSEL5_A::DPI_D1,
            7 => FSEL5_A::SPI4_MISO,
            3 => FSEL5_A::RXD3,
            2 => FSEL5_A::SCL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL5_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL5_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == FSEL5_A::GPCLK1
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn is_sa0(&self) -> bool {
        *self == FSEL5_A::SA0
    }
    #[doc = "Pin is connected to DPI_D1"]
    #[inline(always)]
    pub fn is_dpi_d1(&self) -> bool {
        *self == FSEL5_A::DPI_D1
    }
    #[doc = "Pin is connected to SPI4_MISO"]
    #[inline(always)]
    pub fn is_spi4_miso(&self) -> bool {
        *self == FSEL5_A::SPI4_MISO
    }
    #[doc = "Pin is connected to RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == FSEL5_A::RXD3
    }
    #[doc = "Pin is connected to SCL3"]
    #[inline(always)]
    pub fn is_scl3(&self) -> bool {
        *self == FSEL5_A::SCL3
    }
}
#[doc = "Field `FSEL5` writer - Function Select 5"]
pub type FSEL5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL5_A>;
impl<'a, REG> FSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::GPCLK1)
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn sa0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::SA0)
    }
    #[doc = "Pin is connected to DPI_D1"]
    #[inline(always)]
    pub fn dpi_d1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::DPI_D1)
    }
    #[doc = "Pin is connected to SPI4_MISO"]
    #[inline(always)]
    pub fn spi4_miso(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::SPI4_MISO)
    }
    #[doc = "Pin is connected to RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::RXD3)
    }
    #[doc = "Pin is connected to SCL3"]
    #[inline(always)]
    pub fn scl3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::SCL3)
    }
}
#[doc = "Field `FSEL6` reader - Function Select 6"]
pub type FSEL6_R = crate::FieldReader<FSEL6_A>;
#[doc = "Function Select 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL6_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK2"]
    GPCLK2 = 4,
    #[doc = "5: Pin is connected to SOE_N"]
    SOE_N = 5,
    #[doc = "6: Pin is connected to DPI_D2"]
    DPI_D2 = 6,
    #[doc = "7: Pin is connected to SPI4_MOSI"]
    SPI4_MOSI = 7,
    #[doc = "3: Pin is connected to CTS3"]
    CTS3 = 3,
    #[doc = "2: Pin is connected to SDA4"]
    SDA4 = 2,
}
impl From<FSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL6_A {
    type Ux = u8;
}
impl FSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL6_A {
        match self.bits {
            0 => FSEL6_A::INPUT,
            1 => FSEL6_A::OUTPUT,
            4 => FSEL6_A::GPCLK2,
            5 => FSEL6_A::SOE_N,
            6 => FSEL6_A::DPI_D2,
            7 => FSEL6_A::SPI4_MOSI,
            3 => FSEL6_A::CTS3,
            2 => FSEL6_A::SDA4,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL6_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL6_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn is_gpclk2(&self) -> bool {
        *self == FSEL6_A::GPCLK2
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn is_soe_n(&self) -> bool {
        *self == FSEL6_A::SOE_N
    }
    #[doc = "Pin is connected to DPI_D2"]
    #[inline(always)]
    pub fn is_dpi_d2(&self) -> bool {
        *self == FSEL6_A::DPI_D2
    }
    #[doc = "Pin is connected to SPI4_MOSI"]
    #[inline(always)]
    pub fn is_spi4_mosi(&self) -> bool {
        *self == FSEL6_A::SPI4_MOSI
    }
    #[doc = "Pin is connected to CTS3"]
    #[inline(always)]
    pub fn is_cts3(&self) -> bool {
        *self == FSEL6_A::CTS3
    }
    #[doc = "Pin is connected to SDA4"]
    #[inline(always)]
    pub fn is_sda4(&self) -> bool {
        *self == FSEL6_A::SDA4
    }
}
#[doc = "Field `FSEL6` writer - Function Select 6"]
pub type FSEL6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL6_A>;
impl<'a, REG> FSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn gpclk2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::GPCLK2)
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn soe_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::SOE_N)
    }
    #[doc = "Pin is connected to DPI_D2"]
    #[inline(always)]
    pub fn dpi_d2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::DPI_D2)
    }
    #[doc = "Pin is connected to SPI4_MOSI"]
    #[inline(always)]
    pub fn spi4_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::SPI4_MOSI)
    }
    #[doc = "Pin is connected to CTS3"]
    #[inline(always)]
    pub fn cts3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::CTS3)
    }
    #[doc = "Pin is connected to SDA4"]
    #[inline(always)]
    pub fn sda4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::SDA4)
    }
}
#[doc = "Field `FSEL7` reader - Function Select 7"]
pub type FSEL7_R = crate::FieldReader<FSEL7_A>;
#[doc = "Function Select 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL7_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_CE1_N"]
    SPI0_CE1_N = 4,
    #[doc = "5: Pin is connected to SWE_N"]
    SWE_N = 5,
    #[doc = "6: Pin is connected to DPI_D3"]
    DPI_D3 = 6,
    #[doc = "7: Pin is connected to SPI4_SCLK"]
    SPI4_SCLK = 7,
    #[doc = "3: Pin is connected to RTS3"]
    RTS3 = 3,
    #[doc = "2: Pin is connected to SCL4"]
    SCL4 = 2,
}
impl From<FSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL7_A {
    type Ux = u8;
}
impl FSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL7_A {
        match self.bits {
            0 => FSEL7_A::INPUT,
            1 => FSEL7_A::OUTPUT,
            4 => FSEL7_A::SPI0_CE1_N,
            5 => FSEL7_A::SWE_N,
            6 => FSEL7_A::DPI_D3,
            7 => FSEL7_A::SPI4_SCLK,
            3 => FSEL7_A::RTS3,
            2 => FSEL7_A::SCL4,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL7_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL7_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn is_spi0_ce1_n(&self) -> bool {
        *self == FSEL7_A::SPI0_CE1_N
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn is_swe_n(&self) -> bool {
        *self == FSEL7_A::SWE_N
    }
    #[doc = "Pin is connected to DPI_D3"]
    #[inline(always)]
    pub fn is_dpi_d3(&self) -> bool {
        *self == FSEL7_A::DPI_D3
    }
    #[doc = "Pin is connected to SPI4_SCLK"]
    #[inline(always)]
    pub fn is_spi4_sclk(&self) -> bool {
        *self == FSEL7_A::SPI4_SCLK
    }
    #[doc = "Pin is connected to RTS3"]
    #[inline(always)]
    pub fn is_rts3(&self) -> bool {
        *self == FSEL7_A::RTS3
    }
    #[doc = "Pin is connected to SCL4"]
    #[inline(always)]
    pub fn is_scl4(&self) -> bool {
        *self == FSEL7_A::SCL4
    }
}
#[doc = "Field `FSEL7` writer - Function Select 7"]
pub type FSEL7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL7_A>;
impl<'a, REG> FSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn spi0_ce1_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::SPI0_CE1_N)
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn swe_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::SWE_N)
    }
    #[doc = "Pin is connected to DPI_D3"]
    #[inline(always)]
    pub fn dpi_d3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::DPI_D3)
    }
    #[doc = "Pin is connected to SPI4_SCLK"]
    #[inline(always)]
    pub fn spi4_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::SPI4_SCLK)
    }
    #[doc = "Pin is connected to RTS3"]
    #[inline(always)]
    pub fn rts3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::RTS3)
    }
    #[doc = "Pin is connected to SCL4"]
    #[inline(always)]
    pub fn scl4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::SCL4)
    }
}
#[doc = "Field `FSEL8` reader - Function Select 8"]
pub type FSEL8_R = crate::FieldReader<FSEL8_A>;
#[doc = "Function Select 8"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL8_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_CE0_N"]
    SPI0_CE0_N = 4,
    #[doc = "5: Pin is connected to SD0"]
    SD0 = 5,
    #[doc = "6: Pin is connected to DPI_D4"]
    DPI_D4 = 6,
    #[doc = "7: Pin is connected to BSCSL_CE_N"]
    BSCSL_CE_N = 7,
    #[doc = "3: Pin is connected to TXD4"]
    TXD4 = 3,
    #[doc = "2: Pin is connected to SDA4"]
    SDA4 = 2,
}
impl From<FSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL8_A {
    type Ux = u8;
}
impl FSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL8_A {
        match self.bits {
            0 => FSEL8_A::INPUT,
            1 => FSEL8_A::OUTPUT,
            4 => FSEL8_A::SPI0_CE0_N,
            5 => FSEL8_A::SD0,
            6 => FSEL8_A::DPI_D4,
            7 => FSEL8_A::BSCSL_CE_N,
            3 => FSEL8_A::TXD4,
            2 => FSEL8_A::SDA4,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL8_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL8_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn is_spi0_ce0_n(&self) -> bool {
        *self == FSEL8_A::SPI0_CE0_N
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn is_sd0(&self) -> bool {
        *self == FSEL8_A::SD0
    }
    #[doc = "Pin is connected to DPI_D4"]
    #[inline(always)]
    pub fn is_dpi_d4(&self) -> bool {
        *self == FSEL8_A::DPI_D4
    }
    #[doc = "Pin is connected to BSCSL_CE_N"]
    #[inline(always)]
    pub fn is_bscsl_ce_n(&self) -> bool {
        *self == FSEL8_A::BSCSL_CE_N
    }
    #[doc = "Pin is connected to TXD4"]
    #[inline(always)]
    pub fn is_txd4(&self) -> bool {
        *self == FSEL8_A::TXD4
    }
    #[doc = "Pin is connected to SDA4"]
    #[inline(always)]
    pub fn is_sda4(&self) -> bool {
        *self == FSEL8_A::SDA4
    }
}
#[doc = "Field `FSEL8` writer - Function Select 8"]
pub type FSEL8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL8_A>;
impl<'a, REG> FSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn spi0_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::SPI0_CE0_N)
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn sd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::SD0)
    }
    #[doc = "Pin is connected to DPI_D4"]
    #[inline(always)]
    pub fn dpi_d4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::DPI_D4)
    }
    #[doc = "Pin is connected to BSCSL_CE_N"]
    #[inline(always)]
    pub fn bscsl_ce_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::BSCSL_CE_N)
    }
    #[doc = "Pin is connected to TXD4"]
    #[inline(always)]
    pub fn txd4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::TXD4)
    }
    #[doc = "Pin is connected to SDA4"]
    #[inline(always)]
    pub fn sda4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::SDA4)
    }
}
#[doc = "Field `FSEL9` reader - Function Select 9"]
pub type FSEL9_R = crate::FieldReader<FSEL9_A>;
#[doc = "Function Select 9"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL9_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_MISO"]
    SPI0_MISO = 4,
    #[doc = "5: Pin is connected to SD1"]
    SD1 = 5,
    #[doc = "6: Pin is connected to DPI_D5"]
    DPI_D5 = 6,
    #[doc = "7: Pin is connected to BSCSL_MISO"]
    BSCSL_MISO = 7,
    #[doc = "3: Pin is connected to RXD4"]
    RXD4 = 3,
    #[doc = "2: Pin is connected to SCL4"]
    SCL4 = 2,
}
impl From<FSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL9_A {
    type Ux = u8;
}
impl FSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL9_A {
        match self.bits {
            0 => FSEL9_A::INPUT,
            1 => FSEL9_A::OUTPUT,
            4 => FSEL9_A::SPI0_MISO,
            5 => FSEL9_A::SD1,
            6 => FSEL9_A::DPI_D5,
            7 => FSEL9_A::BSCSL_MISO,
            3 => FSEL9_A::RXD4,
            2 => FSEL9_A::SCL4,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL9_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL9_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn is_spi0_miso(&self) -> bool {
        *self == FSEL9_A::SPI0_MISO
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn is_sd1(&self) -> bool {
        *self == FSEL9_A::SD1
    }
    #[doc = "Pin is connected to DPI_D5"]
    #[inline(always)]
    pub fn is_dpi_d5(&self) -> bool {
        *self == FSEL9_A::DPI_D5
    }
    #[doc = "Pin is connected to BSCSL_MISO"]
    #[inline(always)]
    pub fn is_bscsl_miso(&self) -> bool {
        *self == FSEL9_A::BSCSL_MISO
    }
    #[doc = "Pin is connected to RXD4"]
    #[inline(always)]
    pub fn is_rxd4(&self) -> bool {
        *self == FSEL9_A::RXD4
    }
    #[doc = "Pin is connected to SCL4"]
    #[inline(always)]
    pub fn is_scl4(&self) -> bool {
        *self == FSEL9_A::SCL4
    }
}
#[doc = "Field `FSEL9` writer - Function Select 9"]
pub type FSEL9_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL9_A>;
impl<'a, REG> FSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn spi0_miso(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::SPI0_MISO)
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn sd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::SD1)
    }
    #[doc = "Pin is connected to DPI_D5"]
    #[inline(always)]
    pub fn dpi_d5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::DPI_D5)
    }
    #[doc = "Pin is connected to BSCSL_MISO"]
    #[inline(always)]
    pub fn bscsl_miso(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::BSCSL_MISO)
    }
    #[doc = "Pin is connected to RXD4"]
    #[inline(always)]
    pub fn rxd4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::RXD4)
    }
    #[doc = "Pin is connected to SCL4"]
    #[inline(always)]
    pub fn scl4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::SCL4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 0"]
    #[inline(always)]
    pub fn fsel0(&self) -> FSEL0_R {
        FSEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 1"]
    #[inline(always)]
    pub fn fsel1(&self) -> FSEL1_R {
        FSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 2"]
    #[inline(always)]
    pub fn fsel2(&self) -> FSEL2_R {
        FSEL2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 3"]
    #[inline(always)]
    pub fn fsel3(&self) -> FSEL3_R {
        FSEL3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 4"]
    #[inline(always)]
    pub fn fsel4(&self) -> FSEL4_R {
        FSEL4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 5"]
    #[inline(always)]
    pub fn fsel5(&self) -> FSEL5_R {
        FSEL5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 6"]
    #[inline(always)]
    pub fn fsel6(&self) -> FSEL6_R {
        FSEL6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 7"]
    #[inline(always)]
    pub fn fsel7(&self) -> FSEL7_R {
        FSEL7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 8"]
    #[inline(always)]
    pub fn fsel8(&self) -> FSEL8_R {
        FSEL8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 9"]
    #[inline(always)]
    pub fn fsel9(&self) -> FSEL9_R {
        FSEL9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL0")
            .field("fsel0", &format_args!("{}", self.fsel0().bits()))
            .field("fsel1", &format_args!("{}", self.fsel1().bits()))
            .field("fsel2", &format_args!("{}", self.fsel2().bits()))
            .field("fsel3", &format_args!("{}", self.fsel3().bits()))
            .field("fsel4", &format_args!("{}", self.fsel4().bits()))
            .field("fsel5", &format_args!("{}", self.fsel5().bits()))
            .field("fsel6", &format_args!("{}", self.fsel6().bits()))
            .field("fsel7", &format_args!("{}", self.fsel7().bits()))
            .field("fsel8", &format_args!("{}", self.fsel8().bits()))
            .field("fsel9", &format_args!("{}", self.fsel9().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPFSEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn fsel0(&mut self) -> FSEL0_W<GPFSEL0_SPEC> {
        FSEL0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn fsel1(&mut self) -> FSEL1_W<GPFSEL0_SPEC> {
        FSEL1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn fsel2(&mut self) -> FSEL2_W<GPFSEL0_SPEC> {
        FSEL2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 3"]
    #[inline(always)]
    #[must_use]
    pub fn fsel3(&mut self) -> FSEL3_W<GPFSEL0_SPEC> {
        FSEL3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 4"]
    #[inline(always)]
    #[must_use]
    pub fn fsel4(&mut self) -> FSEL4_W<GPFSEL0_SPEC> {
        FSEL4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 5"]
    #[inline(always)]
    #[must_use]
    pub fn fsel5(&mut self) -> FSEL5_W<GPFSEL0_SPEC> {
        FSEL5_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 6"]
    #[inline(always)]
    #[must_use]
    pub fn fsel6(&mut self) -> FSEL6_W<GPFSEL0_SPEC> {
        FSEL6_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 7"]
    #[inline(always)]
    #[must_use]
    pub fn fsel7(&mut self) -> FSEL7_W<GPFSEL0_SPEC> {
        FSEL7_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 8"]
    #[inline(always)]
    #[must_use]
    pub fn fsel8(&mut self) -> FSEL8_W<GPFSEL0_SPEC> {
        FSEL8_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 9"]
    #[inline(always)]
    #[must_use]
    pub fn fsel9(&mut self) -> FSEL9_W<GPFSEL0_SPEC> {
        FSEL9_W::new(self, 27)
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
#[doc = "GPIO Function Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPFSEL0_SPEC;
impl crate::RegisterSpec for GPFSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel0::R`](R) reader structure"]
impl crate::Readable for GPFSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpfsel0::W`](W) writer structure"]
impl crate::Writable for GPFSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

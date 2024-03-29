#[doc = "Register `GPFSEL4` reader"]
pub type R = crate::R<GPFSEL4_SPEC>;
#[doc = "Register `GPFSEL4` writer"]
pub type W = crate::W<GPFSEL4_SPEC>;
#[doc = "Field `FSEL40` reader - Function Select 40"]
pub type FSEL40_R = crate::FieldReader<FSEL40_A>;
#[doc = "Function Select 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL40_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PWM1_0"]
    PWM1_0 = 4,
    #[doc = "5: Pin is connected to SD4"]
    SD4 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT4"]
    SD1_DAT4 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to TXD1"]
    TXD1 = 2,
}
impl From<FSEL40_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL40_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL40_A {
    type Ux = u8;
}
impl FSEL40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL40_A {
        match self.bits {
            0 => FSEL40_A::INPUT,
            1 => FSEL40_A::OUTPUT,
            4 => FSEL40_A::PWM1_0,
            5 => FSEL40_A::SD4,
            6 => FSEL40_A::RESERVED2,
            7 => FSEL40_A::SD1_DAT4,
            3 => FSEL40_A::RESERVED4,
            2 => FSEL40_A::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL40_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL40_A::OUTPUT
    }
    #[doc = "Pin is connected to PWM1_0"]
    #[inline(always)]
    pub fn is_pwm1_0(&self) -> bool {
        *self == FSEL40_A::PWM1_0
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn is_sd4(&self) -> bool {
        *self == FSEL40_A::SD4
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL40_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT4"]
    #[inline(always)]
    pub fn is_sd1_dat4(&self) -> bool {
        *self == FSEL40_A::SD1_DAT4
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL40_A::RESERVED4
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == FSEL40_A::TXD1
    }
}
#[doc = "Field `FSEL40` writer - Function Select 40"]
pub type FSEL40_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL40_A>;
impl<'a, REG> FSEL40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::OUTPUT)
    }
    #[doc = "Pin is connected to PWM1_0"]
    #[inline(always)]
    pub fn pwm1_0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::PWM1_0)
    }
    #[doc = "Pin is connected to SD4"]
    #[inline(always)]
    pub fn sd4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::SD4)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT4"]
    #[inline(always)]
    pub fn sd1_dat4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::SD1_DAT4)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::RESERVED4)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL40_A::TXD1)
    }
}
#[doc = "Field `FSEL41` reader - Function Select 41"]
pub type FSEL41_R = crate::FieldReader<FSEL41_A>;
#[doc = "Function Select 41"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL41_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PWM1_1"]
    PWM1_1 = 4,
    #[doc = "5: Pin is connected to SD5"]
    SD5 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT5"]
    SD1_DAT5 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to RXD1"]
    RXD1 = 2,
}
impl From<FSEL41_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL41_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL41_A {
    type Ux = u8;
}
impl FSEL41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL41_A {
        match self.bits {
            0 => FSEL41_A::INPUT,
            1 => FSEL41_A::OUTPUT,
            4 => FSEL41_A::PWM1_1,
            5 => FSEL41_A::SD5,
            6 => FSEL41_A::RESERVED2,
            7 => FSEL41_A::SD1_DAT5,
            3 => FSEL41_A::RESERVED4,
            2 => FSEL41_A::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL41_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL41_A::OUTPUT
    }
    #[doc = "Pin is connected to PWM1_1"]
    #[inline(always)]
    pub fn is_pwm1_1(&self) -> bool {
        *self == FSEL41_A::PWM1_1
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn is_sd5(&self) -> bool {
        *self == FSEL41_A::SD5
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL41_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT5"]
    #[inline(always)]
    pub fn is_sd1_dat5(&self) -> bool {
        *self == FSEL41_A::SD1_DAT5
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL41_A::RESERVED4
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == FSEL41_A::RXD1
    }
}
#[doc = "Field `FSEL41` writer - Function Select 41"]
pub type FSEL41_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL41_A>;
impl<'a, REG> FSEL41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::OUTPUT)
    }
    #[doc = "Pin is connected to PWM1_1"]
    #[inline(always)]
    pub fn pwm1_1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::PWM1_1)
    }
    #[doc = "Pin is connected to SD5"]
    #[inline(always)]
    pub fn sd5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::SD5)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT5"]
    #[inline(always)]
    pub fn sd1_dat5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::SD1_DAT5)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::RESERVED4)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL41_A::RXD1)
    }
}
#[doc = "Field `FSEL42` reader - Function Select 42"]
pub type FSEL42_R = crate::FieldReader<FSEL42_A>;
#[doc = "Function Select 42"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL42_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    GPCLK1 = 4,
    #[doc = "5: Pin is connected to SD6"]
    SD6 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT6"]
    SD1_DAT6 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to CTS1"]
    CTS1 = 2,
}
impl From<FSEL42_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL42_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL42_A {
    type Ux = u8;
}
impl FSEL42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL42_A {
        match self.bits {
            0 => FSEL42_A::INPUT,
            1 => FSEL42_A::OUTPUT,
            4 => FSEL42_A::GPCLK1,
            5 => FSEL42_A::SD6,
            6 => FSEL42_A::RESERVED2,
            7 => FSEL42_A::SD1_DAT6,
            3 => FSEL42_A::RESERVED4,
            2 => FSEL42_A::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL42_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL42_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == FSEL42_A::GPCLK1
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn is_sd6(&self) -> bool {
        *self == FSEL42_A::SD6
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL42_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT6"]
    #[inline(always)]
    pub fn is_sd1_dat6(&self) -> bool {
        *self == FSEL42_A::SD1_DAT6
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL42_A::RESERVED4
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == FSEL42_A::CTS1
    }
}
#[doc = "Field `FSEL42` writer - Function Select 42"]
pub type FSEL42_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL42_A>;
impl<'a, REG> FSEL42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::GPCLK1)
    }
    #[doc = "Pin is connected to SD6"]
    #[inline(always)]
    pub fn sd6(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::SD6)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT6"]
    #[inline(always)]
    pub fn sd1_dat6(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::SD1_DAT6)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::RESERVED4)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL42_A::CTS1)
    }
}
#[doc = "Field `FSEL43` reader - Function Select 43"]
pub type FSEL43_R = crate::FieldReader<FSEL43_A>;
#[doc = "Function Select 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL43_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK2"]
    GPCLK2 = 4,
    #[doc = "5: Pin is connected to SD7"]
    SD7 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_DAT7"]
    SD1_DAT7 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to RTS1"]
    RTS1 = 2,
}
impl From<FSEL43_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL43_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL43_A {
    type Ux = u8;
}
impl FSEL43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL43_A {
        match self.bits {
            0 => FSEL43_A::INPUT,
            1 => FSEL43_A::OUTPUT,
            4 => FSEL43_A::GPCLK2,
            5 => FSEL43_A::SD7,
            6 => FSEL43_A::RESERVED2,
            7 => FSEL43_A::SD1_DAT7,
            3 => FSEL43_A::RESERVED4,
            2 => FSEL43_A::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL43_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL43_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn is_gpclk2(&self) -> bool {
        *self == FSEL43_A::GPCLK2
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn is_sd7(&self) -> bool {
        *self == FSEL43_A::SD7
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL43_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT7"]
    #[inline(always)]
    pub fn is_sd1_dat7(&self) -> bool {
        *self == FSEL43_A::SD1_DAT7
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL43_A::RESERVED4
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == FSEL43_A::RTS1
    }
}
#[doc = "Field `FSEL43` writer - Function Select 43"]
pub type FSEL43_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL43_A>;
impl<'a, REG> FSEL43_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK2"]
    #[inline(always)]
    pub fn gpclk2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::GPCLK2)
    }
    #[doc = "Pin is connected to SD7"]
    #[inline(always)]
    pub fn sd7(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::SD7)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT7"]
    #[inline(always)]
    pub fn sd1_dat7(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::SD1_DAT7)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::RESERVED4)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL43_A::RTS1)
    }
}
#[doc = "Field `FSEL44` reader - Function Select 44"]
pub type FSEL44_R = crate::FieldReader<FSEL44_A>;
#[doc = "Function Select 44"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL44_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK1"]
    GPCLK1 = 4,
    #[doc = "5: Pin is connected to SDA0"]
    SDA0 = 5,
    #[doc = "6: Pin is connected to SDA1"]
    SDA1 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to SD_CARD_VOLT"]
    SD_CARD_VOLT = 2,
}
impl From<FSEL44_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL44_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL44_A {
    type Ux = u8;
}
impl FSEL44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL44_A {
        match self.bits {
            0 => FSEL44_A::INPUT,
            1 => FSEL44_A::OUTPUT,
            4 => FSEL44_A::GPCLK1,
            5 => FSEL44_A::SDA0,
            6 => FSEL44_A::SDA1,
            7 => FSEL44_A::RESERVED3,
            3 => FSEL44_A::RESERVED4,
            2 => FSEL44_A::SD_CARD_VOLT,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL44_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL44_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn is_gpclk1(&self) -> bool {
        *self == FSEL44_A::GPCLK1
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn is_sda0(&self) -> bool {
        *self == FSEL44_A::SDA0
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == FSEL44_A::SDA1
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL44_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL44_A::RESERVED4
    }
    #[doc = "Pin is connected to SD_CARD_VOLT"]
    #[inline(always)]
    pub fn is_sd_card_volt(&self) -> bool {
        *self == FSEL44_A::SD_CARD_VOLT
    }
}
#[doc = "Field `FSEL44` writer - Function Select 44"]
pub type FSEL44_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL44_A>;
impl<'a, REG> FSEL44_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK1"]
    #[inline(always)]
    pub fn gpclk1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::GPCLK1)
    }
    #[doc = "Pin is connected to SDA0"]
    #[inline(always)]
    pub fn sda0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::SDA0)
    }
    #[doc = "Pin is connected to SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::SDA1)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::RESERVED4)
    }
    #[doc = "Pin is connected to SD_CARD_VOLT"]
    #[inline(always)]
    pub fn sd_card_volt(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL44_A::SD_CARD_VOLT)
    }
}
#[doc = "Field `FSEL45` reader - Function Select 45"]
pub type FSEL45_R = crate::FieldReader<FSEL45_A>;
#[doc = "Function Select 45"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL45_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to PWM0_1"]
    PWM0_1 = 4,
    #[doc = "5: Pin is connected to SCL0"]
    SCL0 = 5,
    #[doc = "6: Pin is connected to SCL1"]
    SCL1 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to SD_CARD_PWR0"]
    SD_CARD_PWR0 = 2,
}
impl From<FSEL45_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL45_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL45_A {
    type Ux = u8;
}
impl FSEL45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL45_A {
        match self.bits {
            0 => FSEL45_A::INPUT,
            1 => FSEL45_A::OUTPUT,
            4 => FSEL45_A::PWM0_1,
            5 => FSEL45_A::SCL0,
            6 => FSEL45_A::SCL1,
            7 => FSEL45_A::RESERVED3,
            3 => FSEL45_A::RESERVED4,
            2 => FSEL45_A::SD_CARD_PWR0,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL45_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL45_A::OUTPUT
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn is_pwm0_1(&self) -> bool {
        *self == FSEL45_A::PWM0_1
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn is_scl0(&self) -> bool {
        *self == FSEL45_A::SCL0
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == FSEL45_A::SCL1
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL45_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL45_A::RESERVED4
    }
    #[doc = "Pin is connected to SD_CARD_PWR0"]
    #[inline(always)]
    pub fn is_sd_card_pwr0(&self) -> bool {
        *self == FSEL45_A::SD_CARD_PWR0
    }
}
#[doc = "Field `FSEL45` writer - Function Select 45"]
pub type FSEL45_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL45_A>;
impl<'a, REG> FSEL45_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::OUTPUT)
    }
    #[doc = "Pin is connected to PWM0_1"]
    #[inline(always)]
    pub fn pwm0_1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::PWM0_1)
    }
    #[doc = "Pin is connected to SCL0"]
    #[inline(always)]
    pub fn scl0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::SCL0)
    }
    #[doc = "Pin is connected to SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::SCL1)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::RESERVED4)
    }
    #[doc = "Pin is connected to SD_CARD_PWR0"]
    #[inline(always)]
    pub fn sd_card_pwr0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL45_A::SD_CARD_PWR0)
    }
}
#[doc = "Field `FSEL46` reader - Function Select 46"]
pub type FSEL46_R = crate::FieldReader<FSEL46_A>;
#[doc = "Function Select 46"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL46_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL46_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL46_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL46_A {
    type Ux = u8;
}
impl FSEL46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL46_A {
        match self.bits {
            0 => FSEL46_A::INPUT,
            1 => FSEL46_A::OUTPUT,
            4 => FSEL46_A::RESERVED0,
            5 => FSEL46_A::RESERVED1,
            6 => FSEL46_A::RESERVED2,
            7 => FSEL46_A::RESERVED3,
            3 => FSEL46_A::RESERVED4,
            2 => FSEL46_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL46_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL46_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL46_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL46_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL46_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL46_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL46_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL46_A::RESERVED5
    }
}
#[doc = "Field `FSEL46` writer - Function Select 46"]
pub type FSEL46_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL46_A>;
impl<'a, REG> FSEL46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL46_A::RESERVED5)
    }
}
#[doc = "Field `FSEL47` reader - Function Select 47"]
pub type FSEL47_R = crate::FieldReader<FSEL47_A>;
#[doc = "Function Select 47"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL47_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL47_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL47_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL47_A {
    type Ux = u8;
}
impl FSEL47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL47_A {
        match self.bits {
            0 => FSEL47_A::INPUT,
            1 => FSEL47_A::OUTPUT,
            4 => FSEL47_A::RESERVED0,
            5 => FSEL47_A::RESERVED1,
            6 => FSEL47_A::RESERVED2,
            7 => FSEL47_A::RESERVED3,
            3 => FSEL47_A::RESERVED4,
            2 => FSEL47_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL47_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL47_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL47_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL47_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL47_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL47_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL47_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL47_A::RESERVED5
    }
}
#[doc = "Field `FSEL47` writer - Function Select 47"]
pub type FSEL47_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL47_A>;
impl<'a, REG> FSEL47_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL47_A::RESERVED5)
    }
}
#[doc = "Field `FSEL48` reader - Function Select 48"]
pub type FSEL48_R = crate::FieldReader<FSEL48_A>;
#[doc = "Function Select 48"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL48_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_CLK"]
    SD1_CLK = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL48_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL48_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL48_A {
    type Ux = u8;
}
impl FSEL48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL48_A {
        match self.bits {
            0 => FSEL48_A::INPUT,
            1 => FSEL48_A::OUTPUT,
            4 => FSEL48_A::RESERVED0,
            5 => FSEL48_A::RESERVED1,
            6 => FSEL48_A::RESERVED2,
            7 => FSEL48_A::SD1_CLK,
            3 => FSEL48_A::RESERVED4,
            2 => FSEL48_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL48_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL48_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL48_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL48_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL48_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn is_sd1_clk(&self) -> bool {
        *self == FSEL48_A::SD1_CLK
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL48_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL48_A::RESERVED5
    }
}
#[doc = "Field `FSEL48` writer - Function Select 48"]
pub type FSEL48_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL48_A>;
impl<'a, REG> FSEL48_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_CLK"]
    #[inline(always)]
    pub fn sd1_clk(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::SD1_CLK)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL48_A::RESERVED5)
    }
}
#[doc = "Field `FSEL49` reader - Function Select 49"]
pub type FSEL49_R = crate::FieldReader<FSEL49_A>;
#[doc = "Function Select 49"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL49_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Alt function 1 reserved"]
    RESERVED1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to SD1_CMD"]
    SD1_CMD = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL49_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL49_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL49_A {
    type Ux = u8;
}
impl FSEL49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL49_A {
        match self.bits {
            0 => FSEL49_A::INPUT,
            1 => FSEL49_A::OUTPUT,
            4 => FSEL49_A::RESERVED0,
            5 => FSEL49_A::RESERVED1,
            6 => FSEL49_A::RESERVED2,
            7 => FSEL49_A::SD1_CMD,
            3 => FSEL49_A::RESERVED4,
            2 => FSEL49_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL49_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL49_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL49_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL49_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL49_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn is_sd1_cmd(&self) -> bool {
        *self == FSEL49_A::SD1_CMD
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL49_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL49_A::RESERVED5
    }
}
#[doc = "Field `FSEL49` writer - Function Select 49"]
pub type FSEL49_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL49_A>;
impl<'a, REG> FSEL49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_CMD"]
    #[inline(always)]
    pub fn sd1_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::SD1_CMD)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL49_A::RESERVED5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 40"]
    #[inline(always)]
    pub fn fsel40(&self) -> FSEL40_R {
        FSEL40_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 41"]
    #[inline(always)]
    pub fn fsel41(&self) -> FSEL41_R {
        FSEL41_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 42"]
    #[inline(always)]
    pub fn fsel42(&self) -> FSEL42_R {
        FSEL42_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 43"]
    #[inline(always)]
    pub fn fsel43(&self) -> FSEL43_R {
        FSEL43_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 44"]
    #[inline(always)]
    pub fn fsel44(&self) -> FSEL44_R {
        FSEL44_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 45"]
    #[inline(always)]
    pub fn fsel45(&self) -> FSEL45_R {
        FSEL45_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 46"]
    #[inline(always)]
    pub fn fsel46(&self) -> FSEL46_R {
        FSEL46_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 47"]
    #[inline(always)]
    pub fn fsel47(&self) -> FSEL47_R {
        FSEL47_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 48"]
    #[inline(always)]
    pub fn fsel48(&self) -> FSEL48_R {
        FSEL48_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 49"]
    #[inline(always)]
    pub fn fsel49(&self) -> FSEL49_R {
        FSEL49_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL4")
            .field("fsel40", &format_args!("{}", self.fsel40().bits()))
            .field("fsel41", &format_args!("{}", self.fsel41().bits()))
            .field("fsel42", &format_args!("{}", self.fsel42().bits()))
            .field("fsel43", &format_args!("{}", self.fsel43().bits()))
            .field("fsel44", &format_args!("{}", self.fsel44().bits()))
            .field("fsel45", &format_args!("{}", self.fsel45().bits()))
            .field("fsel46", &format_args!("{}", self.fsel46().bits()))
            .field("fsel47", &format_args!("{}", self.fsel47().bits()))
            .field("fsel48", &format_args!("{}", self.fsel48().bits()))
            .field("fsel49", &format_args!("{}", self.fsel49().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPFSEL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 40"]
    #[inline(always)]
    #[must_use]
    pub fn fsel40(&mut self) -> FSEL40_W<GPFSEL4_SPEC> {
        FSEL40_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 41"]
    #[inline(always)]
    #[must_use]
    pub fn fsel41(&mut self) -> FSEL41_W<GPFSEL4_SPEC> {
        FSEL41_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 42"]
    #[inline(always)]
    #[must_use]
    pub fn fsel42(&mut self) -> FSEL42_W<GPFSEL4_SPEC> {
        FSEL42_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 43"]
    #[inline(always)]
    #[must_use]
    pub fn fsel43(&mut self) -> FSEL43_W<GPFSEL4_SPEC> {
        FSEL43_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 44"]
    #[inline(always)]
    #[must_use]
    pub fn fsel44(&mut self) -> FSEL44_W<GPFSEL4_SPEC> {
        FSEL44_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 45"]
    #[inline(always)]
    #[must_use]
    pub fn fsel45(&mut self) -> FSEL45_W<GPFSEL4_SPEC> {
        FSEL45_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 46"]
    #[inline(always)]
    #[must_use]
    pub fn fsel46(&mut self) -> FSEL46_W<GPFSEL4_SPEC> {
        FSEL46_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 47"]
    #[inline(always)]
    #[must_use]
    pub fn fsel47(&mut self) -> FSEL47_W<GPFSEL4_SPEC> {
        FSEL47_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Function Select 48"]
    #[inline(always)]
    #[must_use]
    pub fn fsel48(&mut self) -> FSEL48_W<GPFSEL4_SPEC> {
        FSEL48_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Function Select 49"]
    #[inline(always)]
    #[must_use]
    pub fn fsel49(&mut self) -> FSEL49_W<GPFSEL4_SPEC> {
        FSEL49_W::new(self, 27)
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
#[doc = "GPIO Function Select 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPFSEL4_SPEC;
impl crate::RegisterSpec for GPFSEL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel4::R`](R) reader structure"]
impl crate::Readable for GPFSEL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpfsel4::W`](W) writer structure"]
impl crate::Writable for GPFSEL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

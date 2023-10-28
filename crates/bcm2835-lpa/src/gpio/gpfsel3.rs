#[doc = "Register `GPFSEL3` reader"]
pub type R = crate::R<GPFSEL3_SPEC>;
#[doc = "Register `GPFSEL3` writer"]
pub type W = crate::W<GPFSEL3_SPEC>;
#[doc = "Field `FSEL30` reader - Function Select 30"]
pub type FSEL30_R = crate::FieldReader<FSEL30_A>;
#[doc = "Function Select 30"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL30_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SA3"]
    SA3 = 5,
    #[doc = "6: Pin is connected to PCM_DIN"]
    PCM_DIN = 6,
    #[doc = "7: Pin is connected to CTS0"]
    CTS0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to CTS1"]
    CTS1 = 2,
}
impl From<FSEL30_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL30_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL30_A {
    type Ux = u8;
}
impl FSEL30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL30_A {
        match self.bits {
            0 => FSEL30_A::INPUT,
            1 => FSEL30_A::OUTPUT,
            4 => FSEL30_A::RESERVED0,
            5 => FSEL30_A::SA3,
            6 => FSEL30_A::PCM_DIN,
            7 => FSEL30_A::CTS0,
            3 => FSEL30_A::RESERVED4,
            2 => FSEL30_A::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL30_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL30_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL30_A::RESERVED0
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn is_sa3(&self) -> bool {
        *self == FSEL30_A::SA3
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn is_pcm_din(&self) -> bool {
        *self == FSEL30_A::PCM_DIN
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == FSEL30_A::CTS0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL30_A::RESERVED4
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == FSEL30_A::CTS1
    }
}
#[doc = "Field `FSEL30` writer - Function Select 30"]
pub type FSEL30_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL30_A>;
impl<'a, REG, const O: u8> FSEL30_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::RESERVED0)
    }
    #[doc = "Pin is connected to SA3"]
    #[inline(always)]
    pub fn sa3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::SA3)
    }
    #[doc = "Pin is connected to PCM_DIN"]
    #[inline(always)]
    pub fn pcm_din(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::PCM_DIN)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::CTS0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::RESERVED4)
    }
    #[doc = "Pin is connected to CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL30_A::CTS1)
    }
}
#[doc = "Field `FSEL31` reader - Function Select 31"]
pub type FSEL31_R = crate::FieldReader<FSEL31_A>;
#[doc = "Function Select 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL31_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SA2"]
    SA2 = 5,
    #[doc = "6: Pin is connected to PCM_DOUT"]
    PCM_DOUT = 6,
    #[doc = "7: Pin is connected to RTS0"]
    RTS0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to RTS1"]
    RTS1 = 2,
}
impl From<FSEL31_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL31_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL31_A {
    type Ux = u8;
}
impl FSEL31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL31_A {
        match self.bits {
            0 => FSEL31_A::INPUT,
            1 => FSEL31_A::OUTPUT,
            4 => FSEL31_A::RESERVED0,
            5 => FSEL31_A::SA2,
            6 => FSEL31_A::PCM_DOUT,
            7 => FSEL31_A::RTS0,
            3 => FSEL31_A::RESERVED4,
            2 => FSEL31_A::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL31_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL31_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL31_A::RESERVED0
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn is_sa2(&self) -> bool {
        *self == FSEL31_A::SA2
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn is_pcm_dout(&self) -> bool {
        *self == FSEL31_A::PCM_DOUT
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == FSEL31_A::RTS0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL31_A::RESERVED4
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == FSEL31_A::RTS1
    }
}
#[doc = "Field `FSEL31` writer - Function Select 31"]
pub type FSEL31_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL31_A>;
impl<'a, REG, const O: u8> FSEL31_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::RESERVED0)
    }
    #[doc = "Pin is connected to SA2"]
    #[inline(always)]
    pub fn sa2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::SA2)
    }
    #[doc = "Pin is connected to PCM_DOUT"]
    #[inline(always)]
    pub fn pcm_dout(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::PCM_DOUT)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::RTS0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::RESERVED4)
    }
    #[doc = "Pin is connected to RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL31_A::RTS1)
    }
}
#[doc = "Field `FSEL32` reader - Function Select 32"]
pub type FSEL32_R = crate::FieldReader<FSEL32_A>;
#[doc = "Function Select 32"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL32_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    GPCLK0 = 4,
    #[doc = "5: Pin is connected to SA1"]
    SA1 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to TXD0"]
    TXD0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to TXD1"]
    TXD1 = 2,
}
impl From<FSEL32_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL32_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL32_A {
    type Ux = u8;
}
impl FSEL32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL32_A {
        match self.bits {
            0 => FSEL32_A::INPUT,
            1 => FSEL32_A::OUTPUT,
            4 => FSEL32_A::GPCLK0,
            5 => FSEL32_A::SA1,
            6 => FSEL32_A::RESERVED2,
            7 => FSEL32_A::TXD0,
            3 => FSEL32_A::RESERVED4,
            2 => FSEL32_A::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL32_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL32_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == FSEL32_A::GPCLK0
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn is_sa1(&self) -> bool {
        *self == FSEL32_A::SA1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL32_A::RESERVED2
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == FSEL32_A::TXD0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL32_A::RESERVED4
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == FSEL32_A::TXD1
    }
}
#[doc = "Field `FSEL32` writer - Function Select 32"]
pub type FSEL32_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL32_A>;
impl<'a, REG, const O: u8> FSEL32_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::GPCLK0)
    }
    #[doc = "Pin is connected to SA1"]
    #[inline(always)]
    pub fn sa1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::SA1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::RESERVED2)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::TXD0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::RESERVED4)
    }
    #[doc = "Pin is connected to TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL32_A::TXD1)
    }
}
#[doc = "Field `FSEL33` reader - Function Select 33"]
pub type FSEL33_R = crate::FieldReader<FSEL33_A>;
#[doc = "Function Select 33"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL33_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Alt function 0 reserved"]
    RESERVED0 = 4,
    #[doc = "5: Pin is connected to SA0"]
    SA0 = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Pin is connected to RXD0"]
    RXD0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to RXD1"]
    RXD1 = 2,
}
impl From<FSEL33_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL33_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL33_A {
    type Ux = u8;
}
impl FSEL33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL33_A {
        match self.bits {
            0 => FSEL33_A::INPUT,
            1 => FSEL33_A::OUTPUT,
            4 => FSEL33_A::RESERVED0,
            5 => FSEL33_A::SA0,
            6 => FSEL33_A::RESERVED2,
            7 => FSEL33_A::RXD0,
            3 => FSEL33_A::RESERVED4,
            2 => FSEL33_A::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL33_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL33_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL33_A::RESERVED0
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn is_sa0(&self) -> bool {
        *self == FSEL33_A::SA0
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL33_A::RESERVED2
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == FSEL33_A::RXD0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL33_A::RESERVED4
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == FSEL33_A::RXD1
    }
}
#[doc = "Field `FSEL33` writer - Function Select 33"]
pub type FSEL33_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL33_A>;
impl<'a, REG, const O: u8> FSEL33_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::RESERVED0)
    }
    #[doc = "Pin is connected to SA0"]
    #[inline(always)]
    pub fn sa0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::SA0)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::RESERVED2)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::RXD0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::RESERVED4)
    }
    #[doc = "Pin is connected to RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL33_A::RXD1)
    }
}
#[doc = "Field `FSEL34` reader - Function Select 34"]
pub type FSEL34_R = crate::FieldReader<FSEL34_A>;
#[doc = "Function Select 34"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL34_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to GPCLK0"]
    GPCLK0 = 4,
    #[doc = "5: Pin is connected to SOE_N"]
    SOE_N = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL34_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL34_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL34_A {
    type Ux = u8;
}
impl FSEL34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL34_A {
        match self.bits {
            0 => FSEL34_A::INPUT,
            1 => FSEL34_A::OUTPUT,
            4 => FSEL34_A::GPCLK0,
            5 => FSEL34_A::SOE_N,
            6 => FSEL34_A::RESERVED2,
            7 => FSEL34_A::RESERVED3,
            3 => FSEL34_A::RESERVED4,
            2 => FSEL34_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL34_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL34_A::OUTPUT
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn is_gpclk0(&self) -> bool {
        *self == FSEL34_A::GPCLK0
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn is_soe_n(&self) -> bool {
        *self == FSEL34_A::SOE_N
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL34_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL34_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL34_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL34_A::RESERVED5
    }
}
#[doc = "Field `FSEL34` writer - Function Select 34"]
pub type FSEL34_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL34_A>;
impl<'a, REG, const O: u8> FSEL34_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::OUTPUT)
    }
    #[doc = "Pin is connected to GPCLK0"]
    #[inline(always)]
    pub fn gpclk0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::GPCLK0)
    }
    #[doc = "Pin is connected to SOE_N"]
    #[inline(always)]
    pub fn soe_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::SOE_N)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL34_A::RESERVED5)
    }
}
#[doc = "Field `FSEL35` reader - Function Select 35"]
pub type FSEL35_R = crate::FieldReader<FSEL35_A>;
#[doc = "Function Select 35"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL35_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_CE1_N"]
    SPI0_CE1_N = 4,
    #[doc = "5: Pin is connected to SWE_N"]
    SWE_N = 5,
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL35_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL35_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL35_A {
    type Ux = u8;
}
impl FSEL35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL35_A {
        match self.bits {
            0 => FSEL35_A::INPUT,
            1 => FSEL35_A::OUTPUT,
            4 => FSEL35_A::SPI0_CE1_N,
            5 => FSEL35_A::SWE_N,
            6 => FSEL35_A::RESERVED2,
            7 => FSEL35_A::RESERVED3,
            3 => FSEL35_A::RESERVED4,
            2 => FSEL35_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL35_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL35_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn is_spi0_ce1_n(&self) -> bool {
        *self == FSEL35_A::SPI0_CE1_N
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn is_swe_n(&self) -> bool {
        *self == FSEL35_A::SWE_N
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL35_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL35_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL35_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL35_A::RESERVED5
    }
}
#[doc = "Field `FSEL35` writer - Function Select 35"]
pub type FSEL35_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL35_A>;
impl<'a, REG, const O: u8> FSEL35_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_CE1_N"]
    #[inline(always)]
    pub fn spi0_ce1_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::SPI0_CE1_N)
    }
    #[doc = "Pin is connected to SWE_N"]
    #[inline(always)]
    pub fn swe_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::SWE_N)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL35_A::RESERVED5)
    }
}
#[doc = "Field `FSEL36` reader - Function Select 36"]
pub type FSEL36_R = crate::FieldReader<FSEL36_A>;
#[doc = "Function Select 36"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL36_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_CE0_N"]
    SPI0_CE0_N = 4,
    #[doc = "5: Pin is connected to SD0"]
    SD0 = 5,
    #[doc = "6: Pin is connected to TXD0"]
    TXD0 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL36_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL36_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL36_A {
    type Ux = u8;
}
impl FSEL36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL36_A {
        match self.bits {
            0 => FSEL36_A::INPUT,
            1 => FSEL36_A::OUTPUT,
            4 => FSEL36_A::SPI0_CE0_N,
            5 => FSEL36_A::SD0,
            6 => FSEL36_A::TXD0,
            7 => FSEL36_A::RESERVED3,
            3 => FSEL36_A::RESERVED4,
            2 => FSEL36_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL36_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL36_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn is_spi0_ce0_n(&self) -> bool {
        *self == FSEL36_A::SPI0_CE0_N
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn is_sd0(&self) -> bool {
        *self == FSEL36_A::SD0
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == FSEL36_A::TXD0
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL36_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL36_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL36_A::RESERVED5
    }
}
#[doc = "Field `FSEL36` writer - Function Select 36"]
pub type FSEL36_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL36_A>;
impl<'a, REG, const O: u8> FSEL36_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_CE0_N"]
    #[inline(always)]
    pub fn spi0_ce0_n(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::SPI0_CE0_N)
    }
    #[doc = "Pin is connected to SD0"]
    #[inline(always)]
    pub fn sd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::SD0)
    }
    #[doc = "Pin is connected to TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::TXD0)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL36_A::RESERVED5)
    }
}
#[doc = "Field `FSEL37` reader - Function Select 37"]
pub type FSEL37_R = crate::FieldReader<FSEL37_A>;
#[doc = "Function Select 37"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL37_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_MISO"]
    SPI0_MISO = 4,
    #[doc = "5: Pin is connected to SD1"]
    SD1 = 5,
    #[doc = "6: Pin is connected to RXD0"]
    RXD0 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL37_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL37_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL37_A {
    type Ux = u8;
}
impl FSEL37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL37_A {
        match self.bits {
            0 => FSEL37_A::INPUT,
            1 => FSEL37_A::OUTPUT,
            4 => FSEL37_A::SPI0_MISO,
            5 => FSEL37_A::SD1,
            6 => FSEL37_A::RXD0,
            7 => FSEL37_A::RESERVED3,
            3 => FSEL37_A::RESERVED4,
            2 => FSEL37_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL37_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL37_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn is_spi0_miso(&self) -> bool {
        *self == FSEL37_A::SPI0_MISO
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn is_sd1(&self) -> bool {
        *self == FSEL37_A::SD1
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == FSEL37_A::RXD0
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL37_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL37_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL37_A::RESERVED5
    }
}
#[doc = "Field `FSEL37` writer - Function Select 37"]
pub type FSEL37_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL37_A>;
impl<'a, REG, const O: u8> FSEL37_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_MISO"]
    #[inline(always)]
    pub fn spi0_miso(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::SPI0_MISO)
    }
    #[doc = "Pin is connected to SD1"]
    #[inline(always)]
    pub fn sd1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::SD1)
    }
    #[doc = "Pin is connected to RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::RXD0)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL37_A::RESERVED5)
    }
}
#[doc = "Field `FSEL38` reader - Function Select 38"]
pub type FSEL38_R = crate::FieldReader<FSEL38_A>;
#[doc = "Function Select 38"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL38_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_MOSI"]
    SPI0_MOSI = 4,
    #[doc = "5: Pin is connected to SD2"]
    SD2 = 5,
    #[doc = "6: Pin is connected to CTS0"]
    CTS0 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL38_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL38_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL38_A {
    type Ux = u8;
}
impl FSEL38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL38_A {
        match self.bits {
            0 => FSEL38_A::INPUT,
            1 => FSEL38_A::OUTPUT,
            4 => FSEL38_A::SPI0_MOSI,
            5 => FSEL38_A::SD2,
            6 => FSEL38_A::CTS0,
            7 => FSEL38_A::RESERVED3,
            3 => FSEL38_A::RESERVED4,
            2 => FSEL38_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL38_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL38_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn is_spi0_mosi(&self) -> bool {
        *self == FSEL38_A::SPI0_MOSI
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn is_sd2(&self) -> bool {
        *self == FSEL38_A::SD2
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn is_cts0(&self) -> bool {
        *self == FSEL38_A::CTS0
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL38_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL38_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL38_A::RESERVED5
    }
}
#[doc = "Field `FSEL38` writer - Function Select 38"]
pub type FSEL38_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL38_A>;
impl<'a, REG, const O: u8> FSEL38_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_MOSI"]
    #[inline(always)]
    pub fn spi0_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::SPI0_MOSI)
    }
    #[doc = "Pin is connected to SD2"]
    #[inline(always)]
    pub fn sd2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::SD2)
    }
    #[doc = "Pin is connected to CTS0"]
    #[inline(always)]
    pub fn cts0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::CTS0)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL38_A::RESERVED5)
    }
}
#[doc = "Field `FSEL39` reader - Function Select 39"]
pub type FSEL39_R = crate::FieldReader<FSEL39_A>;
#[doc = "Function Select 39"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL39_A {
    #[doc = "0: Pin is an input"]
    INPUT = 0,
    #[doc = "1: Pin is an output"]
    OUTPUT = 1,
    #[doc = "4: Pin is connected to SPI0_SCLK"]
    SPI0_SCLK = 4,
    #[doc = "5: Pin is connected to SD3"]
    SD3 = 5,
    #[doc = "6: Pin is connected to RTS0"]
    RTS0 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL39_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL39_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL39_A {
    type Ux = u8;
}
impl FSEL39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL39_A {
        match self.bits {
            0 => FSEL39_A::INPUT,
            1 => FSEL39_A::OUTPUT,
            4 => FSEL39_A::SPI0_SCLK,
            5 => FSEL39_A::SD3,
            6 => FSEL39_A::RTS0,
            7 => FSEL39_A::RESERVED3,
            3 => FSEL39_A::RESERVED4,
            2 => FSEL39_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL39_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL39_A::OUTPUT
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn is_spi0_sclk(&self) -> bool {
        *self == FSEL39_A::SPI0_SCLK
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn is_sd3(&self) -> bool {
        *self == FSEL39_A::SD3
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn is_rts0(&self) -> bool {
        *self == FSEL39_A::RTS0
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL39_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL39_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL39_A::RESERVED5
    }
}
#[doc = "Field `FSEL39` writer - Function Select 39"]
pub type FSEL39_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, FSEL39_A>;
impl<'a, REG, const O: u8> FSEL39_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::OUTPUT)
    }
    #[doc = "Pin is connected to SPI0_SCLK"]
    #[inline(always)]
    pub fn spi0_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::SPI0_SCLK)
    }
    #[doc = "Pin is connected to SD3"]
    #[inline(always)]
    pub fn sd3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::SD3)
    }
    #[doc = "Pin is connected to RTS0"]
    #[inline(always)]
    pub fn rts0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::RTS0)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL39_A::RESERVED5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 30"]
    #[inline(always)]
    pub fn fsel30(&self) -> FSEL30_R {
        FSEL30_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 31"]
    #[inline(always)]
    pub fn fsel31(&self) -> FSEL31_R {
        FSEL31_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 32"]
    #[inline(always)]
    pub fn fsel32(&self) -> FSEL32_R {
        FSEL32_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 33"]
    #[inline(always)]
    pub fn fsel33(&self) -> FSEL33_R {
        FSEL33_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 34"]
    #[inline(always)]
    pub fn fsel34(&self) -> FSEL34_R {
        FSEL34_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 35"]
    #[inline(always)]
    pub fn fsel35(&self) -> FSEL35_R {
        FSEL35_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 36"]
    #[inline(always)]
    pub fn fsel36(&self) -> FSEL36_R {
        FSEL36_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 37"]
    #[inline(always)]
    pub fn fsel37(&self) -> FSEL37_R {
        FSEL37_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Function Select 38"]
    #[inline(always)]
    pub fn fsel38(&self) -> FSEL38_R {
        FSEL38_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Function Select 39"]
    #[inline(always)]
    pub fn fsel39(&self) -> FSEL39_R {
        FSEL39_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL3")
            .field("fsel30", &format_args!("{}", self.fsel30().bits()))
            .field("fsel31", &format_args!("{}", self.fsel31().bits()))
            .field("fsel32", &format_args!("{}", self.fsel32().bits()))
            .field("fsel33", &format_args!("{}", self.fsel33().bits()))
            .field("fsel34", &format_args!("{}", self.fsel34().bits()))
            .field("fsel35", &format_args!("{}", self.fsel35().bits()))
            .field("fsel36", &format_args!("{}", self.fsel36().bits()))
            .field("fsel37", &format_args!("{}", self.fsel37().bits()))
            .field("fsel38", &format_args!("{}", self.fsel38().bits()))
            .field("fsel39", &format_args!("{}", self.fsel39().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPFSEL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 30"]
    #[inline(always)]
    #[must_use]
    pub fn fsel30(&mut self) -> FSEL30_W<GPFSEL3_SPEC, 0> {
        FSEL30_W::new(self)
    }
    #[doc = "Bits 3:5 - Function Select 31"]
    #[inline(always)]
    #[must_use]
    pub fn fsel31(&mut self) -> FSEL31_W<GPFSEL3_SPEC, 3> {
        FSEL31_W::new(self)
    }
    #[doc = "Bits 6:8 - Function Select 32"]
    #[inline(always)]
    #[must_use]
    pub fn fsel32(&mut self) -> FSEL32_W<GPFSEL3_SPEC, 6> {
        FSEL32_W::new(self)
    }
    #[doc = "Bits 9:11 - Function Select 33"]
    #[inline(always)]
    #[must_use]
    pub fn fsel33(&mut self) -> FSEL33_W<GPFSEL3_SPEC, 9> {
        FSEL33_W::new(self)
    }
    #[doc = "Bits 12:14 - Function Select 34"]
    #[inline(always)]
    #[must_use]
    pub fn fsel34(&mut self) -> FSEL34_W<GPFSEL3_SPEC, 12> {
        FSEL34_W::new(self)
    }
    #[doc = "Bits 15:17 - Function Select 35"]
    #[inline(always)]
    #[must_use]
    pub fn fsel35(&mut self) -> FSEL35_W<GPFSEL3_SPEC, 15> {
        FSEL35_W::new(self)
    }
    #[doc = "Bits 18:20 - Function Select 36"]
    #[inline(always)]
    #[must_use]
    pub fn fsel36(&mut self) -> FSEL36_W<GPFSEL3_SPEC, 18> {
        FSEL36_W::new(self)
    }
    #[doc = "Bits 21:23 - Function Select 37"]
    #[inline(always)]
    #[must_use]
    pub fn fsel37(&mut self) -> FSEL37_W<GPFSEL3_SPEC, 21> {
        FSEL37_W::new(self)
    }
    #[doc = "Bits 24:26 - Function Select 38"]
    #[inline(always)]
    #[must_use]
    pub fn fsel38(&mut self) -> FSEL38_W<GPFSEL3_SPEC, 24> {
        FSEL38_W::new(self)
    }
    #[doc = "Bits 27:29 - Function Select 39"]
    #[inline(always)]
    #[must_use]
    pub fn fsel39(&mut self) -> FSEL39_W<GPFSEL3_SPEC, 27> {
        FSEL39_W::new(self)
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
#[doc = "GPIO Function Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel3::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPFSEL3_SPEC;
impl crate::RegisterSpec for GPFSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel3::R`](R) reader structure"]
impl crate::Readable for GPFSEL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpfsel3::W`](W) writer structure"]
impl crate::Writable for GPFSEL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

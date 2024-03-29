#[doc = "Register `GPFSEL5` reader"]
pub type R = crate::R<GPFSEL5_SPEC>;
#[doc = "Register `GPFSEL5` writer"]
pub type W = crate::W<GPFSEL5_SPEC>;
#[doc = "Field `FSEL50` reader - Function Select 50"]
pub type FSEL50_R = crate::FieldReader<FSEL50_A>;
#[doc = "Function Select 50"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL50_A {
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
    #[doc = "7: Pin is connected to SD1_DAT0"]
    SD1_DAT0 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL50_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL50_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL50_A {
    type Ux = u8;
}
impl FSEL50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL50_A {
        match self.bits {
            0 => FSEL50_A::INPUT,
            1 => FSEL50_A::OUTPUT,
            4 => FSEL50_A::RESERVED0,
            5 => FSEL50_A::RESERVED1,
            6 => FSEL50_A::RESERVED2,
            7 => FSEL50_A::SD1_DAT0,
            3 => FSEL50_A::RESERVED4,
            2 => FSEL50_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL50_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL50_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL50_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL50_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL50_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn is_sd1_dat0(&self) -> bool {
        *self == FSEL50_A::SD1_DAT0
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL50_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL50_A::RESERVED5
    }
}
#[doc = "Field `FSEL50` writer - Function Select 50"]
pub type FSEL50_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL50_A>;
impl<'a, REG> FSEL50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT0"]
    #[inline(always)]
    pub fn sd1_dat0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::SD1_DAT0)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL50_A::RESERVED5)
    }
}
#[doc = "Field `FSEL51` reader - Function Select 51"]
pub type FSEL51_R = crate::FieldReader<FSEL51_A>;
#[doc = "Function Select 51"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL51_A {
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
    #[doc = "7: Pin is connected to SD1_DAT1"]
    SD1_DAT1 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL51_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL51_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL51_A {
    type Ux = u8;
}
impl FSEL51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL51_A {
        match self.bits {
            0 => FSEL51_A::INPUT,
            1 => FSEL51_A::OUTPUT,
            4 => FSEL51_A::RESERVED0,
            5 => FSEL51_A::RESERVED1,
            6 => FSEL51_A::RESERVED2,
            7 => FSEL51_A::SD1_DAT1,
            3 => FSEL51_A::RESERVED4,
            2 => FSEL51_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL51_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL51_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL51_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL51_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL51_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn is_sd1_dat1(&self) -> bool {
        *self == FSEL51_A::SD1_DAT1
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL51_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL51_A::RESERVED5
    }
}
#[doc = "Field `FSEL51` writer - Function Select 51"]
pub type FSEL51_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL51_A>;
impl<'a, REG> FSEL51_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT1"]
    #[inline(always)]
    pub fn sd1_dat1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::SD1_DAT1)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL51_A::RESERVED5)
    }
}
#[doc = "Field `FSEL52` reader - Function Select 52"]
pub type FSEL52_R = crate::FieldReader<FSEL52_A>;
#[doc = "Function Select 52"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL52_A {
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
    #[doc = "7: Pin is connected to SD1_DAT2"]
    SD1_DAT2 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL52_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL52_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL52_A {
    type Ux = u8;
}
impl FSEL52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL52_A {
        match self.bits {
            0 => FSEL52_A::INPUT,
            1 => FSEL52_A::OUTPUT,
            4 => FSEL52_A::RESERVED0,
            5 => FSEL52_A::RESERVED1,
            6 => FSEL52_A::RESERVED2,
            7 => FSEL52_A::SD1_DAT2,
            3 => FSEL52_A::RESERVED4,
            2 => FSEL52_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL52_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL52_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL52_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL52_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL52_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn is_sd1_dat2(&self) -> bool {
        *self == FSEL52_A::SD1_DAT2
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL52_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL52_A::RESERVED5
    }
}
#[doc = "Field `FSEL52` writer - Function Select 52"]
pub type FSEL52_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL52_A>;
impl<'a, REG> FSEL52_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT2"]
    #[inline(always)]
    pub fn sd1_dat2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::SD1_DAT2)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL52_A::RESERVED5)
    }
}
#[doc = "Field `FSEL53` reader - Function Select 53"]
pub type FSEL53_R = crate::FieldReader<FSEL53_A>;
#[doc = "Function Select 53"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL53_A {
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
    #[doc = "7: Pin is connected to SD1_DAT3"]
    SD1_DAT3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
}
impl From<FSEL53_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL53_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL53_A {
    type Ux = u8;
}
impl FSEL53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL53_A {
        match self.bits {
            0 => FSEL53_A::INPUT,
            1 => FSEL53_A::OUTPUT,
            4 => FSEL53_A::RESERVED0,
            5 => FSEL53_A::RESERVED1,
            6 => FSEL53_A::RESERVED2,
            7 => FSEL53_A::SD1_DAT3,
            3 => FSEL53_A::RESERVED4,
            2 => FSEL53_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL53_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL53_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL53_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL53_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL53_A::RESERVED2
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn is_sd1_dat3(&self) -> bool {
        *self == FSEL53_A::SD1_DAT3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL53_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL53_A::RESERVED5
    }
}
#[doc = "Field `FSEL53` writer - Function Select 53"]
pub type FSEL53_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL53_A>;
impl<'a, REG> FSEL53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::RESERVED2)
    }
    #[doc = "Pin is connected to SD1_DAT3"]
    #[inline(always)]
    pub fn sd1_dat3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::SD1_DAT3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL53_A::RESERVED5)
    }
}
#[doc = "Field `FSEL54` reader - Function Select 54"]
pub type FSEL54_R = crate::FieldReader<FSEL54_A>;
#[doc = "Function Select 54"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL54_A {
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
impl From<FSEL54_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL54_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL54_A {
    type Ux = u8;
}
impl FSEL54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL54_A {
        match self.bits {
            0 => FSEL54_A::INPUT,
            1 => FSEL54_A::OUTPUT,
            4 => FSEL54_A::RESERVED0,
            5 => FSEL54_A::RESERVED1,
            6 => FSEL54_A::RESERVED2,
            7 => FSEL54_A::RESERVED3,
            3 => FSEL54_A::RESERVED4,
            2 => FSEL54_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL54_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL54_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL54_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL54_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL54_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL54_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL54_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL54_A::RESERVED5
    }
}
#[doc = "Field `FSEL54` writer - Function Select 54"]
pub type FSEL54_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL54_A>;
impl<'a, REG> FSEL54_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL54_A::RESERVED5)
    }
}
#[doc = "Field `FSEL55` reader - Function Select 55"]
pub type FSEL55_R = crate::FieldReader<FSEL55_A>;
#[doc = "Function Select 55"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL55_A {
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
impl From<FSEL55_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL55_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL55_A {
    type Ux = u8;
}
impl FSEL55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL55_A {
        match self.bits {
            0 => FSEL55_A::INPUT,
            1 => FSEL55_A::OUTPUT,
            4 => FSEL55_A::RESERVED0,
            5 => FSEL55_A::RESERVED1,
            6 => FSEL55_A::RESERVED2,
            7 => FSEL55_A::RESERVED3,
            3 => FSEL55_A::RESERVED4,
            2 => FSEL55_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL55_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL55_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL55_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL55_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL55_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL55_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL55_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL55_A::RESERVED5
    }
}
#[doc = "Field `FSEL55` writer - Function Select 55"]
pub type FSEL55_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL55_A>;
impl<'a, REG> FSEL55_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL55_A::RESERVED5)
    }
}
#[doc = "Field `FSEL56` reader - Function Select 56"]
pub type FSEL56_R = crate::FieldReader<FSEL56_A>;
#[doc = "Function Select 56"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL56_A {
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
impl From<FSEL56_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL56_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL56_A {
    type Ux = u8;
}
impl FSEL56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL56_A {
        match self.bits {
            0 => FSEL56_A::INPUT,
            1 => FSEL56_A::OUTPUT,
            4 => FSEL56_A::RESERVED0,
            5 => FSEL56_A::RESERVED1,
            6 => FSEL56_A::RESERVED2,
            7 => FSEL56_A::RESERVED3,
            3 => FSEL56_A::RESERVED4,
            2 => FSEL56_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL56_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL56_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL56_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL56_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL56_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL56_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL56_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL56_A::RESERVED5
    }
}
#[doc = "Field `FSEL56` writer - Function Select 56"]
pub type FSEL56_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL56_A>;
impl<'a, REG> FSEL56_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL56_A::RESERVED5)
    }
}
#[doc = "Field `FSEL57` reader - Function Select 57"]
pub type FSEL57_R = crate::FieldReader<FSEL57_A>;
#[doc = "Function Select 57"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL57_A {
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
impl From<FSEL57_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL57_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSEL57_A {
    type Ux = u8;
}
impl FSEL57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSEL57_A {
        match self.bits {
            0 => FSEL57_A::INPUT,
            1 => FSEL57_A::OUTPUT,
            4 => FSEL57_A::RESERVED0,
            5 => FSEL57_A::RESERVED1,
            6 => FSEL57_A::RESERVED2,
            7 => FSEL57_A::RESERVED3,
            3 => FSEL57_A::RESERVED4,
            2 => FSEL57_A::RESERVED5,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FSEL57_A::INPUT
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FSEL57_A::OUTPUT
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn is_reserved0(&self) -> bool {
        *self == FSEL57_A::RESERVED0
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn is_reserved1(&self) -> bool {
        *self == FSEL57_A::RESERVED1
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL57_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL57_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL57_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL57_A::RESERVED5
    }
}
#[doc = "Field `FSEL57` writer - Function Select 57"]
pub type FSEL57_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FSEL57_A>;
impl<'a, REG> FSEL57_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin is an input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::INPUT)
    }
    #[doc = "Pin is an output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::OUTPUT)
    }
    #[doc = "Alt function 0 reserved"]
    #[inline(always)]
    pub fn reserved0(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED0)
    }
    #[doc = "Alt function 1 reserved"]
    #[inline(always)]
    pub fn reserved1(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED1)
    }
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL57_A::RESERVED5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Function Select 50"]
    #[inline(always)]
    pub fn fsel50(&self) -> FSEL50_R {
        FSEL50_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Select 51"]
    #[inline(always)]
    pub fn fsel51(&self) -> FSEL51_R {
        FSEL51_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Select 52"]
    #[inline(always)]
    pub fn fsel52(&self) -> FSEL52_R {
        FSEL52_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Select 53"]
    #[inline(always)]
    pub fn fsel53(&self) -> FSEL53_R {
        FSEL53_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Select 54"]
    #[inline(always)]
    pub fn fsel54(&self) -> FSEL54_R {
        FSEL54_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Select 55"]
    #[inline(always)]
    pub fn fsel55(&self) -> FSEL55_R {
        FSEL55_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Select 56"]
    #[inline(always)]
    pub fn fsel56(&self) -> FSEL56_R {
        FSEL56_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Select 57"]
    #[inline(always)]
    pub fn fsel57(&self) -> FSEL57_R {
        FSEL57_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPFSEL5")
            .field("fsel50", &format_args!("{}", self.fsel50().bits()))
            .field("fsel51", &format_args!("{}", self.fsel51().bits()))
            .field("fsel52", &format_args!("{}", self.fsel52().bits()))
            .field("fsel53", &format_args!("{}", self.fsel53().bits()))
            .field("fsel54", &format_args!("{}", self.fsel54().bits()))
            .field("fsel55", &format_args!("{}", self.fsel55().bits()))
            .field("fsel56", &format_args!("{}", self.fsel56().bits()))
            .field("fsel57", &format_args!("{}", self.fsel57().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPFSEL5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Select 50"]
    #[inline(always)]
    #[must_use]
    pub fn fsel50(&mut self) -> FSEL50_W<GPFSEL5_SPEC> {
        FSEL50_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Select 51"]
    #[inline(always)]
    #[must_use]
    pub fn fsel51(&mut self) -> FSEL51_W<GPFSEL5_SPEC> {
        FSEL51_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Select 52"]
    #[inline(always)]
    #[must_use]
    pub fn fsel52(&mut self) -> FSEL52_W<GPFSEL5_SPEC> {
        FSEL52_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Select 53"]
    #[inline(always)]
    #[must_use]
    pub fn fsel53(&mut self) -> FSEL53_W<GPFSEL5_SPEC> {
        FSEL53_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Select 54"]
    #[inline(always)]
    #[must_use]
    pub fn fsel54(&mut self) -> FSEL54_W<GPFSEL5_SPEC> {
        FSEL54_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Select 55"]
    #[inline(always)]
    #[must_use]
    pub fn fsel55(&mut self) -> FSEL55_W<GPFSEL5_SPEC> {
        FSEL55_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Select 56"]
    #[inline(always)]
    #[must_use]
    pub fn fsel56(&mut self) -> FSEL56_W<GPFSEL5_SPEC> {
        FSEL56_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Select 57"]
    #[inline(always)]
    #[must_use]
    pub fn fsel57(&mut self) -> FSEL57_W<GPFSEL5_SPEC> {
        FSEL57_W::new(self, 21)
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
#[doc = "GPIO Function Select 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPFSEL5_SPEC;
impl crate::RegisterSpec for GPFSEL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpfsel5::R`](R) reader structure"]
impl crate::Readable for GPFSEL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpfsel5::W`](W) writer structure"]
impl crate::Writable for GPFSEL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

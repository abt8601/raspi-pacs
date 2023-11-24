#[doc = "Register `GPFSEL0` reader"]
pub type R = crate::R<GPFSEL0_SPEC>;
#[doc = "Register `GPFSEL0` writer"]
pub type W = crate::W<GPFSEL0_SPEC>;
#[doc = "Field `FSEL0` reader - Function Select 0"]
pub type FSEL0_R = crate::FieldReader<FSEL0_A>;
#[doc = "Function Select 0"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL0_A::RESERVED2,
            7 => FSEL0_A::RESERVED3,
            3 => FSEL0_A::RESERVED4,
            2 => FSEL0_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL0_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL0_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL0_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL0_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL0_A::RESERVED5)
    }
}
#[doc = "Field `FSEL1` reader - Function Select 1"]
pub type FSEL1_R = crate::FieldReader<FSEL1_A>;
#[doc = "Function Select 1"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL1_A::RESERVED2,
            7 => FSEL1_A::RESERVED3,
            3 => FSEL1_A::RESERVED4,
            2 => FSEL1_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL1_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL1_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL1_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL1_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL1_A::RESERVED5)
    }
}
#[doc = "Field `FSEL2` reader - Function Select 2"]
pub type FSEL2_R = crate::FieldReader<FSEL2_A>;
#[doc = "Function Select 2"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL2_A::RESERVED2,
            7 => FSEL2_A::RESERVED3,
            3 => FSEL2_A::RESERVED4,
            2 => FSEL2_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL2_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL2_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL2_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL2_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL2_A::RESERVED5)
    }
}
#[doc = "Field `FSEL3` reader - Function Select 3"]
pub type FSEL3_R = crate::FieldReader<FSEL3_A>;
#[doc = "Function Select 3"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL3_A::RESERVED2,
            7 => FSEL3_A::RESERVED3,
            3 => FSEL3_A::RESERVED4,
            2 => FSEL3_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL3_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL3_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL3_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL3_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL3_A::RESERVED5)
    }
}
#[doc = "Field `FSEL4` reader - Function Select 4"]
pub type FSEL4_R = crate::FieldReader<FSEL4_A>;
#[doc = "Function Select 4"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to ARM_TDI"]
    ARM_TDI = 2,
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
            6 => FSEL4_A::RESERVED2,
            7 => FSEL4_A::RESERVED3,
            3 => FSEL4_A::RESERVED4,
            2 => FSEL4_A::ARM_TDI,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL4_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL4_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL4_A::RESERVED4
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn is_arm_tdi(&self) -> bool {
        *self == FSEL4_A::ARM_TDI
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::RESERVED4)
    }
    #[doc = "Pin is connected to ARM_TDI"]
    #[inline(always)]
    pub fn arm_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL4_A::ARM_TDI)
    }
}
#[doc = "Field `FSEL5` reader - Function Select 5"]
pub type FSEL5_R = crate::FieldReader<FSEL5_A>;
#[doc = "Function Select 5"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to ARM_TDO"]
    ARM_TDO = 2,
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
            6 => FSEL5_A::RESERVED2,
            7 => FSEL5_A::RESERVED3,
            3 => FSEL5_A::RESERVED4,
            2 => FSEL5_A::ARM_TDO,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL5_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL5_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL5_A::RESERVED4
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn is_arm_tdo(&self) -> bool {
        *self == FSEL5_A::ARM_TDO
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::RESERVED4)
    }
    #[doc = "Pin is connected to ARM_TDO"]
    #[inline(always)]
    pub fn arm_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL5_A::ARM_TDO)
    }
}
#[doc = "Field `FSEL6` reader - Function Select 6"]
pub type FSEL6_R = crate::FieldReader<FSEL6_A>;
#[doc = "Function Select 6"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Pin is connected to ARM_RTCK"]
    ARM_RTCK = 2,
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
            6 => FSEL6_A::RESERVED2,
            7 => FSEL6_A::RESERVED3,
            3 => FSEL6_A::RESERVED4,
            2 => FSEL6_A::ARM_RTCK,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL6_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL6_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL6_A::RESERVED4
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn is_arm_rtck(&self) -> bool {
        *self == FSEL6_A::ARM_RTCK
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::RESERVED4)
    }
    #[doc = "Pin is connected to ARM_RTCK"]
    #[inline(always)]
    pub fn arm_rtck(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL6_A::ARM_RTCK)
    }
}
#[doc = "Field `FSEL7` reader - Function Select 7"]
pub type FSEL7_R = crate::FieldReader<FSEL7_A>;
#[doc = "Function Select 7"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL7_A::RESERVED2,
            7 => FSEL7_A::RESERVED3,
            3 => FSEL7_A::RESERVED4,
            2 => FSEL7_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL7_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL7_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL7_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL7_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL7_A::RESERVED5)
    }
}
#[doc = "Field `FSEL8` reader - Function Select 8"]
pub type FSEL8_R = crate::FieldReader<FSEL8_A>;
#[doc = "Function Select 8"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL8_A::RESERVED2,
            7 => FSEL8_A::RESERVED3,
            3 => FSEL8_A::RESERVED4,
            2 => FSEL8_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL8_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL8_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL8_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL8_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL8_A::RESERVED5)
    }
}
#[doc = "Field `FSEL9` reader - Function Select 9"]
pub type FSEL9_R = crate::FieldReader<FSEL9_A>;
#[doc = "Function Select 9"]
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
    #[doc = "6: Alt function 2 reserved"]
    RESERVED2 = 6,
    #[doc = "7: Alt function 3 reserved"]
    RESERVED3 = 7,
    #[doc = "3: Alt function 4 reserved"]
    RESERVED4 = 3,
    #[doc = "2: Alt function 5 reserved"]
    RESERVED5 = 2,
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
            6 => FSEL9_A::RESERVED2,
            7 => FSEL9_A::RESERVED3,
            3 => FSEL9_A::RESERVED4,
            2 => FSEL9_A::RESERVED5,
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn is_reserved2(&self) -> bool {
        *self == FSEL9_A::RESERVED2
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn is_reserved3(&self) -> bool {
        *self == FSEL9_A::RESERVED3
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn is_reserved4(&self) -> bool {
        *self == FSEL9_A::RESERVED4
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn is_reserved5(&self) -> bool {
        *self == FSEL9_A::RESERVED5
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
    #[doc = "Alt function 2 reserved"]
    #[inline(always)]
    pub fn reserved2(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::RESERVED2)
    }
    #[doc = "Alt function 3 reserved"]
    #[inline(always)]
    pub fn reserved3(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::RESERVED3)
    }
    #[doc = "Alt function 4 reserved"]
    #[inline(always)]
    pub fn reserved4(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::RESERVED4)
    }
    #[doc = "Alt function 5 reserved"]
    #[inline(always)]
    pub fn reserved5(self) -> &'a mut crate::W<REG> {
        self.variant(FSEL9_A::RESERVED5)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `FIQ_CONTROL` reader"]
pub type R = crate::R<FIQ_CONTROL_SPEC>;
#[doc = "Register `FIQ_CONTROL` writer"]
pub type W = crate::W<FIQ_CONTROL_SPEC>;
#[doc = "Field `SOURCE` reader - FIQ Source"]
pub type SOURCE_R = crate::FieldReader<SOURCE_A>;
#[doc = "FIQ Source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Interrupt 0"]
    INT0 = 0,
    #[doc = "1: Interrupt 1"]
    INT1 = 1,
    #[doc = "2: Interrupt 2"]
    INT2 = 2,
    #[doc = "3: Interrupt 3"]
    INT3 = 3,
    #[doc = "4: Interrupt 4"]
    INT4 = 4,
    #[doc = "5: Interrupt 5"]
    INT5 = 5,
    #[doc = "6: Interrupt 6"]
    INT6 = 6,
    #[doc = "7: Interrupt 7"]
    INT7 = 7,
    #[doc = "8: Interrupt 8"]
    INT8 = 8,
    #[doc = "9: Interrupt 9"]
    INT9 = 9,
    #[doc = "10: Interrupt 10"]
    INT10 = 10,
    #[doc = "11: Interrupt 11"]
    INT11 = 11,
    #[doc = "12: Interrupt 12"]
    INT12 = 12,
    #[doc = "13: Interrupt 13"]
    INT13 = 13,
    #[doc = "14: Interrupt 14"]
    INT14 = 14,
    #[doc = "15: Interrupt 15"]
    INT15 = 15,
    #[doc = "16: Interrupt 16"]
    INT16 = 16,
    #[doc = "17: Interrupt 17"]
    INT17 = 17,
    #[doc = "18: Interrupt 18"]
    INT18 = 18,
    #[doc = "19: Interrupt 19"]
    INT19 = 19,
    #[doc = "20: Interrupt 20"]
    INT20 = 20,
    #[doc = "21: Interrupt 21"]
    INT21 = 21,
    #[doc = "22: Interrupt 22"]
    INT22 = 22,
    #[doc = "23: Interrupt 23"]
    INT23 = 23,
    #[doc = "24: Interrupt 24"]
    INT24 = 24,
    #[doc = "25: Interrupt 25"]
    INT25 = 25,
    #[doc = "26: Interrupt 26"]
    INT26 = 26,
    #[doc = "27: Interrupt 27"]
    INT27 = 27,
    #[doc = "28: Interrupt 28"]
    INT28 = 28,
    #[doc = "29: Interrupt 29"]
    INT29 = 29,
    #[doc = "30: Interrupt 30"]
    INT30 = 30,
    #[doc = "31: Interrupt 31"]
    INT31 = 31,
    #[doc = "32: Interrupt 32"]
    INT32 = 32,
    #[doc = "33: Interrupt 33"]
    INT33 = 33,
    #[doc = "34: Interrupt 34"]
    INT34 = 34,
    #[doc = "35: Interrupt 35"]
    INT35 = 35,
    #[doc = "36: Interrupt 36"]
    INT36 = 36,
    #[doc = "37: Interrupt 37"]
    INT37 = 37,
    #[doc = "38: Interrupt 38"]
    INT38 = 38,
    #[doc = "39: Interrupt 39"]
    INT39 = 39,
    #[doc = "40: Interrupt 40"]
    INT40 = 40,
    #[doc = "41: Interrupt 41"]
    INT41 = 41,
    #[doc = "42: Interrupt 42"]
    INT42 = 42,
    #[doc = "43: Interrupt 43"]
    INT43 = 43,
    #[doc = "44: Interrupt 44"]
    INT44 = 44,
    #[doc = "45: Interrupt 45"]
    INT45 = 45,
    #[doc = "46: Interrupt 46"]
    INT46 = 46,
    #[doc = "47: Interrupt 47"]
    INT47 = 47,
    #[doc = "48: Interrupt 48"]
    INT48 = 48,
    #[doc = "49: Interrupt 49"]
    INT49 = 49,
    #[doc = "50: Interrupt 50"]
    INT50 = 50,
    #[doc = "51: Interrupt 51"]
    INT51 = 51,
    #[doc = "52: Interrupt 52"]
    INT52 = 52,
    #[doc = "53: Interrupt 53"]
    INT53 = 53,
    #[doc = "54: Interrupt 54"]
    INT54 = 54,
    #[doc = "55: Interrupt 55"]
    INT55 = 55,
    #[doc = "56: Interrupt 56"]
    INT56 = 56,
    #[doc = "57: Interrupt 57"]
    INT57 = 57,
    #[doc = "58: Interrupt 58"]
    INT58 = 58,
    #[doc = "59: Interrupt 59"]
    INT59 = 59,
    #[doc = "60: Interrupt 60"]
    INT60 = 60,
    #[doc = "61: Interrupt 61"]
    INT61 = 61,
    #[doc = "62: Interrupt 62"]
    INT62 = 62,
    #[doc = "63: Interrupt 63"]
    INT63 = 63,
    #[doc = "64: ARMC Timer"]
    TIMER = 64,
    #[doc = "65: Mailbox"]
    MAILBOX = 65,
    #[doc = "66: Doorbell 0"]
    DOORBELL0 = 66,
    #[doc = "67: Doorbell 1"]
    DOORBELL1 = 67,
    #[doc = "68: VPU0 halted"]
    VPU0_HALTED = 68,
    #[doc = "69: VPU1 halted"]
    VPU1_HALTED = 69,
    #[doc = "70: ARM address error"]
    ARM_ADDRESS_ERROR = 70,
    #[doc = "71: ARM AXI error"]
    ARM_AXI_ERROR = 71,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCE_A {
    type Ux = u8;
}
impl SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCE_A> {
        match self.bits {
            0 => Some(SOURCE_A::INT0),
            1 => Some(SOURCE_A::INT1),
            2 => Some(SOURCE_A::INT2),
            3 => Some(SOURCE_A::INT3),
            4 => Some(SOURCE_A::INT4),
            5 => Some(SOURCE_A::INT5),
            6 => Some(SOURCE_A::INT6),
            7 => Some(SOURCE_A::INT7),
            8 => Some(SOURCE_A::INT8),
            9 => Some(SOURCE_A::INT9),
            10 => Some(SOURCE_A::INT10),
            11 => Some(SOURCE_A::INT11),
            12 => Some(SOURCE_A::INT12),
            13 => Some(SOURCE_A::INT13),
            14 => Some(SOURCE_A::INT14),
            15 => Some(SOURCE_A::INT15),
            16 => Some(SOURCE_A::INT16),
            17 => Some(SOURCE_A::INT17),
            18 => Some(SOURCE_A::INT18),
            19 => Some(SOURCE_A::INT19),
            20 => Some(SOURCE_A::INT20),
            21 => Some(SOURCE_A::INT21),
            22 => Some(SOURCE_A::INT22),
            23 => Some(SOURCE_A::INT23),
            24 => Some(SOURCE_A::INT24),
            25 => Some(SOURCE_A::INT25),
            26 => Some(SOURCE_A::INT26),
            27 => Some(SOURCE_A::INT27),
            28 => Some(SOURCE_A::INT28),
            29 => Some(SOURCE_A::INT29),
            30 => Some(SOURCE_A::INT30),
            31 => Some(SOURCE_A::INT31),
            32 => Some(SOURCE_A::INT32),
            33 => Some(SOURCE_A::INT33),
            34 => Some(SOURCE_A::INT34),
            35 => Some(SOURCE_A::INT35),
            36 => Some(SOURCE_A::INT36),
            37 => Some(SOURCE_A::INT37),
            38 => Some(SOURCE_A::INT38),
            39 => Some(SOURCE_A::INT39),
            40 => Some(SOURCE_A::INT40),
            41 => Some(SOURCE_A::INT41),
            42 => Some(SOURCE_A::INT42),
            43 => Some(SOURCE_A::INT43),
            44 => Some(SOURCE_A::INT44),
            45 => Some(SOURCE_A::INT45),
            46 => Some(SOURCE_A::INT46),
            47 => Some(SOURCE_A::INT47),
            48 => Some(SOURCE_A::INT48),
            49 => Some(SOURCE_A::INT49),
            50 => Some(SOURCE_A::INT50),
            51 => Some(SOURCE_A::INT51),
            52 => Some(SOURCE_A::INT52),
            53 => Some(SOURCE_A::INT53),
            54 => Some(SOURCE_A::INT54),
            55 => Some(SOURCE_A::INT55),
            56 => Some(SOURCE_A::INT56),
            57 => Some(SOURCE_A::INT57),
            58 => Some(SOURCE_A::INT58),
            59 => Some(SOURCE_A::INT59),
            60 => Some(SOURCE_A::INT60),
            61 => Some(SOURCE_A::INT61),
            62 => Some(SOURCE_A::INT62),
            63 => Some(SOURCE_A::INT63),
            64 => Some(SOURCE_A::TIMER),
            65 => Some(SOURCE_A::MAILBOX),
            66 => Some(SOURCE_A::DOORBELL0),
            67 => Some(SOURCE_A::DOORBELL1),
            68 => Some(SOURCE_A::VPU0_HALTED),
            69 => Some(SOURCE_A::VPU1_HALTED),
            70 => Some(SOURCE_A::ARM_ADDRESS_ERROR),
            71 => Some(SOURCE_A::ARM_AXI_ERROR),
            _ => None,
        }
    }
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn is_int0(&self) -> bool {
        *self == SOURCE_A::INT0
    }
    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn is_int1(&self) -> bool {
        *self == SOURCE_A::INT1
    }
    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn is_int2(&self) -> bool {
        *self == SOURCE_A::INT2
    }
    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn is_int3(&self) -> bool {
        *self == SOURCE_A::INT3
    }
    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn is_int4(&self) -> bool {
        *self == SOURCE_A::INT4
    }
    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn is_int5(&self) -> bool {
        *self == SOURCE_A::INT5
    }
    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn is_int6(&self) -> bool {
        *self == SOURCE_A::INT6
    }
    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn is_int7(&self) -> bool {
        *self == SOURCE_A::INT7
    }
    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn is_int8(&self) -> bool {
        *self == SOURCE_A::INT8
    }
    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn is_int9(&self) -> bool {
        *self == SOURCE_A::INT9
    }
    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn is_int10(&self) -> bool {
        *self == SOURCE_A::INT10
    }
    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn is_int11(&self) -> bool {
        *self == SOURCE_A::INT11
    }
    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn is_int12(&self) -> bool {
        *self == SOURCE_A::INT12
    }
    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn is_int13(&self) -> bool {
        *self == SOURCE_A::INT13
    }
    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn is_int14(&self) -> bool {
        *self == SOURCE_A::INT14
    }
    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn is_int15(&self) -> bool {
        *self == SOURCE_A::INT15
    }
    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn is_int16(&self) -> bool {
        *self == SOURCE_A::INT16
    }
    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn is_int17(&self) -> bool {
        *self == SOURCE_A::INT17
    }
    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn is_int18(&self) -> bool {
        *self == SOURCE_A::INT18
    }
    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn is_int19(&self) -> bool {
        *self == SOURCE_A::INT19
    }
    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn is_int20(&self) -> bool {
        *self == SOURCE_A::INT20
    }
    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn is_int21(&self) -> bool {
        *self == SOURCE_A::INT21
    }
    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn is_int22(&self) -> bool {
        *self == SOURCE_A::INT22
    }
    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn is_int23(&self) -> bool {
        *self == SOURCE_A::INT23
    }
    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn is_int24(&self) -> bool {
        *self == SOURCE_A::INT24
    }
    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn is_int25(&self) -> bool {
        *self == SOURCE_A::INT25
    }
    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn is_int26(&self) -> bool {
        *self == SOURCE_A::INT26
    }
    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn is_int27(&self) -> bool {
        *self == SOURCE_A::INT27
    }
    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn is_int28(&self) -> bool {
        *self == SOURCE_A::INT28
    }
    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn is_int29(&self) -> bool {
        *self == SOURCE_A::INT29
    }
    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn is_int30(&self) -> bool {
        *self == SOURCE_A::INT30
    }
    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn is_int31(&self) -> bool {
        *self == SOURCE_A::INT31
    }
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn is_int32(&self) -> bool {
        *self == SOURCE_A::INT32
    }
    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn is_int33(&self) -> bool {
        *self == SOURCE_A::INT33
    }
    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn is_int34(&self) -> bool {
        *self == SOURCE_A::INT34
    }
    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn is_int35(&self) -> bool {
        *self == SOURCE_A::INT35
    }
    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn is_int36(&self) -> bool {
        *self == SOURCE_A::INT36
    }
    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn is_int37(&self) -> bool {
        *self == SOURCE_A::INT37
    }
    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn is_int38(&self) -> bool {
        *self == SOURCE_A::INT38
    }
    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn is_int39(&self) -> bool {
        *self == SOURCE_A::INT39
    }
    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn is_int40(&self) -> bool {
        *self == SOURCE_A::INT40
    }
    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn is_int41(&self) -> bool {
        *self == SOURCE_A::INT41
    }
    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn is_int42(&self) -> bool {
        *self == SOURCE_A::INT42
    }
    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn is_int43(&self) -> bool {
        *self == SOURCE_A::INT43
    }
    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn is_int44(&self) -> bool {
        *self == SOURCE_A::INT44
    }
    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn is_int45(&self) -> bool {
        *self == SOURCE_A::INT45
    }
    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn is_int46(&self) -> bool {
        *self == SOURCE_A::INT46
    }
    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn is_int47(&self) -> bool {
        *self == SOURCE_A::INT47
    }
    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn is_int48(&self) -> bool {
        *self == SOURCE_A::INT48
    }
    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn is_int49(&self) -> bool {
        *self == SOURCE_A::INT49
    }
    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn is_int50(&self) -> bool {
        *self == SOURCE_A::INT50
    }
    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn is_int51(&self) -> bool {
        *self == SOURCE_A::INT51
    }
    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn is_int52(&self) -> bool {
        *self == SOURCE_A::INT52
    }
    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn is_int53(&self) -> bool {
        *self == SOURCE_A::INT53
    }
    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn is_int54(&self) -> bool {
        *self == SOURCE_A::INT54
    }
    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn is_int55(&self) -> bool {
        *self == SOURCE_A::INT55
    }
    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn is_int56(&self) -> bool {
        *self == SOURCE_A::INT56
    }
    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn is_int57(&self) -> bool {
        *self == SOURCE_A::INT57
    }
    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn is_int58(&self) -> bool {
        *self == SOURCE_A::INT58
    }
    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn is_int59(&self) -> bool {
        *self == SOURCE_A::INT59
    }
    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn is_int60(&self) -> bool {
        *self == SOURCE_A::INT60
    }
    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn is_int61(&self) -> bool {
        *self == SOURCE_A::INT61
    }
    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn is_int62(&self) -> bool {
        *self == SOURCE_A::INT62
    }
    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn is_int63(&self) -> bool {
        *self == SOURCE_A::INT63
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SOURCE_A::TIMER
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn is_mailbox(&self) -> bool {
        *self == SOURCE_A::MAILBOX
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn is_doorbell0(&self) -> bool {
        *self == SOURCE_A::DOORBELL0
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn is_doorbell1(&self) -> bool {
        *self == SOURCE_A::DOORBELL1
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn is_vpu0_halted(&self) -> bool {
        *self == SOURCE_A::VPU0_HALTED
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn is_vpu1_halted(&self) -> bool {
        *self == SOURCE_A::VPU1_HALTED
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn is_arm_address_error(&self) -> bool {
        *self == SOURCE_A::ARM_ADDRESS_ERROR
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn is_arm_axi_error(&self) -> bool {
        *self == SOURCE_A::ARM_AXI_ERROR
    }
}
#[doc = "Field `SOURCE` writer - FIQ Source"]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 7, SOURCE_A>;
impl<'a, REG> SOURCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn int0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT0)
    }
    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn int1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT1)
    }
    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn int2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT2)
    }
    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn int3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT3)
    }
    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn int4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT4)
    }
    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn int5(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT5)
    }
    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn int6(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT6)
    }
    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT7)
    }
    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn int8(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT8)
    }
    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT9)
    }
    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT10)
    }
    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn int11(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT11)
    }
    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn int12(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT12)
    }
    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn int13(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT13)
    }
    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn int14(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT14)
    }
    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn int15(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT15)
    }
    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn int16(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT16)
    }
    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn int17(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT17)
    }
    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT18)
    }
    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT19)
    }
    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn int20(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT20)
    }
    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn int21(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT21)
    }
    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn int22(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT22)
    }
    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn int23(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT23)
    }
    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn int24(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT24)
    }
    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn int25(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT25)
    }
    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn int26(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT26)
    }
    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn int27(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT27)
    }
    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn int28(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT28)
    }
    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn int29(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT29)
    }
    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn int30(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT30)
    }
    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn int31(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT31)
    }
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn int32(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT32)
    }
    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn int33(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT33)
    }
    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn int34(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT34)
    }
    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn int35(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT35)
    }
    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn int36(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT36)
    }
    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn int37(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT37)
    }
    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn int38(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT38)
    }
    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn int39(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT39)
    }
    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn int40(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT40)
    }
    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn int41(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT41)
    }
    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn int42(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT42)
    }
    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn int43(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT43)
    }
    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn int44(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT44)
    }
    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn int45(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT45)
    }
    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn int46(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT46)
    }
    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn int47(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT47)
    }
    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn int48(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT48)
    }
    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn int49(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT49)
    }
    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn int50(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT50)
    }
    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn int51(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT51)
    }
    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn int52(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT52)
    }
    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT53)
    }
    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT54)
    }
    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT55)
    }
    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT56)
    }
    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT57)
    }
    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn int58(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT58)
    }
    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn int59(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT59)
    }
    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn int60(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT60)
    }
    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn int61(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT61)
    }
    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT62)
    }
    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn int63(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::INT63)
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER)
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MAILBOX)
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DOORBELL0)
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DOORBELL1)
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::VPU0_HALTED)
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::VPU1_HALTED)
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ARM_ADDRESS_ERROR)
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ARM_AXI_ERROR)
    }
}
#[doc = "Field `ENABLE` reader - FIQ Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - FIQ Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIQ_CONTROL")
            .field("enable", &format_args!("{}", self.enable().bit()))
            .field("source", &format_args!("{}", self.source().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIQ_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<FIQ_CONTROL_SPEC> {
        SOURCE_W::new(self, 0)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<FIQ_CONTROL_SPEC> {
        ENABLE_W::new(self, 7)
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
#[doc = "FIQ control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fiq_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fiq_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIQ_CONTROL_SPEC;
impl crate::RegisterSpec for FIQ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiq_control::R`](R) reader structure"]
impl crate::Readable for FIQ_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fiq_control::W`](W) writer structure"]
impl crate::Writable for FIQ_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQ_CONTROL to value 0"]
impl crate::Resettable for FIQ_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PENDING_1` reader"]
pub type R = crate::R<PENDING_1_SPEC>;
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type INT0_R = crate::BitReader;
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type INT1_R = crate::BitReader;
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type INT2_R = crate::BitReader;
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type INT3_R = crate::BitReader;
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type INT4_R = crate::BitReader;
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type INT5_R = crate::BitReader;
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type INT6_R = crate::BitReader;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type INT7_R = crate::BitReader;
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type INT8_R = crate::BitReader;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type INT9_R = crate::BitReader;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type INT10_R = crate::BitReader;
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type INT11_R = crate::BitReader;
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type INT12_R = crate::BitReader;
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type INT13_R = crate::BitReader;
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type INT14_R = crate::BitReader;
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type INT15_R = crate::BitReader;
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type INT16_R = crate::BitReader;
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type INT17_R = crate::BitReader;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type INT18_R = crate::BitReader;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type INT19_R = crate::BitReader;
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type INT20_R = crate::BitReader;
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type INT21_R = crate::BitReader;
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type INT22_R = crate::BitReader;
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type INT23_R = crate::BitReader;
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type INT24_R = crate::BitReader;
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type INT25_R = crate::BitReader;
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type INT26_R = crate::BitReader;
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type INT27_R = crate::BitReader;
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type INT28_R = crate::BitReader;
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type INT29_R = crate::BitReader;
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type INT30_R = crate::BitReader;
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type INT31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_1")
            .field("int0", &format_args!("{}", self.int0().bit()))
            .field("int1", &format_args!("{}", self.int1().bit()))
            .field("int2", &format_args!("{}", self.int2().bit()))
            .field("int3", &format_args!("{}", self.int3().bit()))
            .field("int4", &format_args!("{}", self.int4().bit()))
            .field("int5", &format_args!("{}", self.int5().bit()))
            .field("int6", &format_args!("{}", self.int6().bit()))
            .field("int7", &format_args!("{}", self.int7().bit()))
            .field("int8", &format_args!("{}", self.int8().bit()))
            .field("int9", &format_args!("{}", self.int9().bit()))
            .field("int10", &format_args!("{}", self.int10().bit()))
            .field("int11", &format_args!("{}", self.int11().bit()))
            .field("int12", &format_args!("{}", self.int12().bit()))
            .field("int13", &format_args!("{}", self.int13().bit()))
            .field("int14", &format_args!("{}", self.int14().bit()))
            .field("int15", &format_args!("{}", self.int15().bit()))
            .field("int16", &format_args!("{}", self.int16().bit()))
            .field("int17", &format_args!("{}", self.int17().bit()))
            .field("int18", &format_args!("{}", self.int18().bit()))
            .field("int19", &format_args!("{}", self.int19().bit()))
            .field("int20", &format_args!("{}", self.int20().bit()))
            .field("int21", &format_args!("{}", self.int21().bit()))
            .field("int22", &format_args!("{}", self.int22().bit()))
            .field("int23", &format_args!("{}", self.int23().bit()))
            .field("int24", &format_args!("{}", self.int24().bit()))
            .field("int25", &format_args!("{}", self.int25().bit()))
            .field("int26", &format_args!("{}", self.int26().bit()))
            .field("int27", &format_args!("{}", self.int27().bit()))
            .field("int28", &format_args!("{}", self.int28().bit()))
            .field("int29", &format_args!("{}", self.int29().bit()))
            .field("int30", &format_args!("{}", self.int30().bit()))
            .field("int31", &format_args!("{}", self.int31().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PENDING_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Pending state for interrupts 1 - 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING_1_SPEC;
impl crate::RegisterSpec for PENDING_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_1::R`](R) reader structure"]
impl crate::Readable for PENDING_1_SPEC {}
#[doc = "`reset()` method sets PENDING_1 to value 0"]
impl crate::Resettable for PENDING_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

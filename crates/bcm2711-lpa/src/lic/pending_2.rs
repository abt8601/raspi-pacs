#[doc = "Register `PENDING_2` reader"]
pub type R = crate::R<PENDING_2_SPEC>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type INT32_R = crate::BitReader;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type INT33_R = crate::BitReader;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type INT34_R = crate::BitReader;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type INT35_R = crate::BitReader;
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type INT36_R = crate::BitReader;
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type INT37_R = crate::BitReader;
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type INT38_R = crate::BitReader;
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type INT39_R = crate::BitReader;
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type INT40_R = crate::BitReader;
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type INT41_R = crate::BitReader;
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type INT42_R = crate::BitReader;
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type INT43_R = crate::BitReader;
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type INT44_R = crate::BitReader;
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type INT45_R = crate::BitReader;
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type INT46_R = crate::BitReader;
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type INT47_R = crate::BitReader;
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type INT48_R = crate::BitReader;
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type INT49_R = crate::BitReader;
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type INT50_R = crate::BitReader;
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type INT51_R = crate::BitReader;
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type INT52_R = crate::BitReader;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type INT53_R = crate::BitReader;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type INT54_R = crate::BitReader;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type INT55_R = crate::BitReader;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type INT56_R = crate::BitReader;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type INT57_R = crate::BitReader;
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type INT58_R = crate::BitReader;
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type INT59_R = crate::BitReader;
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type INT60_R = crate::BitReader;
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type INT61_R = crate::BitReader;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type INT62_R = crate::BitReader;
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type INT63_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> INT32_R {
        INT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> INT33_R {
        INT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> INT34_R {
        INT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> INT35_R {
        INT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> INT36_R {
        INT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> INT37_R {
        INT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> INT38_R {
        INT38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> INT39_R {
        INT39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> INT40_R {
        INT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> INT41_R {
        INT41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> INT42_R {
        INT42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> INT43_R {
        INT43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> INT44_R {
        INT44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> INT45_R {
        INT45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> INT46_R {
        INT46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> INT47_R {
        INT47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> INT48_R {
        INT48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> INT49_R {
        INT49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> INT50_R {
        INT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> INT51_R {
        INT51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> INT52_R {
        INT52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> INT53_R {
        INT53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> INT54_R {
        INT54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> INT55_R {
        INT55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> INT56_R {
        INT56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> INT57_R {
        INT57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> INT58_R {
        INT58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> INT59_R {
        INT59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> INT60_R {
        INT60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> INT61_R {
        INT61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> INT62_R {
        INT62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> INT63_R {
        INT63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PENDING_2")
            .field("int32", &format_args!("{}", self.int32().bit()))
            .field("int33", &format_args!("{}", self.int33().bit()))
            .field("int34", &format_args!("{}", self.int34().bit()))
            .field("int35", &format_args!("{}", self.int35().bit()))
            .field("int36", &format_args!("{}", self.int36().bit()))
            .field("int37", &format_args!("{}", self.int37().bit()))
            .field("int38", &format_args!("{}", self.int38().bit()))
            .field("int39", &format_args!("{}", self.int39().bit()))
            .field("int40", &format_args!("{}", self.int40().bit()))
            .field("int41", &format_args!("{}", self.int41().bit()))
            .field("int42", &format_args!("{}", self.int42().bit()))
            .field("int43", &format_args!("{}", self.int43().bit()))
            .field("int44", &format_args!("{}", self.int44().bit()))
            .field("int45", &format_args!("{}", self.int45().bit()))
            .field("int46", &format_args!("{}", self.int46().bit()))
            .field("int47", &format_args!("{}", self.int47().bit()))
            .field("int48", &format_args!("{}", self.int48().bit()))
            .field("int49", &format_args!("{}", self.int49().bit()))
            .field("int50", &format_args!("{}", self.int50().bit()))
            .field("int51", &format_args!("{}", self.int51().bit()))
            .field("int52", &format_args!("{}", self.int52().bit()))
            .field("int53", &format_args!("{}", self.int53().bit()))
            .field("int54", &format_args!("{}", self.int54().bit()))
            .field("int55", &format_args!("{}", self.int55().bit()))
            .field("int56", &format_args!("{}", self.int56().bit()))
            .field("int57", &format_args!("{}", self.int57().bit()))
            .field("int58", &format_args!("{}", self.int58().bit()))
            .field("int59", &format_args!("{}", self.int59().bit()))
            .field("int60", &format_args!("{}", self.int60().bit()))
            .field("int61", &format_args!("{}", self.int61().bit()))
            .field("int62", &format_args!("{}", self.int62().bit()))
            .field("int63", &format_args!("{}", self.int63().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PENDING_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Pending state for interrupts 32 - 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING_2_SPEC;
impl crate::RegisterSpec for PENDING_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_2::R`](R) reader structure"]
impl crate::Readable for PENDING_2_SPEC {}
#[doc = "`reset()` method sets PENDING_2 to value 0"]
impl crate::Resettable for PENDING_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

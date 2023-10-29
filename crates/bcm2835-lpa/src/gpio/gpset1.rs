#[doc = "Register `GPSET1` writer"]
pub type W = crate::W<GPSET1_SPEC>;
#[doc = "Field `SET32` writer - Set 32"]
pub type SET32_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET33` writer - Set 33"]
pub type SET33_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET34` writer - Set 34"]
pub type SET34_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET35` writer - Set 35"]
pub type SET35_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET36` writer - Set 36"]
pub type SET36_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET37` writer - Set 37"]
pub type SET37_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET38` writer - Set 38"]
pub type SET38_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET39` writer - Set 39"]
pub type SET39_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET40` writer - Set 40"]
pub type SET40_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET41` writer - Set 41"]
pub type SET41_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET42` writer - Set 42"]
pub type SET42_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET43` writer - Set 43"]
pub type SET43_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET44` writer - Set 44"]
pub type SET44_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET45` writer - Set 45"]
pub type SET45_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET46` writer - Set 46"]
pub type SET46_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET47` writer - Set 47"]
pub type SET47_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET48` writer - Set 48"]
pub type SET48_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET49` writer - Set 49"]
pub type SET49_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET50` writer - Set 50"]
pub type SET50_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET51` writer - Set 51"]
pub type SET51_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET52` writer - Set 52"]
pub type SET52_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
#[doc = "Field `SET53` writer - Set 53"]
pub type SET53_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<GPSET1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set 32"]
    #[inline(always)]
    #[must_use]
    pub fn set32(&mut self) -> SET32_W<GPSET1_SPEC, 0> {
        SET32_W::new(self)
    }
    #[doc = "Bit 1 - Set 33"]
    #[inline(always)]
    #[must_use]
    pub fn set33(&mut self) -> SET33_W<GPSET1_SPEC, 1> {
        SET33_W::new(self)
    }
    #[doc = "Bit 2 - Set 34"]
    #[inline(always)]
    #[must_use]
    pub fn set34(&mut self) -> SET34_W<GPSET1_SPEC, 2> {
        SET34_W::new(self)
    }
    #[doc = "Bit 3 - Set 35"]
    #[inline(always)]
    #[must_use]
    pub fn set35(&mut self) -> SET35_W<GPSET1_SPEC, 3> {
        SET35_W::new(self)
    }
    #[doc = "Bit 4 - Set 36"]
    #[inline(always)]
    #[must_use]
    pub fn set36(&mut self) -> SET36_W<GPSET1_SPEC, 4> {
        SET36_W::new(self)
    }
    #[doc = "Bit 5 - Set 37"]
    #[inline(always)]
    #[must_use]
    pub fn set37(&mut self) -> SET37_W<GPSET1_SPEC, 5> {
        SET37_W::new(self)
    }
    #[doc = "Bit 6 - Set 38"]
    #[inline(always)]
    #[must_use]
    pub fn set38(&mut self) -> SET38_W<GPSET1_SPEC, 6> {
        SET38_W::new(self)
    }
    #[doc = "Bit 7 - Set 39"]
    #[inline(always)]
    #[must_use]
    pub fn set39(&mut self) -> SET39_W<GPSET1_SPEC, 7> {
        SET39_W::new(self)
    }
    #[doc = "Bit 8 - Set 40"]
    #[inline(always)]
    #[must_use]
    pub fn set40(&mut self) -> SET40_W<GPSET1_SPEC, 8> {
        SET40_W::new(self)
    }
    #[doc = "Bit 9 - Set 41"]
    #[inline(always)]
    #[must_use]
    pub fn set41(&mut self) -> SET41_W<GPSET1_SPEC, 9> {
        SET41_W::new(self)
    }
    #[doc = "Bit 10 - Set 42"]
    #[inline(always)]
    #[must_use]
    pub fn set42(&mut self) -> SET42_W<GPSET1_SPEC, 10> {
        SET42_W::new(self)
    }
    #[doc = "Bit 11 - Set 43"]
    #[inline(always)]
    #[must_use]
    pub fn set43(&mut self) -> SET43_W<GPSET1_SPEC, 11> {
        SET43_W::new(self)
    }
    #[doc = "Bit 12 - Set 44"]
    #[inline(always)]
    #[must_use]
    pub fn set44(&mut self) -> SET44_W<GPSET1_SPEC, 12> {
        SET44_W::new(self)
    }
    #[doc = "Bit 13 - Set 45"]
    #[inline(always)]
    #[must_use]
    pub fn set45(&mut self) -> SET45_W<GPSET1_SPEC, 13> {
        SET45_W::new(self)
    }
    #[doc = "Bit 14 - Set 46"]
    #[inline(always)]
    #[must_use]
    pub fn set46(&mut self) -> SET46_W<GPSET1_SPEC, 14> {
        SET46_W::new(self)
    }
    #[doc = "Bit 15 - Set 47"]
    #[inline(always)]
    #[must_use]
    pub fn set47(&mut self) -> SET47_W<GPSET1_SPEC, 15> {
        SET47_W::new(self)
    }
    #[doc = "Bit 16 - Set 48"]
    #[inline(always)]
    #[must_use]
    pub fn set48(&mut self) -> SET48_W<GPSET1_SPEC, 16> {
        SET48_W::new(self)
    }
    #[doc = "Bit 17 - Set 49"]
    #[inline(always)]
    #[must_use]
    pub fn set49(&mut self) -> SET49_W<GPSET1_SPEC, 17> {
        SET49_W::new(self)
    }
    #[doc = "Bit 18 - Set 50"]
    #[inline(always)]
    #[must_use]
    pub fn set50(&mut self) -> SET50_W<GPSET1_SPEC, 18> {
        SET50_W::new(self)
    }
    #[doc = "Bit 19 - Set 51"]
    #[inline(always)]
    #[must_use]
    pub fn set51(&mut self) -> SET51_W<GPSET1_SPEC, 19> {
        SET51_W::new(self)
    }
    #[doc = "Bit 20 - Set 52"]
    #[inline(always)]
    #[must_use]
    pub fn set52(&mut self) -> SET52_W<GPSET1_SPEC, 20> {
        SET52_W::new(self)
    }
    #[doc = "Bit 21 - Set 53"]
    #[inline(always)]
    #[must_use]
    pub fn set53(&mut self) -> SET53_W<GPSET1_SPEC, 21> {
        SET53_W::new(self)
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
#[doc = "GPIO Pin Output Set 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpset1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSET1_SPEC;
impl crate::RegisterSpec for GPSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpset1::W`](W) writer structure"]
impl crate::Writable for GPSET1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x003f_ffff;
}

#[doc = "Register `GPSET1` writer"]
pub struct W(crate::W<GPSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSET1_SPEC>;
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
impl From<crate::W<GPSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET32` writer - Set 32"]
pub type SET32_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET33` writer - Set 33"]
pub type SET33_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET34` writer - Set 34"]
pub type SET34_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET35` writer - Set 35"]
pub type SET35_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET36` writer - Set 36"]
pub type SET36_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET37` writer - Set 37"]
pub type SET37_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET38` writer - Set 38"]
pub type SET38_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET39` writer - Set 39"]
pub type SET39_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET40` writer - Set 40"]
pub type SET40_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET41` writer - Set 41"]
pub type SET41_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET42` writer - Set 42"]
pub type SET42_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET43` writer - Set 43"]
pub type SET43_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET44` writer - Set 44"]
pub type SET44_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET45` writer - Set 45"]
pub type SET45_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET46` writer - Set 46"]
pub type SET46_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET47` writer - Set 47"]
pub type SET47_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET48` writer - Set 48"]
pub type SET48_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET49` writer - Set 49"]
pub type SET49_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET50` writer - Set 50"]
pub type SET50_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET51` writer - Set 51"]
pub type SET51_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET52` writer - Set 52"]
pub type SET52_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET53` writer - Set 53"]
pub type SET53_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET54` writer - Set 54"]
pub type SET54_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET55` writer - Set 55"]
pub type SET55_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET56` writer - Set 56"]
pub type SET56_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
#[doc = "Field `SET57` writer - Set 57"]
pub type SET57_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set 32"]
    #[inline(always)]
    #[must_use]
    pub fn set32(&mut self) -> SET32_W<0> {
        SET32_W::new(self)
    }
    #[doc = "Bit 1 - Set 33"]
    #[inline(always)]
    #[must_use]
    pub fn set33(&mut self) -> SET33_W<1> {
        SET33_W::new(self)
    }
    #[doc = "Bit 2 - Set 34"]
    #[inline(always)]
    #[must_use]
    pub fn set34(&mut self) -> SET34_W<2> {
        SET34_W::new(self)
    }
    #[doc = "Bit 3 - Set 35"]
    #[inline(always)]
    #[must_use]
    pub fn set35(&mut self) -> SET35_W<3> {
        SET35_W::new(self)
    }
    #[doc = "Bit 4 - Set 36"]
    #[inline(always)]
    #[must_use]
    pub fn set36(&mut self) -> SET36_W<4> {
        SET36_W::new(self)
    }
    #[doc = "Bit 5 - Set 37"]
    #[inline(always)]
    #[must_use]
    pub fn set37(&mut self) -> SET37_W<5> {
        SET37_W::new(self)
    }
    #[doc = "Bit 6 - Set 38"]
    #[inline(always)]
    #[must_use]
    pub fn set38(&mut self) -> SET38_W<6> {
        SET38_W::new(self)
    }
    #[doc = "Bit 7 - Set 39"]
    #[inline(always)]
    #[must_use]
    pub fn set39(&mut self) -> SET39_W<7> {
        SET39_W::new(self)
    }
    #[doc = "Bit 8 - Set 40"]
    #[inline(always)]
    #[must_use]
    pub fn set40(&mut self) -> SET40_W<8> {
        SET40_W::new(self)
    }
    #[doc = "Bit 9 - Set 41"]
    #[inline(always)]
    #[must_use]
    pub fn set41(&mut self) -> SET41_W<9> {
        SET41_W::new(self)
    }
    #[doc = "Bit 10 - Set 42"]
    #[inline(always)]
    #[must_use]
    pub fn set42(&mut self) -> SET42_W<10> {
        SET42_W::new(self)
    }
    #[doc = "Bit 11 - Set 43"]
    #[inline(always)]
    #[must_use]
    pub fn set43(&mut self) -> SET43_W<11> {
        SET43_W::new(self)
    }
    #[doc = "Bit 12 - Set 44"]
    #[inline(always)]
    #[must_use]
    pub fn set44(&mut self) -> SET44_W<12> {
        SET44_W::new(self)
    }
    #[doc = "Bit 13 - Set 45"]
    #[inline(always)]
    #[must_use]
    pub fn set45(&mut self) -> SET45_W<13> {
        SET45_W::new(self)
    }
    #[doc = "Bit 14 - Set 46"]
    #[inline(always)]
    #[must_use]
    pub fn set46(&mut self) -> SET46_W<14> {
        SET46_W::new(self)
    }
    #[doc = "Bit 15 - Set 47"]
    #[inline(always)]
    #[must_use]
    pub fn set47(&mut self) -> SET47_W<15> {
        SET47_W::new(self)
    }
    #[doc = "Bit 16 - Set 48"]
    #[inline(always)]
    #[must_use]
    pub fn set48(&mut self) -> SET48_W<16> {
        SET48_W::new(self)
    }
    #[doc = "Bit 17 - Set 49"]
    #[inline(always)]
    #[must_use]
    pub fn set49(&mut self) -> SET49_W<17> {
        SET49_W::new(self)
    }
    #[doc = "Bit 18 - Set 50"]
    #[inline(always)]
    #[must_use]
    pub fn set50(&mut self) -> SET50_W<18> {
        SET50_W::new(self)
    }
    #[doc = "Bit 19 - Set 51"]
    #[inline(always)]
    #[must_use]
    pub fn set51(&mut self) -> SET51_W<19> {
        SET51_W::new(self)
    }
    #[doc = "Bit 20 - Set 52"]
    #[inline(always)]
    #[must_use]
    pub fn set52(&mut self) -> SET52_W<20> {
        SET52_W::new(self)
    }
    #[doc = "Bit 21 - Set 53"]
    #[inline(always)]
    #[must_use]
    pub fn set53(&mut self) -> SET53_W<21> {
        SET53_W::new(self)
    }
    #[doc = "Bit 22 - Set 54"]
    #[inline(always)]
    #[must_use]
    pub fn set54(&mut self) -> SET54_W<22> {
        SET54_W::new(self)
    }
    #[doc = "Bit 23 - Set 55"]
    #[inline(always)]
    #[must_use]
    pub fn set55(&mut self) -> SET55_W<23> {
        SET55_W::new(self)
    }
    #[doc = "Bit 24 - Set 56"]
    #[inline(always)]
    #[must_use]
    pub fn set56(&mut self) -> SET56_W<24> {
        SET56_W::new(self)
    }
    #[doc = "Bit 25 - Set 57"]
    #[inline(always)]
    #[must_use]
    pub fn set57(&mut self) -> SET57_W<25> {
        SET57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Output Set 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpset1](index.html) module"]
pub struct GPSET1_SPEC;
impl crate::RegisterSpec for GPSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpset1::W](W) writer structure"]
impl crate::Writable for GPSET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff_ffff;
}

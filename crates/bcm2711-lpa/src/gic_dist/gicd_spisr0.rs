#[doc = "Register `GICD_SPISR0` reader"]
pub struct R(crate::R<GICD_SPISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_SPISR0` writer"]
pub struct W(crate::W<GICD_SPISR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SPISR0_SPEC>;
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
impl From<crate::W<GICD_SPISR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SPISR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI32` reader - Shared interrupt 32"]
pub type SPI32_R = crate::BitReader<bool>;
#[doc = "Field `SPI32` writer - Shared interrupt 32"]
pub type SPI32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI33` reader - Shared interrupt 33"]
pub type SPI33_R = crate::BitReader<bool>;
#[doc = "Field `SPI33` writer - Shared interrupt 33"]
pub type SPI33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI34` reader - Shared interrupt 34"]
pub type SPI34_R = crate::BitReader<bool>;
#[doc = "Field `SPI34` writer - Shared interrupt 34"]
pub type SPI34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI35` reader - Shared interrupt 35"]
pub type SPI35_R = crate::BitReader<bool>;
#[doc = "Field `SPI35` writer - Shared interrupt 35"]
pub type SPI35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI36` reader - Shared interrupt 36"]
pub type SPI36_R = crate::BitReader<bool>;
#[doc = "Field `SPI36` writer - Shared interrupt 36"]
pub type SPI36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI37` reader - Shared interrupt 37"]
pub type SPI37_R = crate::BitReader<bool>;
#[doc = "Field `SPI37` writer - Shared interrupt 37"]
pub type SPI37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI38` reader - Shared interrupt 38"]
pub type SPI38_R = crate::BitReader<bool>;
#[doc = "Field `SPI38` writer - Shared interrupt 38"]
pub type SPI38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI39` reader - Shared interrupt 39"]
pub type SPI39_R = crate::BitReader<bool>;
#[doc = "Field `SPI39` writer - Shared interrupt 39"]
pub type SPI39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI40` reader - Shared interrupt 40"]
pub type SPI40_R = crate::BitReader<bool>;
#[doc = "Field `SPI40` writer - Shared interrupt 40"]
pub type SPI40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI41` reader - Shared interrupt 41"]
pub type SPI41_R = crate::BitReader<bool>;
#[doc = "Field `SPI41` writer - Shared interrupt 41"]
pub type SPI41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI42` reader - Shared interrupt 42"]
pub type SPI42_R = crate::BitReader<bool>;
#[doc = "Field `SPI42` writer - Shared interrupt 42"]
pub type SPI42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI43` reader - Shared interrupt 43"]
pub type SPI43_R = crate::BitReader<bool>;
#[doc = "Field `SPI43` writer - Shared interrupt 43"]
pub type SPI43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI44` reader - Shared interrupt 44"]
pub type SPI44_R = crate::BitReader<bool>;
#[doc = "Field `SPI44` writer - Shared interrupt 44"]
pub type SPI44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI45` reader - Shared interrupt 45"]
pub type SPI45_R = crate::BitReader<bool>;
#[doc = "Field `SPI45` writer - Shared interrupt 45"]
pub type SPI45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI46` reader - Shared interrupt 46"]
pub type SPI46_R = crate::BitReader<bool>;
#[doc = "Field `SPI46` writer - Shared interrupt 46"]
pub type SPI46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI47` reader - Shared interrupt 47"]
pub type SPI47_R = crate::BitReader<bool>;
#[doc = "Field `SPI47` writer - Shared interrupt 47"]
pub type SPI47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI48` reader - Shared interrupt 48"]
pub type SPI48_R = crate::BitReader<bool>;
#[doc = "Field `SPI48` writer - Shared interrupt 48"]
pub type SPI48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI49` reader - Shared interrupt 49"]
pub type SPI49_R = crate::BitReader<bool>;
#[doc = "Field `SPI49` writer - Shared interrupt 49"]
pub type SPI49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI50` reader - Shared interrupt 50"]
pub type SPI50_R = crate::BitReader<bool>;
#[doc = "Field `SPI50` writer - Shared interrupt 50"]
pub type SPI50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI51` reader - Shared interrupt 51"]
pub type SPI51_R = crate::BitReader<bool>;
#[doc = "Field `SPI51` writer - Shared interrupt 51"]
pub type SPI51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI52` reader - Shared interrupt 52"]
pub type SPI52_R = crate::BitReader<bool>;
#[doc = "Field `SPI52` writer - Shared interrupt 52"]
pub type SPI52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI53` reader - Shared interrupt 53"]
pub type SPI53_R = crate::BitReader<bool>;
#[doc = "Field `SPI53` writer - Shared interrupt 53"]
pub type SPI53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI54` reader - Shared interrupt 54"]
pub type SPI54_R = crate::BitReader<bool>;
#[doc = "Field `SPI54` writer - Shared interrupt 54"]
pub type SPI54_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI55` reader - Shared interrupt 55"]
pub type SPI55_R = crate::BitReader<bool>;
#[doc = "Field `SPI55` writer - Shared interrupt 55"]
pub type SPI55_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI56` reader - Shared interrupt 56"]
pub type SPI56_R = crate::BitReader<bool>;
#[doc = "Field `SPI56` writer - Shared interrupt 56"]
pub type SPI56_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI57` reader - Shared interrupt 57"]
pub type SPI57_R = crate::BitReader<bool>;
#[doc = "Field `SPI57` writer - Shared interrupt 57"]
pub type SPI57_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI58` reader - Shared interrupt 58"]
pub type SPI58_R = crate::BitReader<bool>;
#[doc = "Field `SPI58` writer - Shared interrupt 58"]
pub type SPI58_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI59` reader - Shared interrupt 59"]
pub type SPI59_R = crate::BitReader<bool>;
#[doc = "Field `SPI59` writer - Shared interrupt 59"]
pub type SPI59_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI60` reader - Shared interrupt 60"]
pub type SPI60_R = crate::BitReader<bool>;
#[doc = "Field `SPI60` writer - Shared interrupt 60"]
pub type SPI60_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI61` reader - Shared interrupt 61"]
pub type SPI61_R = crate::BitReader<bool>;
#[doc = "Field `SPI61` writer - Shared interrupt 61"]
pub type SPI61_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI62` reader - Shared interrupt 62"]
pub type SPI62_R = crate::BitReader<bool>;
#[doc = "Field `SPI62` writer - Shared interrupt 62"]
pub type SPI62_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
#[doc = "Field `SPI63` reader - Shared interrupt 63"]
pub type SPI63_R = crate::BitReader<bool>;
#[doc = "Field `SPI63` writer - Shared interrupt 63"]
pub type SPI63_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Shared interrupt 32"]
    #[inline(always)]
    pub fn spi32(&self) -> SPI32_R {
        SPI32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shared interrupt 33"]
    #[inline(always)]
    pub fn spi33(&self) -> SPI33_R {
        SPI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shared interrupt 34"]
    #[inline(always)]
    pub fn spi34(&self) -> SPI34_R {
        SPI34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shared interrupt 35"]
    #[inline(always)]
    pub fn spi35(&self) -> SPI35_R {
        SPI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shared interrupt 36"]
    #[inline(always)]
    pub fn spi36(&self) -> SPI36_R {
        SPI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shared interrupt 37"]
    #[inline(always)]
    pub fn spi37(&self) -> SPI37_R {
        SPI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shared interrupt 38"]
    #[inline(always)]
    pub fn spi38(&self) -> SPI38_R {
        SPI38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shared interrupt 39"]
    #[inline(always)]
    pub fn spi39(&self) -> SPI39_R {
        SPI39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Shared interrupt 40"]
    #[inline(always)]
    pub fn spi40(&self) -> SPI40_R {
        SPI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shared interrupt 41"]
    #[inline(always)]
    pub fn spi41(&self) -> SPI41_R {
        SPI41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shared interrupt 42"]
    #[inline(always)]
    pub fn spi42(&self) -> SPI42_R {
        SPI42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shared interrupt 43"]
    #[inline(always)]
    pub fn spi43(&self) -> SPI43_R {
        SPI43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Shared interrupt 44"]
    #[inline(always)]
    pub fn spi44(&self) -> SPI44_R {
        SPI44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Shared interrupt 45"]
    #[inline(always)]
    pub fn spi45(&self) -> SPI45_R {
        SPI45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shared interrupt 46"]
    #[inline(always)]
    pub fn spi46(&self) -> SPI46_R {
        SPI46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Shared interrupt 47"]
    #[inline(always)]
    pub fn spi47(&self) -> SPI47_R {
        SPI47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shared interrupt 48"]
    #[inline(always)]
    pub fn spi48(&self) -> SPI48_R {
        SPI48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shared interrupt 49"]
    #[inline(always)]
    pub fn spi49(&self) -> SPI49_R {
        SPI49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shared interrupt 50"]
    #[inline(always)]
    pub fn spi50(&self) -> SPI50_R {
        SPI50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shared interrupt 51"]
    #[inline(always)]
    pub fn spi51(&self) -> SPI51_R {
        SPI51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shared interrupt 52"]
    #[inline(always)]
    pub fn spi52(&self) -> SPI52_R {
        SPI52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shared interrupt 53"]
    #[inline(always)]
    pub fn spi53(&self) -> SPI53_R {
        SPI53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shared interrupt 54"]
    #[inline(always)]
    pub fn spi54(&self) -> SPI54_R {
        SPI54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shared interrupt 55"]
    #[inline(always)]
    pub fn spi55(&self) -> SPI55_R {
        SPI55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared interrupt 56"]
    #[inline(always)]
    pub fn spi56(&self) -> SPI56_R {
        SPI56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shared interrupt 57"]
    #[inline(always)]
    pub fn spi57(&self) -> SPI57_R {
        SPI57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shared interrupt 58"]
    #[inline(always)]
    pub fn spi58(&self) -> SPI58_R {
        SPI58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shared interrupt 59"]
    #[inline(always)]
    pub fn spi59(&self) -> SPI59_R {
        SPI59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shared interrupt 60"]
    #[inline(always)]
    pub fn spi60(&self) -> SPI60_R {
        SPI60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shared interrupt 61"]
    #[inline(always)]
    pub fn spi61(&self) -> SPI61_R {
        SPI61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Shared interrupt 62"]
    #[inline(always)]
    pub fn spi62(&self) -> SPI62_R {
        SPI62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Shared interrupt 63"]
    #[inline(always)]
    pub fn spi63(&self) -> SPI63_R {
        SPI63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shared interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn spi32(&mut self) -> SPI32_W<0> {
        SPI32_W::new(self)
    }
    #[doc = "Bit 1 - Shared interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn spi33(&mut self) -> SPI33_W<1> {
        SPI33_W::new(self)
    }
    #[doc = "Bit 2 - Shared interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn spi34(&mut self) -> SPI34_W<2> {
        SPI34_W::new(self)
    }
    #[doc = "Bit 3 - Shared interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn spi35(&mut self) -> SPI35_W<3> {
        SPI35_W::new(self)
    }
    #[doc = "Bit 4 - Shared interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn spi36(&mut self) -> SPI36_W<4> {
        SPI36_W::new(self)
    }
    #[doc = "Bit 5 - Shared interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn spi37(&mut self) -> SPI37_W<5> {
        SPI37_W::new(self)
    }
    #[doc = "Bit 6 - Shared interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn spi38(&mut self) -> SPI38_W<6> {
        SPI38_W::new(self)
    }
    #[doc = "Bit 7 - Shared interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn spi39(&mut self) -> SPI39_W<7> {
        SPI39_W::new(self)
    }
    #[doc = "Bit 8 - Shared interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn spi40(&mut self) -> SPI40_W<8> {
        SPI40_W::new(self)
    }
    #[doc = "Bit 9 - Shared interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn spi41(&mut self) -> SPI41_W<9> {
        SPI41_W::new(self)
    }
    #[doc = "Bit 10 - Shared interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn spi42(&mut self) -> SPI42_W<10> {
        SPI42_W::new(self)
    }
    #[doc = "Bit 11 - Shared interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn spi43(&mut self) -> SPI43_W<11> {
        SPI43_W::new(self)
    }
    #[doc = "Bit 12 - Shared interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn spi44(&mut self) -> SPI44_W<12> {
        SPI44_W::new(self)
    }
    #[doc = "Bit 13 - Shared interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn spi45(&mut self) -> SPI45_W<13> {
        SPI45_W::new(self)
    }
    #[doc = "Bit 14 - Shared interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn spi46(&mut self) -> SPI46_W<14> {
        SPI46_W::new(self)
    }
    #[doc = "Bit 15 - Shared interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn spi47(&mut self) -> SPI47_W<15> {
        SPI47_W::new(self)
    }
    #[doc = "Bit 16 - Shared interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn spi48(&mut self) -> SPI48_W<16> {
        SPI48_W::new(self)
    }
    #[doc = "Bit 17 - Shared interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn spi49(&mut self) -> SPI49_W<17> {
        SPI49_W::new(self)
    }
    #[doc = "Bit 18 - Shared interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn spi50(&mut self) -> SPI50_W<18> {
        SPI50_W::new(self)
    }
    #[doc = "Bit 19 - Shared interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn spi51(&mut self) -> SPI51_W<19> {
        SPI51_W::new(self)
    }
    #[doc = "Bit 20 - Shared interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn spi52(&mut self) -> SPI52_W<20> {
        SPI52_W::new(self)
    }
    #[doc = "Bit 21 - Shared interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn spi53(&mut self) -> SPI53_W<21> {
        SPI53_W::new(self)
    }
    #[doc = "Bit 22 - Shared interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn spi54(&mut self) -> SPI54_W<22> {
        SPI54_W::new(self)
    }
    #[doc = "Bit 23 - Shared interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn spi55(&mut self) -> SPI55_W<23> {
        SPI55_W::new(self)
    }
    #[doc = "Bit 24 - Shared interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn spi56(&mut self) -> SPI56_W<24> {
        SPI56_W::new(self)
    }
    #[doc = "Bit 25 - Shared interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn spi57(&mut self) -> SPI57_W<25> {
        SPI57_W::new(self)
    }
    #[doc = "Bit 26 - Shared interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn spi58(&mut self) -> SPI58_W<26> {
        SPI58_W::new(self)
    }
    #[doc = "Bit 27 - Shared interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn spi59(&mut self) -> SPI59_W<27> {
        SPI59_W::new(self)
    }
    #[doc = "Bit 28 - Shared interrupt 60"]
    #[inline(always)]
    #[must_use]
    pub fn spi60(&mut self) -> SPI60_W<28> {
        SPI60_W::new(self)
    }
    #[doc = "Bit 29 - Shared interrupt 61"]
    #[inline(always)]
    #[must_use]
    pub fn spi61(&mut self) -> SPI61_W<29> {
        SPI61_W::new(self)
    }
    #[doc = "Bit 30 - Shared interrupt 62"]
    #[inline(always)]
    #[must_use]
    pub fn spi62(&mut self) -> SPI62_W<30> {
        SPI62_W::new(self)
    }
    #[doc = "Bit 31 - Shared interrupt 63"]
    #[inline(always)]
    #[must_use]
    pub fn spi63(&mut self) -> SPI63_W<31> {
        SPI63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared Peripheral Interrupt Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr0](index.html) module"]
pub struct GICD_SPISR0_SPEC;
impl crate::RegisterSpec for GICD_SPISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr0::R](R) reader structure"]
impl crate::Readable for GICD_SPISR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_spisr0::W](W) writer structure"]
impl crate::Writable for GICD_SPISR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SPISR0 to value 0"]
impl crate::Resettable for GICD_SPISR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

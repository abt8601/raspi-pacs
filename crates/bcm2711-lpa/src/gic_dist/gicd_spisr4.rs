#[doc = "Register `GICD_SPISR4` reader"]
pub struct R(crate::R<GICD_SPISR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_SPISR4` writer"]
pub struct W(crate::W<GICD_SPISR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SPISR4_SPEC>;
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
impl From<crate::W<GICD_SPISR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SPISR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI160` reader - Shared interrupt 160"]
pub type SPI160_R = crate::BitReader<bool>;
#[doc = "Field `SPI160` writer - Shared interrupt 160"]
pub type SPI160_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI161` reader - Shared interrupt 161"]
pub type SPI161_R = crate::BitReader<bool>;
#[doc = "Field `SPI161` writer - Shared interrupt 161"]
pub type SPI161_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI162` reader - Shared interrupt 162"]
pub type SPI162_R = crate::BitReader<bool>;
#[doc = "Field `SPI162` writer - Shared interrupt 162"]
pub type SPI162_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI163` reader - Shared interrupt 163"]
pub type SPI163_R = crate::BitReader<bool>;
#[doc = "Field `SPI163` writer - Shared interrupt 163"]
pub type SPI163_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI164` reader - Shared interrupt 164"]
pub type SPI164_R = crate::BitReader<bool>;
#[doc = "Field `SPI164` writer - Shared interrupt 164"]
pub type SPI164_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI165` reader - Shared interrupt 165"]
pub type SPI165_R = crate::BitReader<bool>;
#[doc = "Field `SPI165` writer - Shared interrupt 165"]
pub type SPI165_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI166` reader - Shared interrupt 166"]
pub type SPI166_R = crate::BitReader<bool>;
#[doc = "Field `SPI166` writer - Shared interrupt 166"]
pub type SPI166_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI167` reader - Shared interrupt 167"]
pub type SPI167_R = crate::BitReader<bool>;
#[doc = "Field `SPI167` writer - Shared interrupt 167"]
pub type SPI167_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI168` reader - Shared interrupt 168"]
pub type SPI168_R = crate::BitReader<bool>;
#[doc = "Field `SPI168` writer - Shared interrupt 168"]
pub type SPI168_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI169` reader - Shared interrupt 169"]
pub type SPI169_R = crate::BitReader<bool>;
#[doc = "Field `SPI169` writer - Shared interrupt 169"]
pub type SPI169_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI170` reader - Shared interrupt 170"]
pub type SPI170_R = crate::BitReader<bool>;
#[doc = "Field `SPI170` writer - Shared interrupt 170"]
pub type SPI170_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI171` reader - Shared interrupt 171"]
pub type SPI171_R = crate::BitReader<bool>;
#[doc = "Field `SPI171` writer - Shared interrupt 171"]
pub type SPI171_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI172` reader - Shared interrupt 172"]
pub type SPI172_R = crate::BitReader<bool>;
#[doc = "Field `SPI172` writer - Shared interrupt 172"]
pub type SPI172_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI173` reader - Shared interrupt 173"]
pub type SPI173_R = crate::BitReader<bool>;
#[doc = "Field `SPI173` writer - Shared interrupt 173"]
pub type SPI173_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI174` reader - Shared interrupt 174"]
pub type SPI174_R = crate::BitReader<bool>;
#[doc = "Field `SPI174` writer - Shared interrupt 174"]
pub type SPI174_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI175` reader - Shared interrupt 175"]
pub type SPI175_R = crate::BitReader<bool>;
#[doc = "Field `SPI175` writer - Shared interrupt 175"]
pub type SPI175_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI176` reader - Shared interrupt 176"]
pub type SPI176_R = crate::BitReader<bool>;
#[doc = "Field `SPI176` writer - Shared interrupt 176"]
pub type SPI176_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI177` reader - Shared interrupt 177"]
pub type SPI177_R = crate::BitReader<bool>;
#[doc = "Field `SPI177` writer - Shared interrupt 177"]
pub type SPI177_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI178` reader - Shared interrupt 178"]
pub type SPI178_R = crate::BitReader<bool>;
#[doc = "Field `SPI178` writer - Shared interrupt 178"]
pub type SPI178_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI179` reader - Shared interrupt 179"]
pub type SPI179_R = crate::BitReader<bool>;
#[doc = "Field `SPI179` writer - Shared interrupt 179"]
pub type SPI179_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI180` reader - Shared interrupt 180"]
pub type SPI180_R = crate::BitReader<bool>;
#[doc = "Field `SPI180` writer - Shared interrupt 180"]
pub type SPI180_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI181` reader - Shared interrupt 181"]
pub type SPI181_R = crate::BitReader<bool>;
#[doc = "Field `SPI181` writer - Shared interrupt 181"]
pub type SPI181_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI182` reader - Shared interrupt 182"]
pub type SPI182_R = crate::BitReader<bool>;
#[doc = "Field `SPI182` writer - Shared interrupt 182"]
pub type SPI182_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI183` reader - Shared interrupt 183"]
pub type SPI183_R = crate::BitReader<bool>;
#[doc = "Field `SPI183` writer - Shared interrupt 183"]
pub type SPI183_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI184` reader - Shared interrupt 184"]
pub type SPI184_R = crate::BitReader<bool>;
#[doc = "Field `SPI184` writer - Shared interrupt 184"]
pub type SPI184_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI185` reader - Shared interrupt 185"]
pub type SPI185_R = crate::BitReader<bool>;
#[doc = "Field `SPI185` writer - Shared interrupt 185"]
pub type SPI185_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI186` reader - Shared interrupt 186"]
pub type SPI186_R = crate::BitReader<bool>;
#[doc = "Field `SPI186` writer - Shared interrupt 186"]
pub type SPI186_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI187` reader - Shared interrupt 187"]
pub type SPI187_R = crate::BitReader<bool>;
#[doc = "Field `SPI187` writer - Shared interrupt 187"]
pub type SPI187_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI188` reader - Shared interrupt 188"]
pub type SPI188_R = crate::BitReader<bool>;
#[doc = "Field `SPI188` writer - Shared interrupt 188"]
pub type SPI188_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI189` reader - Shared interrupt 189"]
pub type SPI189_R = crate::BitReader<bool>;
#[doc = "Field `SPI189` writer - Shared interrupt 189"]
pub type SPI189_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI190` reader - Shared interrupt 190"]
pub type SPI190_R = crate::BitReader<bool>;
#[doc = "Field `SPI190` writer - Shared interrupt 190"]
pub type SPI190_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
#[doc = "Field `SPI191` reader - Shared interrupt 191"]
pub type SPI191_R = crate::BitReader<bool>;
#[doc = "Field `SPI191` writer - Shared interrupt 191"]
pub type SPI191_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Shared interrupt 160"]
    #[inline(always)]
    pub fn spi160(&self) -> SPI160_R {
        SPI160_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shared interrupt 161"]
    #[inline(always)]
    pub fn spi161(&self) -> SPI161_R {
        SPI161_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shared interrupt 162"]
    #[inline(always)]
    pub fn spi162(&self) -> SPI162_R {
        SPI162_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shared interrupt 163"]
    #[inline(always)]
    pub fn spi163(&self) -> SPI163_R {
        SPI163_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shared interrupt 164"]
    #[inline(always)]
    pub fn spi164(&self) -> SPI164_R {
        SPI164_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shared interrupt 165"]
    #[inline(always)]
    pub fn spi165(&self) -> SPI165_R {
        SPI165_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shared interrupt 166"]
    #[inline(always)]
    pub fn spi166(&self) -> SPI166_R {
        SPI166_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shared interrupt 167"]
    #[inline(always)]
    pub fn spi167(&self) -> SPI167_R {
        SPI167_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Shared interrupt 168"]
    #[inline(always)]
    pub fn spi168(&self) -> SPI168_R {
        SPI168_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shared interrupt 169"]
    #[inline(always)]
    pub fn spi169(&self) -> SPI169_R {
        SPI169_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shared interrupt 170"]
    #[inline(always)]
    pub fn spi170(&self) -> SPI170_R {
        SPI170_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shared interrupt 171"]
    #[inline(always)]
    pub fn spi171(&self) -> SPI171_R {
        SPI171_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Shared interrupt 172"]
    #[inline(always)]
    pub fn spi172(&self) -> SPI172_R {
        SPI172_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Shared interrupt 173"]
    #[inline(always)]
    pub fn spi173(&self) -> SPI173_R {
        SPI173_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shared interrupt 174"]
    #[inline(always)]
    pub fn spi174(&self) -> SPI174_R {
        SPI174_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Shared interrupt 175"]
    #[inline(always)]
    pub fn spi175(&self) -> SPI175_R {
        SPI175_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shared interrupt 176"]
    #[inline(always)]
    pub fn spi176(&self) -> SPI176_R {
        SPI176_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shared interrupt 177"]
    #[inline(always)]
    pub fn spi177(&self) -> SPI177_R {
        SPI177_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shared interrupt 178"]
    #[inline(always)]
    pub fn spi178(&self) -> SPI178_R {
        SPI178_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shared interrupt 179"]
    #[inline(always)]
    pub fn spi179(&self) -> SPI179_R {
        SPI179_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shared interrupt 180"]
    #[inline(always)]
    pub fn spi180(&self) -> SPI180_R {
        SPI180_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shared interrupt 181"]
    #[inline(always)]
    pub fn spi181(&self) -> SPI181_R {
        SPI181_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shared interrupt 182"]
    #[inline(always)]
    pub fn spi182(&self) -> SPI182_R {
        SPI182_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shared interrupt 183"]
    #[inline(always)]
    pub fn spi183(&self) -> SPI183_R {
        SPI183_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared interrupt 184"]
    #[inline(always)]
    pub fn spi184(&self) -> SPI184_R {
        SPI184_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shared interrupt 185"]
    #[inline(always)]
    pub fn spi185(&self) -> SPI185_R {
        SPI185_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shared interrupt 186"]
    #[inline(always)]
    pub fn spi186(&self) -> SPI186_R {
        SPI186_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shared interrupt 187"]
    #[inline(always)]
    pub fn spi187(&self) -> SPI187_R {
        SPI187_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shared interrupt 188"]
    #[inline(always)]
    pub fn spi188(&self) -> SPI188_R {
        SPI188_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shared interrupt 189"]
    #[inline(always)]
    pub fn spi189(&self) -> SPI189_R {
        SPI189_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Shared interrupt 190"]
    #[inline(always)]
    pub fn spi190(&self) -> SPI190_R {
        SPI190_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Shared interrupt 191"]
    #[inline(always)]
    pub fn spi191(&self) -> SPI191_R {
        SPI191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shared interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn spi160(&mut self) -> SPI160_W<0> {
        SPI160_W::new(self)
    }
    #[doc = "Bit 1 - Shared interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn spi161(&mut self) -> SPI161_W<1> {
        SPI161_W::new(self)
    }
    #[doc = "Bit 2 - Shared interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn spi162(&mut self) -> SPI162_W<2> {
        SPI162_W::new(self)
    }
    #[doc = "Bit 3 - Shared interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn spi163(&mut self) -> SPI163_W<3> {
        SPI163_W::new(self)
    }
    #[doc = "Bit 4 - Shared interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn spi164(&mut self) -> SPI164_W<4> {
        SPI164_W::new(self)
    }
    #[doc = "Bit 5 - Shared interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn spi165(&mut self) -> SPI165_W<5> {
        SPI165_W::new(self)
    }
    #[doc = "Bit 6 - Shared interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn spi166(&mut self) -> SPI166_W<6> {
        SPI166_W::new(self)
    }
    #[doc = "Bit 7 - Shared interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn spi167(&mut self) -> SPI167_W<7> {
        SPI167_W::new(self)
    }
    #[doc = "Bit 8 - Shared interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn spi168(&mut self) -> SPI168_W<8> {
        SPI168_W::new(self)
    }
    #[doc = "Bit 9 - Shared interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn spi169(&mut self) -> SPI169_W<9> {
        SPI169_W::new(self)
    }
    #[doc = "Bit 10 - Shared interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn spi170(&mut self) -> SPI170_W<10> {
        SPI170_W::new(self)
    }
    #[doc = "Bit 11 - Shared interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn spi171(&mut self) -> SPI171_W<11> {
        SPI171_W::new(self)
    }
    #[doc = "Bit 12 - Shared interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn spi172(&mut self) -> SPI172_W<12> {
        SPI172_W::new(self)
    }
    #[doc = "Bit 13 - Shared interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn spi173(&mut self) -> SPI173_W<13> {
        SPI173_W::new(self)
    }
    #[doc = "Bit 14 - Shared interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn spi174(&mut self) -> SPI174_W<14> {
        SPI174_W::new(self)
    }
    #[doc = "Bit 15 - Shared interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn spi175(&mut self) -> SPI175_W<15> {
        SPI175_W::new(self)
    }
    #[doc = "Bit 16 - Shared interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn spi176(&mut self) -> SPI176_W<16> {
        SPI176_W::new(self)
    }
    #[doc = "Bit 17 - Shared interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn spi177(&mut self) -> SPI177_W<17> {
        SPI177_W::new(self)
    }
    #[doc = "Bit 18 - Shared interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn spi178(&mut self) -> SPI178_W<18> {
        SPI178_W::new(self)
    }
    #[doc = "Bit 19 - Shared interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn spi179(&mut self) -> SPI179_W<19> {
        SPI179_W::new(self)
    }
    #[doc = "Bit 20 - Shared interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn spi180(&mut self) -> SPI180_W<20> {
        SPI180_W::new(self)
    }
    #[doc = "Bit 21 - Shared interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn spi181(&mut self) -> SPI181_W<21> {
        SPI181_W::new(self)
    }
    #[doc = "Bit 22 - Shared interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn spi182(&mut self) -> SPI182_W<22> {
        SPI182_W::new(self)
    }
    #[doc = "Bit 23 - Shared interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn spi183(&mut self) -> SPI183_W<23> {
        SPI183_W::new(self)
    }
    #[doc = "Bit 24 - Shared interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn spi184(&mut self) -> SPI184_W<24> {
        SPI184_W::new(self)
    }
    #[doc = "Bit 25 - Shared interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn spi185(&mut self) -> SPI185_W<25> {
        SPI185_W::new(self)
    }
    #[doc = "Bit 26 - Shared interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn spi186(&mut self) -> SPI186_W<26> {
        SPI186_W::new(self)
    }
    #[doc = "Bit 27 - Shared interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn spi187(&mut self) -> SPI187_W<27> {
        SPI187_W::new(self)
    }
    #[doc = "Bit 28 - Shared interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn spi188(&mut self) -> SPI188_W<28> {
        SPI188_W::new(self)
    }
    #[doc = "Bit 29 - Shared interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn spi189(&mut self) -> SPI189_W<29> {
        SPI189_W::new(self)
    }
    #[doc = "Bit 30 - Shared interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn spi190(&mut self) -> SPI190_W<30> {
        SPI190_W::new(self)
    }
    #[doc = "Bit 31 - Shared interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn spi191(&mut self) -> SPI191_W<31> {
        SPI191_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared Peripheral Interrupt Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr4](index.html) module"]
pub struct GICD_SPISR4_SPEC;
impl crate::RegisterSpec for GICD_SPISR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr4::R](R) reader structure"]
impl crate::Readable for GICD_SPISR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_spisr4::W](W) writer structure"]
impl crate::Writable for GICD_SPISR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SPISR4 to value 0"]
impl crate::Resettable for GICD_SPISR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

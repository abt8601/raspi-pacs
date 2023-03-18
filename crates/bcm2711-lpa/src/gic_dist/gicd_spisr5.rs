#[doc = "Register `GICD_SPISR5` reader"]
pub struct R(crate::R<GICD_SPISR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_SPISR5` writer"]
pub struct W(crate::W<GICD_SPISR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SPISR5_SPEC>;
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
impl From<crate::W<GICD_SPISR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SPISR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI192` reader - Shared interrupt 192"]
pub type SPI192_R = crate::BitReader<bool>;
#[doc = "Field `SPI192` writer - Shared interrupt 192"]
pub type SPI192_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI193` reader - Shared interrupt 193"]
pub type SPI193_R = crate::BitReader<bool>;
#[doc = "Field `SPI193` writer - Shared interrupt 193"]
pub type SPI193_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI194` reader - Shared interrupt 194"]
pub type SPI194_R = crate::BitReader<bool>;
#[doc = "Field `SPI194` writer - Shared interrupt 194"]
pub type SPI194_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI195` reader - Shared interrupt 195"]
pub type SPI195_R = crate::BitReader<bool>;
#[doc = "Field `SPI195` writer - Shared interrupt 195"]
pub type SPI195_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI196` reader - Shared interrupt 196"]
pub type SPI196_R = crate::BitReader<bool>;
#[doc = "Field `SPI196` writer - Shared interrupt 196"]
pub type SPI196_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI197` reader - Shared interrupt 197"]
pub type SPI197_R = crate::BitReader<bool>;
#[doc = "Field `SPI197` writer - Shared interrupt 197"]
pub type SPI197_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI198` reader - Shared interrupt 198"]
pub type SPI198_R = crate::BitReader<bool>;
#[doc = "Field `SPI198` writer - Shared interrupt 198"]
pub type SPI198_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI199` reader - Shared interrupt 199"]
pub type SPI199_R = crate::BitReader<bool>;
#[doc = "Field `SPI199` writer - Shared interrupt 199"]
pub type SPI199_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI200` reader - Shared interrupt 200"]
pub type SPI200_R = crate::BitReader<bool>;
#[doc = "Field `SPI200` writer - Shared interrupt 200"]
pub type SPI200_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI201` reader - Shared interrupt 201"]
pub type SPI201_R = crate::BitReader<bool>;
#[doc = "Field `SPI201` writer - Shared interrupt 201"]
pub type SPI201_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI202` reader - Shared interrupt 202"]
pub type SPI202_R = crate::BitReader<bool>;
#[doc = "Field `SPI202` writer - Shared interrupt 202"]
pub type SPI202_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI203` reader - Shared interrupt 203"]
pub type SPI203_R = crate::BitReader<bool>;
#[doc = "Field `SPI203` writer - Shared interrupt 203"]
pub type SPI203_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI204` reader - Shared interrupt 204"]
pub type SPI204_R = crate::BitReader<bool>;
#[doc = "Field `SPI204` writer - Shared interrupt 204"]
pub type SPI204_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI205` reader - Shared interrupt 205"]
pub type SPI205_R = crate::BitReader<bool>;
#[doc = "Field `SPI205` writer - Shared interrupt 205"]
pub type SPI205_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI206` reader - Shared interrupt 206"]
pub type SPI206_R = crate::BitReader<bool>;
#[doc = "Field `SPI206` writer - Shared interrupt 206"]
pub type SPI206_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI207` reader - Shared interrupt 207"]
pub type SPI207_R = crate::BitReader<bool>;
#[doc = "Field `SPI207` writer - Shared interrupt 207"]
pub type SPI207_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI208` reader - Shared interrupt 208"]
pub type SPI208_R = crate::BitReader<bool>;
#[doc = "Field `SPI208` writer - Shared interrupt 208"]
pub type SPI208_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI209` reader - Shared interrupt 209"]
pub type SPI209_R = crate::BitReader<bool>;
#[doc = "Field `SPI209` writer - Shared interrupt 209"]
pub type SPI209_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI210` reader - Shared interrupt 210"]
pub type SPI210_R = crate::BitReader<bool>;
#[doc = "Field `SPI210` writer - Shared interrupt 210"]
pub type SPI210_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI211` reader - Shared interrupt 211"]
pub type SPI211_R = crate::BitReader<bool>;
#[doc = "Field `SPI211` writer - Shared interrupt 211"]
pub type SPI211_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI212` reader - Shared interrupt 212"]
pub type SPI212_R = crate::BitReader<bool>;
#[doc = "Field `SPI212` writer - Shared interrupt 212"]
pub type SPI212_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI213` reader - Shared interrupt 213"]
pub type SPI213_R = crate::BitReader<bool>;
#[doc = "Field `SPI213` writer - Shared interrupt 213"]
pub type SPI213_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI214` reader - Shared interrupt 214"]
pub type SPI214_R = crate::BitReader<bool>;
#[doc = "Field `SPI214` writer - Shared interrupt 214"]
pub type SPI214_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI215` reader - Shared interrupt 215"]
pub type SPI215_R = crate::BitReader<bool>;
#[doc = "Field `SPI215` writer - Shared interrupt 215"]
pub type SPI215_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI216` reader - Shared interrupt 216"]
pub type SPI216_R = crate::BitReader<bool>;
#[doc = "Field `SPI216` writer - Shared interrupt 216"]
pub type SPI216_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI217` reader - Shared interrupt 217"]
pub type SPI217_R = crate::BitReader<bool>;
#[doc = "Field `SPI217` writer - Shared interrupt 217"]
pub type SPI217_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI218` reader - Shared interrupt 218"]
pub type SPI218_R = crate::BitReader<bool>;
#[doc = "Field `SPI218` writer - Shared interrupt 218"]
pub type SPI218_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI219` reader - Shared interrupt 219"]
pub type SPI219_R = crate::BitReader<bool>;
#[doc = "Field `SPI219` writer - Shared interrupt 219"]
pub type SPI219_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI220` reader - Shared interrupt 220"]
pub type SPI220_R = crate::BitReader<bool>;
#[doc = "Field `SPI220` writer - Shared interrupt 220"]
pub type SPI220_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI221` reader - Shared interrupt 221"]
pub type SPI221_R = crate::BitReader<bool>;
#[doc = "Field `SPI221` writer - Shared interrupt 221"]
pub type SPI221_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI222` reader - Shared interrupt 222"]
pub type SPI222_R = crate::BitReader<bool>;
#[doc = "Field `SPI222` writer - Shared interrupt 222"]
pub type SPI222_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
#[doc = "Field `SPI223` reader - Shared interrupt 223"]
pub type SPI223_R = crate::BitReader<bool>;
#[doc = "Field `SPI223` writer - Shared interrupt 223"]
pub type SPI223_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_SPISR5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Shared interrupt 192"]
    #[inline(always)]
    pub fn spi192(&self) -> SPI192_R {
        SPI192_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shared interrupt 193"]
    #[inline(always)]
    pub fn spi193(&self) -> SPI193_R {
        SPI193_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shared interrupt 194"]
    #[inline(always)]
    pub fn spi194(&self) -> SPI194_R {
        SPI194_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shared interrupt 195"]
    #[inline(always)]
    pub fn spi195(&self) -> SPI195_R {
        SPI195_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shared interrupt 196"]
    #[inline(always)]
    pub fn spi196(&self) -> SPI196_R {
        SPI196_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shared interrupt 197"]
    #[inline(always)]
    pub fn spi197(&self) -> SPI197_R {
        SPI197_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shared interrupt 198"]
    #[inline(always)]
    pub fn spi198(&self) -> SPI198_R {
        SPI198_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shared interrupt 199"]
    #[inline(always)]
    pub fn spi199(&self) -> SPI199_R {
        SPI199_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Shared interrupt 200"]
    #[inline(always)]
    pub fn spi200(&self) -> SPI200_R {
        SPI200_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shared interrupt 201"]
    #[inline(always)]
    pub fn spi201(&self) -> SPI201_R {
        SPI201_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shared interrupt 202"]
    #[inline(always)]
    pub fn spi202(&self) -> SPI202_R {
        SPI202_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shared interrupt 203"]
    #[inline(always)]
    pub fn spi203(&self) -> SPI203_R {
        SPI203_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Shared interrupt 204"]
    #[inline(always)]
    pub fn spi204(&self) -> SPI204_R {
        SPI204_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Shared interrupt 205"]
    #[inline(always)]
    pub fn spi205(&self) -> SPI205_R {
        SPI205_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shared interrupt 206"]
    #[inline(always)]
    pub fn spi206(&self) -> SPI206_R {
        SPI206_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Shared interrupt 207"]
    #[inline(always)]
    pub fn spi207(&self) -> SPI207_R {
        SPI207_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shared interrupt 208"]
    #[inline(always)]
    pub fn spi208(&self) -> SPI208_R {
        SPI208_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shared interrupt 209"]
    #[inline(always)]
    pub fn spi209(&self) -> SPI209_R {
        SPI209_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shared interrupt 210"]
    #[inline(always)]
    pub fn spi210(&self) -> SPI210_R {
        SPI210_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shared interrupt 211"]
    #[inline(always)]
    pub fn spi211(&self) -> SPI211_R {
        SPI211_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shared interrupt 212"]
    #[inline(always)]
    pub fn spi212(&self) -> SPI212_R {
        SPI212_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shared interrupt 213"]
    #[inline(always)]
    pub fn spi213(&self) -> SPI213_R {
        SPI213_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shared interrupt 214"]
    #[inline(always)]
    pub fn spi214(&self) -> SPI214_R {
        SPI214_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shared interrupt 215"]
    #[inline(always)]
    pub fn spi215(&self) -> SPI215_R {
        SPI215_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared interrupt 216"]
    #[inline(always)]
    pub fn spi216(&self) -> SPI216_R {
        SPI216_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shared interrupt 217"]
    #[inline(always)]
    pub fn spi217(&self) -> SPI217_R {
        SPI217_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shared interrupt 218"]
    #[inline(always)]
    pub fn spi218(&self) -> SPI218_R {
        SPI218_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shared interrupt 219"]
    #[inline(always)]
    pub fn spi219(&self) -> SPI219_R {
        SPI219_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shared interrupt 220"]
    #[inline(always)]
    pub fn spi220(&self) -> SPI220_R {
        SPI220_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shared interrupt 221"]
    #[inline(always)]
    pub fn spi221(&self) -> SPI221_R {
        SPI221_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Shared interrupt 222"]
    #[inline(always)]
    pub fn spi222(&self) -> SPI222_R {
        SPI222_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Shared interrupt 223"]
    #[inline(always)]
    pub fn spi223(&self) -> SPI223_R {
        SPI223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shared interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn spi192(&mut self) -> SPI192_W<0> {
        SPI192_W::new(self)
    }
    #[doc = "Bit 1 - Shared interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn spi193(&mut self) -> SPI193_W<1> {
        SPI193_W::new(self)
    }
    #[doc = "Bit 2 - Shared interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn spi194(&mut self) -> SPI194_W<2> {
        SPI194_W::new(self)
    }
    #[doc = "Bit 3 - Shared interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn spi195(&mut self) -> SPI195_W<3> {
        SPI195_W::new(self)
    }
    #[doc = "Bit 4 - Shared interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn spi196(&mut self) -> SPI196_W<4> {
        SPI196_W::new(self)
    }
    #[doc = "Bit 5 - Shared interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn spi197(&mut self) -> SPI197_W<5> {
        SPI197_W::new(self)
    }
    #[doc = "Bit 6 - Shared interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn spi198(&mut self) -> SPI198_W<6> {
        SPI198_W::new(self)
    }
    #[doc = "Bit 7 - Shared interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn spi199(&mut self) -> SPI199_W<7> {
        SPI199_W::new(self)
    }
    #[doc = "Bit 8 - Shared interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn spi200(&mut self) -> SPI200_W<8> {
        SPI200_W::new(self)
    }
    #[doc = "Bit 9 - Shared interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn spi201(&mut self) -> SPI201_W<9> {
        SPI201_W::new(self)
    }
    #[doc = "Bit 10 - Shared interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn spi202(&mut self) -> SPI202_W<10> {
        SPI202_W::new(self)
    }
    #[doc = "Bit 11 - Shared interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn spi203(&mut self) -> SPI203_W<11> {
        SPI203_W::new(self)
    }
    #[doc = "Bit 12 - Shared interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn spi204(&mut self) -> SPI204_W<12> {
        SPI204_W::new(self)
    }
    #[doc = "Bit 13 - Shared interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn spi205(&mut self) -> SPI205_W<13> {
        SPI205_W::new(self)
    }
    #[doc = "Bit 14 - Shared interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn spi206(&mut self) -> SPI206_W<14> {
        SPI206_W::new(self)
    }
    #[doc = "Bit 15 - Shared interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn spi207(&mut self) -> SPI207_W<15> {
        SPI207_W::new(self)
    }
    #[doc = "Bit 16 - Shared interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn spi208(&mut self) -> SPI208_W<16> {
        SPI208_W::new(self)
    }
    #[doc = "Bit 17 - Shared interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn spi209(&mut self) -> SPI209_W<17> {
        SPI209_W::new(self)
    }
    #[doc = "Bit 18 - Shared interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn spi210(&mut self) -> SPI210_W<18> {
        SPI210_W::new(self)
    }
    #[doc = "Bit 19 - Shared interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn spi211(&mut self) -> SPI211_W<19> {
        SPI211_W::new(self)
    }
    #[doc = "Bit 20 - Shared interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn spi212(&mut self) -> SPI212_W<20> {
        SPI212_W::new(self)
    }
    #[doc = "Bit 21 - Shared interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn spi213(&mut self) -> SPI213_W<21> {
        SPI213_W::new(self)
    }
    #[doc = "Bit 22 - Shared interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn spi214(&mut self) -> SPI214_W<22> {
        SPI214_W::new(self)
    }
    #[doc = "Bit 23 - Shared interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn spi215(&mut self) -> SPI215_W<23> {
        SPI215_W::new(self)
    }
    #[doc = "Bit 24 - Shared interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn spi216(&mut self) -> SPI216_W<24> {
        SPI216_W::new(self)
    }
    #[doc = "Bit 25 - Shared interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn spi217(&mut self) -> SPI217_W<25> {
        SPI217_W::new(self)
    }
    #[doc = "Bit 26 - Shared interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn spi218(&mut self) -> SPI218_W<26> {
        SPI218_W::new(self)
    }
    #[doc = "Bit 27 - Shared interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn spi219(&mut self) -> SPI219_W<27> {
        SPI219_W::new(self)
    }
    #[doc = "Bit 28 - Shared interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn spi220(&mut self) -> SPI220_W<28> {
        SPI220_W::new(self)
    }
    #[doc = "Bit 29 - Shared interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn spi221(&mut self) -> SPI221_W<29> {
        SPI221_W::new(self)
    }
    #[doc = "Bit 30 - Shared interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn spi222(&mut self) -> SPI222_W<30> {
        SPI222_W::new(self)
    }
    #[doc = "Bit 31 - Shared interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn spi223(&mut self) -> SPI223_W<31> {
        SPI223_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared Peripheral Interrupt Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr5](index.html) module"]
pub struct GICD_SPISR5_SPEC;
impl crate::RegisterSpec for GICD_SPISR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr5::R](R) reader structure"]
impl crate::Readable for GICD_SPISR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_spisr5::W](W) writer structure"]
impl crate::Writable for GICD_SPISR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SPISR5 to value 0"]
impl crate::Resettable for GICD_SPISR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

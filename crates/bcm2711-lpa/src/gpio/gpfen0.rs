#[doc = "Register `GPFEN0` reader"]
pub struct R(crate::R<GPFEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPFEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPFEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPFEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPFEN0` writer"]
pub struct W(crate::W<GPFEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPFEN0_SPEC>;
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
impl From<crate::W<GPFEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPFEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEN0` reader - Falling edge enabled 0"]
pub type FEN0_R = crate::BitReader<bool>;
#[doc = "Field `FEN0` writer - Falling edge enabled 0"]
pub type FEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN1` reader - Falling edge enabled 1"]
pub type FEN1_R = crate::BitReader<bool>;
#[doc = "Field `FEN1` writer - Falling edge enabled 1"]
pub type FEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN2` reader - Falling edge enabled 2"]
pub type FEN2_R = crate::BitReader<bool>;
#[doc = "Field `FEN2` writer - Falling edge enabled 2"]
pub type FEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN3` reader - Falling edge enabled 3"]
pub type FEN3_R = crate::BitReader<bool>;
#[doc = "Field `FEN3` writer - Falling edge enabled 3"]
pub type FEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN4` reader - Falling edge enabled 4"]
pub type FEN4_R = crate::BitReader<bool>;
#[doc = "Field `FEN4` writer - Falling edge enabled 4"]
pub type FEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN5` reader - Falling edge enabled 5"]
pub type FEN5_R = crate::BitReader<bool>;
#[doc = "Field `FEN5` writer - Falling edge enabled 5"]
pub type FEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN6` reader - Falling edge enabled 6"]
pub type FEN6_R = crate::BitReader<bool>;
#[doc = "Field `FEN6` writer - Falling edge enabled 6"]
pub type FEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN7` reader - Falling edge enabled 7"]
pub type FEN7_R = crate::BitReader<bool>;
#[doc = "Field `FEN7` writer - Falling edge enabled 7"]
pub type FEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN8` reader - Falling edge enabled 8"]
pub type FEN8_R = crate::BitReader<bool>;
#[doc = "Field `FEN8` writer - Falling edge enabled 8"]
pub type FEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN9` reader - Falling edge enabled 9"]
pub type FEN9_R = crate::BitReader<bool>;
#[doc = "Field `FEN9` writer - Falling edge enabled 9"]
pub type FEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN10` reader - Falling edge enabled 10"]
pub type FEN10_R = crate::BitReader<bool>;
#[doc = "Field `FEN10` writer - Falling edge enabled 10"]
pub type FEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN11` reader - Falling edge enabled 11"]
pub type FEN11_R = crate::BitReader<bool>;
#[doc = "Field `FEN11` writer - Falling edge enabled 11"]
pub type FEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN12` reader - Falling edge enabled 12"]
pub type FEN12_R = crate::BitReader<bool>;
#[doc = "Field `FEN12` writer - Falling edge enabled 12"]
pub type FEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN13` reader - Falling edge enabled 13"]
pub type FEN13_R = crate::BitReader<bool>;
#[doc = "Field `FEN13` writer - Falling edge enabled 13"]
pub type FEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN14` reader - Falling edge enabled 14"]
pub type FEN14_R = crate::BitReader<bool>;
#[doc = "Field `FEN14` writer - Falling edge enabled 14"]
pub type FEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN15` reader - Falling edge enabled 15"]
pub type FEN15_R = crate::BitReader<bool>;
#[doc = "Field `FEN15` writer - Falling edge enabled 15"]
pub type FEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN16` reader - Falling edge enabled 16"]
pub type FEN16_R = crate::BitReader<bool>;
#[doc = "Field `FEN16` writer - Falling edge enabled 16"]
pub type FEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN17` reader - Falling edge enabled 17"]
pub type FEN17_R = crate::BitReader<bool>;
#[doc = "Field `FEN17` writer - Falling edge enabled 17"]
pub type FEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN18` reader - Falling edge enabled 18"]
pub type FEN18_R = crate::BitReader<bool>;
#[doc = "Field `FEN18` writer - Falling edge enabled 18"]
pub type FEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN19` reader - Falling edge enabled 19"]
pub type FEN19_R = crate::BitReader<bool>;
#[doc = "Field `FEN19` writer - Falling edge enabled 19"]
pub type FEN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN20` reader - Falling edge enabled 20"]
pub type FEN20_R = crate::BitReader<bool>;
#[doc = "Field `FEN20` writer - Falling edge enabled 20"]
pub type FEN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN21` reader - Falling edge enabled 21"]
pub type FEN21_R = crate::BitReader<bool>;
#[doc = "Field `FEN21` writer - Falling edge enabled 21"]
pub type FEN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN22` reader - Falling edge enabled 22"]
pub type FEN22_R = crate::BitReader<bool>;
#[doc = "Field `FEN22` writer - Falling edge enabled 22"]
pub type FEN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN23` reader - Falling edge enabled 23"]
pub type FEN23_R = crate::BitReader<bool>;
#[doc = "Field `FEN23` writer - Falling edge enabled 23"]
pub type FEN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN24` reader - Falling edge enabled 24"]
pub type FEN24_R = crate::BitReader<bool>;
#[doc = "Field `FEN24` writer - Falling edge enabled 24"]
pub type FEN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN25` reader - Falling edge enabled 25"]
pub type FEN25_R = crate::BitReader<bool>;
#[doc = "Field `FEN25` writer - Falling edge enabled 25"]
pub type FEN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN26` reader - Falling edge enabled 26"]
pub type FEN26_R = crate::BitReader<bool>;
#[doc = "Field `FEN26` writer - Falling edge enabled 26"]
pub type FEN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN27` reader - Falling edge enabled 27"]
pub type FEN27_R = crate::BitReader<bool>;
#[doc = "Field `FEN27` writer - Falling edge enabled 27"]
pub type FEN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN28` reader - Falling edge enabled 28"]
pub type FEN28_R = crate::BitReader<bool>;
#[doc = "Field `FEN28` writer - Falling edge enabled 28"]
pub type FEN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN29` reader - Falling edge enabled 29"]
pub type FEN29_R = crate::BitReader<bool>;
#[doc = "Field `FEN29` writer - Falling edge enabled 29"]
pub type FEN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN30` reader - Falling edge enabled 30"]
pub type FEN30_R = crate::BitReader<bool>;
#[doc = "Field `FEN30` writer - Falling edge enabled 30"]
pub type FEN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
#[doc = "Field `FEN31` reader - Falling edge enabled 31"]
pub type FEN31_R = crate::BitReader<bool>;
#[doc = "Field `FEN31` writer - Falling edge enabled 31"]
pub type FEN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Falling edge enabled 0"]
    #[inline(always)]
    pub fn fen0(&self) -> FEN0_R {
        FEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge enabled 1"]
    #[inline(always)]
    pub fn fen1(&self) -> FEN1_R {
        FEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge enabled 2"]
    #[inline(always)]
    pub fn fen2(&self) -> FEN2_R {
        FEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge enabled 3"]
    #[inline(always)]
    pub fn fen3(&self) -> FEN3_R {
        FEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge enabled 4"]
    #[inline(always)]
    pub fn fen4(&self) -> FEN4_R {
        FEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge enabled 5"]
    #[inline(always)]
    pub fn fen5(&self) -> FEN5_R {
        FEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge enabled 6"]
    #[inline(always)]
    pub fn fen6(&self) -> FEN6_R {
        FEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge enabled 7"]
    #[inline(always)]
    pub fn fen7(&self) -> FEN7_R {
        FEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge enabled 8"]
    #[inline(always)]
    pub fn fen8(&self) -> FEN8_R {
        FEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge enabled 9"]
    #[inline(always)]
    pub fn fen9(&self) -> FEN9_R {
        FEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge enabled 10"]
    #[inline(always)]
    pub fn fen10(&self) -> FEN10_R {
        FEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge enabled 11"]
    #[inline(always)]
    pub fn fen11(&self) -> FEN11_R {
        FEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge enabled 12"]
    #[inline(always)]
    pub fn fen12(&self) -> FEN12_R {
        FEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge enabled 13"]
    #[inline(always)]
    pub fn fen13(&self) -> FEN13_R {
        FEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge enabled 14"]
    #[inline(always)]
    pub fn fen14(&self) -> FEN14_R {
        FEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge enabled 15"]
    #[inline(always)]
    pub fn fen15(&self) -> FEN15_R {
        FEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge enabled 16"]
    #[inline(always)]
    pub fn fen16(&self) -> FEN16_R {
        FEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge enabled 17"]
    #[inline(always)]
    pub fn fen17(&self) -> FEN17_R {
        FEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge enabled 18"]
    #[inline(always)]
    pub fn fen18(&self) -> FEN18_R {
        FEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling edge enabled 19"]
    #[inline(always)]
    pub fn fen19(&self) -> FEN19_R {
        FEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling edge enabled 20"]
    #[inline(always)]
    pub fn fen20(&self) -> FEN20_R {
        FEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling edge enabled 21"]
    #[inline(always)]
    pub fn fen21(&self) -> FEN21_R {
        FEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling edge enabled 22"]
    #[inline(always)]
    pub fn fen22(&self) -> FEN22_R {
        FEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Falling edge enabled 23"]
    #[inline(always)]
    pub fn fen23(&self) -> FEN23_R {
        FEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Falling edge enabled 24"]
    #[inline(always)]
    pub fn fen24(&self) -> FEN24_R {
        FEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Falling edge enabled 25"]
    #[inline(always)]
    pub fn fen25(&self) -> FEN25_R {
        FEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Falling edge enabled 26"]
    #[inline(always)]
    pub fn fen26(&self) -> FEN26_R {
        FEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Falling edge enabled 27"]
    #[inline(always)]
    pub fn fen27(&self) -> FEN27_R {
        FEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Falling edge enabled 28"]
    #[inline(always)]
    pub fn fen28(&self) -> FEN28_R {
        FEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Falling edge enabled 29"]
    #[inline(always)]
    pub fn fen29(&self) -> FEN29_R {
        FEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Falling edge enabled 30"]
    #[inline(always)]
    pub fn fen30(&self) -> FEN30_R {
        FEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Falling edge enabled 31"]
    #[inline(always)]
    pub fn fen31(&self) -> FEN31_R {
        FEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge enabled 0"]
    #[inline(always)]
    #[must_use]
    pub fn fen0(&mut self) -> FEN0_W<0> {
        FEN0_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge enabled 1"]
    #[inline(always)]
    #[must_use]
    pub fn fen1(&mut self) -> FEN1_W<1> {
        FEN1_W::new(self)
    }
    #[doc = "Bit 2 - Falling edge enabled 2"]
    #[inline(always)]
    #[must_use]
    pub fn fen2(&mut self) -> FEN2_W<2> {
        FEN2_W::new(self)
    }
    #[doc = "Bit 3 - Falling edge enabled 3"]
    #[inline(always)]
    #[must_use]
    pub fn fen3(&mut self) -> FEN3_W<3> {
        FEN3_W::new(self)
    }
    #[doc = "Bit 4 - Falling edge enabled 4"]
    #[inline(always)]
    #[must_use]
    pub fn fen4(&mut self) -> FEN4_W<4> {
        FEN4_W::new(self)
    }
    #[doc = "Bit 5 - Falling edge enabled 5"]
    #[inline(always)]
    #[must_use]
    pub fn fen5(&mut self) -> FEN5_W<5> {
        FEN5_W::new(self)
    }
    #[doc = "Bit 6 - Falling edge enabled 6"]
    #[inline(always)]
    #[must_use]
    pub fn fen6(&mut self) -> FEN6_W<6> {
        FEN6_W::new(self)
    }
    #[doc = "Bit 7 - Falling edge enabled 7"]
    #[inline(always)]
    #[must_use]
    pub fn fen7(&mut self) -> FEN7_W<7> {
        FEN7_W::new(self)
    }
    #[doc = "Bit 8 - Falling edge enabled 8"]
    #[inline(always)]
    #[must_use]
    pub fn fen8(&mut self) -> FEN8_W<8> {
        FEN8_W::new(self)
    }
    #[doc = "Bit 9 - Falling edge enabled 9"]
    #[inline(always)]
    #[must_use]
    pub fn fen9(&mut self) -> FEN9_W<9> {
        FEN9_W::new(self)
    }
    #[doc = "Bit 10 - Falling edge enabled 10"]
    #[inline(always)]
    #[must_use]
    pub fn fen10(&mut self) -> FEN10_W<10> {
        FEN10_W::new(self)
    }
    #[doc = "Bit 11 - Falling edge enabled 11"]
    #[inline(always)]
    #[must_use]
    pub fn fen11(&mut self) -> FEN11_W<11> {
        FEN11_W::new(self)
    }
    #[doc = "Bit 12 - Falling edge enabled 12"]
    #[inline(always)]
    #[must_use]
    pub fn fen12(&mut self) -> FEN12_W<12> {
        FEN12_W::new(self)
    }
    #[doc = "Bit 13 - Falling edge enabled 13"]
    #[inline(always)]
    #[must_use]
    pub fn fen13(&mut self) -> FEN13_W<13> {
        FEN13_W::new(self)
    }
    #[doc = "Bit 14 - Falling edge enabled 14"]
    #[inline(always)]
    #[must_use]
    pub fn fen14(&mut self) -> FEN14_W<14> {
        FEN14_W::new(self)
    }
    #[doc = "Bit 15 - Falling edge enabled 15"]
    #[inline(always)]
    #[must_use]
    pub fn fen15(&mut self) -> FEN15_W<15> {
        FEN15_W::new(self)
    }
    #[doc = "Bit 16 - Falling edge enabled 16"]
    #[inline(always)]
    #[must_use]
    pub fn fen16(&mut self) -> FEN16_W<16> {
        FEN16_W::new(self)
    }
    #[doc = "Bit 17 - Falling edge enabled 17"]
    #[inline(always)]
    #[must_use]
    pub fn fen17(&mut self) -> FEN17_W<17> {
        FEN17_W::new(self)
    }
    #[doc = "Bit 18 - Falling edge enabled 18"]
    #[inline(always)]
    #[must_use]
    pub fn fen18(&mut self) -> FEN18_W<18> {
        FEN18_W::new(self)
    }
    #[doc = "Bit 19 - Falling edge enabled 19"]
    #[inline(always)]
    #[must_use]
    pub fn fen19(&mut self) -> FEN19_W<19> {
        FEN19_W::new(self)
    }
    #[doc = "Bit 20 - Falling edge enabled 20"]
    #[inline(always)]
    #[must_use]
    pub fn fen20(&mut self) -> FEN20_W<20> {
        FEN20_W::new(self)
    }
    #[doc = "Bit 21 - Falling edge enabled 21"]
    #[inline(always)]
    #[must_use]
    pub fn fen21(&mut self) -> FEN21_W<21> {
        FEN21_W::new(self)
    }
    #[doc = "Bit 22 - Falling edge enabled 22"]
    #[inline(always)]
    #[must_use]
    pub fn fen22(&mut self) -> FEN22_W<22> {
        FEN22_W::new(self)
    }
    #[doc = "Bit 23 - Falling edge enabled 23"]
    #[inline(always)]
    #[must_use]
    pub fn fen23(&mut self) -> FEN23_W<23> {
        FEN23_W::new(self)
    }
    #[doc = "Bit 24 - Falling edge enabled 24"]
    #[inline(always)]
    #[must_use]
    pub fn fen24(&mut self) -> FEN24_W<24> {
        FEN24_W::new(self)
    }
    #[doc = "Bit 25 - Falling edge enabled 25"]
    #[inline(always)]
    #[must_use]
    pub fn fen25(&mut self) -> FEN25_W<25> {
        FEN25_W::new(self)
    }
    #[doc = "Bit 26 - Falling edge enabled 26"]
    #[inline(always)]
    #[must_use]
    pub fn fen26(&mut self) -> FEN26_W<26> {
        FEN26_W::new(self)
    }
    #[doc = "Bit 27 - Falling edge enabled 27"]
    #[inline(always)]
    #[must_use]
    pub fn fen27(&mut self) -> FEN27_W<27> {
        FEN27_W::new(self)
    }
    #[doc = "Bit 28 - Falling edge enabled 28"]
    #[inline(always)]
    #[must_use]
    pub fn fen28(&mut self) -> FEN28_W<28> {
        FEN28_W::new(self)
    }
    #[doc = "Bit 29 - Falling edge enabled 29"]
    #[inline(always)]
    #[must_use]
    pub fn fen29(&mut self) -> FEN29_W<29> {
        FEN29_W::new(self)
    }
    #[doc = "Bit 30 - Falling edge enabled 30"]
    #[inline(always)]
    #[must_use]
    pub fn fen30(&mut self) -> FEN30_W<30> {
        FEN30_W::new(self)
    }
    #[doc = "Bit 31 - Falling edge enabled 31"]
    #[inline(always)]
    #[must_use]
    pub fn fen31(&mut self) -> FEN31_W<31> {
        FEN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Falling Edge Detect Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpfen0](index.html) module"]
pub struct GPFEN0_SPEC;
impl crate::RegisterSpec for GPFEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpfen0::R](R) reader structure"]
impl crate::Readable for GPFEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpfen0::W](W) writer structure"]
impl crate::Writable for GPFEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `GPREN0` reader"]
pub struct R(crate::R<GPREN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREN0` writer"]
pub struct W(crate::W<GPREN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREN0_SPEC>;
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
impl From<crate::W<GPREN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REN0` reader - Rising edge enabled 0"]
pub type REN0_R = crate::BitReader<bool>;
#[doc = "Field `REN0` writer - Rising edge enabled 0"]
pub type REN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN1` reader - Rising edge enabled 1"]
pub type REN1_R = crate::BitReader<bool>;
#[doc = "Field `REN1` writer - Rising edge enabled 1"]
pub type REN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN2` reader - Rising edge enabled 2"]
pub type REN2_R = crate::BitReader<bool>;
#[doc = "Field `REN2` writer - Rising edge enabled 2"]
pub type REN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN3` reader - Rising edge enabled 3"]
pub type REN3_R = crate::BitReader<bool>;
#[doc = "Field `REN3` writer - Rising edge enabled 3"]
pub type REN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN4` reader - Rising edge enabled 4"]
pub type REN4_R = crate::BitReader<bool>;
#[doc = "Field `REN4` writer - Rising edge enabled 4"]
pub type REN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN5` reader - Rising edge enabled 5"]
pub type REN5_R = crate::BitReader<bool>;
#[doc = "Field `REN5` writer - Rising edge enabled 5"]
pub type REN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN6` reader - Rising edge enabled 6"]
pub type REN6_R = crate::BitReader<bool>;
#[doc = "Field `REN6` writer - Rising edge enabled 6"]
pub type REN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN7` reader - Rising edge enabled 7"]
pub type REN7_R = crate::BitReader<bool>;
#[doc = "Field `REN7` writer - Rising edge enabled 7"]
pub type REN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN8` reader - Rising edge enabled 8"]
pub type REN8_R = crate::BitReader<bool>;
#[doc = "Field `REN8` writer - Rising edge enabled 8"]
pub type REN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN9` reader - Rising edge enabled 9"]
pub type REN9_R = crate::BitReader<bool>;
#[doc = "Field `REN9` writer - Rising edge enabled 9"]
pub type REN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN10` reader - Rising edge enabled 10"]
pub type REN10_R = crate::BitReader<bool>;
#[doc = "Field `REN10` writer - Rising edge enabled 10"]
pub type REN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN11` reader - Rising edge enabled 11"]
pub type REN11_R = crate::BitReader<bool>;
#[doc = "Field `REN11` writer - Rising edge enabled 11"]
pub type REN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN12` reader - Rising edge enabled 12"]
pub type REN12_R = crate::BitReader<bool>;
#[doc = "Field `REN12` writer - Rising edge enabled 12"]
pub type REN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN13` reader - Rising edge enabled 13"]
pub type REN13_R = crate::BitReader<bool>;
#[doc = "Field `REN13` writer - Rising edge enabled 13"]
pub type REN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN14` reader - Rising edge enabled 14"]
pub type REN14_R = crate::BitReader<bool>;
#[doc = "Field `REN14` writer - Rising edge enabled 14"]
pub type REN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN15` reader - Rising edge enabled 15"]
pub type REN15_R = crate::BitReader<bool>;
#[doc = "Field `REN15` writer - Rising edge enabled 15"]
pub type REN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN16` reader - Rising edge enabled 16"]
pub type REN16_R = crate::BitReader<bool>;
#[doc = "Field `REN16` writer - Rising edge enabled 16"]
pub type REN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN17` reader - Rising edge enabled 17"]
pub type REN17_R = crate::BitReader<bool>;
#[doc = "Field `REN17` writer - Rising edge enabled 17"]
pub type REN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN18` reader - Rising edge enabled 18"]
pub type REN18_R = crate::BitReader<bool>;
#[doc = "Field `REN18` writer - Rising edge enabled 18"]
pub type REN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN19` reader - Rising edge enabled 19"]
pub type REN19_R = crate::BitReader<bool>;
#[doc = "Field `REN19` writer - Rising edge enabled 19"]
pub type REN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN20` reader - Rising edge enabled 20"]
pub type REN20_R = crate::BitReader<bool>;
#[doc = "Field `REN20` writer - Rising edge enabled 20"]
pub type REN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN21` reader - Rising edge enabled 21"]
pub type REN21_R = crate::BitReader<bool>;
#[doc = "Field `REN21` writer - Rising edge enabled 21"]
pub type REN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN22` reader - Rising edge enabled 22"]
pub type REN22_R = crate::BitReader<bool>;
#[doc = "Field `REN22` writer - Rising edge enabled 22"]
pub type REN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN23` reader - Rising edge enabled 23"]
pub type REN23_R = crate::BitReader<bool>;
#[doc = "Field `REN23` writer - Rising edge enabled 23"]
pub type REN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN24` reader - Rising edge enabled 24"]
pub type REN24_R = crate::BitReader<bool>;
#[doc = "Field `REN24` writer - Rising edge enabled 24"]
pub type REN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN25` reader - Rising edge enabled 25"]
pub type REN25_R = crate::BitReader<bool>;
#[doc = "Field `REN25` writer - Rising edge enabled 25"]
pub type REN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN26` reader - Rising edge enabled 26"]
pub type REN26_R = crate::BitReader<bool>;
#[doc = "Field `REN26` writer - Rising edge enabled 26"]
pub type REN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN27` reader - Rising edge enabled 27"]
pub type REN27_R = crate::BitReader<bool>;
#[doc = "Field `REN27` writer - Rising edge enabled 27"]
pub type REN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN28` reader - Rising edge enabled 28"]
pub type REN28_R = crate::BitReader<bool>;
#[doc = "Field `REN28` writer - Rising edge enabled 28"]
pub type REN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN29` reader - Rising edge enabled 29"]
pub type REN29_R = crate::BitReader<bool>;
#[doc = "Field `REN29` writer - Rising edge enabled 29"]
pub type REN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN30` reader - Rising edge enabled 30"]
pub type REN30_R = crate::BitReader<bool>;
#[doc = "Field `REN30` writer - Rising edge enabled 30"]
pub type REN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
#[doc = "Field `REN31` reader - Rising edge enabled 31"]
pub type REN31_R = crate::BitReader<bool>;
#[doc = "Field `REN31` writer - Rising edge enabled 31"]
pub type REN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPREN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rising edge enabled 0"]
    #[inline(always)]
    pub fn ren0(&self) -> REN0_R {
        REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge enabled 1"]
    #[inline(always)]
    pub fn ren1(&self) -> REN1_R {
        REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge enabled 2"]
    #[inline(always)]
    pub fn ren2(&self) -> REN2_R {
        REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge enabled 3"]
    #[inline(always)]
    pub fn ren3(&self) -> REN3_R {
        REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge enabled 4"]
    #[inline(always)]
    pub fn ren4(&self) -> REN4_R {
        REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising edge enabled 5"]
    #[inline(always)]
    pub fn ren5(&self) -> REN5_R {
        REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge enabled 6"]
    #[inline(always)]
    pub fn ren6(&self) -> REN6_R {
        REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge enabled 7"]
    #[inline(always)]
    pub fn ren7(&self) -> REN7_R {
        REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge enabled 8"]
    #[inline(always)]
    pub fn ren8(&self) -> REN8_R {
        REN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge enabled 9"]
    #[inline(always)]
    pub fn ren9(&self) -> REN9_R {
        REN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge enabled 10"]
    #[inline(always)]
    pub fn ren10(&self) -> REN10_R {
        REN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge enabled 11"]
    #[inline(always)]
    pub fn ren11(&self) -> REN11_R {
        REN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge enabled 12"]
    #[inline(always)]
    pub fn ren12(&self) -> REN12_R {
        REN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge enabled 13"]
    #[inline(always)]
    pub fn ren13(&self) -> REN13_R {
        REN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge enabled 14"]
    #[inline(always)]
    pub fn ren14(&self) -> REN14_R {
        REN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge enabled 15"]
    #[inline(always)]
    pub fn ren15(&self) -> REN15_R {
        REN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising edge enabled 16"]
    #[inline(always)]
    pub fn ren16(&self) -> REN16_R {
        REN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising edge enabled 17"]
    #[inline(always)]
    pub fn ren17(&self) -> REN17_R {
        REN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising edge enabled 18"]
    #[inline(always)]
    pub fn ren18(&self) -> REN18_R {
        REN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising edge enabled 19"]
    #[inline(always)]
    pub fn ren19(&self) -> REN19_R {
        REN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising edge enabled 20"]
    #[inline(always)]
    pub fn ren20(&self) -> REN20_R {
        REN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising edge enabled 21"]
    #[inline(always)]
    pub fn ren21(&self) -> REN21_R {
        REN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising edge enabled 22"]
    #[inline(always)]
    pub fn ren22(&self) -> REN22_R {
        REN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rising edge enabled 23"]
    #[inline(always)]
    pub fn ren23(&self) -> REN23_R {
        REN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rising edge enabled 24"]
    #[inline(always)]
    pub fn ren24(&self) -> REN24_R {
        REN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rising edge enabled 25"]
    #[inline(always)]
    pub fn ren25(&self) -> REN25_R {
        REN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rising edge enabled 26"]
    #[inline(always)]
    pub fn ren26(&self) -> REN26_R {
        REN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Rising edge enabled 27"]
    #[inline(always)]
    pub fn ren27(&self) -> REN27_R {
        REN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rising edge enabled 28"]
    #[inline(always)]
    pub fn ren28(&self) -> REN28_R {
        REN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rising edge enabled 29"]
    #[inline(always)]
    pub fn ren29(&self) -> REN29_R {
        REN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rising edge enabled 30"]
    #[inline(always)]
    pub fn ren30(&self) -> REN30_R {
        REN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rising edge enabled 31"]
    #[inline(always)]
    pub fn ren31(&self) -> REN31_R {
        REN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge enabled 0"]
    #[inline(always)]
    #[must_use]
    pub fn ren0(&mut self) -> REN0_W<0> {
        REN0_W::new(self)
    }
    #[doc = "Bit 1 - Rising edge enabled 1"]
    #[inline(always)]
    #[must_use]
    pub fn ren1(&mut self) -> REN1_W<1> {
        REN1_W::new(self)
    }
    #[doc = "Bit 2 - Rising edge enabled 2"]
    #[inline(always)]
    #[must_use]
    pub fn ren2(&mut self) -> REN2_W<2> {
        REN2_W::new(self)
    }
    #[doc = "Bit 3 - Rising edge enabled 3"]
    #[inline(always)]
    #[must_use]
    pub fn ren3(&mut self) -> REN3_W<3> {
        REN3_W::new(self)
    }
    #[doc = "Bit 4 - Rising edge enabled 4"]
    #[inline(always)]
    #[must_use]
    pub fn ren4(&mut self) -> REN4_W<4> {
        REN4_W::new(self)
    }
    #[doc = "Bit 5 - Rising edge enabled 5"]
    #[inline(always)]
    #[must_use]
    pub fn ren5(&mut self) -> REN5_W<5> {
        REN5_W::new(self)
    }
    #[doc = "Bit 6 - Rising edge enabled 6"]
    #[inline(always)]
    #[must_use]
    pub fn ren6(&mut self) -> REN6_W<6> {
        REN6_W::new(self)
    }
    #[doc = "Bit 7 - Rising edge enabled 7"]
    #[inline(always)]
    #[must_use]
    pub fn ren7(&mut self) -> REN7_W<7> {
        REN7_W::new(self)
    }
    #[doc = "Bit 8 - Rising edge enabled 8"]
    #[inline(always)]
    #[must_use]
    pub fn ren8(&mut self) -> REN8_W<8> {
        REN8_W::new(self)
    }
    #[doc = "Bit 9 - Rising edge enabled 9"]
    #[inline(always)]
    #[must_use]
    pub fn ren9(&mut self) -> REN9_W<9> {
        REN9_W::new(self)
    }
    #[doc = "Bit 10 - Rising edge enabled 10"]
    #[inline(always)]
    #[must_use]
    pub fn ren10(&mut self) -> REN10_W<10> {
        REN10_W::new(self)
    }
    #[doc = "Bit 11 - Rising edge enabled 11"]
    #[inline(always)]
    #[must_use]
    pub fn ren11(&mut self) -> REN11_W<11> {
        REN11_W::new(self)
    }
    #[doc = "Bit 12 - Rising edge enabled 12"]
    #[inline(always)]
    #[must_use]
    pub fn ren12(&mut self) -> REN12_W<12> {
        REN12_W::new(self)
    }
    #[doc = "Bit 13 - Rising edge enabled 13"]
    #[inline(always)]
    #[must_use]
    pub fn ren13(&mut self) -> REN13_W<13> {
        REN13_W::new(self)
    }
    #[doc = "Bit 14 - Rising edge enabled 14"]
    #[inline(always)]
    #[must_use]
    pub fn ren14(&mut self) -> REN14_W<14> {
        REN14_W::new(self)
    }
    #[doc = "Bit 15 - Rising edge enabled 15"]
    #[inline(always)]
    #[must_use]
    pub fn ren15(&mut self) -> REN15_W<15> {
        REN15_W::new(self)
    }
    #[doc = "Bit 16 - Rising edge enabled 16"]
    #[inline(always)]
    #[must_use]
    pub fn ren16(&mut self) -> REN16_W<16> {
        REN16_W::new(self)
    }
    #[doc = "Bit 17 - Rising edge enabled 17"]
    #[inline(always)]
    #[must_use]
    pub fn ren17(&mut self) -> REN17_W<17> {
        REN17_W::new(self)
    }
    #[doc = "Bit 18 - Rising edge enabled 18"]
    #[inline(always)]
    #[must_use]
    pub fn ren18(&mut self) -> REN18_W<18> {
        REN18_W::new(self)
    }
    #[doc = "Bit 19 - Rising edge enabled 19"]
    #[inline(always)]
    #[must_use]
    pub fn ren19(&mut self) -> REN19_W<19> {
        REN19_W::new(self)
    }
    #[doc = "Bit 20 - Rising edge enabled 20"]
    #[inline(always)]
    #[must_use]
    pub fn ren20(&mut self) -> REN20_W<20> {
        REN20_W::new(self)
    }
    #[doc = "Bit 21 - Rising edge enabled 21"]
    #[inline(always)]
    #[must_use]
    pub fn ren21(&mut self) -> REN21_W<21> {
        REN21_W::new(self)
    }
    #[doc = "Bit 22 - Rising edge enabled 22"]
    #[inline(always)]
    #[must_use]
    pub fn ren22(&mut self) -> REN22_W<22> {
        REN22_W::new(self)
    }
    #[doc = "Bit 23 - Rising edge enabled 23"]
    #[inline(always)]
    #[must_use]
    pub fn ren23(&mut self) -> REN23_W<23> {
        REN23_W::new(self)
    }
    #[doc = "Bit 24 - Rising edge enabled 24"]
    #[inline(always)]
    #[must_use]
    pub fn ren24(&mut self) -> REN24_W<24> {
        REN24_W::new(self)
    }
    #[doc = "Bit 25 - Rising edge enabled 25"]
    #[inline(always)]
    #[must_use]
    pub fn ren25(&mut self) -> REN25_W<25> {
        REN25_W::new(self)
    }
    #[doc = "Bit 26 - Rising edge enabled 26"]
    #[inline(always)]
    #[must_use]
    pub fn ren26(&mut self) -> REN26_W<26> {
        REN26_W::new(self)
    }
    #[doc = "Bit 27 - Rising edge enabled 27"]
    #[inline(always)]
    #[must_use]
    pub fn ren27(&mut self) -> REN27_W<27> {
        REN27_W::new(self)
    }
    #[doc = "Bit 28 - Rising edge enabled 28"]
    #[inline(always)]
    #[must_use]
    pub fn ren28(&mut self) -> REN28_W<28> {
        REN28_W::new(self)
    }
    #[doc = "Bit 29 - Rising edge enabled 29"]
    #[inline(always)]
    #[must_use]
    pub fn ren29(&mut self) -> REN29_W<29> {
        REN29_W::new(self)
    }
    #[doc = "Bit 30 - Rising edge enabled 30"]
    #[inline(always)]
    #[must_use]
    pub fn ren30(&mut self) -> REN30_W<30> {
        REN30_W::new(self)
    }
    #[doc = "Bit 31 - Rising edge enabled 31"]
    #[inline(always)]
    #[must_use]
    pub fn ren31(&mut self) -> REN31_W<31> {
        REN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Rising Edge Detect Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpren0](index.html) module"]
pub struct GPREN0_SPEC;
impl crate::RegisterSpec for GPREN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpren0::R](R) reader structure"]
impl crate::Readable for GPREN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpren0::W](W) writer structure"]
impl crate::Writable for GPREN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

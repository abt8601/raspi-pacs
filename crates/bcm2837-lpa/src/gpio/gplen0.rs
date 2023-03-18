#[doc = "Register `GPLEN0` reader"]
pub struct R(crate::R<GPLEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPLEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPLEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPLEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPLEN0` writer"]
pub struct W(crate::W<GPLEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPLEN0_SPEC>;
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
impl From<crate::W<GPLEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPLEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN0` reader - Low detect enabled 0"]
pub type LEN0_R = crate::BitReader<bool>;
#[doc = "Field `LEN0` writer - Low detect enabled 0"]
pub type LEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN1` reader - Low detect enabled 1"]
pub type LEN1_R = crate::BitReader<bool>;
#[doc = "Field `LEN1` writer - Low detect enabled 1"]
pub type LEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN2` reader - Low detect enabled 2"]
pub type LEN2_R = crate::BitReader<bool>;
#[doc = "Field `LEN2` writer - Low detect enabled 2"]
pub type LEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN3` reader - Low detect enabled 3"]
pub type LEN3_R = crate::BitReader<bool>;
#[doc = "Field `LEN3` writer - Low detect enabled 3"]
pub type LEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN4` reader - Low detect enabled 4"]
pub type LEN4_R = crate::BitReader<bool>;
#[doc = "Field `LEN4` writer - Low detect enabled 4"]
pub type LEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN5` reader - Low detect enabled 5"]
pub type LEN5_R = crate::BitReader<bool>;
#[doc = "Field `LEN5` writer - Low detect enabled 5"]
pub type LEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN6` reader - Low detect enabled 6"]
pub type LEN6_R = crate::BitReader<bool>;
#[doc = "Field `LEN6` writer - Low detect enabled 6"]
pub type LEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN7` reader - Low detect enabled 7"]
pub type LEN7_R = crate::BitReader<bool>;
#[doc = "Field `LEN7` writer - Low detect enabled 7"]
pub type LEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN8` reader - Low detect enabled 8"]
pub type LEN8_R = crate::BitReader<bool>;
#[doc = "Field `LEN8` writer - Low detect enabled 8"]
pub type LEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN9` reader - Low detect enabled 9"]
pub type LEN9_R = crate::BitReader<bool>;
#[doc = "Field `LEN9` writer - Low detect enabled 9"]
pub type LEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN10` reader - Low detect enabled 10"]
pub type LEN10_R = crate::BitReader<bool>;
#[doc = "Field `LEN10` writer - Low detect enabled 10"]
pub type LEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN11` reader - Low detect enabled 11"]
pub type LEN11_R = crate::BitReader<bool>;
#[doc = "Field `LEN11` writer - Low detect enabled 11"]
pub type LEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN12` reader - Low detect enabled 12"]
pub type LEN12_R = crate::BitReader<bool>;
#[doc = "Field `LEN12` writer - Low detect enabled 12"]
pub type LEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN13` reader - Low detect enabled 13"]
pub type LEN13_R = crate::BitReader<bool>;
#[doc = "Field `LEN13` writer - Low detect enabled 13"]
pub type LEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN14` reader - Low detect enabled 14"]
pub type LEN14_R = crate::BitReader<bool>;
#[doc = "Field `LEN14` writer - Low detect enabled 14"]
pub type LEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN15` reader - Low detect enabled 15"]
pub type LEN15_R = crate::BitReader<bool>;
#[doc = "Field `LEN15` writer - Low detect enabled 15"]
pub type LEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN16` reader - Low detect enabled 16"]
pub type LEN16_R = crate::BitReader<bool>;
#[doc = "Field `LEN16` writer - Low detect enabled 16"]
pub type LEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN17` reader - Low detect enabled 17"]
pub type LEN17_R = crate::BitReader<bool>;
#[doc = "Field `LEN17` writer - Low detect enabled 17"]
pub type LEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN18` reader - Low detect enabled 18"]
pub type LEN18_R = crate::BitReader<bool>;
#[doc = "Field `LEN18` writer - Low detect enabled 18"]
pub type LEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN19` reader - Low detect enabled 19"]
pub type LEN19_R = crate::BitReader<bool>;
#[doc = "Field `LEN19` writer - Low detect enabled 19"]
pub type LEN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN20` reader - Low detect enabled 20"]
pub type LEN20_R = crate::BitReader<bool>;
#[doc = "Field `LEN20` writer - Low detect enabled 20"]
pub type LEN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN21` reader - Low detect enabled 21"]
pub type LEN21_R = crate::BitReader<bool>;
#[doc = "Field `LEN21` writer - Low detect enabled 21"]
pub type LEN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN22` reader - Low detect enabled 22"]
pub type LEN22_R = crate::BitReader<bool>;
#[doc = "Field `LEN22` writer - Low detect enabled 22"]
pub type LEN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN23` reader - Low detect enabled 23"]
pub type LEN23_R = crate::BitReader<bool>;
#[doc = "Field `LEN23` writer - Low detect enabled 23"]
pub type LEN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN24` reader - Low detect enabled 24"]
pub type LEN24_R = crate::BitReader<bool>;
#[doc = "Field `LEN24` writer - Low detect enabled 24"]
pub type LEN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN25` reader - Low detect enabled 25"]
pub type LEN25_R = crate::BitReader<bool>;
#[doc = "Field `LEN25` writer - Low detect enabled 25"]
pub type LEN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN26` reader - Low detect enabled 26"]
pub type LEN26_R = crate::BitReader<bool>;
#[doc = "Field `LEN26` writer - Low detect enabled 26"]
pub type LEN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN27` reader - Low detect enabled 27"]
pub type LEN27_R = crate::BitReader<bool>;
#[doc = "Field `LEN27` writer - Low detect enabled 27"]
pub type LEN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN28` reader - Low detect enabled 28"]
pub type LEN28_R = crate::BitReader<bool>;
#[doc = "Field `LEN28` writer - Low detect enabled 28"]
pub type LEN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN29` reader - Low detect enabled 29"]
pub type LEN29_R = crate::BitReader<bool>;
#[doc = "Field `LEN29` writer - Low detect enabled 29"]
pub type LEN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN30` reader - Low detect enabled 30"]
pub type LEN30_R = crate::BitReader<bool>;
#[doc = "Field `LEN30` writer - Low detect enabled 30"]
pub type LEN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
#[doc = "Field `LEN31` reader - Low detect enabled 31"]
pub type LEN31_R = crate::BitReader<bool>;
#[doc = "Field `LEN31` writer - Low detect enabled 31"]
pub type LEN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low detect enabled 0"]
    #[inline(always)]
    pub fn len0(&self) -> LEN0_R {
        LEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low detect enabled 1"]
    #[inline(always)]
    pub fn len1(&self) -> LEN1_R {
        LEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low detect enabled 2"]
    #[inline(always)]
    pub fn len2(&self) -> LEN2_R {
        LEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low detect enabled 3"]
    #[inline(always)]
    pub fn len3(&self) -> LEN3_R {
        LEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low detect enabled 4"]
    #[inline(always)]
    pub fn len4(&self) -> LEN4_R {
        LEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low detect enabled 5"]
    #[inline(always)]
    pub fn len5(&self) -> LEN5_R {
        LEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low detect enabled 6"]
    #[inline(always)]
    pub fn len6(&self) -> LEN6_R {
        LEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low detect enabled 7"]
    #[inline(always)]
    pub fn len7(&self) -> LEN7_R {
        LEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low detect enabled 8"]
    #[inline(always)]
    pub fn len8(&self) -> LEN8_R {
        LEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low detect enabled 9"]
    #[inline(always)]
    pub fn len9(&self) -> LEN9_R {
        LEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low detect enabled 10"]
    #[inline(always)]
    pub fn len10(&self) -> LEN10_R {
        LEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low detect enabled 11"]
    #[inline(always)]
    pub fn len11(&self) -> LEN11_R {
        LEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low detect enabled 12"]
    #[inline(always)]
    pub fn len12(&self) -> LEN12_R {
        LEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low detect enabled 13"]
    #[inline(always)]
    pub fn len13(&self) -> LEN13_R {
        LEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low detect enabled 14"]
    #[inline(always)]
    pub fn len14(&self) -> LEN14_R {
        LEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low detect enabled 15"]
    #[inline(always)]
    pub fn len15(&self) -> LEN15_R {
        LEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Low detect enabled 16"]
    #[inline(always)]
    pub fn len16(&self) -> LEN16_R {
        LEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low detect enabled 17"]
    #[inline(always)]
    pub fn len17(&self) -> LEN17_R {
        LEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Low detect enabled 18"]
    #[inline(always)]
    pub fn len18(&self) -> LEN18_R {
        LEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low detect enabled 19"]
    #[inline(always)]
    pub fn len19(&self) -> LEN19_R {
        LEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Low detect enabled 20"]
    #[inline(always)]
    pub fn len20(&self) -> LEN20_R {
        LEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Low detect enabled 21"]
    #[inline(always)]
    pub fn len21(&self) -> LEN21_R {
        LEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Low detect enabled 22"]
    #[inline(always)]
    pub fn len22(&self) -> LEN22_R {
        LEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Low detect enabled 23"]
    #[inline(always)]
    pub fn len23(&self) -> LEN23_R {
        LEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Low detect enabled 24"]
    #[inline(always)]
    pub fn len24(&self) -> LEN24_R {
        LEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low detect enabled 25"]
    #[inline(always)]
    pub fn len25(&self) -> LEN25_R {
        LEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Low detect enabled 26"]
    #[inline(always)]
    pub fn len26(&self) -> LEN26_R {
        LEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Low detect enabled 27"]
    #[inline(always)]
    pub fn len27(&self) -> LEN27_R {
        LEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Low detect enabled 28"]
    #[inline(always)]
    pub fn len28(&self) -> LEN28_R {
        LEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Low detect enabled 29"]
    #[inline(always)]
    pub fn len29(&self) -> LEN29_R {
        LEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low detect enabled 30"]
    #[inline(always)]
    pub fn len30(&self) -> LEN30_R {
        LEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low detect enabled 31"]
    #[inline(always)]
    pub fn len31(&self) -> LEN31_R {
        LEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low detect enabled 0"]
    #[inline(always)]
    #[must_use]
    pub fn len0(&mut self) -> LEN0_W<0> {
        LEN0_W::new(self)
    }
    #[doc = "Bit 1 - Low detect enabled 1"]
    #[inline(always)]
    #[must_use]
    pub fn len1(&mut self) -> LEN1_W<1> {
        LEN1_W::new(self)
    }
    #[doc = "Bit 2 - Low detect enabled 2"]
    #[inline(always)]
    #[must_use]
    pub fn len2(&mut self) -> LEN2_W<2> {
        LEN2_W::new(self)
    }
    #[doc = "Bit 3 - Low detect enabled 3"]
    #[inline(always)]
    #[must_use]
    pub fn len3(&mut self) -> LEN3_W<3> {
        LEN3_W::new(self)
    }
    #[doc = "Bit 4 - Low detect enabled 4"]
    #[inline(always)]
    #[must_use]
    pub fn len4(&mut self) -> LEN4_W<4> {
        LEN4_W::new(self)
    }
    #[doc = "Bit 5 - Low detect enabled 5"]
    #[inline(always)]
    #[must_use]
    pub fn len5(&mut self) -> LEN5_W<5> {
        LEN5_W::new(self)
    }
    #[doc = "Bit 6 - Low detect enabled 6"]
    #[inline(always)]
    #[must_use]
    pub fn len6(&mut self) -> LEN6_W<6> {
        LEN6_W::new(self)
    }
    #[doc = "Bit 7 - Low detect enabled 7"]
    #[inline(always)]
    #[must_use]
    pub fn len7(&mut self) -> LEN7_W<7> {
        LEN7_W::new(self)
    }
    #[doc = "Bit 8 - Low detect enabled 8"]
    #[inline(always)]
    #[must_use]
    pub fn len8(&mut self) -> LEN8_W<8> {
        LEN8_W::new(self)
    }
    #[doc = "Bit 9 - Low detect enabled 9"]
    #[inline(always)]
    #[must_use]
    pub fn len9(&mut self) -> LEN9_W<9> {
        LEN9_W::new(self)
    }
    #[doc = "Bit 10 - Low detect enabled 10"]
    #[inline(always)]
    #[must_use]
    pub fn len10(&mut self) -> LEN10_W<10> {
        LEN10_W::new(self)
    }
    #[doc = "Bit 11 - Low detect enabled 11"]
    #[inline(always)]
    #[must_use]
    pub fn len11(&mut self) -> LEN11_W<11> {
        LEN11_W::new(self)
    }
    #[doc = "Bit 12 - Low detect enabled 12"]
    #[inline(always)]
    #[must_use]
    pub fn len12(&mut self) -> LEN12_W<12> {
        LEN12_W::new(self)
    }
    #[doc = "Bit 13 - Low detect enabled 13"]
    #[inline(always)]
    #[must_use]
    pub fn len13(&mut self) -> LEN13_W<13> {
        LEN13_W::new(self)
    }
    #[doc = "Bit 14 - Low detect enabled 14"]
    #[inline(always)]
    #[must_use]
    pub fn len14(&mut self) -> LEN14_W<14> {
        LEN14_W::new(self)
    }
    #[doc = "Bit 15 - Low detect enabled 15"]
    #[inline(always)]
    #[must_use]
    pub fn len15(&mut self) -> LEN15_W<15> {
        LEN15_W::new(self)
    }
    #[doc = "Bit 16 - Low detect enabled 16"]
    #[inline(always)]
    #[must_use]
    pub fn len16(&mut self) -> LEN16_W<16> {
        LEN16_W::new(self)
    }
    #[doc = "Bit 17 - Low detect enabled 17"]
    #[inline(always)]
    #[must_use]
    pub fn len17(&mut self) -> LEN17_W<17> {
        LEN17_W::new(self)
    }
    #[doc = "Bit 18 - Low detect enabled 18"]
    #[inline(always)]
    #[must_use]
    pub fn len18(&mut self) -> LEN18_W<18> {
        LEN18_W::new(self)
    }
    #[doc = "Bit 19 - Low detect enabled 19"]
    #[inline(always)]
    #[must_use]
    pub fn len19(&mut self) -> LEN19_W<19> {
        LEN19_W::new(self)
    }
    #[doc = "Bit 20 - Low detect enabled 20"]
    #[inline(always)]
    #[must_use]
    pub fn len20(&mut self) -> LEN20_W<20> {
        LEN20_W::new(self)
    }
    #[doc = "Bit 21 - Low detect enabled 21"]
    #[inline(always)]
    #[must_use]
    pub fn len21(&mut self) -> LEN21_W<21> {
        LEN21_W::new(self)
    }
    #[doc = "Bit 22 - Low detect enabled 22"]
    #[inline(always)]
    #[must_use]
    pub fn len22(&mut self) -> LEN22_W<22> {
        LEN22_W::new(self)
    }
    #[doc = "Bit 23 - Low detect enabled 23"]
    #[inline(always)]
    #[must_use]
    pub fn len23(&mut self) -> LEN23_W<23> {
        LEN23_W::new(self)
    }
    #[doc = "Bit 24 - Low detect enabled 24"]
    #[inline(always)]
    #[must_use]
    pub fn len24(&mut self) -> LEN24_W<24> {
        LEN24_W::new(self)
    }
    #[doc = "Bit 25 - Low detect enabled 25"]
    #[inline(always)]
    #[must_use]
    pub fn len25(&mut self) -> LEN25_W<25> {
        LEN25_W::new(self)
    }
    #[doc = "Bit 26 - Low detect enabled 26"]
    #[inline(always)]
    #[must_use]
    pub fn len26(&mut self) -> LEN26_W<26> {
        LEN26_W::new(self)
    }
    #[doc = "Bit 27 - Low detect enabled 27"]
    #[inline(always)]
    #[must_use]
    pub fn len27(&mut self) -> LEN27_W<27> {
        LEN27_W::new(self)
    }
    #[doc = "Bit 28 - Low detect enabled 28"]
    #[inline(always)]
    #[must_use]
    pub fn len28(&mut self) -> LEN28_W<28> {
        LEN28_W::new(self)
    }
    #[doc = "Bit 29 - Low detect enabled 29"]
    #[inline(always)]
    #[must_use]
    pub fn len29(&mut self) -> LEN29_W<29> {
        LEN29_W::new(self)
    }
    #[doc = "Bit 30 - Low detect enabled 30"]
    #[inline(always)]
    #[must_use]
    pub fn len30(&mut self) -> LEN30_W<30> {
        LEN30_W::new(self)
    }
    #[doc = "Bit 31 - Low detect enabled 31"]
    #[inline(always)]
    #[must_use]
    pub fn len31(&mut self) -> LEN31_W<31> {
        LEN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Low Detect Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gplen0](index.html) module"]
pub struct GPLEN0_SPEC;
impl crate::RegisterSpec for GPLEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gplen0::R](R) reader structure"]
impl crate::Readable for GPLEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gplen0::W](W) writer structure"]
impl crate::Writable for GPLEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

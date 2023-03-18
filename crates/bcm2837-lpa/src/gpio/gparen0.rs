#[doc = "Register `GPAREN0` reader"]
pub struct R(crate::R<GPAREN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPAREN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPAREN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPAREN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPAREN0` writer"]
pub struct W(crate::W<GPAREN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPAREN0_SPEC>;
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
impl From<crate::W<GPAREN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPAREN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AREN0` reader - Async rising enabled 0"]
pub type AREN0_R = crate::BitReader<bool>;
#[doc = "Field `AREN0` writer - Async rising enabled 0"]
pub type AREN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN1` reader - Async rising enabled 1"]
pub type AREN1_R = crate::BitReader<bool>;
#[doc = "Field `AREN1` writer - Async rising enabled 1"]
pub type AREN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN2` reader - Async rising enabled 2"]
pub type AREN2_R = crate::BitReader<bool>;
#[doc = "Field `AREN2` writer - Async rising enabled 2"]
pub type AREN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN3` reader - Async rising enabled 3"]
pub type AREN3_R = crate::BitReader<bool>;
#[doc = "Field `AREN3` writer - Async rising enabled 3"]
pub type AREN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN4` reader - Async rising enabled 4"]
pub type AREN4_R = crate::BitReader<bool>;
#[doc = "Field `AREN4` writer - Async rising enabled 4"]
pub type AREN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN5` reader - Async rising enabled 5"]
pub type AREN5_R = crate::BitReader<bool>;
#[doc = "Field `AREN5` writer - Async rising enabled 5"]
pub type AREN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN6` reader - Async rising enabled 6"]
pub type AREN6_R = crate::BitReader<bool>;
#[doc = "Field `AREN6` writer - Async rising enabled 6"]
pub type AREN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN7` reader - Async rising enabled 7"]
pub type AREN7_R = crate::BitReader<bool>;
#[doc = "Field `AREN7` writer - Async rising enabled 7"]
pub type AREN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN8` reader - Async rising enabled 8"]
pub type AREN8_R = crate::BitReader<bool>;
#[doc = "Field `AREN8` writer - Async rising enabled 8"]
pub type AREN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN9` reader - Async rising enabled 9"]
pub type AREN9_R = crate::BitReader<bool>;
#[doc = "Field `AREN9` writer - Async rising enabled 9"]
pub type AREN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN10` reader - Async rising enabled 10"]
pub type AREN10_R = crate::BitReader<bool>;
#[doc = "Field `AREN10` writer - Async rising enabled 10"]
pub type AREN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN11` reader - Async rising enabled 11"]
pub type AREN11_R = crate::BitReader<bool>;
#[doc = "Field `AREN11` writer - Async rising enabled 11"]
pub type AREN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN12` reader - Async rising enabled 12"]
pub type AREN12_R = crate::BitReader<bool>;
#[doc = "Field `AREN12` writer - Async rising enabled 12"]
pub type AREN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN13` reader - Async rising enabled 13"]
pub type AREN13_R = crate::BitReader<bool>;
#[doc = "Field `AREN13` writer - Async rising enabled 13"]
pub type AREN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN14` reader - Async rising enabled 14"]
pub type AREN14_R = crate::BitReader<bool>;
#[doc = "Field `AREN14` writer - Async rising enabled 14"]
pub type AREN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN15` reader - Async rising enabled 15"]
pub type AREN15_R = crate::BitReader<bool>;
#[doc = "Field `AREN15` writer - Async rising enabled 15"]
pub type AREN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN16` reader - Async rising enabled 16"]
pub type AREN16_R = crate::BitReader<bool>;
#[doc = "Field `AREN16` writer - Async rising enabled 16"]
pub type AREN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN17` reader - Async rising enabled 17"]
pub type AREN17_R = crate::BitReader<bool>;
#[doc = "Field `AREN17` writer - Async rising enabled 17"]
pub type AREN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN18` reader - Async rising enabled 18"]
pub type AREN18_R = crate::BitReader<bool>;
#[doc = "Field `AREN18` writer - Async rising enabled 18"]
pub type AREN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN19` reader - Async rising enabled 19"]
pub type AREN19_R = crate::BitReader<bool>;
#[doc = "Field `AREN19` writer - Async rising enabled 19"]
pub type AREN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN20` reader - Async rising enabled 20"]
pub type AREN20_R = crate::BitReader<bool>;
#[doc = "Field `AREN20` writer - Async rising enabled 20"]
pub type AREN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN21` reader - Async rising enabled 21"]
pub type AREN21_R = crate::BitReader<bool>;
#[doc = "Field `AREN21` writer - Async rising enabled 21"]
pub type AREN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN22` reader - Async rising enabled 22"]
pub type AREN22_R = crate::BitReader<bool>;
#[doc = "Field `AREN22` writer - Async rising enabled 22"]
pub type AREN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN23` reader - Async rising enabled 23"]
pub type AREN23_R = crate::BitReader<bool>;
#[doc = "Field `AREN23` writer - Async rising enabled 23"]
pub type AREN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN24` reader - Async rising enabled 24"]
pub type AREN24_R = crate::BitReader<bool>;
#[doc = "Field `AREN24` writer - Async rising enabled 24"]
pub type AREN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN25` reader - Async rising enabled 25"]
pub type AREN25_R = crate::BitReader<bool>;
#[doc = "Field `AREN25` writer - Async rising enabled 25"]
pub type AREN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN26` reader - Async rising enabled 26"]
pub type AREN26_R = crate::BitReader<bool>;
#[doc = "Field `AREN26` writer - Async rising enabled 26"]
pub type AREN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN27` reader - Async rising enabled 27"]
pub type AREN27_R = crate::BitReader<bool>;
#[doc = "Field `AREN27` writer - Async rising enabled 27"]
pub type AREN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN28` reader - Async rising enabled 28"]
pub type AREN28_R = crate::BitReader<bool>;
#[doc = "Field `AREN28` writer - Async rising enabled 28"]
pub type AREN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN29` reader - Async rising enabled 29"]
pub type AREN29_R = crate::BitReader<bool>;
#[doc = "Field `AREN29` writer - Async rising enabled 29"]
pub type AREN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN30` reader - Async rising enabled 30"]
pub type AREN30_R = crate::BitReader<bool>;
#[doc = "Field `AREN30` writer - Async rising enabled 30"]
pub type AREN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
#[doc = "Field `AREN31` reader - Async rising enabled 31"]
pub type AREN31_R = crate::BitReader<bool>;
#[doc = "Field `AREN31` writer - Async rising enabled 31"]
pub type AREN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Async rising enabled 0"]
    #[inline(always)]
    pub fn aren0(&self) -> AREN0_R {
        AREN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Async rising enabled 1"]
    #[inline(always)]
    pub fn aren1(&self) -> AREN1_R {
        AREN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Async rising enabled 2"]
    #[inline(always)]
    pub fn aren2(&self) -> AREN2_R {
        AREN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Async rising enabled 3"]
    #[inline(always)]
    pub fn aren3(&self) -> AREN3_R {
        AREN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async rising enabled 4"]
    #[inline(always)]
    pub fn aren4(&self) -> AREN4_R {
        AREN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async rising enabled 5"]
    #[inline(always)]
    pub fn aren5(&self) -> AREN5_R {
        AREN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async rising enabled 6"]
    #[inline(always)]
    pub fn aren6(&self) -> AREN6_R {
        AREN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async rising enabled 7"]
    #[inline(always)]
    pub fn aren7(&self) -> AREN7_R {
        AREN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Async rising enabled 8"]
    #[inline(always)]
    pub fn aren8(&self) -> AREN8_R {
        AREN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Async rising enabled 9"]
    #[inline(always)]
    pub fn aren9(&self) -> AREN9_R {
        AREN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Async rising enabled 10"]
    #[inline(always)]
    pub fn aren10(&self) -> AREN10_R {
        AREN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Async rising enabled 11"]
    #[inline(always)]
    pub fn aren11(&self) -> AREN11_R {
        AREN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Async rising enabled 12"]
    #[inline(always)]
    pub fn aren12(&self) -> AREN12_R {
        AREN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Async rising enabled 13"]
    #[inline(always)]
    pub fn aren13(&self) -> AREN13_R {
        AREN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Async rising enabled 14"]
    #[inline(always)]
    pub fn aren14(&self) -> AREN14_R {
        AREN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Async rising enabled 15"]
    #[inline(always)]
    pub fn aren15(&self) -> AREN15_R {
        AREN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Async rising enabled 16"]
    #[inline(always)]
    pub fn aren16(&self) -> AREN16_R {
        AREN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Async rising enabled 17"]
    #[inline(always)]
    pub fn aren17(&self) -> AREN17_R {
        AREN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Async rising enabled 18"]
    #[inline(always)]
    pub fn aren18(&self) -> AREN18_R {
        AREN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Async rising enabled 19"]
    #[inline(always)]
    pub fn aren19(&self) -> AREN19_R {
        AREN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Async rising enabled 20"]
    #[inline(always)]
    pub fn aren20(&self) -> AREN20_R {
        AREN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Async rising enabled 21"]
    #[inline(always)]
    pub fn aren21(&self) -> AREN21_R {
        AREN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Async rising enabled 22"]
    #[inline(always)]
    pub fn aren22(&self) -> AREN22_R {
        AREN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Async rising enabled 23"]
    #[inline(always)]
    pub fn aren23(&self) -> AREN23_R {
        AREN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Async rising enabled 24"]
    #[inline(always)]
    pub fn aren24(&self) -> AREN24_R {
        AREN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Async rising enabled 25"]
    #[inline(always)]
    pub fn aren25(&self) -> AREN25_R {
        AREN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Async rising enabled 26"]
    #[inline(always)]
    pub fn aren26(&self) -> AREN26_R {
        AREN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Async rising enabled 27"]
    #[inline(always)]
    pub fn aren27(&self) -> AREN27_R {
        AREN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Async rising enabled 28"]
    #[inline(always)]
    pub fn aren28(&self) -> AREN28_R {
        AREN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Async rising enabled 29"]
    #[inline(always)]
    pub fn aren29(&self) -> AREN29_R {
        AREN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Async rising enabled 30"]
    #[inline(always)]
    pub fn aren30(&self) -> AREN30_R {
        AREN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Async rising enabled 31"]
    #[inline(always)]
    pub fn aren31(&self) -> AREN31_R {
        AREN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Async rising enabled 0"]
    #[inline(always)]
    #[must_use]
    pub fn aren0(&mut self) -> AREN0_W<0> {
        AREN0_W::new(self)
    }
    #[doc = "Bit 1 - Async rising enabled 1"]
    #[inline(always)]
    #[must_use]
    pub fn aren1(&mut self) -> AREN1_W<1> {
        AREN1_W::new(self)
    }
    #[doc = "Bit 2 - Async rising enabled 2"]
    #[inline(always)]
    #[must_use]
    pub fn aren2(&mut self) -> AREN2_W<2> {
        AREN2_W::new(self)
    }
    #[doc = "Bit 3 - Async rising enabled 3"]
    #[inline(always)]
    #[must_use]
    pub fn aren3(&mut self) -> AREN3_W<3> {
        AREN3_W::new(self)
    }
    #[doc = "Bit 4 - Async rising enabled 4"]
    #[inline(always)]
    #[must_use]
    pub fn aren4(&mut self) -> AREN4_W<4> {
        AREN4_W::new(self)
    }
    #[doc = "Bit 5 - Async rising enabled 5"]
    #[inline(always)]
    #[must_use]
    pub fn aren5(&mut self) -> AREN5_W<5> {
        AREN5_W::new(self)
    }
    #[doc = "Bit 6 - Async rising enabled 6"]
    #[inline(always)]
    #[must_use]
    pub fn aren6(&mut self) -> AREN6_W<6> {
        AREN6_W::new(self)
    }
    #[doc = "Bit 7 - Async rising enabled 7"]
    #[inline(always)]
    #[must_use]
    pub fn aren7(&mut self) -> AREN7_W<7> {
        AREN7_W::new(self)
    }
    #[doc = "Bit 8 - Async rising enabled 8"]
    #[inline(always)]
    #[must_use]
    pub fn aren8(&mut self) -> AREN8_W<8> {
        AREN8_W::new(self)
    }
    #[doc = "Bit 9 - Async rising enabled 9"]
    #[inline(always)]
    #[must_use]
    pub fn aren9(&mut self) -> AREN9_W<9> {
        AREN9_W::new(self)
    }
    #[doc = "Bit 10 - Async rising enabled 10"]
    #[inline(always)]
    #[must_use]
    pub fn aren10(&mut self) -> AREN10_W<10> {
        AREN10_W::new(self)
    }
    #[doc = "Bit 11 - Async rising enabled 11"]
    #[inline(always)]
    #[must_use]
    pub fn aren11(&mut self) -> AREN11_W<11> {
        AREN11_W::new(self)
    }
    #[doc = "Bit 12 - Async rising enabled 12"]
    #[inline(always)]
    #[must_use]
    pub fn aren12(&mut self) -> AREN12_W<12> {
        AREN12_W::new(self)
    }
    #[doc = "Bit 13 - Async rising enabled 13"]
    #[inline(always)]
    #[must_use]
    pub fn aren13(&mut self) -> AREN13_W<13> {
        AREN13_W::new(self)
    }
    #[doc = "Bit 14 - Async rising enabled 14"]
    #[inline(always)]
    #[must_use]
    pub fn aren14(&mut self) -> AREN14_W<14> {
        AREN14_W::new(self)
    }
    #[doc = "Bit 15 - Async rising enabled 15"]
    #[inline(always)]
    #[must_use]
    pub fn aren15(&mut self) -> AREN15_W<15> {
        AREN15_W::new(self)
    }
    #[doc = "Bit 16 - Async rising enabled 16"]
    #[inline(always)]
    #[must_use]
    pub fn aren16(&mut self) -> AREN16_W<16> {
        AREN16_W::new(self)
    }
    #[doc = "Bit 17 - Async rising enabled 17"]
    #[inline(always)]
    #[must_use]
    pub fn aren17(&mut self) -> AREN17_W<17> {
        AREN17_W::new(self)
    }
    #[doc = "Bit 18 - Async rising enabled 18"]
    #[inline(always)]
    #[must_use]
    pub fn aren18(&mut self) -> AREN18_W<18> {
        AREN18_W::new(self)
    }
    #[doc = "Bit 19 - Async rising enabled 19"]
    #[inline(always)]
    #[must_use]
    pub fn aren19(&mut self) -> AREN19_W<19> {
        AREN19_W::new(self)
    }
    #[doc = "Bit 20 - Async rising enabled 20"]
    #[inline(always)]
    #[must_use]
    pub fn aren20(&mut self) -> AREN20_W<20> {
        AREN20_W::new(self)
    }
    #[doc = "Bit 21 - Async rising enabled 21"]
    #[inline(always)]
    #[must_use]
    pub fn aren21(&mut self) -> AREN21_W<21> {
        AREN21_W::new(self)
    }
    #[doc = "Bit 22 - Async rising enabled 22"]
    #[inline(always)]
    #[must_use]
    pub fn aren22(&mut self) -> AREN22_W<22> {
        AREN22_W::new(self)
    }
    #[doc = "Bit 23 - Async rising enabled 23"]
    #[inline(always)]
    #[must_use]
    pub fn aren23(&mut self) -> AREN23_W<23> {
        AREN23_W::new(self)
    }
    #[doc = "Bit 24 - Async rising enabled 24"]
    #[inline(always)]
    #[must_use]
    pub fn aren24(&mut self) -> AREN24_W<24> {
        AREN24_W::new(self)
    }
    #[doc = "Bit 25 - Async rising enabled 25"]
    #[inline(always)]
    #[must_use]
    pub fn aren25(&mut self) -> AREN25_W<25> {
        AREN25_W::new(self)
    }
    #[doc = "Bit 26 - Async rising enabled 26"]
    #[inline(always)]
    #[must_use]
    pub fn aren26(&mut self) -> AREN26_W<26> {
        AREN26_W::new(self)
    }
    #[doc = "Bit 27 - Async rising enabled 27"]
    #[inline(always)]
    #[must_use]
    pub fn aren27(&mut self) -> AREN27_W<27> {
        AREN27_W::new(self)
    }
    #[doc = "Bit 28 - Async rising enabled 28"]
    #[inline(always)]
    #[must_use]
    pub fn aren28(&mut self) -> AREN28_W<28> {
        AREN28_W::new(self)
    }
    #[doc = "Bit 29 - Async rising enabled 29"]
    #[inline(always)]
    #[must_use]
    pub fn aren29(&mut self) -> AREN29_W<29> {
        AREN29_W::new(self)
    }
    #[doc = "Bit 30 - Async rising enabled 30"]
    #[inline(always)]
    #[must_use]
    pub fn aren30(&mut self) -> AREN30_W<30> {
        AREN30_W::new(self)
    }
    #[doc = "Bit 31 - Async rising enabled 31"]
    #[inline(always)]
    #[must_use]
    pub fn aren31(&mut self) -> AREN31_W<31> {
        AREN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Async. Rising Edge Detect 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gparen0](index.html) module"]
pub struct GPAREN0_SPEC;
impl crate::RegisterSpec for GPAREN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gparen0::R](R) reader structure"]
impl crate::Readable for GPAREN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gparen0::W](W) writer structure"]
impl crate::Writable for GPAREN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

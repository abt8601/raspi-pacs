#[doc = "Register `GPLEN1` reader"]
pub struct R(crate::R<GPLEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPLEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPLEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPLEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPLEN1` writer"]
pub struct W(crate::W<GPLEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPLEN1_SPEC>;
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
impl From<crate::W<GPLEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPLEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN32` reader - Low detect enabled 32"]
pub type LEN32_R = crate::BitReader<bool>;
#[doc = "Field `LEN32` writer - Low detect enabled 32"]
pub type LEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN33` reader - Low detect enabled 33"]
pub type LEN33_R = crate::BitReader<bool>;
#[doc = "Field `LEN33` writer - Low detect enabled 33"]
pub type LEN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN34` reader - Low detect enabled 34"]
pub type LEN34_R = crate::BitReader<bool>;
#[doc = "Field `LEN34` writer - Low detect enabled 34"]
pub type LEN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN35` reader - Low detect enabled 35"]
pub type LEN35_R = crate::BitReader<bool>;
#[doc = "Field `LEN35` writer - Low detect enabled 35"]
pub type LEN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN36` reader - Low detect enabled 36"]
pub type LEN36_R = crate::BitReader<bool>;
#[doc = "Field `LEN36` writer - Low detect enabled 36"]
pub type LEN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN37` reader - Low detect enabled 37"]
pub type LEN37_R = crate::BitReader<bool>;
#[doc = "Field `LEN37` writer - Low detect enabled 37"]
pub type LEN37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN38` reader - Low detect enabled 38"]
pub type LEN38_R = crate::BitReader<bool>;
#[doc = "Field `LEN38` writer - Low detect enabled 38"]
pub type LEN38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN39` reader - Low detect enabled 39"]
pub type LEN39_R = crate::BitReader<bool>;
#[doc = "Field `LEN39` writer - Low detect enabled 39"]
pub type LEN39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN40` reader - Low detect enabled 40"]
pub type LEN40_R = crate::BitReader<bool>;
#[doc = "Field `LEN40` writer - Low detect enabled 40"]
pub type LEN40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN41` reader - Low detect enabled 41"]
pub type LEN41_R = crate::BitReader<bool>;
#[doc = "Field `LEN41` writer - Low detect enabled 41"]
pub type LEN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN42` reader - Low detect enabled 42"]
pub type LEN42_R = crate::BitReader<bool>;
#[doc = "Field `LEN42` writer - Low detect enabled 42"]
pub type LEN42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN43` reader - Low detect enabled 43"]
pub type LEN43_R = crate::BitReader<bool>;
#[doc = "Field `LEN43` writer - Low detect enabled 43"]
pub type LEN43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN44` reader - Low detect enabled 44"]
pub type LEN44_R = crate::BitReader<bool>;
#[doc = "Field `LEN44` writer - Low detect enabled 44"]
pub type LEN44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN45` reader - Low detect enabled 45"]
pub type LEN45_R = crate::BitReader<bool>;
#[doc = "Field `LEN45` writer - Low detect enabled 45"]
pub type LEN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN46` reader - Low detect enabled 46"]
pub type LEN46_R = crate::BitReader<bool>;
#[doc = "Field `LEN46` writer - Low detect enabled 46"]
pub type LEN46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN47` reader - Low detect enabled 47"]
pub type LEN47_R = crate::BitReader<bool>;
#[doc = "Field `LEN47` writer - Low detect enabled 47"]
pub type LEN47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN48` reader - Low detect enabled 48"]
pub type LEN48_R = crate::BitReader<bool>;
#[doc = "Field `LEN48` writer - Low detect enabled 48"]
pub type LEN48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN49` reader - Low detect enabled 49"]
pub type LEN49_R = crate::BitReader<bool>;
#[doc = "Field `LEN49` writer - Low detect enabled 49"]
pub type LEN49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN50` reader - Low detect enabled 50"]
pub type LEN50_R = crate::BitReader<bool>;
#[doc = "Field `LEN50` writer - Low detect enabled 50"]
pub type LEN50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN51` reader - Low detect enabled 51"]
pub type LEN51_R = crate::BitReader<bool>;
#[doc = "Field `LEN51` writer - Low detect enabled 51"]
pub type LEN51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN52` reader - Low detect enabled 52"]
pub type LEN52_R = crate::BitReader<bool>;
#[doc = "Field `LEN52` writer - Low detect enabled 52"]
pub type LEN52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
#[doc = "Field `LEN53` reader - Low detect enabled 53"]
pub type LEN53_R = crate::BitReader<bool>;
#[doc = "Field `LEN53` writer - Low detect enabled 53"]
pub type LEN53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPLEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Low detect enabled 32"]
    #[inline(always)]
    pub fn len32(&self) -> LEN32_R {
        LEN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low detect enabled 33"]
    #[inline(always)]
    pub fn len33(&self) -> LEN33_R {
        LEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low detect enabled 34"]
    #[inline(always)]
    pub fn len34(&self) -> LEN34_R {
        LEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low detect enabled 35"]
    #[inline(always)]
    pub fn len35(&self) -> LEN35_R {
        LEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low detect enabled 36"]
    #[inline(always)]
    pub fn len36(&self) -> LEN36_R {
        LEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low detect enabled 37"]
    #[inline(always)]
    pub fn len37(&self) -> LEN37_R {
        LEN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low detect enabled 38"]
    #[inline(always)]
    pub fn len38(&self) -> LEN38_R {
        LEN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low detect enabled 39"]
    #[inline(always)]
    pub fn len39(&self) -> LEN39_R {
        LEN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low detect enabled 40"]
    #[inline(always)]
    pub fn len40(&self) -> LEN40_R {
        LEN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low detect enabled 41"]
    #[inline(always)]
    pub fn len41(&self) -> LEN41_R {
        LEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low detect enabled 42"]
    #[inline(always)]
    pub fn len42(&self) -> LEN42_R {
        LEN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low detect enabled 43"]
    #[inline(always)]
    pub fn len43(&self) -> LEN43_R {
        LEN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low detect enabled 44"]
    #[inline(always)]
    pub fn len44(&self) -> LEN44_R {
        LEN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Low detect enabled 45"]
    #[inline(always)]
    pub fn len45(&self) -> LEN45_R {
        LEN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low detect enabled 46"]
    #[inline(always)]
    pub fn len46(&self) -> LEN46_R {
        LEN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Low detect enabled 47"]
    #[inline(always)]
    pub fn len47(&self) -> LEN47_R {
        LEN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Low detect enabled 48"]
    #[inline(always)]
    pub fn len48(&self) -> LEN48_R {
        LEN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low detect enabled 49"]
    #[inline(always)]
    pub fn len49(&self) -> LEN49_R {
        LEN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Low detect enabled 50"]
    #[inline(always)]
    pub fn len50(&self) -> LEN50_R {
        LEN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Low detect enabled 51"]
    #[inline(always)]
    pub fn len51(&self) -> LEN51_R {
        LEN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Low detect enabled 52"]
    #[inline(always)]
    pub fn len52(&self) -> LEN52_R {
        LEN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Low detect enabled 53"]
    #[inline(always)]
    pub fn len53(&self) -> LEN53_R {
        LEN53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low detect enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn len32(&mut self) -> LEN32_W<0> {
        LEN32_W::new(self)
    }
    #[doc = "Bit 1 - Low detect enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn len33(&mut self) -> LEN33_W<1> {
        LEN33_W::new(self)
    }
    #[doc = "Bit 2 - Low detect enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn len34(&mut self) -> LEN34_W<2> {
        LEN34_W::new(self)
    }
    #[doc = "Bit 3 - Low detect enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn len35(&mut self) -> LEN35_W<3> {
        LEN35_W::new(self)
    }
    #[doc = "Bit 4 - Low detect enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn len36(&mut self) -> LEN36_W<4> {
        LEN36_W::new(self)
    }
    #[doc = "Bit 5 - Low detect enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn len37(&mut self) -> LEN37_W<5> {
        LEN37_W::new(self)
    }
    #[doc = "Bit 6 - Low detect enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn len38(&mut self) -> LEN38_W<6> {
        LEN38_W::new(self)
    }
    #[doc = "Bit 7 - Low detect enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn len39(&mut self) -> LEN39_W<7> {
        LEN39_W::new(self)
    }
    #[doc = "Bit 8 - Low detect enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn len40(&mut self) -> LEN40_W<8> {
        LEN40_W::new(self)
    }
    #[doc = "Bit 9 - Low detect enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn len41(&mut self) -> LEN41_W<9> {
        LEN41_W::new(self)
    }
    #[doc = "Bit 10 - Low detect enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn len42(&mut self) -> LEN42_W<10> {
        LEN42_W::new(self)
    }
    #[doc = "Bit 11 - Low detect enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn len43(&mut self) -> LEN43_W<11> {
        LEN43_W::new(self)
    }
    #[doc = "Bit 12 - Low detect enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn len44(&mut self) -> LEN44_W<12> {
        LEN44_W::new(self)
    }
    #[doc = "Bit 13 - Low detect enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn len45(&mut self) -> LEN45_W<13> {
        LEN45_W::new(self)
    }
    #[doc = "Bit 14 - Low detect enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn len46(&mut self) -> LEN46_W<14> {
        LEN46_W::new(self)
    }
    #[doc = "Bit 15 - Low detect enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn len47(&mut self) -> LEN47_W<15> {
        LEN47_W::new(self)
    }
    #[doc = "Bit 16 - Low detect enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn len48(&mut self) -> LEN48_W<16> {
        LEN48_W::new(self)
    }
    #[doc = "Bit 17 - Low detect enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn len49(&mut self) -> LEN49_W<17> {
        LEN49_W::new(self)
    }
    #[doc = "Bit 18 - Low detect enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn len50(&mut self) -> LEN50_W<18> {
        LEN50_W::new(self)
    }
    #[doc = "Bit 19 - Low detect enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn len51(&mut self) -> LEN51_W<19> {
        LEN51_W::new(self)
    }
    #[doc = "Bit 20 - Low detect enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn len52(&mut self) -> LEN52_W<20> {
        LEN52_W::new(self)
    }
    #[doc = "Bit 21 - Low detect enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn len53(&mut self) -> LEN53_W<21> {
        LEN53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Low Detect Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gplen1](index.html) module"]
pub struct GPLEN1_SPEC;
impl crate::RegisterSpec for GPLEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gplen1::R](R) reader structure"]
impl crate::Readable for GPLEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gplen1::W](W) writer structure"]
impl crate::Writable for GPLEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

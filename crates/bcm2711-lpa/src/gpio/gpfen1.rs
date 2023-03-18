#[doc = "Register `GPFEN1` reader"]
pub struct R(crate::R<GPFEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPFEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPFEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPFEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPFEN1` writer"]
pub struct W(crate::W<GPFEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPFEN1_SPEC>;
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
impl From<crate::W<GPFEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPFEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEN32` reader - Falling edge enabled 32"]
pub type FEN32_R = crate::BitReader<bool>;
#[doc = "Field `FEN32` writer - Falling edge enabled 32"]
pub type FEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN33` reader - Falling edge enabled 33"]
pub type FEN33_R = crate::BitReader<bool>;
#[doc = "Field `FEN33` writer - Falling edge enabled 33"]
pub type FEN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN34` reader - Falling edge enabled 34"]
pub type FEN34_R = crate::BitReader<bool>;
#[doc = "Field `FEN34` writer - Falling edge enabled 34"]
pub type FEN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN35` reader - Falling edge enabled 35"]
pub type FEN35_R = crate::BitReader<bool>;
#[doc = "Field `FEN35` writer - Falling edge enabled 35"]
pub type FEN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN36` reader - Falling edge enabled 36"]
pub type FEN36_R = crate::BitReader<bool>;
#[doc = "Field `FEN36` writer - Falling edge enabled 36"]
pub type FEN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN37` reader - Falling edge enabled 37"]
pub type FEN37_R = crate::BitReader<bool>;
#[doc = "Field `FEN37` writer - Falling edge enabled 37"]
pub type FEN37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN38` reader - Falling edge enabled 38"]
pub type FEN38_R = crate::BitReader<bool>;
#[doc = "Field `FEN38` writer - Falling edge enabled 38"]
pub type FEN38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN39` reader - Falling edge enabled 39"]
pub type FEN39_R = crate::BitReader<bool>;
#[doc = "Field `FEN39` writer - Falling edge enabled 39"]
pub type FEN39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN40` reader - Falling edge enabled 40"]
pub type FEN40_R = crate::BitReader<bool>;
#[doc = "Field `FEN40` writer - Falling edge enabled 40"]
pub type FEN40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN41` reader - Falling edge enabled 41"]
pub type FEN41_R = crate::BitReader<bool>;
#[doc = "Field `FEN41` writer - Falling edge enabled 41"]
pub type FEN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN42` reader - Falling edge enabled 42"]
pub type FEN42_R = crate::BitReader<bool>;
#[doc = "Field `FEN42` writer - Falling edge enabled 42"]
pub type FEN42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN43` reader - Falling edge enabled 43"]
pub type FEN43_R = crate::BitReader<bool>;
#[doc = "Field `FEN43` writer - Falling edge enabled 43"]
pub type FEN43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN44` reader - Falling edge enabled 44"]
pub type FEN44_R = crate::BitReader<bool>;
#[doc = "Field `FEN44` writer - Falling edge enabled 44"]
pub type FEN44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN45` reader - Falling edge enabled 45"]
pub type FEN45_R = crate::BitReader<bool>;
#[doc = "Field `FEN45` writer - Falling edge enabled 45"]
pub type FEN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN46` reader - Falling edge enabled 46"]
pub type FEN46_R = crate::BitReader<bool>;
#[doc = "Field `FEN46` writer - Falling edge enabled 46"]
pub type FEN46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN47` reader - Falling edge enabled 47"]
pub type FEN47_R = crate::BitReader<bool>;
#[doc = "Field `FEN47` writer - Falling edge enabled 47"]
pub type FEN47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN48` reader - Falling edge enabled 48"]
pub type FEN48_R = crate::BitReader<bool>;
#[doc = "Field `FEN48` writer - Falling edge enabled 48"]
pub type FEN48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN49` reader - Falling edge enabled 49"]
pub type FEN49_R = crate::BitReader<bool>;
#[doc = "Field `FEN49` writer - Falling edge enabled 49"]
pub type FEN49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN50` reader - Falling edge enabled 50"]
pub type FEN50_R = crate::BitReader<bool>;
#[doc = "Field `FEN50` writer - Falling edge enabled 50"]
pub type FEN50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN51` reader - Falling edge enabled 51"]
pub type FEN51_R = crate::BitReader<bool>;
#[doc = "Field `FEN51` writer - Falling edge enabled 51"]
pub type FEN51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN52` reader - Falling edge enabled 52"]
pub type FEN52_R = crate::BitReader<bool>;
#[doc = "Field `FEN52` writer - Falling edge enabled 52"]
pub type FEN52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN53` reader - Falling edge enabled 53"]
pub type FEN53_R = crate::BitReader<bool>;
#[doc = "Field `FEN53` writer - Falling edge enabled 53"]
pub type FEN53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN54` reader - Falling edge enabled 54"]
pub type FEN54_R = crate::BitReader<bool>;
#[doc = "Field `FEN54` writer - Falling edge enabled 54"]
pub type FEN54_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN55` reader - Falling edge enabled 55"]
pub type FEN55_R = crate::BitReader<bool>;
#[doc = "Field `FEN55` writer - Falling edge enabled 55"]
pub type FEN55_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN56` reader - Falling edge enabled 56"]
pub type FEN56_R = crate::BitReader<bool>;
#[doc = "Field `FEN56` writer - Falling edge enabled 56"]
pub type FEN56_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
#[doc = "Field `FEN57` reader - Falling edge enabled 57"]
pub type FEN57_R = crate::BitReader<bool>;
#[doc = "Field `FEN57` writer - Falling edge enabled 57"]
pub type FEN57_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPFEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Falling edge enabled 32"]
    #[inline(always)]
    pub fn fen32(&self) -> FEN32_R {
        FEN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge enabled 33"]
    #[inline(always)]
    pub fn fen33(&self) -> FEN33_R {
        FEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge enabled 34"]
    #[inline(always)]
    pub fn fen34(&self) -> FEN34_R {
        FEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge enabled 35"]
    #[inline(always)]
    pub fn fen35(&self) -> FEN35_R {
        FEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge enabled 36"]
    #[inline(always)]
    pub fn fen36(&self) -> FEN36_R {
        FEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge enabled 37"]
    #[inline(always)]
    pub fn fen37(&self) -> FEN37_R {
        FEN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge enabled 38"]
    #[inline(always)]
    pub fn fen38(&self) -> FEN38_R {
        FEN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge enabled 39"]
    #[inline(always)]
    pub fn fen39(&self) -> FEN39_R {
        FEN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge enabled 40"]
    #[inline(always)]
    pub fn fen40(&self) -> FEN40_R {
        FEN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge enabled 41"]
    #[inline(always)]
    pub fn fen41(&self) -> FEN41_R {
        FEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge enabled 42"]
    #[inline(always)]
    pub fn fen42(&self) -> FEN42_R {
        FEN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge enabled 43"]
    #[inline(always)]
    pub fn fen43(&self) -> FEN43_R {
        FEN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge enabled 44"]
    #[inline(always)]
    pub fn fen44(&self) -> FEN44_R {
        FEN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge enabled 45"]
    #[inline(always)]
    pub fn fen45(&self) -> FEN45_R {
        FEN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge enabled 46"]
    #[inline(always)]
    pub fn fen46(&self) -> FEN46_R {
        FEN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge enabled 47"]
    #[inline(always)]
    pub fn fen47(&self) -> FEN47_R {
        FEN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling edge enabled 48"]
    #[inline(always)]
    pub fn fen48(&self) -> FEN48_R {
        FEN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling edge enabled 49"]
    #[inline(always)]
    pub fn fen49(&self) -> FEN49_R {
        FEN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling edge enabled 50"]
    #[inline(always)]
    pub fn fen50(&self) -> FEN50_R {
        FEN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling edge enabled 51"]
    #[inline(always)]
    pub fn fen51(&self) -> FEN51_R {
        FEN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling edge enabled 52"]
    #[inline(always)]
    pub fn fen52(&self) -> FEN52_R {
        FEN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling edge enabled 53"]
    #[inline(always)]
    pub fn fen53(&self) -> FEN53_R {
        FEN53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling edge enabled 54"]
    #[inline(always)]
    pub fn fen54(&self) -> FEN54_R {
        FEN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Falling edge enabled 55"]
    #[inline(always)]
    pub fn fen55(&self) -> FEN55_R {
        FEN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Falling edge enabled 56"]
    #[inline(always)]
    pub fn fen56(&self) -> FEN56_R {
        FEN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Falling edge enabled 57"]
    #[inline(always)]
    pub fn fen57(&self) -> FEN57_R {
        FEN57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn fen32(&mut self) -> FEN32_W<0> {
        FEN32_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn fen33(&mut self) -> FEN33_W<1> {
        FEN33_W::new(self)
    }
    #[doc = "Bit 2 - Falling edge enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn fen34(&mut self) -> FEN34_W<2> {
        FEN34_W::new(self)
    }
    #[doc = "Bit 3 - Falling edge enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn fen35(&mut self) -> FEN35_W<3> {
        FEN35_W::new(self)
    }
    #[doc = "Bit 4 - Falling edge enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn fen36(&mut self) -> FEN36_W<4> {
        FEN36_W::new(self)
    }
    #[doc = "Bit 5 - Falling edge enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn fen37(&mut self) -> FEN37_W<5> {
        FEN37_W::new(self)
    }
    #[doc = "Bit 6 - Falling edge enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn fen38(&mut self) -> FEN38_W<6> {
        FEN38_W::new(self)
    }
    #[doc = "Bit 7 - Falling edge enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn fen39(&mut self) -> FEN39_W<7> {
        FEN39_W::new(self)
    }
    #[doc = "Bit 8 - Falling edge enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn fen40(&mut self) -> FEN40_W<8> {
        FEN40_W::new(self)
    }
    #[doc = "Bit 9 - Falling edge enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn fen41(&mut self) -> FEN41_W<9> {
        FEN41_W::new(self)
    }
    #[doc = "Bit 10 - Falling edge enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn fen42(&mut self) -> FEN42_W<10> {
        FEN42_W::new(self)
    }
    #[doc = "Bit 11 - Falling edge enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn fen43(&mut self) -> FEN43_W<11> {
        FEN43_W::new(self)
    }
    #[doc = "Bit 12 - Falling edge enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn fen44(&mut self) -> FEN44_W<12> {
        FEN44_W::new(self)
    }
    #[doc = "Bit 13 - Falling edge enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn fen45(&mut self) -> FEN45_W<13> {
        FEN45_W::new(self)
    }
    #[doc = "Bit 14 - Falling edge enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn fen46(&mut self) -> FEN46_W<14> {
        FEN46_W::new(self)
    }
    #[doc = "Bit 15 - Falling edge enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn fen47(&mut self) -> FEN47_W<15> {
        FEN47_W::new(self)
    }
    #[doc = "Bit 16 - Falling edge enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn fen48(&mut self) -> FEN48_W<16> {
        FEN48_W::new(self)
    }
    #[doc = "Bit 17 - Falling edge enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn fen49(&mut self) -> FEN49_W<17> {
        FEN49_W::new(self)
    }
    #[doc = "Bit 18 - Falling edge enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn fen50(&mut self) -> FEN50_W<18> {
        FEN50_W::new(self)
    }
    #[doc = "Bit 19 - Falling edge enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn fen51(&mut self) -> FEN51_W<19> {
        FEN51_W::new(self)
    }
    #[doc = "Bit 20 - Falling edge enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn fen52(&mut self) -> FEN52_W<20> {
        FEN52_W::new(self)
    }
    #[doc = "Bit 21 - Falling edge enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn fen53(&mut self) -> FEN53_W<21> {
        FEN53_W::new(self)
    }
    #[doc = "Bit 22 - Falling edge enabled 54"]
    #[inline(always)]
    #[must_use]
    pub fn fen54(&mut self) -> FEN54_W<22> {
        FEN54_W::new(self)
    }
    #[doc = "Bit 23 - Falling edge enabled 55"]
    #[inline(always)]
    #[must_use]
    pub fn fen55(&mut self) -> FEN55_W<23> {
        FEN55_W::new(self)
    }
    #[doc = "Bit 24 - Falling edge enabled 56"]
    #[inline(always)]
    #[must_use]
    pub fn fen56(&mut self) -> FEN56_W<24> {
        FEN56_W::new(self)
    }
    #[doc = "Bit 25 - Falling edge enabled 57"]
    #[inline(always)]
    #[must_use]
    pub fn fen57(&mut self) -> FEN57_W<25> {
        FEN57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Falling Edge Detect Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpfen1](index.html) module"]
pub struct GPFEN1_SPEC;
impl crate::RegisterSpec for GPFEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpfen1::R](R) reader structure"]
impl crate::Readable for GPFEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpfen1::W](W) writer structure"]
impl crate::Writable for GPFEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

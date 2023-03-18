#[doc = "Register `GPAFEN1` reader"]
pub struct R(crate::R<GPAFEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPAFEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPAFEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPAFEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPAFEN1` writer"]
pub struct W(crate::W<GPAFEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPAFEN1_SPEC>;
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
impl From<crate::W<GPAFEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPAFEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFEN32` reader - Async falling enabled 32"]
pub type AFEN32_R = crate::BitReader<bool>;
#[doc = "Field `AFEN32` writer - Async falling enabled 32"]
pub type AFEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN33` reader - Async falling enabled 33"]
pub type AFEN33_R = crate::BitReader<bool>;
#[doc = "Field `AFEN33` writer - Async falling enabled 33"]
pub type AFEN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN34` reader - Async falling enabled 34"]
pub type AFEN34_R = crate::BitReader<bool>;
#[doc = "Field `AFEN34` writer - Async falling enabled 34"]
pub type AFEN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN35` reader - Async falling enabled 35"]
pub type AFEN35_R = crate::BitReader<bool>;
#[doc = "Field `AFEN35` writer - Async falling enabled 35"]
pub type AFEN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN36` reader - Async falling enabled 36"]
pub type AFEN36_R = crate::BitReader<bool>;
#[doc = "Field `AFEN36` writer - Async falling enabled 36"]
pub type AFEN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN37` reader - Async falling enabled 37"]
pub type AFEN37_R = crate::BitReader<bool>;
#[doc = "Field `AFEN37` writer - Async falling enabled 37"]
pub type AFEN37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN38` reader - Async falling enabled 38"]
pub type AFEN38_R = crate::BitReader<bool>;
#[doc = "Field `AFEN38` writer - Async falling enabled 38"]
pub type AFEN38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN39` reader - Async falling enabled 39"]
pub type AFEN39_R = crate::BitReader<bool>;
#[doc = "Field `AFEN39` writer - Async falling enabled 39"]
pub type AFEN39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN40` reader - Async falling enabled 40"]
pub type AFEN40_R = crate::BitReader<bool>;
#[doc = "Field `AFEN40` writer - Async falling enabled 40"]
pub type AFEN40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN41` reader - Async falling enabled 41"]
pub type AFEN41_R = crate::BitReader<bool>;
#[doc = "Field `AFEN41` writer - Async falling enabled 41"]
pub type AFEN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN42` reader - Async falling enabled 42"]
pub type AFEN42_R = crate::BitReader<bool>;
#[doc = "Field `AFEN42` writer - Async falling enabled 42"]
pub type AFEN42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN43` reader - Async falling enabled 43"]
pub type AFEN43_R = crate::BitReader<bool>;
#[doc = "Field `AFEN43` writer - Async falling enabled 43"]
pub type AFEN43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN44` reader - Async falling enabled 44"]
pub type AFEN44_R = crate::BitReader<bool>;
#[doc = "Field `AFEN44` writer - Async falling enabled 44"]
pub type AFEN44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN45` reader - Async falling enabled 45"]
pub type AFEN45_R = crate::BitReader<bool>;
#[doc = "Field `AFEN45` writer - Async falling enabled 45"]
pub type AFEN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN46` reader - Async falling enabled 46"]
pub type AFEN46_R = crate::BitReader<bool>;
#[doc = "Field `AFEN46` writer - Async falling enabled 46"]
pub type AFEN46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN47` reader - Async falling enabled 47"]
pub type AFEN47_R = crate::BitReader<bool>;
#[doc = "Field `AFEN47` writer - Async falling enabled 47"]
pub type AFEN47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN48` reader - Async falling enabled 48"]
pub type AFEN48_R = crate::BitReader<bool>;
#[doc = "Field `AFEN48` writer - Async falling enabled 48"]
pub type AFEN48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN49` reader - Async falling enabled 49"]
pub type AFEN49_R = crate::BitReader<bool>;
#[doc = "Field `AFEN49` writer - Async falling enabled 49"]
pub type AFEN49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN50` reader - Async falling enabled 50"]
pub type AFEN50_R = crate::BitReader<bool>;
#[doc = "Field `AFEN50` writer - Async falling enabled 50"]
pub type AFEN50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN51` reader - Async falling enabled 51"]
pub type AFEN51_R = crate::BitReader<bool>;
#[doc = "Field `AFEN51` writer - Async falling enabled 51"]
pub type AFEN51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN52` reader - Async falling enabled 52"]
pub type AFEN52_R = crate::BitReader<bool>;
#[doc = "Field `AFEN52` writer - Async falling enabled 52"]
pub type AFEN52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
#[doc = "Field `AFEN53` reader - Async falling enabled 53"]
pub type AFEN53_R = crate::BitReader<bool>;
#[doc = "Field `AFEN53` writer - Async falling enabled 53"]
pub type AFEN53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAFEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Async falling enabled 32"]
    #[inline(always)]
    pub fn afen32(&self) -> AFEN32_R {
        AFEN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Async falling enabled 33"]
    #[inline(always)]
    pub fn afen33(&self) -> AFEN33_R {
        AFEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Async falling enabled 34"]
    #[inline(always)]
    pub fn afen34(&self) -> AFEN34_R {
        AFEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Async falling enabled 35"]
    #[inline(always)]
    pub fn afen35(&self) -> AFEN35_R {
        AFEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async falling enabled 36"]
    #[inline(always)]
    pub fn afen36(&self) -> AFEN36_R {
        AFEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async falling enabled 37"]
    #[inline(always)]
    pub fn afen37(&self) -> AFEN37_R {
        AFEN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async falling enabled 38"]
    #[inline(always)]
    pub fn afen38(&self) -> AFEN38_R {
        AFEN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async falling enabled 39"]
    #[inline(always)]
    pub fn afen39(&self) -> AFEN39_R {
        AFEN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Async falling enabled 40"]
    #[inline(always)]
    pub fn afen40(&self) -> AFEN40_R {
        AFEN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Async falling enabled 41"]
    #[inline(always)]
    pub fn afen41(&self) -> AFEN41_R {
        AFEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Async falling enabled 42"]
    #[inline(always)]
    pub fn afen42(&self) -> AFEN42_R {
        AFEN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Async falling enabled 43"]
    #[inline(always)]
    pub fn afen43(&self) -> AFEN43_R {
        AFEN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Async falling enabled 44"]
    #[inline(always)]
    pub fn afen44(&self) -> AFEN44_R {
        AFEN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Async falling enabled 45"]
    #[inline(always)]
    pub fn afen45(&self) -> AFEN45_R {
        AFEN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Async falling enabled 46"]
    #[inline(always)]
    pub fn afen46(&self) -> AFEN46_R {
        AFEN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Async falling enabled 47"]
    #[inline(always)]
    pub fn afen47(&self) -> AFEN47_R {
        AFEN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Async falling enabled 48"]
    #[inline(always)]
    pub fn afen48(&self) -> AFEN48_R {
        AFEN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Async falling enabled 49"]
    #[inline(always)]
    pub fn afen49(&self) -> AFEN49_R {
        AFEN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Async falling enabled 50"]
    #[inline(always)]
    pub fn afen50(&self) -> AFEN50_R {
        AFEN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Async falling enabled 51"]
    #[inline(always)]
    pub fn afen51(&self) -> AFEN51_R {
        AFEN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Async falling enabled 52"]
    #[inline(always)]
    pub fn afen52(&self) -> AFEN52_R {
        AFEN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Async falling enabled 53"]
    #[inline(always)]
    pub fn afen53(&self) -> AFEN53_R {
        AFEN53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Async falling enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn afen32(&mut self) -> AFEN32_W<0> {
        AFEN32_W::new(self)
    }
    #[doc = "Bit 1 - Async falling enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn afen33(&mut self) -> AFEN33_W<1> {
        AFEN33_W::new(self)
    }
    #[doc = "Bit 2 - Async falling enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn afen34(&mut self) -> AFEN34_W<2> {
        AFEN34_W::new(self)
    }
    #[doc = "Bit 3 - Async falling enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn afen35(&mut self) -> AFEN35_W<3> {
        AFEN35_W::new(self)
    }
    #[doc = "Bit 4 - Async falling enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn afen36(&mut self) -> AFEN36_W<4> {
        AFEN36_W::new(self)
    }
    #[doc = "Bit 5 - Async falling enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn afen37(&mut self) -> AFEN37_W<5> {
        AFEN37_W::new(self)
    }
    #[doc = "Bit 6 - Async falling enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn afen38(&mut self) -> AFEN38_W<6> {
        AFEN38_W::new(self)
    }
    #[doc = "Bit 7 - Async falling enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn afen39(&mut self) -> AFEN39_W<7> {
        AFEN39_W::new(self)
    }
    #[doc = "Bit 8 - Async falling enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn afen40(&mut self) -> AFEN40_W<8> {
        AFEN40_W::new(self)
    }
    #[doc = "Bit 9 - Async falling enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn afen41(&mut self) -> AFEN41_W<9> {
        AFEN41_W::new(self)
    }
    #[doc = "Bit 10 - Async falling enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn afen42(&mut self) -> AFEN42_W<10> {
        AFEN42_W::new(self)
    }
    #[doc = "Bit 11 - Async falling enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn afen43(&mut self) -> AFEN43_W<11> {
        AFEN43_W::new(self)
    }
    #[doc = "Bit 12 - Async falling enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn afen44(&mut self) -> AFEN44_W<12> {
        AFEN44_W::new(self)
    }
    #[doc = "Bit 13 - Async falling enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn afen45(&mut self) -> AFEN45_W<13> {
        AFEN45_W::new(self)
    }
    #[doc = "Bit 14 - Async falling enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn afen46(&mut self) -> AFEN46_W<14> {
        AFEN46_W::new(self)
    }
    #[doc = "Bit 15 - Async falling enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn afen47(&mut self) -> AFEN47_W<15> {
        AFEN47_W::new(self)
    }
    #[doc = "Bit 16 - Async falling enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn afen48(&mut self) -> AFEN48_W<16> {
        AFEN48_W::new(self)
    }
    #[doc = "Bit 17 - Async falling enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn afen49(&mut self) -> AFEN49_W<17> {
        AFEN49_W::new(self)
    }
    #[doc = "Bit 18 - Async falling enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn afen50(&mut self) -> AFEN50_W<18> {
        AFEN50_W::new(self)
    }
    #[doc = "Bit 19 - Async falling enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn afen51(&mut self) -> AFEN51_W<19> {
        AFEN51_W::new(self)
    }
    #[doc = "Bit 20 - Async falling enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn afen52(&mut self) -> AFEN52_W<20> {
        AFEN52_W::new(self)
    }
    #[doc = "Bit 21 - Async falling enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn afen53(&mut self) -> AFEN53_W<21> {
        AFEN53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Async. Falling Edge Detect 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpafen1](index.html) module"]
pub struct GPAFEN1_SPEC;
impl crate::RegisterSpec for GPAFEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpafen1::R](R) reader structure"]
impl crate::Readable for GPAFEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpafen1::W](W) writer structure"]
impl crate::Writable for GPAFEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

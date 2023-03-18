#[doc = "Register `GPAREN1` reader"]
pub struct R(crate::R<GPAREN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPAREN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPAREN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPAREN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPAREN1` writer"]
pub struct W(crate::W<GPAREN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPAREN1_SPEC>;
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
impl From<crate::W<GPAREN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPAREN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AREN32` reader - Async rising enabled 32"]
pub type AREN32_R = crate::BitReader<bool>;
#[doc = "Field `AREN32` writer - Async rising enabled 32"]
pub type AREN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN33` reader - Async rising enabled 33"]
pub type AREN33_R = crate::BitReader<bool>;
#[doc = "Field `AREN33` writer - Async rising enabled 33"]
pub type AREN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN34` reader - Async rising enabled 34"]
pub type AREN34_R = crate::BitReader<bool>;
#[doc = "Field `AREN34` writer - Async rising enabled 34"]
pub type AREN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN35` reader - Async rising enabled 35"]
pub type AREN35_R = crate::BitReader<bool>;
#[doc = "Field `AREN35` writer - Async rising enabled 35"]
pub type AREN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN36` reader - Async rising enabled 36"]
pub type AREN36_R = crate::BitReader<bool>;
#[doc = "Field `AREN36` writer - Async rising enabled 36"]
pub type AREN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN37` reader - Async rising enabled 37"]
pub type AREN37_R = crate::BitReader<bool>;
#[doc = "Field `AREN37` writer - Async rising enabled 37"]
pub type AREN37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN38` reader - Async rising enabled 38"]
pub type AREN38_R = crate::BitReader<bool>;
#[doc = "Field `AREN38` writer - Async rising enabled 38"]
pub type AREN38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN39` reader - Async rising enabled 39"]
pub type AREN39_R = crate::BitReader<bool>;
#[doc = "Field `AREN39` writer - Async rising enabled 39"]
pub type AREN39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN40` reader - Async rising enabled 40"]
pub type AREN40_R = crate::BitReader<bool>;
#[doc = "Field `AREN40` writer - Async rising enabled 40"]
pub type AREN40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN41` reader - Async rising enabled 41"]
pub type AREN41_R = crate::BitReader<bool>;
#[doc = "Field `AREN41` writer - Async rising enabled 41"]
pub type AREN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN42` reader - Async rising enabled 42"]
pub type AREN42_R = crate::BitReader<bool>;
#[doc = "Field `AREN42` writer - Async rising enabled 42"]
pub type AREN42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN43` reader - Async rising enabled 43"]
pub type AREN43_R = crate::BitReader<bool>;
#[doc = "Field `AREN43` writer - Async rising enabled 43"]
pub type AREN43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN44` reader - Async rising enabled 44"]
pub type AREN44_R = crate::BitReader<bool>;
#[doc = "Field `AREN44` writer - Async rising enabled 44"]
pub type AREN44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN45` reader - Async rising enabled 45"]
pub type AREN45_R = crate::BitReader<bool>;
#[doc = "Field `AREN45` writer - Async rising enabled 45"]
pub type AREN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN46` reader - Async rising enabled 46"]
pub type AREN46_R = crate::BitReader<bool>;
#[doc = "Field `AREN46` writer - Async rising enabled 46"]
pub type AREN46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN47` reader - Async rising enabled 47"]
pub type AREN47_R = crate::BitReader<bool>;
#[doc = "Field `AREN47` writer - Async rising enabled 47"]
pub type AREN47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN48` reader - Async rising enabled 48"]
pub type AREN48_R = crate::BitReader<bool>;
#[doc = "Field `AREN48` writer - Async rising enabled 48"]
pub type AREN48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN49` reader - Async rising enabled 49"]
pub type AREN49_R = crate::BitReader<bool>;
#[doc = "Field `AREN49` writer - Async rising enabled 49"]
pub type AREN49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN50` reader - Async rising enabled 50"]
pub type AREN50_R = crate::BitReader<bool>;
#[doc = "Field `AREN50` writer - Async rising enabled 50"]
pub type AREN50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN51` reader - Async rising enabled 51"]
pub type AREN51_R = crate::BitReader<bool>;
#[doc = "Field `AREN51` writer - Async rising enabled 51"]
pub type AREN51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN52` reader - Async rising enabled 52"]
pub type AREN52_R = crate::BitReader<bool>;
#[doc = "Field `AREN52` writer - Async rising enabled 52"]
pub type AREN52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN53` reader - Async rising enabled 53"]
pub type AREN53_R = crate::BitReader<bool>;
#[doc = "Field `AREN53` writer - Async rising enabled 53"]
pub type AREN53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN54` reader - Async rising enabled 54"]
pub type AREN54_R = crate::BitReader<bool>;
#[doc = "Field `AREN54` writer - Async rising enabled 54"]
pub type AREN54_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN55` reader - Async rising enabled 55"]
pub type AREN55_R = crate::BitReader<bool>;
#[doc = "Field `AREN55` writer - Async rising enabled 55"]
pub type AREN55_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN56` reader - Async rising enabled 56"]
pub type AREN56_R = crate::BitReader<bool>;
#[doc = "Field `AREN56` writer - Async rising enabled 56"]
pub type AREN56_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
#[doc = "Field `AREN57` reader - Async rising enabled 57"]
pub type AREN57_R = crate::BitReader<bool>;
#[doc = "Field `AREN57` writer - Async rising enabled 57"]
pub type AREN57_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPAREN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Async rising enabled 32"]
    #[inline(always)]
    pub fn aren32(&self) -> AREN32_R {
        AREN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Async rising enabled 33"]
    #[inline(always)]
    pub fn aren33(&self) -> AREN33_R {
        AREN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Async rising enabled 34"]
    #[inline(always)]
    pub fn aren34(&self) -> AREN34_R {
        AREN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Async rising enabled 35"]
    #[inline(always)]
    pub fn aren35(&self) -> AREN35_R {
        AREN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async rising enabled 36"]
    #[inline(always)]
    pub fn aren36(&self) -> AREN36_R {
        AREN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async rising enabled 37"]
    #[inline(always)]
    pub fn aren37(&self) -> AREN37_R {
        AREN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async rising enabled 38"]
    #[inline(always)]
    pub fn aren38(&self) -> AREN38_R {
        AREN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async rising enabled 39"]
    #[inline(always)]
    pub fn aren39(&self) -> AREN39_R {
        AREN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Async rising enabled 40"]
    #[inline(always)]
    pub fn aren40(&self) -> AREN40_R {
        AREN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Async rising enabled 41"]
    #[inline(always)]
    pub fn aren41(&self) -> AREN41_R {
        AREN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Async rising enabled 42"]
    #[inline(always)]
    pub fn aren42(&self) -> AREN42_R {
        AREN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Async rising enabled 43"]
    #[inline(always)]
    pub fn aren43(&self) -> AREN43_R {
        AREN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Async rising enabled 44"]
    #[inline(always)]
    pub fn aren44(&self) -> AREN44_R {
        AREN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Async rising enabled 45"]
    #[inline(always)]
    pub fn aren45(&self) -> AREN45_R {
        AREN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Async rising enabled 46"]
    #[inline(always)]
    pub fn aren46(&self) -> AREN46_R {
        AREN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Async rising enabled 47"]
    #[inline(always)]
    pub fn aren47(&self) -> AREN47_R {
        AREN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Async rising enabled 48"]
    #[inline(always)]
    pub fn aren48(&self) -> AREN48_R {
        AREN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Async rising enabled 49"]
    #[inline(always)]
    pub fn aren49(&self) -> AREN49_R {
        AREN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Async rising enabled 50"]
    #[inline(always)]
    pub fn aren50(&self) -> AREN50_R {
        AREN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Async rising enabled 51"]
    #[inline(always)]
    pub fn aren51(&self) -> AREN51_R {
        AREN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Async rising enabled 52"]
    #[inline(always)]
    pub fn aren52(&self) -> AREN52_R {
        AREN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Async rising enabled 53"]
    #[inline(always)]
    pub fn aren53(&self) -> AREN53_R {
        AREN53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Async rising enabled 54"]
    #[inline(always)]
    pub fn aren54(&self) -> AREN54_R {
        AREN54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Async rising enabled 55"]
    #[inline(always)]
    pub fn aren55(&self) -> AREN55_R {
        AREN55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Async rising enabled 56"]
    #[inline(always)]
    pub fn aren56(&self) -> AREN56_R {
        AREN56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Async rising enabled 57"]
    #[inline(always)]
    pub fn aren57(&self) -> AREN57_R {
        AREN57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Async rising enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn aren32(&mut self) -> AREN32_W<0> {
        AREN32_W::new(self)
    }
    #[doc = "Bit 1 - Async rising enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn aren33(&mut self) -> AREN33_W<1> {
        AREN33_W::new(self)
    }
    #[doc = "Bit 2 - Async rising enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn aren34(&mut self) -> AREN34_W<2> {
        AREN34_W::new(self)
    }
    #[doc = "Bit 3 - Async rising enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn aren35(&mut self) -> AREN35_W<3> {
        AREN35_W::new(self)
    }
    #[doc = "Bit 4 - Async rising enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn aren36(&mut self) -> AREN36_W<4> {
        AREN36_W::new(self)
    }
    #[doc = "Bit 5 - Async rising enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn aren37(&mut self) -> AREN37_W<5> {
        AREN37_W::new(self)
    }
    #[doc = "Bit 6 - Async rising enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn aren38(&mut self) -> AREN38_W<6> {
        AREN38_W::new(self)
    }
    #[doc = "Bit 7 - Async rising enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn aren39(&mut self) -> AREN39_W<7> {
        AREN39_W::new(self)
    }
    #[doc = "Bit 8 - Async rising enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn aren40(&mut self) -> AREN40_W<8> {
        AREN40_W::new(self)
    }
    #[doc = "Bit 9 - Async rising enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn aren41(&mut self) -> AREN41_W<9> {
        AREN41_W::new(self)
    }
    #[doc = "Bit 10 - Async rising enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn aren42(&mut self) -> AREN42_W<10> {
        AREN42_W::new(self)
    }
    #[doc = "Bit 11 - Async rising enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn aren43(&mut self) -> AREN43_W<11> {
        AREN43_W::new(self)
    }
    #[doc = "Bit 12 - Async rising enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn aren44(&mut self) -> AREN44_W<12> {
        AREN44_W::new(self)
    }
    #[doc = "Bit 13 - Async rising enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn aren45(&mut self) -> AREN45_W<13> {
        AREN45_W::new(self)
    }
    #[doc = "Bit 14 - Async rising enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn aren46(&mut self) -> AREN46_W<14> {
        AREN46_W::new(self)
    }
    #[doc = "Bit 15 - Async rising enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn aren47(&mut self) -> AREN47_W<15> {
        AREN47_W::new(self)
    }
    #[doc = "Bit 16 - Async rising enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn aren48(&mut self) -> AREN48_W<16> {
        AREN48_W::new(self)
    }
    #[doc = "Bit 17 - Async rising enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn aren49(&mut self) -> AREN49_W<17> {
        AREN49_W::new(self)
    }
    #[doc = "Bit 18 - Async rising enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn aren50(&mut self) -> AREN50_W<18> {
        AREN50_W::new(self)
    }
    #[doc = "Bit 19 - Async rising enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn aren51(&mut self) -> AREN51_W<19> {
        AREN51_W::new(self)
    }
    #[doc = "Bit 20 - Async rising enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn aren52(&mut self) -> AREN52_W<20> {
        AREN52_W::new(self)
    }
    #[doc = "Bit 21 - Async rising enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn aren53(&mut self) -> AREN53_W<21> {
        AREN53_W::new(self)
    }
    #[doc = "Bit 22 - Async rising enabled 54"]
    #[inline(always)]
    #[must_use]
    pub fn aren54(&mut self) -> AREN54_W<22> {
        AREN54_W::new(self)
    }
    #[doc = "Bit 23 - Async rising enabled 55"]
    #[inline(always)]
    #[must_use]
    pub fn aren55(&mut self) -> AREN55_W<23> {
        AREN55_W::new(self)
    }
    #[doc = "Bit 24 - Async rising enabled 56"]
    #[inline(always)]
    #[must_use]
    pub fn aren56(&mut self) -> AREN56_W<24> {
        AREN56_W::new(self)
    }
    #[doc = "Bit 25 - Async rising enabled 57"]
    #[inline(always)]
    #[must_use]
    pub fn aren57(&mut self) -> AREN57_W<25> {
        AREN57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Async. Rising Edge Detect 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gparen1](index.html) module"]
pub struct GPAREN1_SPEC;
impl crate::RegisterSpec for GPAREN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gparen1::R](R) reader structure"]
impl crate::Readable for GPAREN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gparen1::W](W) writer structure"]
impl crate::Writable for GPAREN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

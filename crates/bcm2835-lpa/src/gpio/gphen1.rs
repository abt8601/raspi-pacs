#[doc = "Register `GPHEN1` reader"]
pub struct R(crate::R<GPHEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPHEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPHEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPHEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPHEN1` writer"]
pub struct W(crate::W<GPHEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPHEN1_SPEC>;
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
impl From<crate::W<GPHEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPHEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEN32` reader - High detect enabled 32"]
pub type HEN32_R = crate::BitReader<bool>;
#[doc = "Field `HEN32` writer - High detect enabled 32"]
pub type HEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN33` reader - High detect enabled 33"]
pub type HEN33_R = crate::BitReader<bool>;
#[doc = "Field `HEN33` writer - High detect enabled 33"]
pub type HEN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN34` reader - High detect enabled 34"]
pub type HEN34_R = crate::BitReader<bool>;
#[doc = "Field `HEN34` writer - High detect enabled 34"]
pub type HEN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN35` reader - High detect enabled 35"]
pub type HEN35_R = crate::BitReader<bool>;
#[doc = "Field `HEN35` writer - High detect enabled 35"]
pub type HEN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN36` reader - High detect enabled 36"]
pub type HEN36_R = crate::BitReader<bool>;
#[doc = "Field `HEN36` writer - High detect enabled 36"]
pub type HEN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN37` reader - High detect enabled 37"]
pub type HEN37_R = crate::BitReader<bool>;
#[doc = "Field `HEN37` writer - High detect enabled 37"]
pub type HEN37_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN38` reader - High detect enabled 38"]
pub type HEN38_R = crate::BitReader<bool>;
#[doc = "Field `HEN38` writer - High detect enabled 38"]
pub type HEN38_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN39` reader - High detect enabled 39"]
pub type HEN39_R = crate::BitReader<bool>;
#[doc = "Field `HEN39` writer - High detect enabled 39"]
pub type HEN39_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN40` reader - High detect enabled 40"]
pub type HEN40_R = crate::BitReader<bool>;
#[doc = "Field `HEN40` writer - High detect enabled 40"]
pub type HEN40_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN41` reader - High detect enabled 41"]
pub type HEN41_R = crate::BitReader<bool>;
#[doc = "Field `HEN41` writer - High detect enabled 41"]
pub type HEN41_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN42` reader - High detect enabled 42"]
pub type HEN42_R = crate::BitReader<bool>;
#[doc = "Field `HEN42` writer - High detect enabled 42"]
pub type HEN42_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN43` reader - High detect enabled 43"]
pub type HEN43_R = crate::BitReader<bool>;
#[doc = "Field `HEN43` writer - High detect enabled 43"]
pub type HEN43_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN44` reader - High detect enabled 44"]
pub type HEN44_R = crate::BitReader<bool>;
#[doc = "Field `HEN44` writer - High detect enabled 44"]
pub type HEN44_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN45` reader - High detect enabled 45"]
pub type HEN45_R = crate::BitReader<bool>;
#[doc = "Field `HEN45` writer - High detect enabled 45"]
pub type HEN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN46` reader - High detect enabled 46"]
pub type HEN46_R = crate::BitReader<bool>;
#[doc = "Field `HEN46` writer - High detect enabled 46"]
pub type HEN46_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN47` reader - High detect enabled 47"]
pub type HEN47_R = crate::BitReader<bool>;
#[doc = "Field `HEN47` writer - High detect enabled 47"]
pub type HEN47_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN48` reader - High detect enabled 48"]
pub type HEN48_R = crate::BitReader<bool>;
#[doc = "Field `HEN48` writer - High detect enabled 48"]
pub type HEN48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN49` reader - High detect enabled 49"]
pub type HEN49_R = crate::BitReader<bool>;
#[doc = "Field `HEN49` writer - High detect enabled 49"]
pub type HEN49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN50` reader - High detect enabled 50"]
pub type HEN50_R = crate::BitReader<bool>;
#[doc = "Field `HEN50` writer - High detect enabled 50"]
pub type HEN50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN51` reader - High detect enabled 51"]
pub type HEN51_R = crate::BitReader<bool>;
#[doc = "Field `HEN51` writer - High detect enabled 51"]
pub type HEN51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN52` reader - High detect enabled 52"]
pub type HEN52_R = crate::BitReader<bool>;
#[doc = "Field `HEN52` writer - High detect enabled 52"]
pub type HEN52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
#[doc = "Field `HEN53` reader - High detect enabled 53"]
pub type HEN53_R = crate::BitReader<bool>;
#[doc = "Field `HEN53` writer - High detect enabled 53"]
pub type HEN53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPHEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - High detect enabled 32"]
    #[inline(always)]
    pub fn hen32(&self) -> HEN32_R {
        HEN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High detect enabled 33"]
    #[inline(always)]
    pub fn hen33(&self) -> HEN33_R {
        HEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High detect enabled 34"]
    #[inline(always)]
    pub fn hen34(&self) -> HEN34_R {
        HEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High detect enabled 35"]
    #[inline(always)]
    pub fn hen35(&self) -> HEN35_R {
        HEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High detect enabled 36"]
    #[inline(always)]
    pub fn hen36(&self) -> HEN36_R {
        HEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High detect enabled 37"]
    #[inline(always)]
    pub fn hen37(&self) -> HEN37_R {
        HEN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High detect enabled 38"]
    #[inline(always)]
    pub fn hen38(&self) -> HEN38_R {
        HEN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High detect enabled 39"]
    #[inline(always)]
    pub fn hen39(&self) -> HEN39_R {
        HEN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High detect enabled 40"]
    #[inline(always)]
    pub fn hen40(&self) -> HEN40_R {
        HEN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - High detect enabled 41"]
    #[inline(always)]
    pub fn hen41(&self) -> HEN41_R {
        HEN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High detect enabled 42"]
    #[inline(always)]
    pub fn hen42(&self) -> HEN42_R {
        HEN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - High detect enabled 43"]
    #[inline(always)]
    pub fn hen43(&self) -> HEN43_R {
        HEN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - High detect enabled 44"]
    #[inline(always)]
    pub fn hen44(&self) -> HEN44_R {
        HEN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - High detect enabled 45"]
    #[inline(always)]
    pub fn hen45(&self) -> HEN45_R {
        HEN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - High detect enabled 46"]
    #[inline(always)]
    pub fn hen46(&self) -> HEN46_R {
        HEN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - High detect enabled 47"]
    #[inline(always)]
    pub fn hen47(&self) -> HEN47_R {
        HEN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High detect enabled 48"]
    #[inline(always)]
    pub fn hen48(&self) -> HEN48_R {
        HEN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High detect enabled 49"]
    #[inline(always)]
    pub fn hen49(&self) -> HEN49_R {
        HEN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High detect enabled 50"]
    #[inline(always)]
    pub fn hen50(&self) -> HEN50_R {
        HEN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - High detect enabled 51"]
    #[inline(always)]
    pub fn hen51(&self) -> HEN51_R {
        HEN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - High detect enabled 52"]
    #[inline(always)]
    pub fn hen52(&self) -> HEN52_R {
        HEN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - High detect enabled 53"]
    #[inline(always)]
    pub fn hen53(&self) -> HEN53_R {
        HEN53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High detect enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn hen32(&mut self) -> HEN32_W<0> {
        HEN32_W::new(self)
    }
    #[doc = "Bit 1 - High detect enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn hen33(&mut self) -> HEN33_W<1> {
        HEN33_W::new(self)
    }
    #[doc = "Bit 2 - High detect enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn hen34(&mut self) -> HEN34_W<2> {
        HEN34_W::new(self)
    }
    #[doc = "Bit 3 - High detect enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn hen35(&mut self) -> HEN35_W<3> {
        HEN35_W::new(self)
    }
    #[doc = "Bit 4 - High detect enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn hen36(&mut self) -> HEN36_W<4> {
        HEN36_W::new(self)
    }
    #[doc = "Bit 5 - High detect enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn hen37(&mut self) -> HEN37_W<5> {
        HEN37_W::new(self)
    }
    #[doc = "Bit 6 - High detect enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn hen38(&mut self) -> HEN38_W<6> {
        HEN38_W::new(self)
    }
    #[doc = "Bit 7 - High detect enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn hen39(&mut self) -> HEN39_W<7> {
        HEN39_W::new(self)
    }
    #[doc = "Bit 8 - High detect enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn hen40(&mut self) -> HEN40_W<8> {
        HEN40_W::new(self)
    }
    #[doc = "Bit 9 - High detect enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn hen41(&mut self) -> HEN41_W<9> {
        HEN41_W::new(self)
    }
    #[doc = "Bit 10 - High detect enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn hen42(&mut self) -> HEN42_W<10> {
        HEN42_W::new(self)
    }
    #[doc = "Bit 11 - High detect enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn hen43(&mut self) -> HEN43_W<11> {
        HEN43_W::new(self)
    }
    #[doc = "Bit 12 - High detect enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn hen44(&mut self) -> HEN44_W<12> {
        HEN44_W::new(self)
    }
    #[doc = "Bit 13 - High detect enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn hen45(&mut self) -> HEN45_W<13> {
        HEN45_W::new(self)
    }
    #[doc = "Bit 14 - High detect enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn hen46(&mut self) -> HEN46_W<14> {
        HEN46_W::new(self)
    }
    #[doc = "Bit 15 - High detect enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn hen47(&mut self) -> HEN47_W<15> {
        HEN47_W::new(self)
    }
    #[doc = "Bit 16 - High detect enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn hen48(&mut self) -> HEN48_W<16> {
        HEN48_W::new(self)
    }
    #[doc = "Bit 17 - High detect enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn hen49(&mut self) -> HEN49_W<17> {
        HEN49_W::new(self)
    }
    #[doc = "Bit 18 - High detect enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn hen50(&mut self) -> HEN50_W<18> {
        HEN50_W::new(self)
    }
    #[doc = "Bit 19 - High detect enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn hen51(&mut self) -> HEN51_W<19> {
        HEN51_W::new(self)
    }
    #[doc = "Bit 20 - High detect enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn hen52(&mut self) -> HEN52_W<20> {
        HEN52_W::new(self)
    }
    #[doc = "Bit 21 - High detect enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn hen53(&mut self) -> HEN53_W<21> {
        HEN53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin High Detect Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gphen1](index.html) module"]
pub struct GPHEN1_SPEC;
impl crate::RegisterSpec for GPHEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gphen1::R](R) reader structure"]
impl crate::Readable for GPHEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gphen1::W](W) writer structure"]
impl crate::Writable for GPHEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

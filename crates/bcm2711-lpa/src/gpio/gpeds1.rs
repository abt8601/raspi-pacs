#[doc = "Register `GPEDS1` reader"]
pub struct R(crate::R<GPEDS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPEDS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPEDS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPEDS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPEDS1` writer"]
pub struct W(crate::W<GPEDS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPEDS1_SPEC>;
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
impl From<crate::W<GPEDS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPEDS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDS32` reader - Event detected 32"]
pub type EDS32_R = crate::BitReader<bool>;
#[doc = "Field `EDS32` writer - Event detected 32"]
pub type EDS32_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS33` reader - Event detected 33"]
pub type EDS33_R = crate::BitReader<bool>;
#[doc = "Field `EDS33` writer - Event detected 33"]
pub type EDS33_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS34` reader - Event detected 34"]
pub type EDS34_R = crate::BitReader<bool>;
#[doc = "Field `EDS34` writer - Event detected 34"]
pub type EDS34_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS35` reader - Event detected 35"]
pub type EDS35_R = crate::BitReader<bool>;
#[doc = "Field `EDS35` writer - Event detected 35"]
pub type EDS35_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS36` reader - Event detected 36"]
pub type EDS36_R = crate::BitReader<bool>;
#[doc = "Field `EDS36` writer - Event detected 36"]
pub type EDS36_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS37` reader - Event detected 37"]
pub type EDS37_R = crate::BitReader<bool>;
#[doc = "Field `EDS37` writer - Event detected 37"]
pub type EDS37_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS38` reader - Event detected 38"]
pub type EDS38_R = crate::BitReader<bool>;
#[doc = "Field `EDS38` writer - Event detected 38"]
pub type EDS38_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS39` reader - Event detected 39"]
pub type EDS39_R = crate::BitReader<bool>;
#[doc = "Field `EDS39` writer - Event detected 39"]
pub type EDS39_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS40` reader - Event detected 40"]
pub type EDS40_R = crate::BitReader<bool>;
#[doc = "Field `EDS40` writer - Event detected 40"]
pub type EDS40_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS41` reader - Event detected 41"]
pub type EDS41_R = crate::BitReader<bool>;
#[doc = "Field `EDS41` writer - Event detected 41"]
pub type EDS41_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS42` reader - Event detected 42"]
pub type EDS42_R = crate::BitReader<bool>;
#[doc = "Field `EDS42` writer - Event detected 42"]
pub type EDS42_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS43` reader - Event detected 43"]
pub type EDS43_R = crate::BitReader<bool>;
#[doc = "Field `EDS43` writer - Event detected 43"]
pub type EDS43_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS44` reader - Event detected 44"]
pub type EDS44_R = crate::BitReader<bool>;
#[doc = "Field `EDS44` writer - Event detected 44"]
pub type EDS44_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS45` reader - Event detected 45"]
pub type EDS45_R = crate::BitReader<bool>;
#[doc = "Field `EDS45` writer - Event detected 45"]
pub type EDS45_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS46` reader - Event detected 46"]
pub type EDS46_R = crate::BitReader<bool>;
#[doc = "Field `EDS46` writer - Event detected 46"]
pub type EDS46_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS47` reader - Event detected 47"]
pub type EDS47_R = crate::BitReader<bool>;
#[doc = "Field `EDS47` writer - Event detected 47"]
pub type EDS47_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS48` reader - Event detected 48"]
pub type EDS48_R = crate::BitReader<bool>;
#[doc = "Field `EDS48` writer - Event detected 48"]
pub type EDS48_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS49` reader - Event detected 49"]
pub type EDS49_R = crate::BitReader<bool>;
#[doc = "Field `EDS49` writer - Event detected 49"]
pub type EDS49_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS50` reader - Event detected 50"]
pub type EDS50_R = crate::BitReader<bool>;
#[doc = "Field `EDS50` writer - Event detected 50"]
pub type EDS50_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS51` reader - Event detected 51"]
pub type EDS51_R = crate::BitReader<bool>;
#[doc = "Field `EDS51` writer - Event detected 51"]
pub type EDS51_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS52` reader - Event detected 52"]
pub type EDS52_R = crate::BitReader<bool>;
#[doc = "Field `EDS52` writer - Event detected 52"]
pub type EDS52_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS53` reader - Event detected 53"]
pub type EDS53_R = crate::BitReader<bool>;
#[doc = "Field `EDS53` writer - Event detected 53"]
pub type EDS53_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS54` reader - Event detected 54"]
pub type EDS54_R = crate::BitReader<bool>;
#[doc = "Field `EDS54` writer - Event detected 54"]
pub type EDS54_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS55` reader - Event detected 55"]
pub type EDS55_R = crate::BitReader<bool>;
#[doc = "Field `EDS55` writer - Event detected 55"]
pub type EDS55_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS56` reader - Event detected 56"]
pub type EDS56_R = crate::BitReader<bool>;
#[doc = "Field `EDS56` writer - Event detected 56"]
pub type EDS56_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
#[doc = "Field `EDS57` reader - Event detected 57"]
pub type EDS57_R = crate::BitReader<bool>;
#[doc = "Field `EDS57` writer - Event detected 57"]
pub type EDS57_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPEDS1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event detected 32"]
    #[inline(always)]
    pub fn eds32(&self) -> EDS32_R {
        EDS32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event detected 33"]
    #[inline(always)]
    pub fn eds33(&self) -> EDS33_R {
        EDS33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event detected 34"]
    #[inline(always)]
    pub fn eds34(&self) -> EDS34_R {
        EDS34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event detected 35"]
    #[inline(always)]
    pub fn eds35(&self) -> EDS35_R {
        EDS35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event detected 36"]
    #[inline(always)]
    pub fn eds36(&self) -> EDS36_R {
        EDS36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event detected 37"]
    #[inline(always)]
    pub fn eds37(&self) -> EDS37_R {
        EDS37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event detected 38"]
    #[inline(always)]
    pub fn eds38(&self) -> EDS38_R {
        EDS38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event detected 39"]
    #[inline(always)]
    pub fn eds39(&self) -> EDS39_R {
        EDS39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event detected 40"]
    #[inline(always)]
    pub fn eds40(&self) -> EDS40_R {
        EDS40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event detected 41"]
    #[inline(always)]
    pub fn eds41(&self) -> EDS41_R {
        EDS41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event detected 42"]
    #[inline(always)]
    pub fn eds42(&self) -> EDS42_R {
        EDS42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event detected 43"]
    #[inline(always)]
    pub fn eds43(&self) -> EDS43_R {
        EDS43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event detected 44"]
    #[inline(always)]
    pub fn eds44(&self) -> EDS44_R {
        EDS44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event detected 45"]
    #[inline(always)]
    pub fn eds45(&self) -> EDS45_R {
        EDS45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event detected 46"]
    #[inline(always)]
    pub fn eds46(&self) -> EDS46_R {
        EDS46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event detected 47"]
    #[inline(always)]
    pub fn eds47(&self) -> EDS47_R {
        EDS47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event detected 48"]
    #[inline(always)]
    pub fn eds48(&self) -> EDS48_R {
        EDS48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event detected 49"]
    #[inline(always)]
    pub fn eds49(&self) -> EDS49_R {
        EDS49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event detected 50"]
    #[inline(always)]
    pub fn eds50(&self) -> EDS50_R {
        EDS50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event detected 51"]
    #[inline(always)]
    pub fn eds51(&self) -> EDS51_R {
        EDS51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event detected 52"]
    #[inline(always)]
    pub fn eds52(&self) -> EDS52_R {
        EDS52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event detected 53"]
    #[inline(always)]
    pub fn eds53(&self) -> EDS53_R {
        EDS53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event detected 54"]
    #[inline(always)]
    pub fn eds54(&self) -> EDS54_R {
        EDS54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Event detected 55"]
    #[inline(always)]
    pub fn eds55(&self) -> EDS55_R {
        EDS55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Event detected 56"]
    #[inline(always)]
    pub fn eds56(&self) -> EDS56_R {
        EDS56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Event detected 57"]
    #[inline(always)]
    pub fn eds57(&self) -> EDS57_R {
        EDS57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event detected 32"]
    #[inline(always)]
    #[must_use]
    pub fn eds32(&mut self) -> EDS32_W<0> {
        EDS32_W::new(self)
    }
    #[doc = "Bit 1 - Event detected 33"]
    #[inline(always)]
    #[must_use]
    pub fn eds33(&mut self) -> EDS33_W<1> {
        EDS33_W::new(self)
    }
    #[doc = "Bit 2 - Event detected 34"]
    #[inline(always)]
    #[must_use]
    pub fn eds34(&mut self) -> EDS34_W<2> {
        EDS34_W::new(self)
    }
    #[doc = "Bit 3 - Event detected 35"]
    #[inline(always)]
    #[must_use]
    pub fn eds35(&mut self) -> EDS35_W<3> {
        EDS35_W::new(self)
    }
    #[doc = "Bit 4 - Event detected 36"]
    #[inline(always)]
    #[must_use]
    pub fn eds36(&mut self) -> EDS36_W<4> {
        EDS36_W::new(self)
    }
    #[doc = "Bit 5 - Event detected 37"]
    #[inline(always)]
    #[must_use]
    pub fn eds37(&mut self) -> EDS37_W<5> {
        EDS37_W::new(self)
    }
    #[doc = "Bit 6 - Event detected 38"]
    #[inline(always)]
    #[must_use]
    pub fn eds38(&mut self) -> EDS38_W<6> {
        EDS38_W::new(self)
    }
    #[doc = "Bit 7 - Event detected 39"]
    #[inline(always)]
    #[must_use]
    pub fn eds39(&mut self) -> EDS39_W<7> {
        EDS39_W::new(self)
    }
    #[doc = "Bit 8 - Event detected 40"]
    #[inline(always)]
    #[must_use]
    pub fn eds40(&mut self) -> EDS40_W<8> {
        EDS40_W::new(self)
    }
    #[doc = "Bit 9 - Event detected 41"]
    #[inline(always)]
    #[must_use]
    pub fn eds41(&mut self) -> EDS41_W<9> {
        EDS41_W::new(self)
    }
    #[doc = "Bit 10 - Event detected 42"]
    #[inline(always)]
    #[must_use]
    pub fn eds42(&mut self) -> EDS42_W<10> {
        EDS42_W::new(self)
    }
    #[doc = "Bit 11 - Event detected 43"]
    #[inline(always)]
    #[must_use]
    pub fn eds43(&mut self) -> EDS43_W<11> {
        EDS43_W::new(self)
    }
    #[doc = "Bit 12 - Event detected 44"]
    #[inline(always)]
    #[must_use]
    pub fn eds44(&mut self) -> EDS44_W<12> {
        EDS44_W::new(self)
    }
    #[doc = "Bit 13 - Event detected 45"]
    #[inline(always)]
    #[must_use]
    pub fn eds45(&mut self) -> EDS45_W<13> {
        EDS45_W::new(self)
    }
    #[doc = "Bit 14 - Event detected 46"]
    #[inline(always)]
    #[must_use]
    pub fn eds46(&mut self) -> EDS46_W<14> {
        EDS46_W::new(self)
    }
    #[doc = "Bit 15 - Event detected 47"]
    #[inline(always)]
    #[must_use]
    pub fn eds47(&mut self) -> EDS47_W<15> {
        EDS47_W::new(self)
    }
    #[doc = "Bit 16 - Event detected 48"]
    #[inline(always)]
    #[must_use]
    pub fn eds48(&mut self) -> EDS48_W<16> {
        EDS48_W::new(self)
    }
    #[doc = "Bit 17 - Event detected 49"]
    #[inline(always)]
    #[must_use]
    pub fn eds49(&mut self) -> EDS49_W<17> {
        EDS49_W::new(self)
    }
    #[doc = "Bit 18 - Event detected 50"]
    #[inline(always)]
    #[must_use]
    pub fn eds50(&mut self) -> EDS50_W<18> {
        EDS50_W::new(self)
    }
    #[doc = "Bit 19 - Event detected 51"]
    #[inline(always)]
    #[must_use]
    pub fn eds51(&mut self) -> EDS51_W<19> {
        EDS51_W::new(self)
    }
    #[doc = "Bit 20 - Event detected 52"]
    #[inline(always)]
    #[must_use]
    pub fn eds52(&mut self) -> EDS52_W<20> {
        EDS52_W::new(self)
    }
    #[doc = "Bit 21 - Event detected 53"]
    #[inline(always)]
    #[must_use]
    pub fn eds53(&mut self) -> EDS53_W<21> {
        EDS53_W::new(self)
    }
    #[doc = "Bit 22 - Event detected 54"]
    #[inline(always)]
    #[must_use]
    pub fn eds54(&mut self) -> EDS54_W<22> {
        EDS54_W::new(self)
    }
    #[doc = "Bit 23 - Event detected 55"]
    #[inline(always)]
    #[must_use]
    pub fn eds55(&mut self) -> EDS55_W<23> {
        EDS55_W::new(self)
    }
    #[doc = "Bit 24 - Event detected 56"]
    #[inline(always)]
    #[must_use]
    pub fn eds56(&mut self) -> EDS56_W<24> {
        EDS56_W::new(self)
    }
    #[doc = "Bit 25 - Event detected 57"]
    #[inline(always)]
    #[must_use]
    pub fn eds57(&mut self) -> EDS57_W<25> {
        EDS57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Event Detect Status 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpeds1](index.html) module"]
pub struct GPEDS1_SPEC;
impl crate::RegisterSpec for GPEDS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpeds1::R](R) reader structure"]
impl crate::Readable for GPEDS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpeds1::W](W) writer structure"]
impl crate::Writable for GPEDS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff_ffff;
}

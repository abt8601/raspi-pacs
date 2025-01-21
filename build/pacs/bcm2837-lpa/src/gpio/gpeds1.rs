#[doc = "Register `GPEDS1` reader"]
pub type R = crate::R<GPEDS1_SPEC>;
#[doc = "Register `GPEDS1` writer"]
pub type W = crate::W<GPEDS1_SPEC>;
#[doc = "Field `EDS32` reader - Event detected 32"]
pub type EDS32_R = crate::BitReader;
#[doc = "Field `EDS32` writer - Event detected 32"]
pub type EDS32_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS33` reader - Event detected 33"]
pub type EDS33_R = crate::BitReader;
#[doc = "Field `EDS33` writer - Event detected 33"]
pub type EDS33_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS34` reader - Event detected 34"]
pub type EDS34_R = crate::BitReader;
#[doc = "Field `EDS34` writer - Event detected 34"]
pub type EDS34_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS35` reader - Event detected 35"]
pub type EDS35_R = crate::BitReader;
#[doc = "Field `EDS35` writer - Event detected 35"]
pub type EDS35_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS36` reader - Event detected 36"]
pub type EDS36_R = crate::BitReader;
#[doc = "Field `EDS36` writer - Event detected 36"]
pub type EDS36_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS37` reader - Event detected 37"]
pub type EDS37_R = crate::BitReader;
#[doc = "Field `EDS37` writer - Event detected 37"]
pub type EDS37_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS38` reader - Event detected 38"]
pub type EDS38_R = crate::BitReader;
#[doc = "Field `EDS38` writer - Event detected 38"]
pub type EDS38_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS39` reader - Event detected 39"]
pub type EDS39_R = crate::BitReader;
#[doc = "Field `EDS39` writer - Event detected 39"]
pub type EDS39_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS40` reader - Event detected 40"]
pub type EDS40_R = crate::BitReader;
#[doc = "Field `EDS40` writer - Event detected 40"]
pub type EDS40_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS41` reader - Event detected 41"]
pub type EDS41_R = crate::BitReader;
#[doc = "Field `EDS41` writer - Event detected 41"]
pub type EDS41_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS42` reader - Event detected 42"]
pub type EDS42_R = crate::BitReader;
#[doc = "Field `EDS42` writer - Event detected 42"]
pub type EDS42_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS43` reader - Event detected 43"]
pub type EDS43_R = crate::BitReader;
#[doc = "Field `EDS43` writer - Event detected 43"]
pub type EDS43_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS44` reader - Event detected 44"]
pub type EDS44_R = crate::BitReader;
#[doc = "Field `EDS44` writer - Event detected 44"]
pub type EDS44_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS45` reader - Event detected 45"]
pub type EDS45_R = crate::BitReader;
#[doc = "Field `EDS45` writer - Event detected 45"]
pub type EDS45_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS46` reader - Event detected 46"]
pub type EDS46_R = crate::BitReader;
#[doc = "Field `EDS46` writer - Event detected 46"]
pub type EDS46_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS47` reader - Event detected 47"]
pub type EDS47_R = crate::BitReader;
#[doc = "Field `EDS47` writer - Event detected 47"]
pub type EDS47_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS48` reader - Event detected 48"]
pub type EDS48_R = crate::BitReader;
#[doc = "Field `EDS48` writer - Event detected 48"]
pub type EDS48_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS49` reader - Event detected 49"]
pub type EDS49_R = crate::BitReader;
#[doc = "Field `EDS49` writer - Event detected 49"]
pub type EDS49_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS50` reader - Event detected 50"]
pub type EDS50_R = crate::BitReader;
#[doc = "Field `EDS50` writer - Event detected 50"]
pub type EDS50_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS51` reader - Event detected 51"]
pub type EDS51_R = crate::BitReader;
#[doc = "Field `EDS51` writer - Event detected 51"]
pub type EDS51_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS52` reader - Event detected 52"]
pub type EDS52_R = crate::BitReader;
#[doc = "Field `EDS52` writer - Event detected 52"]
pub type EDS52_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EDS53` reader - Event detected 53"]
pub type EDS53_R = crate::BitReader;
#[doc = "Field `EDS53` writer - Event detected 53"]
pub type EDS53_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPEDS1")
            .field("eds32", &format_args!("{}", self.eds32().bit()))
            .field("eds33", &format_args!("{}", self.eds33().bit()))
            .field("eds34", &format_args!("{}", self.eds34().bit()))
            .field("eds35", &format_args!("{}", self.eds35().bit()))
            .field("eds36", &format_args!("{}", self.eds36().bit()))
            .field("eds37", &format_args!("{}", self.eds37().bit()))
            .field("eds38", &format_args!("{}", self.eds38().bit()))
            .field("eds39", &format_args!("{}", self.eds39().bit()))
            .field("eds40", &format_args!("{}", self.eds40().bit()))
            .field("eds41", &format_args!("{}", self.eds41().bit()))
            .field("eds42", &format_args!("{}", self.eds42().bit()))
            .field("eds43", &format_args!("{}", self.eds43().bit()))
            .field("eds44", &format_args!("{}", self.eds44().bit()))
            .field("eds45", &format_args!("{}", self.eds45().bit()))
            .field("eds46", &format_args!("{}", self.eds46().bit()))
            .field("eds47", &format_args!("{}", self.eds47().bit()))
            .field("eds48", &format_args!("{}", self.eds48().bit()))
            .field("eds49", &format_args!("{}", self.eds49().bit()))
            .field("eds50", &format_args!("{}", self.eds50().bit()))
            .field("eds51", &format_args!("{}", self.eds51().bit()))
            .field("eds52", &format_args!("{}", self.eds52().bit()))
            .field("eds53", &format_args!("{}", self.eds53().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPEDS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Event detected 32"]
    #[inline(always)]
    #[must_use]
    pub fn eds32(&mut self) -> EDS32_W<GPEDS1_SPEC> {
        EDS32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event detected 33"]
    #[inline(always)]
    #[must_use]
    pub fn eds33(&mut self) -> EDS33_W<GPEDS1_SPEC> {
        EDS33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event detected 34"]
    #[inline(always)]
    #[must_use]
    pub fn eds34(&mut self) -> EDS34_W<GPEDS1_SPEC> {
        EDS34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event detected 35"]
    #[inline(always)]
    #[must_use]
    pub fn eds35(&mut self) -> EDS35_W<GPEDS1_SPEC> {
        EDS35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event detected 36"]
    #[inline(always)]
    #[must_use]
    pub fn eds36(&mut self) -> EDS36_W<GPEDS1_SPEC> {
        EDS36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event detected 37"]
    #[inline(always)]
    #[must_use]
    pub fn eds37(&mut self) -> EDS37_W<GPEDS1_SPEC> {
        EDS37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event detected 38"]
    #[inline(always)]
    #[must_use]
    pub fn eds38(&mut self) -> EDS38_W<GPEDS1_SPEC> {
        EDS38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event detected 39"]
    #[inline(always)]
    #[must_use]
    pub fn eds39(&mut self) -> EDS39_W<GPEDS1_SPEC> {
        EDS39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event detected 40"]
    #[inline(always)]
    #[must_use]
    pub fn eds40(&mut self) -> EDS40_W<GPEDS1_SPEC> {
        EDS40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event detected 41"]
    #[inline(always)]
    #[must_use]
    pub fn eds41(&mut self) -> EDS41_W<GPEDS1_SPEC> {
        EDS41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event detected 42"]
    #[inline(always)]
    #[must_use]
    pub fn eds42(&mut self) -> EDS42_W<GPEDS1_SPEC> {
        EDS42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event detected 43"]
    #[inline(always)]
    #[must_use]
    pub fn eds43(&mut self) -> EDS43_W<GPEDS1_SPEC> {
        EDS43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Event detected 44"]
    #[inline(always)]
    #[must_use]
    pub fn eds44(&mut self) -> EDS44_W<GPEDS1_SPEC> {
        EDS44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Event detected 45"]
    #[inline(always)]
    #[must_use]
    pub fn eds45(&mut self) -> EDS45_W<GPEDS1_SPEC> {
        EDS45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Event detected 46"]
    #[inline(always)]
    #[must_use]
    pub fn eds46(&mut self) -> EDS46_W<GPEDS1_SPEC> {
        EDS46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Event detected 47"]
    #[inline(always)]
    #[must_use]
    pub fn eds47(&mut self) -> EDS47_W<GPEDS1_SPEC> {
        EDS47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Event detected 48"]
    #[inline(always)]
    #[must_use]
    pub fn eds48(&mut self) -> EDS48_W<GPEDS1_SPEC> {
        EDS48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Event detected 49"]
    #[inline(always)]
    #[must_use]
    pub fn eds49(&mut self) -> EDS49_W<GPEDS1_SPEC> {
        EDS49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Event detected 50"]
    #[inline(always)]
    #[must_use]
    pub fn eds50(&mut self) -> EDS50_W<GPEDS1_SPEC> {
        EDS50_W::new(self, 18)
    }
    #[doc = "Bit 19 - Event detected 51"]
    #[inline(always)]
    #[must_use]
    pub fn eds51(&mut self) -> EDS51_W<GPEDS1_SPEC> {
        EDS51_W::new(self, 19)
    }
    #[doc = "Bit 20 - Event detected 52"]
    #[inline(always)]
    #[must_use]
    pub fn eds52(&mut self) -> EDS52_W<GPEDS1_SPEC> {
        EDS52_W::new(self, 20)
    }
    #[doc = "Bit 21 - Event detected 53"]
    #[inline(always)]
    #[must_use]
    pub fn eds53(&mut self) -> EDS53_W<GPEDS1_SPEC> {
        EDS53_W::new(self, 21)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO Pin Event Detect Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpeds1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpeds1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPEDS1_SPEC;
impl crate::RegisterSpec for GPEDS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpeds1::R`](R) reader structure"]
impl crate::Readable for GPEDS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpeds1::W`](W) writer structure"]
impl crate::Writable for GPEDS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x003f_ffff;
}

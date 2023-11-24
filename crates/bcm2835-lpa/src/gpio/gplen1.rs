#[doc = "Register `GPLEN1` reader"]
pub type R = crate::R<GPLEN1_SPEC>;
#[doc = "Register `GPLEN1` writer"]
pub type W = crate::W<GPLEN1_SPEC>;
#[doc = "Field `LEN32` reader - Low detect enabled 32"]
pub type LEN32_R = crate::BitReader;
#[doc = "Field `LEN32` writer - Low detect enabled 32"]
pub type LEN32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN33` reader - Low detect enabled 33"]
pub type LEN33_R = crate::BitReader;
#[doc = "Field `LEN33` writer - Low detect enabled 33"]
pub type LEN33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN34` reader - Low detect enabled 34"]
pub type LEN34_R = crate::BitReader;
#[doc = "Field `LEN34` writer - Low detect enabled 34"]
pub type LEN34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN35` reader - Low detect enabled 35"]
pub type LEN35_R = crate::BitReader;
#[doc = "Field `LEN35` writer - Low detect enabled 35"]
pub type LEN35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN36` reader - Low detect enabled 36"]
pub type LEN36_R = crate::BitReader;
#[doc = "Field `LEN36` writer - Low detect enabled 36"]
pub type LEN36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN37` reader - Low detect enabled 37"]
pub type LEN37_R = crate::BitReader;
#[doc = "Field `LEN37` writer - Low detect enabled 37"]
pub type LEN37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN38` reader - Low detect enabled 38"]
pub type LEN38_R = crate::BitReader;
#[doc = "Field `LEN38` writer - Low detect enabled 38"]
pub type LEN38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN39` reader - Low detect enabled 39"]
pub type LEN39_R = crate::BitReader;
#[doc = "Field `LEN39` writer - Low detect enabled 39"]
pub type LEN39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN40` reader - Low detect enabled 40"]
pub type LEN40_R = crate::BitReader;
#[doc = "Field `LEN40` writer - Low detect enabled 40"]
pub type LEN40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN41` reader - Low detect enabled 41"]
pub type LEN41_R = crate::BitReader;
#[doc = "Field `LEN41` writer - Low detect enabled 41"]
pub type LEN41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN42` reader - Low detect enabled 42"]
pub type LEN42_R = crate::BitReader;
#[doc = "Field `LEN42` writer - Low detect enabled 42"]
pub type LEN42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN43` reader - Low detect enabled 43"]
pub type LEN43_R = crate::BitReader;
#[doc = "Field `LEN43` writer - Low detect enabled 43"]
pub type LEN43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN44` reader - Low detect enabled 44"]
pub type LEN44_R = crate::BitReader;
#[doc = "Field `LEN44` writer - Low detect enabled 44"]
pub type LEN44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN45` reader - Low detect enabled 45"]
pub type LEN45_R = crate::BitReader;
#[doc = "Field `LEN45` writer - Low detect enabled 45"]
pub type LEN45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN46` reader - Low detect enabled 46"]
pub type LEN46_R = crate::BitReader;
#[doc = "Field `LEN46` writer - Low detect enabled 46"]
pub type LEN46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN47` reader - Low detect enabled 47"]
pub type LEN47_R = crate::BitReader;
#[doc = "Field `LEN47` writer - Low detect enabled 47"]
pub type LEN47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN48` reader - Low detect enabled 48"]
pub type LEN48_R = crate::BitReader;
#[doc = "Field `LEN48` writer - Low detect enabled 48"]
pub type LEN48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN49` reader - Low detect enabled 49"]
pub type LEN49_R = crate::BitReader;
#[doc = "Field `LEN49` writer - Low detect enabled 49"]
pub type LEN49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN50` reader - Low detect enabled 50"]
pub type LEN50_R = crate::BitReader;
#[doc = "Field `LEN50` writer - Low detect enabled 50"]
pub type LEN50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN51` reader - Low detect enabled 51"]
pub type LEN51_R = crate::BitReader;
#[doc = "Field `LEN51` writer - Low detect enabled 51"]
pub type LEN51_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN52` reader - Low detect enabled 52"]
pub type LEN52_R = crate::BitReader;
#[doc = "Field `LEN52` writer - Low detect enabled 52"]
pub type LEN52_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN53` reader - Low detect enabled 53"]
pub type LEN53_R = crate::BitReader;
#[doc = "Field `LEN53` writer - Low detect enabled 53"]
pub type LEN53_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPLEN1")
            .field("len32", &format_args!("{}", self.len32().bit()))
            .field("len33", &format_args!("{}", self.len33().bit()))
            .field("len34", &format_args!("{}", self.len34().bit()))
            .field("len35", &format_args!("{}", self.len35().bit()))
            .field("len36", &format_args!("{}", self.len36().bit()))
            .field("len37", &format_args!("{}", self.len37().bit()))
            .field("len38", &format_args!("{}", self.len38().bit()))
            .field("len39", &format_args!("{}", self.len39().bit()))
            .field("len40", &format_args!("{}", self.len40().bit()))
            .field("len41", &format_args!("{}", self.len41().bit()))
            .field("len42", &format_args!("{}", self.len42().bit()))
            .field("len43", &format_args!("{}", self.len43().bit()))
            .field("len44", &format_args!("{}", self.len44().bit()))
            .field("len45", &format_args!("{}", self.len45().bit()))
            .field("len46", &format_args!("{}", self.len46().bit()))
            .field("len47", &format_args!("{}", self.len47().bit()))
            .field("len48", &format_args!("{}", self.len48().bit()))
            .field("len49", &format_args!("{}", self.len49().bit()))
            .field("len50", &format_args!("{}", self.len50().bit()))
            .field("len51", &format_args!("{}", self.len51().bit()))
            .field("len52", &format_args!("{}", self.len52().bit()))
            .field("len53", &format_args!("{}", self.len53().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPLEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Low detect enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn len32(&mut self) -> LEN32_W<GPLEN1_SPEC> {
        LEN32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low detect enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn len33(&mut self) -> LEN33_W<GPLEN1_SPEC> {
        LEN33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low detect enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn len34(&mut self) -> LEN34_W<GPLEN1_SPEC> {
        LEN34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Low detect enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn len35(&mut self) -> LEN35_W<GPLEN1_SPEC> {
        LEN35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Low detect enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn len36(&mut self) -> LEN36_W<GPLEN1_SPEC> {
        LEN36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Low detect enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn len37(&mut self) -> LEN37_W<GPLEN1_SPEC> {
        LEN37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low detect enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn len38(&mut self) -> LEN38_W<GPLEN1_SPEC> {
        LEN38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Low detect enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn len39(&mut self) -> LEN39_W<GPLEN1_SPEC> {
        LEN39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Low detect enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn len40(&mut self) -> LEN40_W<GPLEN1_SPEC> {
        LEN40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low detect enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn len41(&mut self) -> LEN41_W<GPLEN1_SPEC> {
        LEN41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Low detect enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn len42(&mut self) -> LEN42_W<GPLEN1_SPEC> {
        LEN42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Low detect enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn len43(&mut self) -> LEN43_W<GPLEN1_SPEC> {
        LEN43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Low detect enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn len44(&mut self) -> LEN44_W<GPLEN1_SPEC> {
        LEN44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Low detect enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn len45(&mut self) -> LEN45_W<GPLEN1_SPEC> {
        LEN45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Low detect enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn len46(&mut self) -> LEN46_W<GPLEN1_SPEC> {
        LEN46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Low detect enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn len47(&mut self) -> LEN47_W<GPLEN1_SPEC> {
        LEN47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Low detect enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn len48(&mut self) -> LEN48_W<GPLEN1_SPEC> {
        LEN48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Low detect enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn len49(&mut self) -> LEN49_W<GPLEN1_SPEC> {
        LEN49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Low detect enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn len50(&mut self) -> LEN50_W<GPLEN1_SPEC> {
        LEN50_W::new(self, 18)
    }
    #[doc = "Bit 19 - Low detect enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn len51(&mut self) -> LEN51_W<GPLEN1_SPEC> {
        LEN51_W::new(self, 19)
    }
    #[doc = "Bit 20 - Low detect enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn len52(&mut self) -> LEN52_W<GPLEN1_SPEC> {
        LEN52_W::new(self, 20)
    }
    #[doc = "Bit 21 - Low detect enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn len53(&mut self) -> LEN53_W<GPLEN1_SPEC> {
        LEN53_W::new(self, 21)
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
#[doc = "GPIO Pin Low Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplen1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gplen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPLEN1_SPEC;
impl crate::RegisterSpec for GPLEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gplen1::R`](R) reader structure"]
impl crate::Readable for GPLEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gplen1::W`](W) writer structure"]
impl crate::Writable for GPLEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

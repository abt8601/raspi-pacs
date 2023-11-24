#[doc = "Register `GPREN1` reader"]
pub type R = crate::R<GPREN1_SPEC>;
#[doc = "Register `GPREN1` writer"]
pub type W = crate::W<GPREN1_SPEC>;
#[doc = "Field `REN32` reader - Rising edge enabled 32"]
pub type REN32_R = crate::BitReader;
#[doc = "Field `REN32` writer - Rising edge enabled 32"]
pub type REN32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN33` reader - Rising edge enabled 33"]
pub type REN33_R = crate::BitReader;
#[doc = "Field `REN33` writer - Rising edge enabled 33"]
pub type REN33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN34` reader - Rising edge enabled 34"]
pub type REN34_R = crate::BitReader;
#[doc = "Field `REN34` writer - Rising edge enabled 34"]
pub type REN34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN35` reader - Rising edge enabled 35"]
pub type REN35_R = crate::BitReader;
#[doc = "Field `REN35` writer - Rising edge enabled 35"]
pub type REN35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN36` reader - Rising edge enabled 36"]
pub type REN36_R = crate::BitReader;
#[doc = "Field `REN36` writer - Rising edge enabled 36"]
pub type REN36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN37` reader - Rising edge enabled 37"]
pub type REN37_R = crate::BitReader;
#[doc = "Field `REN37` writer - Rising edge enabled 37"]
pub type REN37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN38` reader - Rising edge enabled 38"]
pub type REN38_R = crate::BitReader;
#[doc = "Field `REN38` writer - Rising edge enabled 38"]
pub type REN38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN39` reader - Rising edge enabled 39"]
pub type REN39_R = crate::BitReader;
#[doc = "Field `REN39` writer - Rising edge enabled 39"]
pub type REN39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN40` reader - Rising edge enabled 40"]
pub type REN40_R = crate::BitReader;
#[doc = "Field `REN40` writer - Rising edge enabled 40"]
pub type REN40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN41` reader - Rising edge enabled 41"]
pub type REN41_R = crate::BitReader;
#[doc = "Field `REN41` writer - Rising edge enabled 41"]
pub type REN41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN42` reader - Rising edge enabled 42"]
pub type REN42_R = crate::BitReader;
#[doc = "Field `REN42` writer - Rising edge enabled 42"]
pub type REN42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN43` reader - Rising edge enabled 43"]
pub type REN43_R = crate::BitReader;
#[doc = "Field `REN43` writer - Rising edge enabled 43"]
pub type REN43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN44` reader - Rising edge enabled 44"]
pub type REN44_R = crate::BitReader;
#[doc = "Field `REN44` writer - Rising edge enabled 44"]
pub type REN44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN45` reader - Rising edge enabled 45"]
pub type REN45_R = crate::BitReader;
#[doc = "Field `REN45` writer - Rising edge enabled 45"]
pub type REN45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN46` reader - Rising edge enabled 46"]
pub type REN46_R = crate::BitReader;
#[doc = "Field `REN46` writer - Rising edge enabled 46"]
pub type REN46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN47` reader - Rising edge enabled 47"]
pub type REN47_R = crate::BitReader;
#[doc = "Field `REN47` writer - Rising edge enabled 47"]
pub type REN47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN48` reader - Rising edge enabled 48"]
pub type REN48_R = crate::BitReader;
#[doc = "Field `REN48` writer - Rising edge enabled 48"]
pub type REN48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN49` reader - Rising edge enabled 49"]
pub type REN49_R = crate::BitReader;
#[doc = "Field `REN49` writer - Rising edge enabled 49"]
pub type REN49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN50` reader - Rising edge enabled 50"]
pub type REN50_R = crate::BitReader;
#[doc = "Field `REN50` writer - Rising edge enabled 50"]
pub type REN50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN51` reader - Rising edge enabled 51"]
pub type REN51_R = crate::BitReader;
#[doc = "Field `REN51` writer - Rising edge enabled 51"]
pub type REN51_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN52` reader - Rising edge enabled 52"]
pub type REN52_R = crate::BitReader;
#[doc = "Field `REN52` writer - Rising edge enabled 52"]
pub type REN52_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN53` reader - Rising edge enabled 53"]
pub type REN53_R = crate::BitReader;
#[doc = "Field `REN53` writer - Rising edge enabled 53"]
pub type REN53_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising edge enabled 32"]
    #[inline(always)]
    pub fn ren32(&self) -> REN32_R {
        REN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising edge enabled 33"]
    #[inline(always)]
    pub fn ren33(&self) -> REN33_R {
        REN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising edge enabled 34"]
    #[inline(always)]
    pub fn ren34(&self) -> REN34_R {
        REN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge enabled 35"]
    #[inline(always)]
    pub fn ren35(&self) -> REN35_R {
        REN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising edge enabled 36"]
    #[inline(always)]
    pub fn ren36(&self) -> REN36_R {
        REN36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising edge enabled 37"]
    #[inline(always)]
    pub fn ren37(&self) -> REN37_R {
        REN37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge enabled 38"]
    #[inline(always)]
    pub fn ren38(&self) -> REN38_R {
        REN38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising edge enabled 39"]
    #[inline(always)]
    pub fn ren39(&self) -> REN39_R {
        REN39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising edge enabled 40"]
    #[inline(always)]
    pub fn ren40(&self) -> REN40_R {
        REN40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge enabled 41"]
    #[inline(always)]
    pub fn ren41(&self) -> REN41_R {
        REN41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising edge enabled 42"]
    #[inline(always)]
    pub fn ren42(&self) -> REN42_R {
        REN42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising edge enabled 43"]
    #[inline(always)]
    pub fn ren43(&self) -> REN43_R {
        REN43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising edge enabled 44"]
    #[inline(always)]
    pub fn ren44(&self) -> REN44_R {
        REN44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising edge enabled 45"]
    #[inline(always)]
    pub fn ren45(&self) -> REN45_R {
        REN45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising edge enabled 46"]
    #[inline(always)]
    pub fn ren46(&self) -> REN46_R {
        REN46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising edge enabled 47"]
    #[inline(always)]
    pub fn ren47(&self) -> REN47_R {
        REN47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising edge enabled 48"]
    #[inline(always)]
    pub fn ren48(&self) -> REN48_R {
        REN48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising edge enabled 49"]
    #[inline(always)]
    pub fn ren49(&self) -> REN49_R {
        REN49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising edge enabled 50"]
    #[inline(always)]
    pub fn ren50(&self) -> REN50_R {
        REN50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising edge enabled 51"]
    #[inline(always)]
    pub fn ren51(&self) -> REN51_R {
        REN51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising edge enabled 52"]
    #[inline(always)]
    pub fn ren52(&self) -> REN52_R {
        REN52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising edge enabled 53"]
    #[inline(always)]
    pub fn ren53(&self) -> REN53_R {
        REN53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPREN1")
            .field("ren32", &format_args!("{}", self.ren32().bit()))
            .field("ren33", &format_args!("{}", self.ren33().bit()))
            .field("ren34", &format_args!("{}", self.ren34().bit()))
            .field("ren35", &format_args!("{}", self.ren35().bit()))
            .field("ren36", &format_args!("{}", self.ren36().bit()))
            .field("ren37", &format_args!("{}", self.ren37().bit()))
            .field("ren38", &format_args!("{}", self.ren38().bit()))
            .field("ren39", &format_args!("{}", self.ren39().bit()))
            .field("ren40", &format_args!("{}", self.ren40().bit()))
            .field("ren41", &format_args!("{}", self.ren41().bit()))
            .field("ren42", &format_args!("{}", self.ren42().bit()))
            .field("ren43", &format_args!("{}", self.ren43().bit()))
            .field("ren44", &format_args!("{}", self.ren44().bit()))
            .field("ren45", &format_args!("{}", self.ren45().bit()))
            .field("ren46", &format_args!("{}", self.ren46().bit()))
            .field("ren47", &format_args!("{}", self.ren47().bit()))
            .field("ren48", &format_args!("{}", self.ren48().bit()))
            .field("ren49", &format_args!("{}", self.ren49().bit()))
            .field("ren50", &format_args!("{}", self.ren50().bit()))
            .field("ren51", &format_args!("{}", self.ren51().bit()))
            .field("ren52", &format_args!("{}", self.ren52().bit()))
            .field("ren53", &format_args!("{}", self.ren53().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPREN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge enabled 32"]
    #[inline(always)]
    #[must_use]
    pub fn ren32(&mut self) -> REN32_W<GPREN1_SPEC> {
        REN32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising edge enabled 33"]
    #[inline(always)]
    #[must_use]
    pub fn ren33(&mut self) -> REN33_W<GPREN1_SPEC> {
        REN33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising edge enabled 34"]
    #[inline(always)]
    #[must_use]
    pub fn ren34(&mut self) -> REN34_W<GPREN1_SPEC> {
        REN34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge enabled 35"]
    #[inline(always)]
    #[must_use]
    pub fn ren35(&mut self) -> REN35_W<GPREN1_SPEC> {
        REN35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising edge enabled 36"]
    #[inline(always)]
    #[must_use]
    pub fn ren36(&mut self) -> REN36_W<GPREN1_SPEC> {
        REN36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising edge enabled 37"]
    #[inline(always)]
    #[must_use]
    pub fn ren37(&mut self) -> REN37_W<GPREN1_SPEC> {
        REN37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising edge enabled 38"]
    #[inline(always)]
    #[must_use]
    pub fn ren38(&mut self) -> REN38_W<GPREN1_SPEC> {
        REN38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising edge enabled 39"]
    #[inline(always)]
    #[must_use]
    pub fn ren39(&mut self) -> REN39_W<GPREN1_SPEC> {
        REN39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising edge enabled 40"]
    #[inline(always)]
    #[must_use]
    pub fn ren40(&mut self) -> REN40_W<GPREN1_SPEC> {
        REN40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising edge enabled 41"]
    #[inline(always)]
    #[must_use]
    pub fn ren41(&mut self) -> REN41_W<GPREN1_SPEC> {
        REN41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising edge enabled 42"]
    #[inline(always)]
    #[must_use]
    pub fn ren42(&mut self) -> REN42_W<GPREN1_SPEC> {
        REN42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising edge enabled 43"]
    #[inline(always)]
    #[must_use]
    pub fn ren43(&mut self) -> REN43_W<GPREN1_SPEC> {
        REN43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising edge enabled 44"]
    #[inline(always)]
    #[must_use]
    pub fn ren44(&mut self) -> REN44_W<GPREN1_SPEC> {
        REN44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising edge enabled 45"]
    #[inline(always)]
    #[must_use]
    pub fn ren45(&mut self) -> REN45_W<GPREN1_SPEC> {
        REN45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising edge enabled 46"]
    #[inline(always)]
    #[must_use]
    pub fn ren46(&mut self) -> REN46_W<GPREN1_SPEC> {
        REN46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising edge enabled 47"]
    #[inline(always)]
    #[must_use]
    pub fn ren47(&mut self) -> REN47_W<GPREN1_SPEC> {
        REN47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising edge enabled 48"]
    #[inline(always)]
    #[must_use]
    pub fn ren48(&mut self) -> REN48_W<GPREN1_SPEC> {
        REN48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising edge enabled 49"]
    #[inline(always)]
    #[must_use]
    pub fn ren49(&mut self) -> REN49_W<GPREN1_SPEC> {
        REN49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising edge enabled 50"]
    #[inline(always)]
    #[must_use]
    pub fn ren50(&mut self) -> REN50_W<GPREN1_SPEC> {
        REN50_W::new(self, 18)
    }
    #[doc = "Bit 19 - Rising edge enabled 51"]
    #[inline(always)]
    #[must_use]
    pub fn ren51(&mut self) -> REN51_W<GPREN1_SPEC> {
        REN51_W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising edge enabled 52"]
    #[inline(always)]
    #[must_use]
    pub fn ren52(&mut self) -> REN52_W<GPREN1_SPEC> {
        REN52_W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising edge enabled 53"]
    #[inline(always)]
    #[must_use]
    pub fn ren53(&mut self) -> REN53_W<GPREN1_SPEC> {
        REN53_W::new(self, 21)
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
#[doc = "GPIO Pin Rising Edge Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpren1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpren1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPREN1_SPEC;
impl crate::RegisterSpec for GPREN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpren1::R`](R) reader structure"]
impl crate::Readable for GPREN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpren1::W`](W) writer structure"]
impl crate::Writable for GPREN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `RNG2` reader"]
pub type R = crate::R<RNG2_SPEC>;
#[doc = "Register `RNG2` writer"]
pub type W = crate::W<RNG2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RNG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "Range for channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG2_SPEC;
impl crate::RegisterSpec for RNG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng2::R`](R) reader structure"]
impl crate::Readable for RNG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng2::W`](W) writer structure"]
impl crate::Writable for RNG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNG2 to value 0x20"]
impl crate::Resettable for RNG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}

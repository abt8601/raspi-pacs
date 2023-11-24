#[doc = "Register `SENDER0` reader"]
pub type R = crate::R<SENDER0_SPEC>;
#[doc = "Register `SENDER0` writer"]
pub type W = crate::W<SENDER0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SENDER0_SPEC> {
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sender0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sender0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENDER0_SPEC;
impl crate::RegisterSpec for SENDER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sender0::R`](R) reader structure"]
impl crate::Readable for SENDER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sender0::W`](W) writer structure"]
impl crate::Writable for SENDER0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

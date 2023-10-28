#[doc = "Register `PEEK0` reader"]
pub type R = crate::R<PEEK0_SPEC>;
#[doc = "Register `PEEK0` writer"]
pub type W = crate::W<PEEK0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PEEK0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEEK0_SPEC;
impl crate::RegisterSpec for PEEK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek0::R`](R) reader structure"]
impl crate::Readable for PEEK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peek0::W`](W) writer structure"]
impl crate::Writable for PEEK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `GICD_SGIR` writer"]
pub type W = crate::W<GICD_SGIR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<GICD_SGIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
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
#[doc = "Software Generated Interrupt Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_sgir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SGIR_SPEC;
impl crate::RegisterSpec for GICD_SGIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicd_sgir::W`](W) writer structure"]
impl crate::Writable for GICD_SGIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

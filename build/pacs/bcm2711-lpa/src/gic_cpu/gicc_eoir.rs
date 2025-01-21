#[doc = "Register `GICC_EOIR` writer"]
pub type W = crate::W<GICC_EOIR_SPEC>;
#[doc = "Field `INTERRUPT_ID` writer - Interrupt ID"]
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl core::fmt::Debug for crate::generic::Reg<GICC_EOIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:9 - Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICC_EOIR_SPEC> {
        INTERRUPT_ID_W::new(self, 0)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_EOIR_SPEC> {
        CPUID_W::new(self, 10)
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
#[doc = "End of Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_eoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_EOIR_SPEC;
impl crate::RegisterSpec for GICC_EOIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicc_eoir::W`](W) writer structure"]
impl crate::Writable for GICC_EOIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_EOIR to value 0"]
impl crate::Resettable for GICC_EOIR_SPEC {
    const RESET_VALUE: u32 = 0;
}

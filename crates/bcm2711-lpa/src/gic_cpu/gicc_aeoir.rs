#[doc = "Register `GICC_AEOIR` writer"]
pub type W = crate::W<GICC_AEOIR_SPEC>;
#[doc = "Field `INTERRUPT_ID` writer - Interrupt ID"]
pub type INTERRUPT_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl core::fmt::Debug for crate::generic::Reg<GICC_AEOIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:9 - Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICC_AEOIR_SPEC, 0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_AEOIR_SPEC, 10> {
        CPUID_W::new(self)
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
#[doc = "Aliased End of Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_aeoir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_AEOIR_SPEC;
impl crate::RegisterSpec for GICC_AEOIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicc_aeoir::W`](W) writer structure"]
impl crate::Writable for GICC_AEOIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_AEOIR to value 0"]
impl crate::Resettable for GICC_AEOIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

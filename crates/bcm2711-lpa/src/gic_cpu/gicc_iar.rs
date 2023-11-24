#[doc = "Register `GICC_IAR` reader"]
pub type R = crate::R<GICC_IAR_SPEC>;
#[doc = "Field `INTERRUPT_ID` reader - Interrupt ID"]
pub type INTERRUPT_ID_R = crate::FieldReader<u16>;
#[doc = "Field `CPUID` reader - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_IAR")
            .field("cpuid", &format_args!("{}", self.cpuid().bits()))
            .field(
                "interrupt_id",
                &format_args!("{}", self.interrupt_id().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_IAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_iar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_IAR_SPEC;
impl crate::RegisterSpec for GICC_IAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_iar::R`](R) reader structure"]
impl crate::Readable for GICC_IAR_SPEC {}
#[doc = "`reset()` method sets GICC_IAR to value 0"]
impl crate::Resettable for GICC_IAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

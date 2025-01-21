#[doc = "Register `GICC_HPPIR` reader"]
pub type R = crate::R<GICC_HPPIR_SPEC>;
#[doc = "Register `GICC_HPPIR` writer"]
pub type W = crate::W<GICC_HPPIR_SPEC>;
#[doc = "Field `INTERRUPT_ID` reader - Pending Interrupt ID"]
pub type INTERRUPT_ID_R = crate::FieldReader<u16>;
#[doc = "Field `INTERRUPT_ID` writer - Pending Interrupt ID"]
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` reader - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_R = crate::FieldReader;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
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
        f.debug_struct("GICC_HPPIR")
            .field("cpuid", &format_args!("{}", self.cpuid().bits()))
            .field(
                "interrupt_id",
                &format_args!("{}", self.interrupt_id().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_HPPIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICC_HPPIR_SPEC> {
        INTERRUPT_ID_W::new(self, 0)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_HPPIR_SPEC> {
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
#[doc = "Highest Priority Pending Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_hppir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_hppir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_HPPIR_SPEC;
impl crate::RegisterSpec for GICC_HPPIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_hppir::R`](R) reader structure"]
impl crate::Readable for GICC_HPPIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicc_hppir::W`](W) writer structure"]
impl crate::Writable for GICC_HPPIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_HPPIR to value 0"]
impl crate::Resettable for GICC_HPPIR_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICC_PMR` reader"]
pub type R = crate::R<GICC_PMR_SPEC>;
#[doc = "Register `GICC_PMR` writer"]
pub type W = crate::W<GICC_PMR_SPEC>;
#[doc = "Field `PRIORITY` reader - Interrupts with a higher number are not signaled"]
pub type PRIORITY_R = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - Interrupts with a higher number are not signaled"]
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupts with a higher number are not signaled"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_PMR")
            .field("priority", &format_args!("{}", self.priority().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_PMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupts with a higher number are not signaled"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICC_PMR_SPEC> {
        PRIORITY_W::new(self, 0)
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
#[doc = "Interrupt Priority Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_pmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_pmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_PMR_SPEC;
impl crate::RegisterSpec for GICC_PMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_pmr::R`](R) reader structure"]
impl crate::Readable for GICC_PMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicc_pmr::W`](W) writer structure"]
impl crate::Writable for GICC_PMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_PMR to value 0"]
impl crate::Resettable for GICC_PMR_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `TUNE_STEPS_STD` reader"]
pub type R = crate::R<TUNE_STEPS_STD_SPEC>;
#[doc = "Register `TUNE_STEPS_STD` writer"]
pub type W = crate::W<TUNE_STEPS_STD_SPEC>;
#[doc = "Field `STEPS` reader - "]
pub type STEPS_R = crate::FieldReader;
#[doc = "Field `STEPS` writer - "]
pub type STEPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE_STEPS_STD")
            .field("steps", &format_args!("{}", self.steps().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TUNE_STEPS_STD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn steps(&mut self) -> STEPS_W<TUNE_STEPS_STD_SPEC, 0> {
        STEPS_W::new(self)
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
#[doc = "Sample clock delay step count for SDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune_steps_std::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune_steps_std::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TUNE_STEPS_STD_SPEC;
impl crate::RegisterSpec for TUNE_STEPS_STD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tune_steps_std::R`](R) reader structure"]
impl crate::Readable for TUNE_STEPS_STD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tune_steps_std::W`](W) writer structure"]
impl crate::Writable for TUNE_STEPS_STD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNE_STEPS_STD to value 0"]
impl crate::Resettable for TUNE_STEPS_STD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

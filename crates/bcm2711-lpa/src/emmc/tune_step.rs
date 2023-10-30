#[doc = "Register `TUNE_STEP` reader"]
pub type R = crate::R<TUNE_STEP_SPEC>;
#[doc = "Register `TUNE_STEP` writer"]
pub type W = crate::W<TUNE_STEP_SPEC>;
#[doc = "Field `DELAY` reader - "]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - "]
pub type DELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE_STEP")
            .field("delay", &format_args!("{}", self.delay().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TUNE_STEP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<TUNE_STEP_SPEC, 0> {
        DELAY_W::new(self)
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
#[doc = "Sample clock delay step duration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune_step::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune_step::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TUNE_STEP_SPEC;
impl crate::RegisterSpec for TUNE_STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tune_step::R`](R) reader structure"]
impl crate::Readable for TUNE_STEP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tune_step::W`](W) writer structure"]
impl crate::Writable for TUNE_STEP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNE_STEP to value 0"]
impl crate::Resettable for TUNE_STEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

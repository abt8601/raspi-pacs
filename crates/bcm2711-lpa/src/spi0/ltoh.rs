#[doc = "Register `LTOH` reader"]
pub type R = crate::R<LTOH_SPEC>;
#[doc = "Register `LTOH` writer"]
pub type W = crate::W<LTOH_SPEC>;
#[doc = "Field `TOH` reader - Output hold delay"]
pub type TOH_R = crate::FieldReader;
#[doc = "Field `TOH` writer - Output hold delay"]
pub type TOH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    pub fn toh(&self) -> TOH_R {
        TOH_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTOH")
            .field("toh", &format_args!("{}", self.toh().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LTOH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    #[must_use]
    pub fn toh(&mut self) -> TOH_W<LTOH_SPEC> {
        TOH_W::new(self, 0)
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
#[doc = "LoSSI output hold delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltoh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltoh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTOH_SPEC;
impl crate::RegisterSpec for LTOH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltoh::R`](R) reader structure"]
impl crate::Readable for LTOH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ltoh::W`](W) writer structure"]
impl crate::Writable for LTOH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTOH to value 0x01"]
impl crate::Resettable for LTOH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

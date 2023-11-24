#[doc = "Register `DLEN` reader"]
pub type R = crate::R<DLEN_SPEC>;
#[doc = "Register `DLEN` writer"]
pub type W = crate::W<DLEN_SPEC>;
#[doc = "Field `DLEN` reader - Data length"]
pub type DLEN_R = crate::FieldReader<u16>;
#[doc = "Field `DLEN` writer - Data length"]
pub type DLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    pub fn dlen(&self) -> DLEN_R {
        DLEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLEN")
            .field("dlen", &format_args!("{}", self.dlen().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dlen(&mut self) -> DLEN_W<DLEN_SPEC> {
        DLEN_W::new(self, 0)
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
#[doc = "Data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLEN_SPEC;
impl crate::RegisterSpec for DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlen::R`](R) reader structure"]
impl crate::Readable for DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlen::W`](W) writer structure"]
impl crate::Writable for DLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

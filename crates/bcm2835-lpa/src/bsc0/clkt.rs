#[doc = "Register `CLKT` reader"]
pub type R = crate::R<CLKT_SPEC>;
#[doc = "Register `CLKT` writer"]
pub type W = crate::W<CLKT_SPEC>;
#[doc = "Field `TOUT` reader - Number of SCL clock cycles to wait"]
pub type TOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TOUT` writer - Number of SCL clock cycles to wait"]
pub type TOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKT")
            .field("tout", &format_args!("{}", self.tout().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLKT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<CLKT_SPEC> {
        TOUT_W::new(self, 0)
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
#[doc = "Clock stretch timeout (broken on 283x)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKT_SPEC;
impl crate::RegisterSpec for CLKT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkt::R`](R) reader structure"]
impl crate::Readable for CLKT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkt::W`](W) writer structure"]
impl crate::Writable for CLKT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKT to value 0"]
impl crate::Resettable for CLKT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

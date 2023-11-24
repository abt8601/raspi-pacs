#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DMACR_SPEC>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DMACR_SPEC>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TXDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAONERR` reader - DMAONERR"]
pub type DMAONERR_R = crate::BitReader;
#[doc = "Field `DMAONERR` writer - DMAONERR"]
pub type DMAONERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACR")
            .field("rxdmae", &format_args!("{}", self.rxdmae().bit()))
            .field("txdmae", &format_args!("{}", self.txdmae().bit()))
            .field("dmaonerr", &format_args!("{}", self.dmaonerr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMACR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<DMACR_SPEC> {
        RXDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<DMACR_SPEC> {
        TXDMAE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DMAONERR_W<DMACR_SPEC> {
        DMAONERR_W::new(self, 2)
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
#[doc = "DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

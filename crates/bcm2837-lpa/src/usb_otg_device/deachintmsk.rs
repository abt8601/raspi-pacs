#[doc = "Register `DEACHINTMSK` reader"]
pub type R = crate::R<DEACHINTMSK_SPEC>;
#[doc = "Register `DEACHINTMSK` writer"]
pub type W = crate::W<DEACHINTMSK_SPEC>;
#[doc = "Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_R = crate::BitReader;
#[doc = "Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_R = crate::BitReader;
#[doc = "Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINTMSK")
            .field("iep1intm", &format_args!("{}", self.iep1intm().bit()))
            .field("oep1intm", &format_args!("{}", self.oep1intm().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEACHINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<DEACHINTMSK_SPEC> {
        IEP1INTM_W::new(self, 1)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<DEACHINTMSK_SPEC> {
        OEP1INTM_W::new(self, 17)
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
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINTMSK_SPEC;
impl crate::RegisterSpec for DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachintmsk::R`](R) reader structure"]
impl crate::Readable for DEACHINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure"]
impl crate::Writable for DEACHINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DEACHINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

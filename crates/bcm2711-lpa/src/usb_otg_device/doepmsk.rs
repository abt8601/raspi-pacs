#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DOEPMSK_SPEC>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DOEPMSK_SPEC>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub type STUPM_R = crate::BitReader;
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub type STUPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub type OTEPDM_R = crate::BitReader;
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub type OTEPDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received mask"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received mask"]
pub type B2BSTUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPEM` reader - OUT packet error mask"]
pub type OPEM_R = crate::BitReader;
#[doc = "Field `OPEM` writer - OUT packet error mask"]
pub type OPEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOIM` reader - BNA interrupt mask"]
pub type BOIM_R = crate::BitReader;
#[doc = "Field `BOIM` writer - BNA interrupt mask"]
pub type BOIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn opem(&self) -> OPEM_R {
        OPEM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(&self) -> BOIM_R {
        BOIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfrcm", &format_args!("{}", self.xfrcm().bit()))
            .field("epdm", &format_args!("{}", self.epdm().bit()))
            .field("stupm", &format_args!("{}", self.stupm().bit()))
            .field("otepdm", &format_args!("{}", self.otepdm().bit()))
            .field("b2bstup", &format_args!("{}", self.b2bstup().bit()))
            .field("opem", &format_args!("{}", self.opem().bit()))
            .field("boim", &format_args!("{}", self.boim().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DOEPMSK_SPEC, 0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DOEPMSK_SPEC, 1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<DOEPMSK_SPEC, 3> {
        STUPM_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<DOEPMSK_SPEC, 4> {
        OTEPDM_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPMSK_SPEC, 6> {
        B2BSTUP_W::new(self)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn opem(&mut self) -> OPEM_W<DOEPMSK_SPEC, 8> {
        OPEM_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn boim(&mut self) -> BOIM_W<DOEPMSK_SPEC, 9> {
        BOIM_W::new(self)
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
#[doc = "OTG_HS device OUT endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

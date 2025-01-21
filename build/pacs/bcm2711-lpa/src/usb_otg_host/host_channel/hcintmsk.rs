#[doc = "Register `HCINTMSK` reader"]
pub type R = crate::R<HCINTMSK_SPEC>;
#[doc = "Register `HCINTMSK` writer"]
pub type W = crate::W<HCINTMSK_SPEC>;
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type CHHM_R = crate::BitReader;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub type STALLM_R = crate::BitReader;
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type ACKM_R = crate::BitReader;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub type TXERRM_R = crate::BitReader;
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub type BBERRM_R = crate::BitReader;
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FRMORM_R = crate::BitReader;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DTERRM_R = crate::BitReader;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK")
            .field("xfrcm", &format_args!("{}", self.xfrcm().bit()))
            .field("chhm", &format_args!("{}", self.chhm().bit()))
            .field("ahberr", &format_args!("{}", self.ahberr().bit()))
            .field("stallm", &format_args!("{}", self.stallm().bit()))
            .field("nakm", &format_args!("{}", self.nakm().bit()))
            .field("ackm", &format_args!("{}", self.ackm().bit()))
            .field("nyet", &format_args!("{}", self.nyet().bit()))
            .field("txerrm", &format_args!("{}", self.txerrm().bit()))
            .field("bberrm", &format_args!("{}", self.bberrm().bit()))
            .field("frmorm", &format_args!("{}", self.frmorm().bit()))
            .field("dterrm", &format_args!("{}", self.dterrm().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<HCINTMSK_SPEC> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhm(&mut self) -> CHHM_W<HCINTMSK_SPEC> {
        CHHM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<HCINTMSK_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallm(&mut self) -> STALLM_W<HCINTMSK_SPEC> {
        STALLM_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<HCINTMSK_SPEC> {
        NAKM_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackm(&mut self) -> ACKM_W<HCINTMSK_SPEC> {
        ACKM_W::new(self, 5)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<HCINTMSK_SPEC> {
        NYET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    #[must_use]
    pub fn txerrm(&mut self) -> TXERRM_W<HCINTMSK_SPEC> {
        TXERRM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    #[must_use]
    pub fn bberrm(&mut self) -> BBERRM_W<HCINTMSK_SPEC> {
        BBERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmorm(&mut self) -> FRMORM_W<HCINTMSK_SPEC> {
        FRMORM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    #[must_use]
    pub fn dterrm(&mut self) -> DTERRM_W<HCINTMSK_SPEC> {
        DTERRM_W::new(self, 10)
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
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK_SPEC;
impl crate::RegisterSpec for HCINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk::R`](R) reader structure"]
impl crate::Readable for HCINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk::W`](W) writer structure"]
impl crate::Writable for HCINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK to value 0"]
impl crate::Resettable for HCINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}

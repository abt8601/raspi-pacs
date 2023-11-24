#[doc = "Register `DOEPEACHMSK1` reader"]
pub type R = crate::R<DOEPEACHMSK1_SPEC>;
#[doc = "Register `DOEPEACHMSK1` writer"]
pub type W = crate::W<DOEPEACHMSK1_SPEC>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - Timeout condition mask"]
pub type TOM_R = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask"]
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_R = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type INEPNMM_R = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type INEPNMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type INEPNEM_R = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFURM` reader - OUT packet error mask"]
pub type TXFURM_R = crate::BitReader;
#[doc = "Field `TXFURM` writer - OUT packet error mask"]
pub type TXFURM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BIM_R = crate::BitReader;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRM` reader - Bubble error interrupt mask"]
pub type BERRM_R = crate::BitReader;
#[doc = "Field `BERRM` writer - Bubble error interrupt mask"]
pub type BERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK interrupt mask"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK interrupt mask"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETM` reader - NYET interrupt mask"]
pub type NYETM_R = crate::BitReader;
#[doc = "Field `NYETM` writer - NYET interrupt mask"]
pub type NYETM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&self) -> NYETM_R {
        NYETM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPEACHMSK1")
            .field("xfrcm", &format_args!("{}", self.xfrcm().bit()))
            .field("epdm", &format_args!("{}", self.epdm().bit()))
            .field("tom", &format_args!("{}", self.tom().bit()))
            .field("ittxfemsk", &format_args!("{}", self.ittxfemsk().bit()))
            .field("inepnmm", &format_args!("{}", self.inepnmm().bit()))
            .field("inepnem", &format_args!("{}", self.inepnem().bit()))
            .field("txfurm", &format_args!("{}", self.txfurm().bit()))
            .field("bim", &format_args!("{}", self.bim().bit()))
            .field("berrm", &format_args!("{}", self.berrm().bit()))
            .field("nakm", &format_args!("{}", self.nakm().bit()))
            .field("nyetm", &format_args!("{}", self.nyetm().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPEACHMSK1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DOEPEACHMSK1_SPEC> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DOEPEACHMSK1_SPEC> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<DOEPEACHMSK1_SPEC> {
        TOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<DOEPEACHMSK1_SPEC> {
        ITTXFEMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnmm(&mut self) -> INEPNMM_W<DOEPEACHMSK1_SPEC> {
        INEPNMM_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<DOEPEACHMSK1_SPEC> {
        INEPNEM_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TXFURM_W<DOEPEACHMSK1_SPEC> {
        TXFURM_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bim(&mut self) -> BIM_W<DOEPEACHMSK1_SPEC> {
        BIM_W::new(self, 9)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BERRM_W<DOEPEACHMSK1_SPEC> {
        BERRM_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<DOEPEACHMSK1_SPEC> {
        NAKM_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetm(&mut self) -> NYETM_W<DOEPEACHMSK1_SPEC> {
        NYETM_W::new(self, 14)
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
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPEACHMSK1_SPEC;
impl crate::RegisterSpec for DOEPEACHMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepeachmsk1::R`](R) reader structure"]
impl crate::Readable for DOEPEACHMSK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepeachmsk1::W`](W) writer structure"]
impl crate::Writable for DOEPEACHMSK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for DOEPEACHMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DIEPMSK_SPEC>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DIEPMSK_SPEC>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOM` reader - Timeout condition mask (nonisochronous endpoints)"]
pub type TOM_R = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask (nonisochronous endpoints)"]
pub type TOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_R = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type INEPNMM_R = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type INEPNMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type INEPNEM_R = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type INEPNEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFURM` reader - FIFO underrun mask"]
pub type TXFURM_R = crate::BitReader;
#[doc = "Field `TXFURM` writer - FIFO underrun mask"]
pub type TXFURM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BIM_R = crate::BitReader;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
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
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfrcm", &format_args!("{}", self.xfrcm().bit()))
            .field("epdm", &format_args!("{}", self.epdm().bit()))
            .field("tom", &format_args!("{}", self.tom().bit()))
            .field("ittxfemsk", &format_args!("{}", self.ittxfemsk().bit()))
            .field("inepnmm", &format_args!("{}", self.inepnmm().bit()))
            .field("inepnem", &format_args!("{}", self.inepnem().bit()))
            .field("txfurm", &format_args!("{}", self.txfurm().bit()))
            .field("bim", &format_args!("{}", self.bim().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DIEPMSK_SPEC, 0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DIEPMSK_SPEC, 1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<DIEPMSK_SPEC, 3> {
        TOM_W::new(self)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<DIEPMSK_SPEC, 4> {
        ITTXFEMSK_W::new(self)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnmm(&mut self) -> INEPNMM_W<DIEPMSK_SPEC, 5> {
        INEPNMM_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<DIEPMSK_SPEC, 6> {
        INEPNEM_W::new(self)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TXFURM_W<DIEPMSK_SPEC, 8> {
        TXFURM_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bim(&mut self) -> BIM_W<DIEPMSK_SPEC, 9> {
        BIM_W::new(self)
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
#[doc = "OTG_HS device IN endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

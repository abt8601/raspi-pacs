#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GINTSTS_SPEC>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GINTSTS_SPEC>;
#[doc = "Field `CMOD` reader - Current mode of operation"]
pub type CMOD_R = crate::BitReader;
#[doc = "Field `MMIS` reader - Mode mismatch interrupt"]
pub type MMIS_R = crate::BitReader;
#[doc = "Field `MMIS` writer - Mode mismatch interrupt"]
pub type MMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO nonempty"]
pub type RXFLVL_R = crate::BitReader;
#[doc = "Field `NPTXFE` reader - Nonperiodic TxFIFO empty"]
pub type NPTXFE_R = crate::BitReader;
#[doc = "Field `GINAKEFF` reader - Global IN nonperiodic NAK effective"]
pub type GINAKEFF_R = crate::BitReader;
#[doc = "Field `BOUTNAKEFF` reader - Global OUT NAK effective"]
pub type BOUTNAKEFF_R = crate::BitReader;
#[doc = "Field `ESUSP` reader - Early suspend"]
pub type ESUSP_R = crate::BitReader;
#[doc = "Field `ESUSP` writer - Early suspend"]
pub type ESUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDNE` reader - Enumeration done"]
pub type ENUMDNE_R = crate::BitReader;
#[doc = "Field `ENUMDNE` writer - Enumeration done"]
pub type ENUMDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt"]
pub type ISOODRP_R = crate::BitReader;
#[doc = "Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt"]
pub type ISOODRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `IISOIXFR` reader - Incomplete isochronous IN transfer"]
pub type IISOIXFR_R = crate::BitReader;
#[doc = "Field `IISOIXFR` writer - Incomplete isochronous IN transfer"]
pub type IISOIXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXFR_INCOMPISOOUT` reader - Incomplete periodic transfer"]
pub type PXFR_INCOMPISOOUT_R = crate::BitReader;
#[doc = "Field `PXFR_INCOMPISOOUT` writer - Incomplete periodic transfer"]
pub type PXFR_INCOMPISOOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAFSUSP` reader - Data fetch suspended"]
pub type DATAFSUSP_R = crate::BitReader;
#[doc = "Field `DATAFSUSP` writer - Data fetch suspended"]
pub type DATAFSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPRTINT` reader - Host port interrupt"]
pub type HPRTINT_R = crate::BitReader;
#[doc = "Field `HCINT` reader - Host channels interrupt"]
pub type HCINT_R = crate::BitReader;
#[doc = "Field `PTXFE` reader - Periodic TxFIFO empty"]
pub type PTXFE_R = crate::BitReader;
#[doc = "Field `CIDSCHG` reader - Connector ID status change"]
pub type CIDSCHG_R = crate::BitReader;
#[doc = "Field `CIDSCHG` writer - Connector ID status change"]
pub type CIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt"]
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRQINT` reader - Session request/new session detected interrupt"]
pub type SRQINT_R = crate::BitReader;
#[doc = "Field `SRQINT` writer - Session request/new session detected interrupt"]
pub type SRQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUINT` reader - Resume/remote wakeup detected interrupt"]
pub type WKUINT_R = crate::BitReader;
#[doc = "Field `WKUINT` writer - Resume/remote wakeup detected interrupt"]
pub type WKUINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO nonempty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Nonperiodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN nonperiodic NAK effective"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn boutnakeff(&self) -> BOUTNAKEFF_R {
        BOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(&self) -> PXFR_INCOMPISOOUT_R {
        PXFR_INCOMPISOOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkuint(&self) -> WKUINT_R {
        WKUINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("cmod", &format_args!("{}", self.cmod().bit()))
            .field("mmis", &format_args!("{}", self.mmis().bit()))
            .field("otgint", &format_args!("{}", self.otgint().bit()))
            .field("sof", &format_args!("{}", self.sof().bit()))
            .field("rxflvl", &format_args!("{}", self.rxflvl().bit()))
            .field("nptxfe", &format_args!("{}", self.nptxfe().bit()))
            .field("ginakeff", &format_args!("{}", self.ginakeff().bit()))
            .field("boutnakeff", &format_args!("{}", self.boutnakeff().bit()))
            .field("esusp", &format_args!("{}", self.esusp().bit()))
            .field("usbsusp", &format_args!("{}", self.usbsusp().bit()))
            .field("usbrst", &format_args!("{}", self.usbrst().bit()))
            .field("enumdne", &format_args!("{}", self.enumdne().bit()))
            .field("isoodrp", &format_args!("{}", self.isoodrp().bit()))
            .field("eopf", &format_args!("{}", self.eopf().bit()))
            .field("iepint", &format_args!("{}", self.iepint().bit()))
            .field("oepint", &format_args!("{}", self.oepint().bit()))
            .field("iisoixfr", &format_args!("{}", self.iisoixfr().bit()))
            .field(
                "pxfr_incompisoout",
                &format_args!("{}", self.pxfr_incompisoout().bit()),
            )
            .field("datafsusp", &format_args!("{}", self.datafsusp().bit()))
            .field("hprtint", &format_args!("{}", self.hprtint().bit()))
            .field("hcint", &format_args!("{}", self.hcint().bit()))
            .field("ptxfe", &format_args!("{}", self.ptxfe().bit()))
            .field("cidschg", &format_args!("{}", self.cidschg().bit()))
            .field("discint", &format_args!("{}", self.discint().bit()))
            .field("srqint", &format_args!("{}", self.srqint().bit()))
            .field("wkuint", &format_args!("{}", self.wkuint().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GINTSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mmis(&mut self) -> MMIS_W<GINTSTS_SPEC> {
        MMIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    #[must_use]
    pub fn esusp(&mut self) -> ESUSP_W<GINTSTS_SPEC> {
        ESUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTS_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTS_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    #[must_use]
    pub fn enumdne(&mut self) -> ENUMDNE_W<GINTSTS_SPEC> {
        ENUMDNE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrp(&mut self) -> ISOODRP_W<GINTSTS_SPEC> {
        ISOODRP_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTS_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<GINTSTS_SPEC> {
        IISOIXFR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pxfr_incompisoout(&mut self) -> PXFR_INCOMPISOOUT_W<GINTSTS_SPEC> {
        PXFR_INCOMPISOOUT_W::new(self, 21)
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    #[must_use]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<GINTSTS_SPEC> {
        DATAFSUSP_W::new(self, 22)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    #[must_use]
    pub fn cidschg(&mut self) -> CIDSCHG_W<GINTSTS_SPEC> {
        CIDSCHG_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<GINTSTS_SPEC> {
        DISCINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn srqint(&mut self) -> SRQINT_W<GINTSTS_SPEC> {
        SRQINT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkuint(&mut self) -> WKUINT_W<GINTSTS_SPEC> {
        WKUINT_W::new(self, 31)
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
#[doc = "OTG_HS core interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0020;
}

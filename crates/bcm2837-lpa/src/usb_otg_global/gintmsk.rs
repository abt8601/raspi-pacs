#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSK_SPEC>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSK_SPEC>;
#[doc = "Field `MMISM` reader - Mode mismatch interrupt mask"]
pub type MMISM_R = crate::BitReader;
#[doc = "Field `MMISM` writer - Mode mismatch interrupt mask"]
pub type MMISM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGINT` reader - OTG interrupt mask"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `OTGINT` writer - OTG interrupt mask"]
pub type OTGINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFM` reader - Start of frame mask"]
pub type SOFM_R = crate::BitReader;
#[doc = "Field `SOFM` writer - Start of frame mask"]
pub type SOFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFLVLM` reader - Receive FIFO nonempty mask"]
pub type RXFLVLM_R = crate::BitReader;
#[doc = "Field `RXFLVLM` writer - Receive FIFO nonempty mask"]
pub type RXFLVLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NPTXFEM` reader - Nonperiodic TxFIFO empty mask"]
pub type NPTXFEM_R = crate::BitReader;
#[doc = "Field `NPTXFEM` writer - Nonperiodic TxFIFO empty mask"]
pub type NPTXFEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GINAKEFFM` reader - Global nonperiodic IN NAK effective mask"]
pub type GINAKEFFM_R = crate::BitReader;
#[doc = "Field `GINAKEFFM` writer - Global nonperiodic IN NAK effective mask"]
pub type GINAKEFFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GONAKEFFM` reader - Global OUT NAK effective mask"]
pub type GONAKEFFM_R = crate::BitReader;
#[doc = "Field `GONAKEFFM` writer - Global OUT NAK effective mask"]
pub type GONAKEFFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESUSPM` reader - Early suspend mask"]
pub type ESUSPM_R = crate::BitReader;
#[doc = "Field `ESUSPM` writer - Early suspend mask"]
pub type ESUSPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBSUSPM` reader - USB suspend mask"]
pub type USBSUSPM_R = crate::BitReader;
#[doc = "Field `USBSUSPM` writer - USB suspend mask"]
pub type USBSUSPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBRST` reader - USB reset mask"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset mask"]
pub type USBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENUMDNEM` reader - Enumeration done mask"]
pub type ENUMDNEM_R = crate::BitReader;
#[doc = "Field `ENUMDNEM` writer - Enumeration done mask"]
pub type ENUMDNEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_R = crate::BitReader;
#[doc = "Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask"]
pub type ISOODRPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPFM` reader - End of periodic frame interrupt mask"]
pub type EOPFM_R = crate::BitReader;
#[doc = "Field `EOPFM` writer - End of periodic frame interrupt mask"]
pub type EOPFM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPMISM` reader - Endpoint mismatch interrupt mask"]
pub type EPMISM_R = crate::BitReader;
#[doc = "Field `EPMISM` writer - Endpoint mismatch interrupt mask"]
pub type EPMISM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IEPINT` reader - IN endpoints interrupt mask"]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `IEPINT` writer - IN endpoints interrupt mask"]
pub type IEPINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEPINT` reader - OUT endpoints interrupt mask"]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` writer - OUT endpoints interrupt mask"]
pub type OEPINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_R = crate::BitReader;
#[doc = "Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask"]
pub type IISOIXFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PXFRM_IISOOXFRM` reader - Incomplete periodic transfer mask"]
pub type PXFRM_IISOOXFRM_R = crate::BitReader;
#[doc = "Field `PXFRM_IISOOXFRM` writer - Incomplete periodic transfer mask"]
pub type PXFRM_IISOOXFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FSUSPM` reader - Data fetch suspended mask"]
pub type FSUSPM_R = crate::BitReader;
#[doc = "Field `FSUSPM` writer - Data fetch suspended mask"]
pub type FSUSPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRTIM` reader - Host port interrupt mask"]
pub type PRTIM_R = crate::BitReader;
#[doc = "Field `HCIM` reader - Host channels interrupt mask"]
pub type HCIM_R = crate::BitReader;
#[doc = "Field `HCIM` writer - Host channels interrupt mask"]
pub type HCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTXFEM` reader - Periodic TxFIFO empty mask"]
pub type PTXFEM_R = crate::BitReader;
#[doc = "Field `PTXFEM` writer - Periodic TxFIFO empty mask"]
pub type PTXFEM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CIDSCHGM` reader - Connector ID status change mask"]
pub type CIDSCHGM_R = crate::BitReader;
#[doc = "Field `CIDSCHGM` writer - Connector ID status change mask"]
pub type CIDSCHGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt mask"]
pub type DISCINT_R = crate::BitReader;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt mask"]
pub type DISCINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRQIM` reader - Session request/new session detected interrupt mask"]
pub type SRQIM_R = crate::BitReader;
#[doc = "Field `SRQIM` writer - Session request/new session detected interrupt mask"]
pub type SRQIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUIM` reader - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_R = crate::BitReader;
#[doc = "Field `WUIM` writer - Resume/remote wakeup detected interrupt mask"]
pub type WUIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO nonempty mask"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Nonperiodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global nonperiodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&self) -> EPMISM_R {
        EPMISM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask"]
    #[inline(always)]
    pub fn pxfrm_iisooxfrm(&self) -> PXFRM_IISOOXFRM_R {
        PXFRM_IISOOXFRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data fetch suspended mask"]
    #[inline(always)]
    pub fn fsuspm(&self) -> FSUSPM_R {
        FSUSPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("mmism", &format_args!("{}", self.mmism().bit()))
            .field("otgint", &format_args!("{}", self.otgint().bit()))
            .field("sofm", &format_args!("{}", self.sofm().bit()))
            .field("rxflvlm", &format_args!("{}", self.rxflvlm().bit()))
            .field("nptxfem", &format_args!("{}", self.nptxfem().bit()))
            .field("ginakeffm", &format_args!("{}", self.ginakeffm().bit()))
            .field("gonakeffm", &format_args!("{}", self.gonakeffm().bit()))
            .field("esuspm", &format_args!("{}", self.esuspm().bit()))
            .field("usbsuspm", &format_args!("{}", self.usbsuspm().bit()))
            .field("usbrst", &format_args!("{}", self.usbrst().bit()))
            .field("enumdnem", &format_args!("{}", self.enumdnem().bit()))
            .field("isoodrpm", &format_args!("{}", self.isoodrpm().bit()))
            .field("eopfm", &format_args!("{}", self.eopfm().bit()))
            .field("epmism", &format_args!("{}", self.epmism().bit()))
            .field("iepint", &format_args!("{}", self.iepint().bit()))
            .field("oepint", &format_args!("{}", self.oepint().bit()))
            .field("iisoixfrm", &format_args!("{}", self.iisoixfrm().bit()))
            .field(
                "pxfrm_iisooxfrm",
                &format_args!("{}", self.pxfrm_iisooxfrm().bit()),
            )
            .field("fsuspm", &format_args!("{}", self.fsuspm().bit()))
            .field("prtim", &format_args!("{}", self.prtim().bit()))
            .field("hcim", &format_args!("{}", self.hcim().bit()))
            .field("ptxfem", &format_args!("{}", self.ptxfem().bit()))
            .field("cidschgm", &format_args!("{}", self.cidschgm().bit()))
            .field("discint", &format_args!("{}", self.discint().bit()))
            .field("srqim", &format_args!("{}", self.srqim().bit()))
            .field("wuim", &format_args!("{}", self.wuim().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn mmism(&mut self) -> MMISM_W<GINTMSK_SPEC, 1> {
        MMISM_W::new(self)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgint(&mut self) -> OTGINT_W<GINTMSK_SPEC, 2> {
        OTGINT_W::new(self)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<GINTMSK_SPEC, 3> {
        SOFM_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO nonempty mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W<GINTMSK_SPEC, 4> {
        RXFLVLM_W::new(self)
    }
    #[doc = "Bit 5 - Nonperiodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfem(&mut self) -> NPTXFEM_W<GINTMSK_SPEC, 5> {
        NPTXFEM_W::new(self)
    }
    #[doc = "Bit 6 - Global nonperiodic IN NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<GINTMSK_SPEC, 6> {
        GINAKEFFM_W::new(self)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W<GINTMSK_SPEC, 7> {
        GONAKEFFM_W::new(self)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn esuspm(&mut self) -> ESUSPM_W<GINTMSK_SPEC, 10> {
        ESUSPM_W::new(self)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<GINTMSK_SPEC, 11> {
        USBSUSPM_W::new(self)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTMSK_SPEC, 12> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    #[must_use]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<GINTMSK_SPEC, 13> {
        ENUMDNEM_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<GINTMSK_SPEC, 14> {
        ISOODRPM_W::new(self)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfm(&mut self) -> EOPFM_W<GINTMSK_SPEC, 15> {
        EOPFM_W::new(self)
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epmism(&mut self) -> EPMISM_W<GINTMSK_SPEC, 17> {
        EPMISM_W::new(self)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn iepint(&mut self) -> IEPINT_W<GINTMSK_SPEC, 18> {
        IEPINT_W::new(self)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<GINTMSK_SPEC, 19> {
        OEPINT_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    #[must_use]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<GINTMSK_SPEC, 20> {
        IISOIXFRM_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask"]
    #[inline(always)]
    #[must_use]
    pub fn pxfrm_iisooxfrm(&mut self) -> PXFRM_IISOOXFRM_W<GINTMSK_SPEC, 21> {
        PXFRM_IISOOXFRM_W::new(self)
    }
    #[doc = "Bit 22 - Data fetch suspended mask"]
    #[inline(always)]
    #[must_use]
    pub fn fsuspm(&mut self) -> FSUSPM_W<GINTMSK_SPEC, 22> {
        FSUSPM_W::new(self)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hcim(&mut self) -> HCIM_W<GINTMSK_SPEC, 25> {
        HCIM_W::new(self)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfem(&mut self) -> PTXFEM_W<GINTMSK_SPEC, 26> {
        PTXFEM_W::new(self)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    #[must_use]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W<GINTMSK_SPEC, 28> {
        CIDSCHGM_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn discint(&mut self) -> DISCINT_W<GINTMSK_SPEC, 29> {
        DISCINT_W::new(self)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn srqim(&mut self) -> SRQIM_W<GINTMSK_SPEC, 30> {
        SRQIM_W::new(self)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wuim(&mut self) -> WUIM_W<GINTMSK_SPEC, 31> {
        WUIM_W::new(self)
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
#[doc = "OTG_HS interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

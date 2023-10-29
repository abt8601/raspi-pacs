#[doc = "Register `HCINT` reader"]
pub type R = crate::R<HCINT_SPEC>;
#[doc = "Register `HCINT` writer"]
pub type W = crate::W<HCINT_SPEC>;
#[doc = "Field `XFRC` reader - Transfer completed"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed"]
pub type XFRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHH` reader - Channel halted"]
pub type CHH_R = crate::BitReader;
#[doc = "Field `CHH` writer - Channel halted"]
pub type CHH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AHBERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYET` reader - Response received interrupt"]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - Response received interrupt"]
pub type NYET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERR` reader - Transaction error"]
pub type TXERR_R = crate::BitReader;
#[doc = "Field `TXERR` writer - Transaction error"]
pub type TXERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBERR` reader - Babble error"]
pub type BBERR_R = crate::BitReader;
#[doc = "Field `BBERR` writer - Babble error"]
pub type BBERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRMOR` reader - Frame overrun"]
pub type FRMOR_R = crate::BitReader;
#[doc = "Field `FRMOR` writer - Frame overrun"]
pub type FRMOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTERR` reader - Data toggle error"]
pub type DTERR_R = crate::BitReader;
#[doc = "Field `DTERR` writer - Data toggle error"]
pub type DTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT")
            .field("xfrc", &format_args!("{}", self.xfrc().bit()))
            .field("chh", &format_args!("{}", self.chh().bit()))
            .field("ahberr", &format_args!("{}", self.ahberr().bit()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("nak", &format_args!("{}", self.nak().bit()))
            .field("ack", &format_args!("{}", self.ack().bit()))
            .field("nyet", &format_args!("{}", self.nyet().bit()))
            .field("txerr", &format_args!("{}", self.txerr().bit()))
            .field("bberr", &format_args!("{}", self.bberr().bit()))
            .field("frmor", &format_args!("{}", self.frmor().bit()))
            .field("dterr", &format_args!("{}", self.dterr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<HCINT_SPEC, 0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn chh(&mut self) -> CHH_W<HCINT_SPEC, 1> {
        CHH_W::new(self)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<HCINT_SPEC, 2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<HCINT_SPEC, 3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<HCINT_SPEC, 4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<HCINT_SPEC, 5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<HCINT_SPEC, 6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<HCINT_SPEC, 7> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bberr(&mut self) -> BBERR_W<HCINT_SPEC, 8> {
        BBERR_W::new(self)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frmor(&mut self) -> FRMOR_W<HCINT_SPEC, 9> {
        FRMOR_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dterr(&mut self) -> DTERR_W<HCINT_SPEC, 10> {
        DTERR_W::new(self)
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
#[doc = "Interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINT_SPEC;
impl crate::RegisterSpec for HCINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint::R`](R) reader structure"]
impl crate::Readable for HCINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcint::W`](W) writer structure"]
impl crate::Writable for HCINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT to value 0"]
impl crate::Resettable for HCINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

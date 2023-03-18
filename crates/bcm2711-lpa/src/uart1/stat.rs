#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least one symbol"]
pub type DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least one symbol"]
pub type DATA_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_READY` reader - Transmit FIFO has space for at least one symbol"]
pub type TX_READY_R = crate::BitReader<bool>;
#[doc = "Field `TX_READY` writer - Transmit FIFO has space for at least one symbol"]
pub type TX_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_IDLE` reader - Receiver is idle"]
pub type RX_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `RX_IDLE` writer - Receiver is idle"]
pub type RX_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_IDLE` reader - Transmitter is idle"]
pub type TX_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `TX_IDLE` writer - Transmitter is idle"]
pub type TX_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_OVERRUN` reader - Receive FIFO overrun"]
pub type RX_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERRUN` writer - Receive FIFO overrun"]
pub type RX_OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_FULL` reader - Transmit FIFO is full"]
pub type TX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_FULL` writer - Transmit FIFO is full"]
pub type TX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RTS_STATUS` reader - RTS state"]
pub type RTS_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `RTS_STATUS` writer - RTS state"]
pub type RTS_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `CTS_STATUS` reader - CTS state"]
pub type CTS_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `CTS_STATUS` writer - CTS state"]
pub type CTS_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_EMPTY` reader - Transmit FIFO is completely empty"]
pub type TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_EMPTY` writer - Transmit FIFO is completely empty"]
pub type TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_DONE` reader - Transmit FIFO is empty and transmitter is idle"]
pub type TX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_DONE` writer - Transmit FIFO is empty and transmitter is idle"]
pub type TX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_LEVEL` reader - How many entries are filled in the RX FIFO"]
pub type RX_FIFO_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_FIFO_LEVEL` writer - How many entries are filled in the RX FIFO"]
pub type RX_FIFO_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `TX_FIFO_LEVEL` reader - How many entries are filled in the TX FIFO"]
pub type TX_FIFO_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_FIFO_LEVEL` writer - How many entries are filled in the TX FIFO"]
pub type TX_FIFO_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least one symbol"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO has space for at least one symbol"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver is idle"]
    #[inline(always)]
    pub fn rx_idle(&self) -> RX_IDLE_R {
        RX_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter is idle"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RX_OVERRUN_R {
        RX_OVERRUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO is full"]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS state"]
    #[inline(always)]
    pub fn rts_status(&self) -> RTS_STATUS_R {
        RTS_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS state"]
    #[inline(always)]
    pub fn cts_status(&self) -> CTS_STATUS_R {
        CTS_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO is completely empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO is empty and transmitter is idle"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - How many entries are filled in the RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_level(&self) -> RX_FIFO_LEVEL_R {
        RX_FIFO_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - How many entries are filled in the TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_level(&self) -> TX_FIFO_LEVEL_R {
        TX_FIFO_LEVEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least one symbol"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DATA_READY_W<0> {
        DATA_READY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO has space for at least one symbol"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TX_READY_W<1> {
        TX_READY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver is idle"]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle(&mut self) -> RX_IDLE_W<2> {
        RX_IDLE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter is idle"]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle(&mut self) -> TX_IDLE_W<3> {
        TX_IDLE_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W<4> {
        RX_OVERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TX_FULL_W<5> {
        TX_FULL_W::new(self)
    }
    #[doc = "Bit 6 - RTS state"]
    #[inline(always)]
    #[must_use]
    pub fn rts_status(&mut self) -> RTS_STATUS_W<6> {
        RTS_STATUS_W::new(self)
    }
    #[doc = "Bit 7 - CTS state"]
    #[inline(always)]
    #[must_use]
    pub fn cts_status(&mut self) -> CTS_STATUS_W<7> {
        CTS_STATUS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit FIFO is completely empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<8> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 9 - Transmit FIFO is empty and transmitter is idle"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<9> {
        TX_DONE_W::new(self)
    }
    #[doc = "Bits 16:19 - How many entries are filled in the RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_level(&mut self) -> RX_FIFO_LEVEL_W<16> {
        RX_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Bits 24:27 - How many entries are filled in the TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_level(&mut self) -> TX_FIFO_LEVEL_W<24> {
        TX_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

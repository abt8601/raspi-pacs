#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSR` writer"]
pub struct W(crate::W<LSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSR_SPEC>;
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
impl From<crate::W<LSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least one byte"]
pub type DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least one byte"]
pub type DATA_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `RX_OVERRUN` reader - Receive FIFO overrun"]
pub type RX_OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERRUN` writer - Receive FIFO overrun"]
pub type RX_OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `TX_EMPTY` reader - Transmit FIFO has room for at least one byte"]
pub type TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_EMPTY` writer - Transmit FIFO has room for at least one byte"]
pub type TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
#[doc = "Field `TX_IDLE` reader - Transmit FIFO empty and all bits shifted out"]
pub type TX_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `TX_IDLE` writer - Transmit FIFO empty and all bits shifted out"]
pub type TX_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least one byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RX_OVERRUN_R {
        RX_OVERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DATA_READY_W<0> {
        DATA_READY_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W<1> {
        RX_OVERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<5> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 6 - Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle(&mut self) -> TX_IDLE_W<6> {
        TX_IDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsr::W](W) writer structure"]
impl crate::Writable for LSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `LSR` reader"]
pub type R = crate::R<LSR_SPEC>;
#[doc = "Register `LSR` writer"]
pub type W = crate::W<LSR_SPEC>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least one byte"]
pub type DATA_READY_R = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least one byte"]
pub type DATA_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERRUN` reader - Receive FIFO overrun"]
pub type RX_OVERRUN_R = crate::BitReader;
#[doc = "Field `RX_OVERRUN` writer - Receive FIFO overrun"]
pub type RX_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - Transmit FIFO has room for at least one byte"]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - Transmit FIFO has room for at least one byte"]
pub type TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IDLE` reader - Transmit FIFO empty and all bits shifted out"]
pub type TX_IDLE_R = crate::BitReader;
#[doc = "Field `TX_IDLE` writer - Transmit FIFO empty and all bits shifted out"]
pub type TX_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSR")
            .field("tx_idle", &format_args!("{}", self.tx_idle().bit()))
            .field("tx_empty", &format_args!("{}", self.tx_empty().bit()))
            .field("rx_overrun", &format_args!("{}", self.rx_overrun().bit()))
            .field("data_ready", &format_args!("{}", self.data_ready().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DATA_READY_W<LSR_SPEC> {
        DATA_READY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W<LSR_SPEC> {
        RX_OVERRUN_W::new(self, 1)
    }
    #[doc = "Bit 5 - Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<LSR_SPEC> {
        TX_EMPTY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    #[must_use]
    pub fn tx_idle(&mut self) -> TX_IDLE_W<LSR_SPEC> {
        TX_IDLE_W::new(self, 6)
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
#[doc = "Line Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lsr::W`](W) writer structure"]
impl crate::Writable for LSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `IIR` reader"]
pub type R = crate::R<IIR_SPEC>;
#[doc = "Register `IIR` writer"]
pub type W = crate::W<IIR_SPEC>;
#[doc = "Field `nPENDING` reader - No pending interrupt"]
pub type N_PENDING_R = crate::BitReader;
#[doc = "Field `nPENDING` writer - No pending interrupt"]
pub type N_PENDING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least 1 byte"]
pub type DATA_READY_R = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least 1 byte"]
pub type DATA_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_READY` reader - Transmit FIFO is empty"]
pub type TX_READY_R = crate::BitReader;
#[doc = "Field `TX_READY` writer - Transmit FIFO is empty"]
pub type TX_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    pub fn n_pending(&self) -> N_PENDING_R {
        N_PENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIR")
            .field("tx_ready", &format_args!("{}", self.tx_ready().bit()))
            .field("data_ready", &format_args!("{}", self.data_ready().bit()))
            .field("n_pending", &format_args!("{}", self.n_pending().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - No pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn n_pending(&mut self) -> N_PENDING_W<IIR_SPEC> {
        N_PENDING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DATA_READY_W<IIR_SPEC> {
        DATA_READY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TX_READY_W<IIR_SPEC> {
        TX_READY_W::new(self, 2)
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
#[doc = "Interrupt Identify\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iir::W`](W) writer structure"]
impl crate::Writable for IIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIR to value 0xb001"]
impl crate::Resettable for IIR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb001;
}

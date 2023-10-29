#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least 1 byte"]
pub type DATA_READY_R = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least 1 byte"]
pub type DATA_READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_READY` reader - Transmit FIFO is empty"]
pub type TX_READY_R = crate::BitReader;
#[doc = "Field `TX_READY` writer - Transmit FIFO is empty"]
pub type TX_READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DATA_READY_R {
        DATA_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("tx_ready", &format_args!("{}", self.tx_ready().bit()))
            .field("data_ready", &format_args!("{}", self.data_ready().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DATA_READY_W<IER_SPEC, 0> {
        DATA_READY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TX_READY_W<IER_SPEC, 1> {
        TX_READY_W::new(self)
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
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

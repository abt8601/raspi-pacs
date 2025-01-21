#[doc = "Register `S` reader"]
pub type R = crate::R<S_SPEC>;
#[doc = "Register `S` writer"]
pub type W = crate::W<S_SPEC>;
#[doc = "Field `TA` reader - Transfer active"]
pub type TA_R = crate::BitReader;
#[doc = "Field `DONE` reader - Transfer done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Transfer done"]
pub type DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXW` reader - FIFO needs to be written"]
pub type TXW_R = crate::BitReader;
#[doc = "Field `RXR` reader - FIFO needs to be read"]
pub type RXR_R = crate::BitReader;
#[doc = "Field `TXD` reader - FIFO has space for at least one byte"]
pub type TXD_R = crate::BitReader;
#[doc = "Field `RXD` reader - FIFO contains at least one byte"]
pub type RXD_R = crate::BitReader;
#[doc = "Field `TXE` reader - FIFO is empty. Nothing to transmit"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `RXF` reader - FIFO is full. Can't receive anything else"]
pub type RXF_R = crate::BitReader;
#[doc = "Field `ERR` reader - Error: No ack"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - Error: No ack"]
pub type ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLKT` reader - Clock stretch timeout"]
pub type CLKT_R = crate::BitReader;
#[doc = "Field `CLKT` writer - Clock stretch timeout"]
pub type CLKT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer active"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO needs to be written"]
    #[inline(always)]
    pub fn txw(&self) -> TXW_R {
        TXW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO needs to be read"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO has space for at least one byte"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO contains at least one byte"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO is empty. Nothing to transmit"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO is full. Can't receive anything else"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error: No ack"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock stretch timeout"]
    #[inline(always)]
    pub fn clkt(&self) -> CLKT_R {
        CLKT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S")
            .field("clkt", &format_args!("{}", self.clkt().bit()))
            .field("err", &format_args!("{}", self.err().bit()))
            .field("rxf", &format_args!("{}", self.rxf().bit()))
            .field("txe", &format_args!("{}", self.txe().bit()))
            .field("rxd", &format_args!("{}", self.rxd().bit()))
            .field("txd", &format_args!("{}", self.txd().bit()))
            .field("rxr", &format_args!("{}", self.rxr().bit()))
            .field("txw", &format_args!("{}", self.txw().bit()))
            .field("done", &format_args!("{}", self.done().bit()))
            .field("ta", &format_args!("{}", self.ta().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<S_SPEC> {
        DONE_W::new(self, 1)
    }
    #[doc = "Bit 8 - Error: No ack"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<S_SPEC> {
        ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock stretch timeout"]
    #[inline(always)]
    #[must_use]
    pub fn clkt(&mut self) -> CLKT_W<S_SPEC> {
        CLKT_W::new(self, 9)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s::W`](W) writer structure"]
impl crate::Writable for S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0302;
}
#[doc = "`reset()` method sets S to value 0x50"]
impl crate::Resettable for S_SPEC {
    const RESET_VALUE: u32 = 0x50;
}

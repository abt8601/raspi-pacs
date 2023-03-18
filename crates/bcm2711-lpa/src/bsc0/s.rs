#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S` writer"]
pub struct W(crate::W<S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_SPEC>;
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
impl From<crate::W<S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TA` reader - Transfer active"]
pub type TA_R = crate::BitReader<bool>;
#[doc = "Field `DONE` reader - Transfer done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Transfer done"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, S_SPEC, bool, O>;
#[doc = "Field `TXW` reader - FIFO needs to be written"]
pub type TXW_R = crate::BitReader<bool>;
#[doc = "Field `RXR` reader - FIFO needs to be read"]
pub type RXR_R = crate::BitReader<bool>;
#[doc = "Field `TXD` reader - FIFO has space for at least one byte"]
pub type TXD_R = crate::BitReader<bool>;
#[doc = "Field `RXD` reader - FIFO contains at least one byte"]
pub type RXD_R = crate::BitReader<bool>;
#[doc = "Field `TXE` reader - FIFO is empty. Nothing to transmit"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `RXF` reader - FIFO is full. Can't receive anything else"]
pub type RXF_R = crate::BitReader<bool>;
#[doc = "Field `ERR` reader - Error: No ack"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - Error: No ack"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, S_SPEC, bool, O>;
#[doc = "Field `CLKT` reader - Clock stretch timeout"]
pub type CLKT_R = crate::BitReader<bool>;
#[doc = "Field `CLKT` writer - Clock stretch timeout"]
pub type CLKT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, S_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<1> {
        DONE_W::new(self)
    }
    #[doc = "Bit 8 - Error: No ack"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<8> {
        ERR_W::new(self)
    }
    #[doc = "Bit 9 - Clock stretch timeout"]
    #[inline(always)]
    #[must_use]
    pub fn clkt(&mut self) -> CLKT_W<9> {
        CLKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s::W](W) writer structure"]
impl crate::Writable for S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0302;
}
#[doc = "`reset()` method sets S to value 0x50"]
impl crate::Resettable for S_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}

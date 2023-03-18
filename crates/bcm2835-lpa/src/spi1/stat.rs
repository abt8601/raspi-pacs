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
#[doc = "Field `BIT_COUNT` reader - Number of bits left to be processed."]
pub type BIT_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIT_COUNT` writer - Number of bits left to be processed."]
pub type BIT_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 6, O>;
#[doc = "Field `BUSY` reader - Indicates a transfer is ongoing"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - Indicates a transfer is ongoing"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_EMPTY` reader - RX FIFO is empty"]
pub type RX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_EMPTY` writer - RX FIFO is empty"]
pub type RX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_FULL` reader - RX FIFO is full"]
pub type RX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL` writer - RX FIFO is full"]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_EMPTY` reader - TX FIFO is empty"]
pub type TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_EMPTY` writer - TX FIFO is empty"]
pub type TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TX_FULL` reader - TX FIFO is full"]
pub type TX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_FULL` writer - TX FIFO is full"]
pub type TX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RX_LEVEL` reader - Number of entries in RX FIFO"]
pub type RX_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_LEVEL` writer - Number of entries in RX FIFO"]
pub type RX_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `TX_LEVEL` reader - Number of entries in TX FIFO"]
pub type TX_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_LEVEL` writer - Number of entries in TX FIFO"]
pub type TX_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - Number of bits left to be processed."]
    #[inline(always)]
    pub fn bit_count(&self) -> BIT_COUNT_R {
        BIT_COUNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Indicates a transfer is ongoing"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO is empty"]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO is full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO is empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO is full"]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of entries in RX FIFO"]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of entries in TX FIFO"]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of bits left to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn bit_count(&mut self) -> BIT_COUNT_W<0> {
        BIT_COUNT_W::new(self)
    }
    #[doc = "Bit 6 - Indicates a transfer is ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<6> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 7 - RX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W<7> {
        RX_EMPTY_W::new(self)
    }
    #[doc = "Bit 8 - RX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<8> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 9 - TX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<9> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 10 - TX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TX_FULL_W<10> {
        TX_FULL_W::new(self)
    }
    #[doc = "Bits 16:19 - Number of entries in RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<16> {
        RX_LEVEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Number of entries in TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<24> {
        TX_LEVEL_W::new(self)
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

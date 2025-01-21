#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `BIT_COUNT` reader - Number of bits left to be processed."]
pub type BIT_COUNT_R = crate::FieldReader;
#[doc = "Field `BIT_COUNT` writer - Number of bits left to be processed."]
pub type BIT_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BUSY` reader - Indicates a transfer is ongoing"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Indicates a transfer is ongoing"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EMPTY` reader - RX FIFO is empty"]
pub type RX_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_EMPTY` writer - RX FIFO is empty"]
pub type RX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - RX FIFO is full"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `RX_FULL` writer - RX FIFO is full"]
pub type RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - TX FIFO is empty"]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - TX FIFO is empty"]
pub type TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FULL` reader - TX FIFO is full"]
pub type TX_FULL_R = crate::BitReader;
#[doc = "Field `TX_FULL` writer - TX FIFO is full"]
pub type TX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LEVEL` reader - Number of entries in RX FIFO"]
pub type RX_LEVEL_R = crate::FieldReader;
#[doc = "Field `RX_LEVEL` writer - Number of entries in RX FIFO"]
pub type RX_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_LEVEL` reader - Number of entries in TX FIFO"]
pub type TX_LEVEL_R = crate::FieldReader;
#[doc = "Field `TX_LEVEL` writer - Number of entries in TX FIFO"]
pub type TX_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("tx_level", &format_args!("{}", self.tx_level().bits()))
            .field("rx_level", &format_args!("{}", self.rx_level().bits()))
            .field("tx_full", &format_args!("{}", self.tx_full().bit()))
            .field("tx_empty", &format_args!("{}", self.tx_empty().bit()))
            .field("rx_full", &format_args!("{}", self.rx_full().bit()))
            .field("rx_empty", &format_args!("{}", self.rx_empty().bit()))
            .field("busy", &format_args!("{}", self.busy().bit()))
            .field("bit_count", &format_args!("{}", self.bit_count().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of bits left to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn bit_count(&mut self) -> BIT_COUNT_W<STAT_SPEC> {
        BIT_COUNT_W::new(self, 0)
    }
    #[doc = "Bit 6 - Indicates a transfer is ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<STAT_SPEC> {
        BUSY_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W<STAT_SPEC> {
        RX_EMPTY_W::new(self, 7)
    }
    #[doc = "Bit 8 - RX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<STAT_SPEC> {
        RX_FULL_W::new(self, 8)
    }
    #[doc = "Bit 9 - TX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<STAT_SPEC> {
        TX_EMPTY_W::new(self, 9)
    }
    #[doc = "Bit 10 - TX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TX_FULL_W<STAT_SPEC> {
        TX_FULL_W::new(self, 10)
    }
    #[doc = "Bits 16:19 - Number of entries in RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RX_LEVEL_W<STAT_SPEC> {
        RX_LEVEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of entries in TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TX_LEVEL_W<STAT_SPEC> {
        TX_LEVEL_W::new(self, 24)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}

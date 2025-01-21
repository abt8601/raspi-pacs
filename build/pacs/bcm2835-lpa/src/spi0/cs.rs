#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `CS` reader - Chip select"]
pub type CS_R = crate::FieldReader;
#[doc = "Field `CS` writer - Chip select"]
pub type CS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` reader - Clear the FIFO(s)"]
pub type CLEAR_R = crate::FieldReader<CLEAR_A>;
#[doc = "Clear the FIFO(s)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLEAR_A {
    #[doc = "1: `1`"]
    TX = 1,
    #[doc = "2: `10`"]
    RX = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<CLEAR_A> for u8 {
    #[inline(always)]
    fn from(variant: CLEAR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLEAR_A {
    type Ux = u8;
}
impl CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLEAR_A> {
        match self.bits {
            1 => Some(CLEAR_A::TX),
            2 => Some(CLEAR_A::RX),
            3 => Some(CLEAR_A::BOTH),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == CLEAR_A::TX
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == CLEAR_A::RX
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CLEAR_A::BOTH
    }
}
#[doc = "Field `CLEAR` writer - Clear the FIFO(s)"]
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLEAR_A>;
impl<'a, REG> CLEAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(CLEAR_A::TX)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(CLEAR_A::RX)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CLEAR_A::BOTH)
    }
}
#[doc = "Field `CSPOL` reader - Chip select polarity"]
pub type CSPOL_R = crate::BitReader;
#[doc = "Field `CSPOL` writer - Chip select polarity"]
pub type CSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA` reader - Transfer active"]
pub type TA_R = crate::BitReader;
#[doc = "Field `TA` writer - Transfer active"]
pub type TA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - Enable DMA"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - Enable DMA"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTD` reader - Interrupt on done"]
pub type INTD_R = crate::BitReader;
#[doc = "Field `INTD` writer - Interrupt on done"]
pub type INTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR` reader - Interrupt on RX"]
pub type INTR_R = crate::BitReader;
#[doc = "Field `INTR` writer - Interrupt on RX"]
pub type INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCS` reader - Automatically deassert chip select"]
pub type ADCS_R = crate::BitReader;
#[doc = "Field `ADCS` writer - Automatically deassert chip select"]
pub type ADCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Read enable"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - Read enable"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN` reader - LoSSI enable"]
pub type LEN_R = crate::BitReader;
#[doc = "Field `LEN` writer - LoSSI enable"]
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMONO` reader - "]
pub type LMONO_R = crate::BitReader;
#[doc = "Field `LMONO` writer - "]
pub type LMONO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE_EN` reader - "]
pub type TE_EN_R = crate::BitReader;
#[doc = "Field `TE_EN` writer - "]
pub type TE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Transfer is done"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `RXD` reader - RX FIFO contains data"]
pub type RXD_R = crate::BitReader;
#[doc = "Field `TXD` reader - TX FIFO can accept data"]
pub type TXD_R = crate::BitReader;
#[doc = "Field `RXR` reader - RX FIFO has data to be read"]
pub type RXR_R = crate::BitReader;
#[doc = "Field `RXF` reader - RX FIFO full"]
pub type RXF_R = crate::BitReader;
#[doc = "Field `CSPOL0` reader - Chip select 0 polarity"]
pub type CSPOL0_R = crate::BitReader;
#[doc = "Field `CSPOL0` writer - Chip select 0 polarity"]
pub type CSPOL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPOL1` reader - Chip select 1 polarity"]
pub type CSPOL1_R = crate::BitReader;
#[doc = "Field `CSPOL1` writer - Chip select 1 polarity"]
pub type CSPOL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPOL2` reader - Chip select 2 polarity"]
pub type CSPOL2_R = crate::BitReader;
#[doc = "Field `CSPOL2` writer - Chip select 2 polarity"]
pub type CSPOL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_LEN` reader - Enable DMA in LoSSI mode"]
pub type DMA_LEN_R = crate::BitReader;
#[doc = "Field `DMA_LEN` writer - Enable DMA in LoSSI mode"]
pub type DMA_LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_LONG` reader - Enable long data word in LoSSI mode"]
pub type LEN_LONG_R = crate::BitReader;
#[doc = "Field `LEN_LONG` writer - Enable long data word in LoSSI mode"]
pub type LEN_LONG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Chip select"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO(s)"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Chip select polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer active"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on done"]
    #[inline(always)]
    pub fn intd(&self) -> INTD_R {
        INTD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatically deassert chip select"]
    #[inline(always)]
    pub fn adcs(&self) -> ADCS_R {
        ADCS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LoSSI enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lmono(&self) -> LMONO_R {
        LMONO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn te_en(&self) -> TE_EN_R {
        TE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer is done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX FIFO has data to be read"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RX FIFO full"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Chip select 0 polarity"]
    #[inline(always)]
    pub fn cspol0(&self) -> CSPOL0_R {
        CSPOL0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chip select 1 polarity"]
    #[inline(always)]
    pub fn cspol1(&self) -> CSPOL1_R {
        CSPOL1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Chip select 2 polarity"]
    #[inline(always)]
    pub fn cspol2(&self) -> CSPOL2_R {
        CSPOL2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DMA in LoSSI mode"]
    #[inline(always)]
    pub fn dma_len(&self) -> DMA_LEN_R {
        DMA_LEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable long data word in LoSSI mode"]
    #[inline(always)]
    pub fn len_long(&self) -> LEN_LONG_R {
        LEN_LONG_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("len_long", &format_args!("{}", self.len_long().bit()))
            .field("dma_len", &format_args!("{}", self.dma_len().bit()))
            .field("cspol2", &format_args!("{}", self.cspol2().bit()))
            .field("cspol1", &format_args!("{}", self.cspol1().bit()))
            .field("cspol0", &format_args!("{}", self.cspol0().bit()))
            .field("rxf", &format_args!("{}", self.rxf().bit()))
            .field("rxr", &format_args!("{}", self.rxr().bit()))
            .field("txd", &format_args!("{}", self.txd().bit()))
            .field("rxd", &format_args!("{}", self.rxd().bit()))
            .field("done", &format_args!("{}", self.done().bit()))
            .field("te_en", &format_args!("{}", self.te_en().bit()))
            .field("lmono", &format_args!("{}", self.lmono().bit()))
            .field("len", &format_args!("{}", self.len().bit()))
            .field("ren", &format_args!("{}", self.ren().bit()))
            .field("adcs", &format_args!("{}", self.adcs().bit()))
            .field("intr", &format_args!("{}", self.intr().bit()))
            .field("intd", &format_args!("{}", self.intd().bit()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("ta", &format_args!("{}", self.ta().bit()))
            .field("cspol", &format_args!("{}", self.cspol().bit()))
            .field("clear", &format_args!("{}", self.clear().bits()))
            .field("cpol", &format_args!("{}", self.cpol().bit()))
            .field("cpha", &format_args!("{}", self.cpha().bit()))
            .field("cs", &format_args!("{}", self.cs().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chip select"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<CS_SPEC> {
        CS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CS_SPEC> {
        CPHA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CS_SPEC> {
        CPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Clear the FIFO(s)"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CS_SPEC> {
        CLEAR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Chip select polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<CS_SPEC> {
        CSPOL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer active"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<CS_SPEC> {
        TA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CS_SPEC> {
        DMAEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on done"]
    #[inline(always)]
    #[must_use]
    pub fn intd(&mut self) -> INTD_W<CS_SPEC> {
        INTD_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> INTR_W<CS_SPEC> {
        INTR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Automatically deassert chip select"]
    #[inline(always)]
    #[must_use]
    pub fn adcs(&mut self) -> ADCS_W<CS_SPEC> {
        ADCS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Read enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CS_SPEC> {
        REN_W::new(self, 12)
    }
    #[doc = "Bit 13 - LoSSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<CS_SPEC> {
        LEN_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn lmono(&mut self) -> LMONO_W<CS_SPEC> {
        LMONO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn te_en(&mut self) -> TE_EN_W<CS_SPEC> {
        TE_EN_W::new(self, 15)
    }
    #[doc = "Bit 21 - Chip select 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol0(&mut self) -> CSPOL0_W<CS_SPEC> {
        CSPOL0_W::new(self, 21)
    }
    #[doc = "Bit 22 - Chip select 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol1(&mut self) -> CSPOL1_W<CS_SPEC> {
        CSPOL1_W::new(self, 22)
    }
    #[doc = "Bit 23 - Chip select 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol2(&mut self) -> CSPOL2_W<CS_SPEC> {
        CSPOL2_W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable DMA in LoSSI mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma_len(&mut self) -> DMA_LEN_W<CS_SPEC> {
        DMA_LEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable long data word in LoSSI mode"]
    #[inline(always)]
    #[must_use]
    pub fn len_long(&mut self) -> LEN_LONG_W<CS_SPEC> {
        LEN_LONG_W::new(self, 25)
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
#[doc = "Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0x0004_1000"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: u32 = 0x0004_1000;
}

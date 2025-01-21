#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_BURST` reader - Maximum AXI burst length"]
pub type AXI_BURST_R = crate::FieldReader<BURST_A>;
#[doc = "Maximum AXI burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURST_A {
    #[doc = "0: `0`"]
    _4 = 0,
    #[doc = "1: `1`"]
    _3 = 1,
    #[doc = "2: `10`"]
    _2 = 2,
    #[doc = "3: `11`"]
    _1 = 3,
}
impl From<BURST_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BURST_A {
    type Ux = u8;
}
impl AXI_BURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::_4,
            1 => BURST_A::_3,
            2 => BURST_A::_2,
            3 => BURST_A::_1,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == BURST_A::_4
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == BURST_A::_3
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == BURST_A::_2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BURST_A::_1
    }
}
#[doc = "Field `AXI_BURST` writer - Maximum AXI burst length"]
pub type AXI_BURST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BURST_A>;
impl<'a, REG> AXI_BURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::_4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::_3)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::_1)
    }
}
#[doc = "Field `AXI_WAIT` reader - Wait for all AXI writes before signaling DMA"]
pub type AXI_WAIT_R = crate::BitReader;
#[doc = "Field `AXI_WAIT` writer - Wait for all AXI writes before signaling DMA"]
pub type AXI_WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Maximum AXI burst length"]
    #[inline(always)]
    pub fn axi_burst(&self) -> AXI_BURST_R {
        AXI_BURST_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    pub fn axi_wait(&self) -> AXI_WAIT_R {
        AXI_WAIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("gint", &format_args!("{}", self.gint().bit()))
            .field("axi_wait", &format_args!("{}", self.axi_wait().bit()))
            .field("axi_burst", &format_args!("{}", self.axi_burst().bits()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("txfelvl", &format_args!("{}", self.txfelvl().bit()))
            .field("ptxfelvl", &format_args!("{}", self.ptxfelvl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GAHBCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<GAHBCFG_SPEC> {
        GINT_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Maximum AXI burst length"]
    #[inline(always)]
    #[must_use]
    pub fn axi_burst(&mut self) -> AXI_BURST_W<GAHBCFG_SPEC> {
        AXI_BURST_W::new(self, 1)
    }
    #[doc = "Bit 4 - Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    #[must_use]
    pub fn axi_wait(&mut self) -> AXI_WAIT_W<GAHBCFG_SPEC> {
        AXI_WAIT_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFG_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<GAHBCFG_SPEC> {
        TXFELVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<GAHBCFG_SPEC> {
        PTXFELVL_W::new(self, 8)
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
#[doc = "OTG_HS AHB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
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
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader<bool>;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `AXI_BURST` reader - Maximum AXI burst length"]
pub type AXI_BURST_R = crate::FieldReader<u8, BURST_A>;
#[doc = "Maximum AXI burst length\n\nValue on reset: 0"]
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
impl AXI_BURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::_4,
            1 => BURST_A::_3,
            2 => BURST_A::_2,
            3 => BURST_A::_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == BURST_A::_4
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == BURST_A::_3
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == BURST_A::_2
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BURST_A::_1
    }
}
#[doc = "Field `AXI_BURST` writer - Maximum AXI burst length"]
pub type AXI_BURST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GAHBCFG_SPEC, u8, BURST_A, 2, O>;
impl<'a, const O: u8> AXI_BURST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(BURST_A::_4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(BURST_A::_3)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(BURST_A::_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BURST_A::_1)
    }
}
#[doc = "Field `AXI_WAIT` reader - Wait for all AXI writes before signaling DMA"]
pub type AXI_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `AXI_WAIT` writer - Wait for all AXI writes before signaling DMA"]
pub type AXI_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<0> {
        GINT_W::new(self)
    }
    #[doc = "Bits 1:2 - Maximum AXI burst length"]
    #[inline(always)]
    #[must_use]
    pub fn axi_burst(&mut self) -> AXI_BURST_W<1> {
        AXI_BURST_W::new(self)
    }
    #[doc = "Bit 4 - Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    #[must_use]
    pub fn axi_wait(&mut self) -> AXI_WAIT_W<4> {
        AXI_WAIT_W::new(self)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<7> {
        TXFELVL_W::new(self)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<8> {
        PTXFELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS AHB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

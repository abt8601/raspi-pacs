#[doc = "Register `GICD_ICFGR36` reader"]
pub struct R(crate::R<GICD_ICFGR36_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR36_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR36_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR36_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR36` writer"]
pub struct W(crate::W<GICD_ICFGR36_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR36_SPEC>;
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
impl From<crate::W<GICD_ICFGR36_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR36_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMI` reader - SMI"]
pub type SMI_R = crate::BitReader<SMI_A>;
#[doc = "SMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMI_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SMI_A> for bool {
    #[inline(always)]
    fn from(variant: SMI_A) -> Self {
        variant as u8 != 0
    }
}
impl SMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMI_A {
        match self.bits {
            false => SMI_A::LEVEL,
            true => SMI_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SMI_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SMI_A::EDGE
    }
}
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, SMI_A, O>;
impl<'a, const O: u8> SMI_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SMI_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SMI_A::EDGE)
    }
}
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type GPIO_0_R = crate::BitReader<GPIO_0_A>;
#[doc = "GPIO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<GPIO_0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_0_A {
        match self.bits {
            false => GPIO_0_A::LEVEL,
            true => GPIO_0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_0_A::EDGE
    }
}
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, GPIO_0_A, O>;
impl<'a, const O: u8> GPIO_0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_0_A::EDGE)
    }
}
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type GPIO_1_R = crate::BitReader<GPIO_1_A>;
#[doc = "GPIO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<GPIO_1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_1_A {
        match self.bits {
            false => GPIO_1_A::LEVEL,
            true => GPIO_1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_1_A::EDGE
    }
}
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, GPIO_1_A, O>;
impl<'a, const O: u8> GPIO_1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_1_A::EDGE)
    }
}
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type GPIO_2_R = crate::BitReader<GPIO_2_A>;
#[doc = "GPIO 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<GPIO_2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_2_A {
        match self.bits {
            false => GPIO_2_A::LEVEL,
            true => GPIO_2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_2_A::EDGE
    }
}
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, GPIO_2_A, O>;
impl<'a, const O: u8> GPIO_2_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_2_A::EDGE)
    }
}
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type GPIO_3_R = crate::BitReader<GPIO_3_A>;
#[doc = "GPIO 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<GPIO_3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_3_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_3_A {
        match self.bits {
            false => GPIO_3_A::LEVEL,
            true => GPIO_3_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_3_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_3_A::EDGE
    }
}
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, GPIO_3_A, O>;
impl<'a, const O: u8> GPIO_3_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(GPIO_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(GPIO_3_A::EDGE)
    }
}
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::BitReader<I2C_A>;
#[doc = "OR of all I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::LEVEL,
            true => I2C_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == I2C_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == I2C_A::EDGE
    }
}
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, I2C_A, O>;
impl<'a, const O: u8> I2C_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(I2C_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(I2C_A::EDGE)
    }
}
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::BitReader<SPI_A>;
#[doc = "OR of all SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SPI_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_A {
        match self.bits {
            false => SPI_A::LEVEL,
            true => SPI_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SPI_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SPI_A::EDGE
    }
}
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, SPI_A, O>;
impl<'a, const O: u8> SPI_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SPI_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SPI_A::EDGE)
    }
}
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::BitReader<PCM_I2S_A>;
#[doc = "PCM/I2S\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCM_I2S_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<PCM_I2S_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_I2S_A) -> Self {
        variant as u8 != 0
    }
}
impl PCM_I2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_I2S_A {
        match self.bits {
            false => PCM_I2S_A::LEVEL,
            true => PCM_I2S_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PCM_I2S_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PCM_I2S_A::EDGE
    }
}
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, PCM_I2S_A, O>;
impl<'a, const O: u8> PCM_I2S_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(PCM_I2S_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(PCM_I2S_A::EDGE)
    }
}
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::BitReader<SDHOST_A>;
#[doc = "SDHOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDHOST_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SDHOST_A> for bool {
    #[inline(always)]
    fn from(variant: SDHOST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDHOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHOST_A {
        match self.bits {
            false => SDHOST_A::LEVEL,
            true => SDHOST_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SDHOST_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SDHOST_A::EDGE
    }
}
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, SDHOST_A, O>;
impl<'a, const O: u8> SDHOST_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SDHOST_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SDHOST_A::EDGE)
    }
}
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::BitReader<UART_A>;
#[doc = "OR of all PL011 UARTs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<UART_A> for bool {
    #[inline(always)]
    fn from(variant: UART_A) -> Self {
        variant as u8 != 0
    }
}
impl UART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_A {
        match self.bits {
            false => UART_A::LEVEL,
            true => UART_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == UART_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == UART_A::EDGE
    }
}
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, UART_A, O>;
impl<'a, const O: u8> UART_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(UART_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(UART_A::EDGE)
    }
}
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_R = crate::BitReader<ETH_PCIE_A>;
#[doc = "OR of all ETH_PCIe L2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH_PCIE_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ETH_PCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ETH_PCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ETH_PCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH_PCIE_A {
        match self.bits {
            false => ETH_PCIE_A::LEVEL,
            true => ETH_PCIE_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ETH_PCIE_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETH_PCIE_A::EDGE
    }
}
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, ETH_PCIE_A, O>;
impl<'a, const O: u8> ETH_PCIE_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ETH_PCIE_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETH_PCIE_A::EDGE)
    }
}
#[doc = "Field `VEC` reader - VEC"]
pub type VEC_R = crate::BitReader<VEC_A>;
#[doc = "VEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEC_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<VEC_A> for bool {
    #[inline(always)]
    fn from(variant: VEC_A) -> Self {
        variant as u8 != 0
    }
}
impl VEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEC_A {
        match self.bits {
            false => VEC_A::LEVEL,
            true => VEC_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == VEC_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == VEC_A::EDGE
    }
}
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, VEC_A, O>;
impl<'a, const O: u8> VEC_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(VEC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(VEC_A::EDGE)
    }
}
#[doc = "Field `CPG` reader - CPG"]
pub type CPG_R = crate::BitReader<CPG_A>;
#[doc = "CPG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPG_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<CPG_A> for bool {
    #[inline(always)]
    fn from(variant: CPG_A) -> Self {
        variant as u8 != 0
    }
}
impl CPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPG_A {
        match self.bits {
            false => CPG_A::LEVEL,
            true => CPG_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == CPG_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == CPG_A::EDGE
    }
}
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, CPG_A, O>;
impl<'a, const O: u8> CPG_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(CPG_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(CPG_A::EDGE)
    }
}
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader<RNG_A>;
#[doc = "RNG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::LEVEL,
            true => RNG_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == RNG_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == RNG_A::EDGE
    }
}
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, RNG_A, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(RNG_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(RNG_A::EDGE)
    }
}
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::BitReader<EMMC_A>;
#[doc = "OR of EMMC and EMMC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMMC_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<EMMC_A> for bool {
    #[inline(always)]
    fn from(variant: EMMC_A) -> Self {
        variant as u8 != 0
    }
}
impl EMMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMMC_A {
        match self.bits {
            false => EMMC_A::LEVEL,
            true => EMMC_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EMMC_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EMMC_A::EDGE
    }
}
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, EMMC_A, O>;
impl<'a, const O: u8> EMMC_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(EMMC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EMMC_A::EDGE)
    }
}
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_R = crate::BitReader<ETH_PCIE_SECURE_A>;
#[doc = "ETH_PCIe secure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH_PCIE_SECURE_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ETH_PCIE_SECURE_A> for bool {
    #[inline(always)]
    fn from(variant: ETH_PCIE_SECURE_A) -> Self {
        variant as u8 != 0
    }
}
impl ETH_PCIE_SECURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH_PCIE_SECURE_A {
        match self.bits {
            false => ETH_PCIE_SECURE_A::LEVEL,
            true => ETH_PCIE_SECURE_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ETH_PCIE_SECURE_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETH_PCIE_SECURE_A::EDGE
    }
}
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR36_SPEC, ETH_PCIE_SECURE_A, O>;
impl<'a, const O: u8> ETH_PCIE_SECURE_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ETH_PCIE_SECURE_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETH_PCIE_SECURE_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SMI_R {
        SMI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> GPIO_0_R {
        GPIO_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> GPIO_1_R {
        GPIO_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> GPIO_2_R {
        GPIO_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> GPIO_3_R {
        GPIO_3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PCM_I2S_R {
        PCM_I2S_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SDHOST_R {
        SDHOST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> ETH_PCIE_R {
        ETH_PCIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CPG_R {
        CPG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EMMC_R {
        EMMC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> ETH_PCIE_SECURE_R {
        ETH_PCIE_SECURE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<1> {
        SMI_W::new(self)
    }
    #[doc = "Bit 3 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<3> {
        GPIO_0_W::new(self)
    }
    #[doc = "Bit 5 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<5> {
        GPIO_1_W::new(self)
    }
    #[doc = "Bit 7 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<7> {
        GPIO_2_W::new(self)
    }
    #[doc = "Bit 9 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<9> {
        GPIO_3_W::new(self)
    }
    #[doc = "Bit 11 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<11> {
        I2C_W::new(self)
    }
    #[doc = "Bit 13 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<13> {
        SPI_W::new(self)
    }
    #[doc = "Bit 15 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<15> {
        PCM_I2S_W::new(self)
    }
    #[doc = "Bit 17 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<17> {
        SDHOST_W::new(self)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<19> {
        UART_W::new(self)
    }
    #[doc = "Bit 21 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<21> {
        ETH_PCIE_W::new(self)
    }
    #[doc = "Bit 23 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<23> {
        VEC_W::new(self)
    }
    #[doc = "Bit 25 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<25> {
        CPG_W::new(self)
    }
    #[doc = "Bit 27 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<27> {
        RNG_W::new(self)
    }
    #[doc = "Bit 29 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<29> {
        EMMC_W::new(self)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<31> {
        ETH_PCIE_SECURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 144 - 159\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr36](index.html) module"]
pub struct GICD_ICFGR36_SPEC;
impl crate::RegisterSpec for GICD_ICFGR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr36::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR36_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr36::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR36_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR36 to value 0"]
impl crate::Resettable for GICD_ICFGR36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

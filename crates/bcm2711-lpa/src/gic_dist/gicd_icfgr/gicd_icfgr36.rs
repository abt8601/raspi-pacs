#[doc = "Register `GICD_ICFGR36` reader"]
pub type R = crate::R<GICD_ICFGR36_SPEC>;
#[doc = "Register `GICD_ICFGR36` writer"]
pub type W = crate::W<GICD_ICFGR36_SPEC>;
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
    pub const fn variant(&self) -> SMI_A {
        match self.bits {
            false => SMI_A::LEVEL,
            true => SMI_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SMI_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SMI_A::EDGE
    }
}
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMI_A>;
impl<'a, REG, const O: u8> SMI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(SMI_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> GPIO_0_A {
        match self.bits {
            false => GPIO_0_A::LEVEL,
            true => GPIO_0_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_0_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_0_A::EDGE
    }
}
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO_0_A>;
impl<'a, REG, const O: u8> GPIO_0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> GPIO_1_A {
        match self.bits {
            false => GPIO_1_A::LEVEL,
            true => GPIO_1_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_1_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_1_A::EDGE
    }
}
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO_1_A>;
impl<'a, REG, const O: u8> GPIO_1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> GPIO_2_A {
        match self.bits {
            false => GPIO_2_A::LEVEL,
            true => GPIO_2_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_2_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_2_A::EDGE
    }
}
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO_2_A>;
impl<'a, REG, const O: u8> GPIO_2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> GPIO_3_A {
        match self.bits {
            false => GPIO_3_A::LEVEL,
            true => GPIO_3_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GPIO_3_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GPIO_3_A::EDGE
    }
}
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO_3_A>;
impl<'a, REG, const O: u8> GPIO_3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::LEVEL,
            true => I2C_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == I2C_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == I2C_A::EDGE
    }
}
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C_A>;
impl<'a, REG, const O: u8> I2C_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> SPI_A {
        match self.bits {
            false => SPI_A::LEVEL,
            true => SPI_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SPI_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SPI_A::EDGE
    }
}
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPI_A>;
impl<'a, REG, const O: u8> SPI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> PCM_I2S_A {
        match self.bits {
            false => PCM_I2S_A::LEVEL,
            true => PCM_I2S_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == PCM_I2S_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == PCM_I2S_A::EDGE
    }
}
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCM_I2S_A>;
impl<'a, REG, const O: u8> PCM_I2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(PCM_I2S_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> SDHOST_A {
        match self.bits {
            false => SDHOST_A::LEVEL,
            true => SDHOST_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SDHOST_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SDHOST_A::EDGE
    }
}
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDHOST_A>;
impl<'a, REG, const O: u8> SDHOST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(SDHOST_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> UART_A {
        match self.bits {
            false => UART_A::LEVEL,
            true => UART_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == UART_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == UART_A::EDGE
    }
}
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UART_A>;
impl<'a, REG, const O: u8> UART_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(UART_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> ETH_PCIE_A {
        match self.bits {
            false => ETH_PCIE_A::LEVEL,
            true => ETH_PCIE_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ETH_PCIE_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETH_PCIE_A::EDGE
    }
}
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETH_PCIE_A>;
impl<'a, REG, const O: u8> ETH_PCIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ETH_PCIE_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> VEC_A {
        match self.bits {
            false => VEC_A::LEVEL,
            true => VEC_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == VEC_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == VEC_A::EDGE
    }
}
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VEC_A>;
impl<'a, REG, const O: u8> VEC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(VEC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CPG_A {
        match self.bits {
            false => CPG_A::LEVEL,
            true => CPG_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == CPG_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == CPG_A::EDGE
    }
}
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPG_A>;
impl<'a, REG, const O: u8> CPG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(CPG_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::LEVEL,
            true => RNG_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == RNG_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == RNG_A::EDGE
    }
}
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RNG_A>;
impl<'a, REG, const O: u8> RNG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(RNG_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> EMMC_A {
        match self.bits {
            false => EMMC_A::LEVEL,
            true => EMMC_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == EMMC_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EMMC_A::EDGE
    }
}
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EMMC_A>;
impl<'a, REG, const O: u8> EMMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(EMMC_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> ETH_PCIE_SECURE_A {
        match self.bits {
            false => ETH_PCIE_SECURE_A::LEVEL,
            true => ETH_PCIE_SECURE_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ETH_PCIE_SECURE_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETH_PCIE_SECURE_A::EDGE
    }
}
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETH_PCIE_SECURE_A>;
impl<'a, REG, const O: u8> ETH_PCIE_SECURE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ETH_PCIE_SECURE_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR36")
            .field("smi", &format_args!("{}", self.smi().bit()))
            .field("gpio_0", &format_args!("{}", self.gpio_0().bit()))
            .field("gpio_1", &format_args!("{}", self.gpio_1().bit()))
            .field("gpio_2", &format_args!("{}", self.gpio_2().bit()))
            .field("gpio_3", &format_args!("{}", self.gpio_3().bit()))
            .field("i2c", &format_args!("{}", self.i2c().bit()))
            .field("spi", &format_args!("{}", self.spi().bit()))
            .field("pcm_i2s", &format_args!("{}", self.pcm_i2s().bit()))
            .field("sdhost", &format_args!("{}", self.sdhost().bit()))
            .field("uart", &format_args!("{}", self.uart().bit()))
            .field("eth_pcie", &format_args!("{}", self.eth_pcie().bit()))
            .field("vec", &format_args!("{}", self.vec().bit()))
            .field("cpg", &format_args!("{}", self.cpg().bit()))
            .field("rng", &format_args!("{}", self.rng().bit()))
            .field("emmc", &format_args!("{}", self.emmc().bit()))
            .field(
                "eth_pcie_secure",
                &format_args!("{}", self.eth_pcie_secure().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR36_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<GICD_ICFGR36_SPEC, 1> {
        SMI_W::new(self)
    }
    #[doc = "Bit 3 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<GICD_ICFGR36_SPEC, 3> {
        GPIO_0_W::new(self)
    }
    #[doc = "Bit 5 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<GICD_ICFGR36_SPEC, 5> {
        GPIO_1_W::new(self)
    }
    #[doc = "Bit 7 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<GICD_ICFGR36_SPEC, 7> {
        GPIO_2_W::new(self)
    }
    #[doc = "Bit 9 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<GICD_ICFGR36_SPEC, 9> {
        GPIO_3_W::new(self)
    }
    #[doc = "Bit 11 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<GICD_ICFGR36_SPEC, 11> {
        I2C_W::new(self)
    }
    #[doc = "Bit 13 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<GICD_ICFGR36_SPEC, 13> {
        SPI_W::new(self)
    }
    #[doc = "Bit 15 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<GICD_ICFGR36_SPEC, 15> {
        PCM_I2S_W::new(self)
    }
    #[doc = "Bit 17 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<GICD_ICFGR36_SPEC, 17> {
        SDHOST_W::new(self)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<GICD_ICFGR36_SPEC, 19> {
        UART_W::new(self)
    }
    #[doc = "Bit 21 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<GICD_ICFGR36_SPEC, 21> {
        ETH_PCIE_W::new(self)
    }
    #[doc = "Bit 23 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<GICD_ICFGR36_SPEC, 23> {
        VEC_W::new(self)
    }
    #[doc = "Bit 25 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<GICD_ICFGR36_SPEC, 25> {
        CPG_W::new(self)
    }
    #[doc = "Bit 27 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<GICD_ICFGR36_SPEC, 27> {
        RNG_W::new(self)
    }
    #[doc = "Bit 29 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<GICD_ICFGR36_SPEC, 29> {
        EMMC_W::new(self)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<GICD_ICFGR36_SPEC, 31> {
        ETH_PCIE_SECURE_W::new(self)
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
#[doc = "Interrupt Configuration 144 - 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR36_SPEC;
impl crate::RegisterSpec for GICD_ICFGR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr36::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr36::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR36 to value 0"]
impl crate::Resettable for GICD_ICFGR36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

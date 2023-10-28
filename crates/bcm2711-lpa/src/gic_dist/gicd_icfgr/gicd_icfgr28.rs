#[doc = "Register `GICD_ICFGR28` reader"]
pub type R = crate::R<GICD_ICFGR28_SPEC>;
#[doc = "Register `GICD_ICFGR28` writer"]
pub type W = crate::W<GICD_ICFGR28_SPEC>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type DMA_0_R = crate::BitReader<DMA_0_A>;
#[doc = "DMA 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_0_A {
        match self.bits {
            false => DMA_0_A::LEVEL,
            true => DMA_0_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_0_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_0_A::EDGE
    }
}
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type DMA_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_0_A>;
impl<'a, REG, const O: u8> DMA_0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_0_A::EDGE)
    }
}
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type DMA_1_R = crate::BitReader<DMA_1_A>;
#[doc = "DMA 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_1_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_1_A {
        match self.bits {
            false => DMA_1_A::LEVEL,
            true => DMA_1_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_1_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_1_A::EDGE
    }
}
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type DMA_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_1_A>;
impl<'a, REG, const O: u8> DMA_1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_1_A::EDGE)
    }
}
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::BitReader<DMA_2_A>;
#[doc = "DMA 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_2_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_2_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_2_A {
        match self.bits {
            false => DMA_2_A::LEVEL,
            true => DMA_2_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_2_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_2_A::EDGE
    }
}
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type DMA_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_2_A>;
impl<'a, REG, const O: u8> DMA_2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_2_A::EDGE)
    }
}
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::BitReader<DMA_3_A>;
#[doc = "DMA 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_3_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_3_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_3_A {
        match self.bits {
            false => DMA_3_A::LEVEL,
            true => DMA_3_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_3_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_3_A::EDGE
    }
}
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type DMA_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_3_A>;
impl<'a, REG, const O: u8> DMA_3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_3_A::EDGE)
    }
}
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type DMA_4_R = crate::BitReader<DMA_4_A>;
#[doc = "DMA 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_4_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_4_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_4_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_4_A {
        match self.bits {
            false => DMA_4_A::LEVEL,
            true => DMA_4_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_4_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_4_A::EDGE
    }
}
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type DMA_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_4_A>;
impl<'a, REG, const O: u8> DMA_4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_4_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_4_A::EDGE)
    }
}
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type DMA_5_R = crate::BitReader<DMA_5_A>;
#[doc = "DMA 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_5_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_5_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_5_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_5_A {
        match self.bits {
            false => DMA_5_A::LEVEL,
            true => DMA_5_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_5_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_5_A::EDGE
    }
}
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type DMA_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_5_A>;
impl<'a, REG, const O: u8> DMA_5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_5_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_5_A::EDGE)
    }
}
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type DMA_6_R = crate::BitReader<DMA_6_A>;
#[doc = "DMA 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_6_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_6_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_6_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_6_A {
        match self.bits {
            false => DMA_6_A::LEVEL,
            true => DMA_6_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_6_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_6_A::EDGE
    }
}
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type DMA_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_6_A>;
impl<'a, REG, const O: u8> DMA_6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_6_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_6_A::EDGE)
    }
}
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type DMA_7_8_R = crate::BitReader<DMA_7_8_A>;
#[doc = "OR of DMA 7 and 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_7_8_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_7_8_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_7_8_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_7_8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_7_8_A {
        match self.bits {
            false => DMA_7_8_A::LEVEL,
            true => DMA_7_8_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_7_8_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_7_8_A::EDGE
    }
}
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type DMA_7_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_7_8_A>;
impl<'a, REG, const O: u8> DMA_7_8_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_7_8_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_7_8_A::EDGE)
    }
}
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type DMA_9_10_R = crate::BitReader<DMA_9_10_A>;
#[doc = "OR of DMA 9 and 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_9_10_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_9_10_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_9_10_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_9_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_9_10_A {
        match self.bits {
            false => DMA_9_10_A::LEVEL,
            true => DMA_9_10_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_9_10_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_9_10_A::EDGE
    }
}
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type DMA_9_10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_9_10_A>;
impl<'a, REG, const O: u8> DMA_9_10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_9_10_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_9_10_A::EDGE)
    }
}
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type DMA_11_R = crate::BitReader<DMA_11_A>;
#[doc = "DMA 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_11_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_11_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_11_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_11_A {
        match self.bits {
            false => DMA_11_A::LEVEL,
            true => DMA_11_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_11_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_11_A::EDGE
    }
}
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type DMA_11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_11_A>;
impl<'a, REG, const O: u8> DMA_11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_11_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_11_A::EDGE)
    }
}
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type DMA_12_R = crate::BitReader<DMA_12_A>;
#[doc = "DMA 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_12_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_12_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_12_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_12_A {
        match self.bits {
            false => DMA_12_A::LEVEL,
            true => DMA_12_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_12_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_12_A::EDGE
    }
}
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type DMA_12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_12_A>;
impl<'a, REG, const O: u8> DMA_12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_12_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_12_A::EDGE)
    }
}
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type DMA_13_R = crate::BitReader<DMA_13_A>;
#[doc = "DMA 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_13_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_13_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_13_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_13_A {
        match self.bits {
            false => DMA_13_A::LEVEL,
            true => DMA_13_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_13_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_13_A::EDGE
    }
}
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type DMA_13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_13_A>;
impl<'a, REG, const O: u8> DMA_13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_13_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_13_A::EDGE)
    }
}
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::BitReader<DMA_14_A>;
#[doc = "DMA 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_14_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_14_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_14_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_14_A {
        match self.bits {
            false => DMA_14_A::LEVEL,
            true => DMA_14_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_14_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_14_A::EDGE
    }
}
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type DMA_14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_14_A>;
impl<'a, REG, const O: u8> DMA_14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_14_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_14_A::EDGE)
    }
}
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::BitReader<AUX_A>;
#[doc = "OR of UART1, SPI1 and SPI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUX_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<AUX_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_A) -> Self {
        variant as u8 != 0
    }
}
impl AUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUX_A {
        match self.bits {
            false => AUX_A::LEVEL,
            true => AUX_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == AUX_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == AUX_A::EDGE
    }
}
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AUX_A>;
impl<'a, REG, const O: u8> AUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(AUX_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(AUX_A::EDGE)
    }
}
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::BitReader<ARM_A>;
#[doc = "ARM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ARM_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARM_A {
        match self.bits {
            false => ARM_A::LEVEL,
            true => ARM_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ARM_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ARM_A::EDGE
    }
}
#[doc = "Field `ARM` writer - ARM"]
pub type ARM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ARM_A>;
impl<'a, REG, const O: u8> ARM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ARM_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ARM_A::EDGE)
    }
}
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::BitReader<DMA_15_A>;
#[doc = "DMA 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_15_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DMA_15_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_15_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_15_A {
        match self.bits {
            false => DMA_15_A::LEVEL,
            true => DMA_15_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMA_15_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMA_15_A::EDGE
    }
}
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type DMA_15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_15_A>;
impl<'a, REG, const O: u8> DMA_15_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_15_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_15_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> DMA_7_8_R {
        DMA_7_8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> DMA_9_10_R {
        DMA_9_10_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> DMA_11_R {
        DMA_11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> DMA_12_R {
        DMA_12_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> DMA_13_R {
        DMA_13_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> DMA_14_R {
        DMA_14_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> DMA_15_R {
        DMA_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR28")
            .field("dma_0", &format_args!("{}", self.dma_0().bit()))
            .field("dma_1", &format_args!("{}", self.dma_1().bit()))
            .field("dma_2", &format_args!("{}", self.dma_2().bit()))
            .field("dma_3", &format_args!("{}", self.dma_3().bit()))
            .field("dma_4", &format_args!("{}", self.dma_4().bit()))
            .field("dma_5", &format_args!("{}", self.dma_5().bit()))
            .field("dma_6", &format_args!("{}", self.dma_6().bit()))
            .field("dma_7_8", &format_args!("{}", self.dma_7_8().bit()))
            .field("dma_9_10", &format_args!("{}", self.dma_9_10().bit()))
            .field("dma_11", &format_args!("{}", self.dma_11().bit()))
            .field("dma_12", &format_args!("{}", self.dma_12().bit()))
            .field("dma_13", &format_args!("{}", self.dma_13().bit()))
            .field("dma_14", &format_args!("{}", self.dma_14().bit()))
            .field("aux", &format_args!("{}", self.aux().bit()))
            .field("arm", &format_args!("{}", self.arm().bit()))
            .field("dma_15", &format_args!("{}", self.dma_15().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR28_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> DMA_0_W<GICD_ICFGR28_SPEC, 1> {
        DMA_0_W::new(self)
    }
    #[doc = "Bit 3 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<GICD_ICFGR28_SPEC, 3> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 5 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<GICD_ICFGR28_SPEC, 5> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 7 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<GICD_ICFGR28_SPEC, 7> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 9 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<GICD_ICFGR28_SPEC, 9> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 11 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<GICD_ICFGR28_SPEC, 11> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 13 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<GICD_ICFGR28_SPEC, 13> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 15 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> DMA_7_8_W<GICD_ICFGR28_SPEC, 15> {
        DMA_7_8_W::new(self)
    }
    #[doc = "Bit 17 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> DMA_9_10_W<GICD_ICFGR28_SPEC, 17> {
        DMA_9_10_W::new(self)
    }
    #[doc = "Bit 19 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> DMA_11_W<GICD_ICFGR28_SPEC, 19> {
        DMA_11_W::new(self)
    }
    #[doc = "Bit 21 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> DMA_12_W<GICD_ICFGR28_SPEC, 21> {
        DMA_12_W::new(self)
    }
    #[doc = "Bit 23 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> DMA_13_W<GICD_ICFGR28_SPEC, 23> {
        DMA_13_W::new(self)
    }
    #[doc = "Bit 25 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> DMA_14_W<GICD_ICFGR28_SPEC, 25> {
        DMA_14_W::new(self)
    }
    #[doc = "Bit 27 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<GICD_ICFGR28_SPEC, 27> {
        AUX_W::new(self)
    }
    #[doc = "Bit 29 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<GICD_ICFGR28_SPEC, 29> {
        ARM_W::new(self)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> DMA_15_W<GICD_ICFGR28_SPEC, 31> {
        DMA_15_W::new(self)
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
#[doc = "Interrupt Configuration 112 - 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR28_SPEC;
impl crate::RegisterSpec for GICD_ICFGR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr28::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR28_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr28::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR28_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR28 to value 0"]
impl crate::Resettable for GICD_ICFGR28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

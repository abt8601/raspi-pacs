#[doc = "Register `GICD_ICFGR8` reader"]
pub type R = crate::R<GICD_ICFGR8_SPEC>;
#[doc = "Register `GICD_ICFGR8` writer"]
pub type W = crate::W<GICD_ICFGR8_SPEC>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type INT32_R = crate::BitReader<INT32_A>;
#[doc = "Interrupt 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT32_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT32_A> for bool {
    #[inline(always)]
    fn from(variant: INT32_A) -> Self {
        variant as u8 != 0
    }
}
impl INT32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT32_A {
        match self.bits {
            false => INT32_A::LEVEL,
            true => INT32_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT32_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT32_A::EDGE
    }
}
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type INT32_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT32_A>;
impl<'a, REG, const O: u8> INT32_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT32_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT32_A::EDGE)
    }
}
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type INT33_R = crate::BitReader<INT33_A>;
#[doc = "Interrupt 33\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT33_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT33_A> for bool {
    #[inline(always)]
    fn from(variant: INT33_A) -> Self {
        variant as u8 != 0
    }
}
impl INT33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT33_A {
        match self.bits {
            false => INT33_A::LEVEL,
            true => INT33_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT33_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT33_A::EDGE
    }
}
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type INT33_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT33_A>;
impl<'a, REG, const O: u8> INT33_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT33_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT33_A::EDGE)
    }
}
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type INT34_R = crate::BitReader<INT34_A>;
#[doc = "Interrupt 34\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT34_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT34_A> for bool {
    #[inline(always)]
    fn from(variant: INT34_A) -> Self {
        variant as u8 != 0
    }
}
impl INT34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT34_A {
        match self.bits {
            false => INT34_A::LEVEL,
            true => INT34_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT34_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT34_A::EDGE
    }
}
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type INT34_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT34_A>;
impl<'a, REG, const O: u8> INT34_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT34_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT34_A::EDGE)
    }
}
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type INT35_R = crate::BitReader<INT35_A>;
#[doc = "Interrupt 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT35_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT35_A> for bool {
    #[inline(always)]
    fn from(variant: INT35_A) -> Self {
        variant as u8 != 0
    }
}
impl INT35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT35_A {
        match self.bits {
            false => INT35_A::LEVEL,
            true => INT35_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT35_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT35_A::EDGE
    }
}
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type INT35_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT35_A>;
impl<'a, REG, const O: u8> INT35_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT35_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT35_A::EDGE)
    }
}
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type INT36_R = crate::BitReader<INT36_A>;
#[doc = "Interrupt 36\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT36_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT36_A> for bool {
    #[inline(always)]
    fn from(variant: INT36_A) -> Self {
        variant as u8 != 0
    }
}
impl INT36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT36_A {
        match self.bits {
            false => INT36_A::LEVEL,
            true => INT36_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT36_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT36_A::EDGE
    }
}
#[doc = "Field `INT36` writer - Interrupt 36"]
pub type INT36_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT36_A>;
impl<'a, REG, const O: u8> INT36_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT36_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT36_A::EDGE)
    }
}
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type INT37_R = crate::BitReader<INT37_A>;
#[doc = "Interrupt 37\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT37_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT37_A> for bool {
    #[inline(always)]
    fn from(variant: INT37_A) -> Self {
        variant as u8 != 0
    }
}
impl INT37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT37_A {
        match self.bits {
            false => INT37_A::LEVEL,
            true => INT37_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT37_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT37_A::EDGE
    }
}
#[doc = "Field `INT37` writer - Interrupt 37"]
pub type INT37_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT37_A>;
impl<'a, REG, const O: u8> INT37_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT37_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT37_A::EDGE)
    }
}
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type INT38_R = crate::BitReader<INT38_A>;
#[doc = "Interrupt 38\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT38_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT38_A> for bool {
    #[inline(always)]
    fn from(variant: INT38_A) -> Self {
        variant as u8 != 0
    }
}
impl INT38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT38_A {
        match self.bits {
            false => INT38_A::LEVEL,
            true => INT38_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT38_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT38_A::EDGE
    }
}
#[doc = "Field `INT38` writer - Interrupt 38"]
pub type INT38_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT38_A>;
impl<'a, REG, const O: u8> INT38_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT38_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT38_A::EDGE)
    }
}
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type INT39_R = crate::BitReader<INT39_A>;
#[doc = "Interrupt 39\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT39_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT39_A> for bool {
    #[inline(always)]
    fn from(variant: INT39_A) -> Self {
        variant as u8 != 0
    }
}
impl INT39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT39_A {
        match self.bits {
            false => INT39_A::LEVEL,
            true => INT39_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT39_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT39_A::EDGE
    }
}
#[doc = "Field `INT39` writer - Interrupt 39"]
pub type INT39_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT39_A>;
impl<'a, REG, const O: u8> INT39_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT39_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT39_A::EDGE)
    }
}
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type INT40_R = crate::BitReader<INT40_A>;
#[doc = "Interrupt 40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT40_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT40_A> for bool {
    #[inline(always)]
    fn from(variant: INT40_A) -> Self {
        variant as u8 != 0
    }
}
impl INT40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT40_A {
        match self.bits {
            false => INT40_A::LEVEL,
            true => INT40_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT40_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT40_A::EDGE
    }
}
#[doc = "Field `INT40` writer - Interrupt 40"]
pub type INT40_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT40_A>;
impl<'a, REG, const O: u8> INT40_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT40_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT40_A::EDGE)
    }
}
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type INT41_R = crate::BitReader<INT41_A>;
#[doc = "Interrupt 41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT41_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT41_A> for bool {
    #[inline(always)]
    fn from(variant: INT41_A) -> Self {
        variant as u8 != 0
    }
}
impl INT41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT41_A {
        match self.bits {
            false => INT41_A::LEVEL,
            true => INT41_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT41_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT41_A::EDGE
    }
}
#[doc = "Field `INT41` writer - Interrupt 41"]
pub type INT41_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT41_A>;
impl<'a, REG, const O: u8> INT41_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT41_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT41_A::EDGE)
    }
}
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type INT42_R = crate::BitReader<INT42_A>;
#[doc = "Interrupt 42\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT42_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT42_A> for bool {
    #[inline(always)]
    fn from(variant: INT42_A) -> Self {
        variant as u8 != 0
    }
}
impl INT42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT42_A {
        match self.bits {
            false => INT42_A::LEVEL,
            true => INT42_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT42_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT42_A::EDGE
    }
}
#[doc = "Field `INT42` writer - Interrupt 42"]
pub type INT42_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT42_A>;
impl<'a, REG, const O: u8> INT42_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT42_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT42_A::EDGE)
    }
}
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type INT43_R = crate::BitReader<INT43_A>;
#[doc = "Interrupt 43\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT43_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT43_A> for bool {
    #[inline(always)]
    fn from(variant: INT43_A) -> Self {
        variant as u8 != 0
    }
}
impl INT43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT43_A {
        match self.bits {
            false => INT43_A::LEVEL,
            true => INT43_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT43_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT43_A::EDGE
    }
}
#[doc = "Field `INT43` writer - Interrupt 43"]
pub type INT43_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT43_A>;
impl<'a, REG, const O: u8> INT43_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT43_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT43_A::EDGE)
    }
}
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type INT44_R = crate::BitReader<INT44_A>;
#[doc = "Interrupt 44\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT44_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT44_A> for bool {
    #[inline(always)]
    fn from(variant: INT44_A) -> Self {
        variant as u8 != 0
    }
}
impl INT44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT44_A {
        match self.bits {
            false => INT44_A::LEVEL,
            true => INT44_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT44_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT44_A::EDGE
    }
}
#[doc = "Field `INT44` writer - Interrupt 44"]
pub type INT44_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT44_A>;
impl<'a, REG, const O: u8> INT44_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT44_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT44_A::EDGE)
    }
}
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type INT45_R = crate::BitReader<INT45_A>;
#[doc = "Interrupt 45\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT45_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT45_A> for bool {
    #[inline(always)]
    fn from(variant: INT45_A) -> Self {
        variant as u8 != 0
    }
}
impl INT45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT45_A {
        match self.bits {
            false => INT45_A::LEVEL,
            true => INT45_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT45_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT45_A::EDGE
    }
}
#[doc = "Field `INT45` writer - Interrupt 45"]
pub type INT45_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT45_A>;
impl<'a, REG, const O: u8> INT45_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT45_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT45_A::EDGE)
    }
}
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type INT46_R = crate::BitReader<INT46_A>;
#[doc = "Interrupt 46\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT46_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT46_A> for bool {
    #[inline(always)]
    fn from(variant: INT46_A) -> Self {
        variant as u8 != 0
    }
}
impl INT46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT46_A {
        match self.bits {
            false => INT46_A::LEVEL,
            true => INT46_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT46_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT46_A::EDGE
    }
}
#[doc = "Field `INT46` writer - Interrupt 46"]
pub type INT46_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT46_A>;
impl<'a, REG, const O: u8> INT46_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT46_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT46_A::EDGE)
    }
}
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type INT47_R = crate::BitReader<INT47_A>;
#[doc = "Interrupt 47\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT47_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT47_A> for bool {
    #[inline(always)]
    fn from(variant: INT47_A) -> Self {
        variant as u8 != 0
    }
}
impl INT47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT47_A {
        match self.bits {
            false => INT47_A::LEVEL,
            true => INT47_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT47_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT47_A::EDGE
    }
}
#[doc = "Field `INT47` writer - Interrupt 47"]
pub type INT47_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT47_A>;
impl<'a, REG, const O: u8> INT47_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT47_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT47_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> INT32_R {
        INT32_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> INT33_R {
        INT33_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> INT34_R {
        INT34_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> INT35_R {
        INT35_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> INT36_R {
        INT36_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> INT37_R {
        INT37_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> INT38_R {
        INT38_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> INT39_R {
        INT39_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> INT40_R {
        INT40_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> INT41_R {
        INT41_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> INT42_R {
        INT42_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> INT43_R {
        INT43_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> INT44_R {
        INT44_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> INT45_R {
        INT45_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> INT46_R {
        INT46_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> INT47_R {
        INT47_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR8")
            .field("int32", &format_args!("{}", self.int32().bit()))
            .field("int33", &format_args!("{}", self.int33().bit()))
            .field("int34", &format_args!("{}", self.int34().bit()))
            .field("int35", &format_args!("{}", self.int35().bit()))
            .field("int36", &format_args!("{}", self.int36().bit()))
            .field("int37", &format_args!("{}", self.int37().bit()))
            .field("int38", &format_args!("{}", self.int38().bit()))
            .field("int39", &format_args!("{}", self.int39().bit()))
            .field("int40", &format_args!("{}", self.int40().bit()))
            .field("int41", &format_args!("{}", self.int41().bit()))
            .field("int42", &format_args!("{}", self.int42().bit()))
            .field("int43", &format_args!("{}", self.int43().bit()))
            .field("int44", &format_args!("{}", self.int44().bit()))
            .field("int45", &format_args!("{}", self.int45().bit()))
            .field("int46", &format_args!("{}", self.int46().bit()))
            .field("int47", &format_args!("{}", self.int47().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> INT32_W<GICD_ICFGR8_SPEC, 1> {
        INT32_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> INT33_W<GICD_ICFGR8_SPEC, 3> {
        INT33_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> INT34_W<GICD_ICFGR8_SPEC, 5> {
        INT34_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> INT35_W<GICD_ICFGR8_SPEC, 7> {
        INT35_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn int36(&mut self) -> INT36_W<GICD_ICFGR8_SPEC, 9> {
        INT36_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn int37(&mut self) -> INT37_W<GICD_ICFGR8_SPEC, 11> {
        INT37_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn int38(&mut self) -> INT38_W<GICD_ICFGR8_SPEC, 13> {
        INT38_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn int39(&mut self) -> INT39_W<GICD_ICFGR8_SPEC, 15> {
        INT39_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn int40(&mut self) -> INT40_W<GICD_ICFGR8_SPEC, 17> {
        INT40_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn int41(&mut self) -> INT41_W<GICD_ICFGR8_SPEC, 19> {
        INT41_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn int42(&mut self) -> INT42_W<GICD_ICFGR8_SPEC, 21> {
        INT42_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn int43(&mut self) -> INT43_W<GICD_ICFGR8_SPEC, 23> {
        INT43_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn int44(&mut self) -> INT44_W<GICD_ICFGR8_SPEC, 25> {
        INT44_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn int45(&mut self) -> INT45_W<GICD_ICFGR8_SPEC, 27> {
        INT45_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn int46(&mut self) -> INT46_W<GICD_ICFGR8_SPEC, 29> {
        INT46_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn int47(&mut self) -> INT47_W<GICD_ICFGR8_SPEC, 31> {
        INT47_W::new(self)
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
#[doc = "Interrupt Configuration 32 - 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR8_SPEC;
impl crate::RegisterSpec for GICD_ICFGR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr8::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr8::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR8 to value 0"]
impl crate::Resettable for GICD_ICFGR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

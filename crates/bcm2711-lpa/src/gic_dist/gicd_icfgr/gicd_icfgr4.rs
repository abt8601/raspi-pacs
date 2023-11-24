#[doc = "Register `GICD_ICFGR4` reader"]
pub type R = crate::R<GICD_ICFGR4_SPEC>;
#[doc = "Register `GICD_ICFGR4` writer"]
pub type W = crate::W<GICD_ICFGR4_SPEC>;
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type INT16_R = crate::BitReader<INT16_A>;
#[doc = "Interrupt 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT16_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT16_A> for bool {
    #[inline(always)]
    fn from(variant: INT16_A) -> Self {
        variant as u8 != 0
    }
}
impl INT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT16_A {
        match self.bits {
            false => INT16_A::LEVEL,
            true => INT16_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT16_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT16_A::EDGE
    }
}
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type INT16_W<'a, REG> = crate::BitWriter<'a, REG, INT16_A>;
impl<'a, REG> INT16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT16_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT16_A::EDGE)
    }
}
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type INT17_R = crate::BitReader<INT17_A>;
#[doc = "Interrupt 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT17_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT17_A> for bool {
    #[inline(always)]
    fn from(variant: INT17_A) -> Self {
        variant as u8 != 0
    }
}
impl INT17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT17_A {
        match self.bits {
            false => INT17_A::LEVEL,
            true => INT17_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT17_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT17_A::EDGE
    }
}
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type INT17_W<'a, REG> = crate::BitWriter<'a, REG, INT17_A>;
impl<'a, REG> INT17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT17_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT17_A::EDGE)
    }
}
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type INT18_R = crate::BitReader<INT18_A>;
#[doc = "Interrupt 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT18_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT18_A> for bool {
    #[inline(always)]
    fn from(variant: INT18_A) -> Self {
        variant as u8 != 0
    }
}
impl INT18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT18_A {
        match self.bits {
            false => INT18_A::LEVEL,
            true => INT18_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT18_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT18_A::EDGE
    }
}
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type INT18_W<'a, REG> = crate::BitWriter<'a, REG, INT18_A>;
impl<'a, REG> INT18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT18_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT18_A::EDGE)
    }
}
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type INT19_R = crate::BitReader<INT19_A>;
#[doc = "Interrupt 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT19_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT19_A> for bool {
    #[inline(always)]
    fn from(variant: INT19_A) -> Self {
        variant as u8 != 0
    }
}
impl INT19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT19_A {
        match self.bits {
            false => INT19_A::LEVEL,
            true => INT19_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT19_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT19_A::EDGE
    }
}
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type INT19_W<'a, REG> = crate::BitWriter<'a, REG, INT19_A>;
impl<'a, REG> INT19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT19_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT19_A::EDGE)
    }
}
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type INT20_R = crate::BitReader<INT20_A>;
#[doc = "Interrupt 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT20_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT20_A> for bool {
    #[inline(always)]
    fn from(variant: INT20_A) -> Self {
        variant as u8 != 0
    }
}
impl INT20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT20_A {
        match self.bits {
            false => INT20_A::LEVEL,
            true => INT20_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT20_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT20_A::EDGE
    }
}
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type INT20_W<'a, REG> = crate::BitWriter<'a, REG, INT20_A>;
impl<'a, REG> INT20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT20_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT20_A::EDGE)
    }
}
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type INT21_R = crate::BitReader<INT21_A>;
#[doc = "Interrupt 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT21_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT21_A> for bool {
    #[inline(always)]
    fn from(variant: INT21_A) -> Self {
        variant as u8 != 0
    }
}
impl INT21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT21_A {
        match self.bits {
            false => INT21_A::LEVEL,
            true => INT21_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT21_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT21_A::EDGE
    }
}
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type INT21_W<'a, REG> = crate::BitWriter<'a, REG, INT21_A>;
impl<'a, REG> INT21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT21_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT21_A::EDGE)
    }
}
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type INT22_R = crate::BitReader<INT22_A>;
#[doc = "Interrupt 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT22_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT22_A> for bool {
    #[inline(always)]
    fn from(variant: INT22_A) -> Self {
        variant as u8 != 0
    }
}
impl INT22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT22_A {
        match self.bits {
            false => INT22_A::LEVEL,
            true => INT22_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT22_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT22_A::EDGE
    }
}
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type INT22_W<'a, REG> = crate::BitWriter<'a, REG, INT22_A>;
impl<'a, REG> INT22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT22_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT22_A::EDGE)
    }
}
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type INT23_R = crate::BitReader<INT23_A>;
#[doc = "Interrupt 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT23_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT23_A> for bool {
    #[inline(always)]
    fn from(variant: INT23_A) -> Self {
        variant as u8 != 0
    }
}
impl INT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT23_A {
        match self.bits {
            false => INT23_A::LEVEL,
            true => INT23_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT23_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT23_A::EDGE
    }
}
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type INT23_W<'a, REG> = crate::BitWriter<'a, REG, INT23_A>;
impl<'a, REG> INT23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT23_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT23_A::EDGE)
    }
}
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type INT24_R = crate::BitReader<INT24_A>;
#[doc = "Interrupt 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT24_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT24_A> for bool {
    #[inline(always)]
    fn from(variant: INT24_A) -> Self {
        variant as u8 != 0
    }
}
impl INT24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT24_A {
        match self.bits {
            false => INT24_A::LEVEL,
            true => INT24_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT24_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT24_A::EDGE
    }
}
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type INT24_W<'a, REG> = crate::BitWriter<'a, REG, INT24_A>;
impl<'a, REG> INT24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT24_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT24_A::EDGE)
    }
}
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type INT25_R = crate::BitReader<INT25_A>;
#[doc = "Interrupt 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT25_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT25_A> for bool {
    #[inline(always)]
    fn from(variant: INT25_A) -> Self {
        variant as u8 != 0
    }
}
impl INT25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT25_A {
        match self.bits {
            false => INT25_A::LEVEL,
            true => INT25_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT25_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT25_A::EDGE
    }
}
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type INT25_W<'a, REG> = crate::BitWriter<'a, REG, INT25_A>;
impl<'a, REG> INT25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT25_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT25_A::EDGE)
    }
}
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type INT26_R = crate::BitReader<INT26_A>;
#[doc = "Interrupt 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT26_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT26_A> for bool {
    #[inline(always)]
    fn from(variant: INT26_A) -> Self {
        variant as u8 != 0
    }
}
impl INT26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT26_A {
        match self.bits {
            false => INT26_A::LEVEL,
            true => INT26_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT26_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT26_A::EDGE
    }
}
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type INT26_W<'a, REG> = crate::BitWriter<'a, REG, INT26_A>;
impl<'a, REG> INT26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT26_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT26_A::EDGE)
    }
}
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type INT27_R = crate::BitReader<INT27_A>;
#[doc = "Interrupt 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT27_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT27_A> for bool {
    #[inline(always)]
    fn from(variant: INT27_A) -> Self {
        variant as u8 != 0
    }
}
impl INT27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT27_A {
        match self.bits {
            false => INT27_A::LEVEL,
            true => INT27_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT27_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT27_A::EDGE
    }
}
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type INT27_W<'a, REG> = crate::BitWriter<'a, REG, INT27_A>;
impl<'a, REG> INT27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT27_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT27_A::EDGE)
    }
}
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type INT28_R = crate::BitReader<INT28_A>;
#[doc = "Interrupt 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT28_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT28_A> for bool {
    #[inline(always)]
    fn from(variant: INT28_A) -> Self {
        variant as u8 != 0
    }
}
impl INT28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT28_A {
        match self.bits {
            false => INT28_A::LEVEL,
            true => INT28_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT28_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT28_A::EDGE
    }
}
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type INT28_W<'a, REG> = crate::BitWriter<'a, REG, INT28_A>;
impl<'a, REG> INT28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT28_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT28_A::EDGE)
    }
}
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type INT29_R = crate::BitReader<INT29_A>;
#[doc = "Interrupt 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT29_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT29_A> for bool {
    #[inline(always)]
    fn from(variant: INT29_A) -> Self {
        variant as u8 != 0
    }
}
impl INT29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT29_A {
        match self.bits {
            false => INT29_A::LEVEL,
            true => INT29_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT29_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT29_A::EDGE
    }
}
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type INT29_W<'a, REG> = crate::BitWriter<'a, REG, INT29_A>;
impl<'a, REG> INT29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT29_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT29_A::EDGE)
    }
}
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type INT30_R = crate::BitReader<INT30_A>;
#[doc = "Interrupt 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT30_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT30_A> for bool {
    #[inline(always)]
    fn from(variant: INT30_A) -> Self {
        variant as u8 != 0
    }
}
impl INT30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT30_A {
        match self.bits {
            false => INT30_A::LEVEL,
            true => INT30_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT30_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT30_A::EDGE
    }
}
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type INT30_W<'a, REG> = crate::BitWriter<'a, REG, INT30_A>;
impl<'a, REG> INT30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT30_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT30_A::EDGE)
    }
}
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type INT31_R = crate::BitReader<INT31_A>;
#[doc = "Interrupt 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT31_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT31_A> for bool {
    #[inline(always)]
    fn from(variant: INT31_A) -> Self {
        variant as u8 != 0
    }
}
impl INT31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT31_A {
        match self.bits {
            false => INT31_A::LEVEL,
            true => INT31_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT31_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT31_A::EDGE
    }
}
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type INT31_W<'a, REG> = crate::BitWriter<'a, REG, INT31_A>;
impl<'a, REG> INT31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT31_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT31_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR4")
            .field("int16", &format_args!("{}", self.int16().bit()))
            .field("int17", &format_args!("{}", self.int17().bit()))
            .field("int18", &format_args!("{}", self.int18().bit()))
            .field("int19", &format_args!("{}", self.int19().bit()))
            .field("int20", &format_args!("{}", self.int20().bit()))
            .field("int21", &format_args!("{}", self.int21().bit()))
            .field("int22", &format_args!("{}", self.int22().bit()))
            .field("int23", &format_args!("{}", self.int23().bit()))
            .field("int24", &format_args!("{}", self.int24().bit()))
            .field("int25", &format_args!("{}", self.int25().bit()))
            .field("int26", &format_args!("{}", self.int26().bit()))
            .field("int27", &format_args!("{}", self.int27().bit()))
            .field("int28", &format_args!("{}", self.int28().bit()))
            .field("int29", &format_args!("{}", self.int29().bit()))
            .field("int30", &format_args!("{}", self.int30().bit()))
            .field("int31", &format_args!("{}", self.int31().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<GICD_ICFGR4_SPEC> {
        INT16_W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> INT17_W<GICD_ICFGR4_SPEC> {
        INT17_W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> INT18_W<GICD_ICFGR4_SPEC> {
        INT18_W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> INT19_W<GICD_ICFGR4_SPEC> {
        INT19_W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> INT20_W<GICD_ICFGR4_SPEC> {
        INT20_W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> INT21_W<GICD_ICFGR4_SPEC> {
        INT21_W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> INT22_W<GICD_ICFGR4_SPEC> {
        INT22_W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> INT23_W<GICD_ICFGR4_SPEC> {
        INT23_W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> INT24_W<GICD_ICFGR4_SPEC> {
        INT24_W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> INT25_W<GICD_ICFGR4_SPEC> {
        INT25_W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> INT26_W<GICD_ICFGR4_SPEC> {
        INT26_W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> INT27_W<GICD_ICFGR4_SPEC> {
        INT27_W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> INT28_W<GICD_ICFGR4_SPEC> {
        INT28_W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> INT29_W<GICD_ICFGR4_SPEC> {
        INT29_W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> INT30_W<GICD_ICFGR4_SPEC> {
        INT30_W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> INT31_W<GICD_ICFGR4_SPEC> {
        INT31_W::new(self, 31)
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
#[doc = "Interrupt Configuration 16 - 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR4_SPEC;
impl crate::RegisterSpec for GICD_ICFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr4::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr4::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR4 to value 0"]
impl crate::Resettable for GICD_ICFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

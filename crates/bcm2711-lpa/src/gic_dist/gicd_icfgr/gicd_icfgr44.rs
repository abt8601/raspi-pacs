#[doc = "Register `GICD_ICFGR44` reader"]
pub type R = crate::R<GICD_ICFGR44_SPEC>;
#[doc = "Register `GICD_ICFGR44` writer"]
pub type W = crate::W<GICD_ICFGR44_SPEC>;
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type INT176_R = crate::BitReader<INT176_A>;
#[doc = "Interrupt 176\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT176_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT176_A> for bool {
    #[inline(always)]
    fn from(variant: INT176_A) -> Self {
        variant as u8 != 0
    }
}
impl INT176_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT176_A {
        match self.bits {
            false => INT176_A::LEVEL,
            true => INT176_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT176_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT176_A::EDGE
    }
}
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type INT176_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT176_A>;
impl<'a, REG, const O: u8> INT176_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT176_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT176_A::EDGE)
    }
}
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type INT177_R = crate::BitReader<INT177_A>;
#[doc = "Interrupt 177\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT177_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT177_A> for bool {
    #[inline(always)]
    fn from(variant: INT177_A) -> Self {
        variant as u8 != 0
    }
}
impl INT177_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT177_A {
        match self.bits {
            false => INT177_A::LEVEL,
            true => INT177_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT177_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT177_A::EDGE
    }
}
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type INT177_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT177_A>;
impl<'a, REG, const O: u8> INT177_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT177_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT177_A::EDGE)
    }
}
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type INT178_R = crate::BitReader<INT178_A>;
#[doc = "Interrupt 178\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT178_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT178_A> for bool {
    #[inline(always)]
    fn from(variant: INT178_A) -> Self {
        variant as u8 != 0
    }
}
impl INT178_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT178_A {
        match self.bits {
            false => INT178_A::LEVEL,
            true => INT178_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT178_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT178_A::EDGE
    }
}
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type INT178_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT178_A>;
impl<'a, REG, const O: u8> INT178_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT178_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT178_A::EDGE)
    }
}
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type INT179_R = crate::BitReader<INT179_A>;
#[doc = "Interrupt 179\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT179_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT179_A> for bool {
    #[inline(always)]
    fn from(variant: INT179_A) -> Self {
        variant as u8 != 0
    }
}
impl INT179_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT179_A {
        match self.bits {
            false => INT179_A::LEVEL,
            true => INT179_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT179_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT179_A::EDGE
    }
}
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type INT179_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT179_A>;
impl<'a, REG, const O: u8> INT179_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT179_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT179_A::EDGE)
    }
}
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type INT180_R = crate::BitReader<INT180_A>;
#[doc = "Interrupt 180\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT180_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT180_A> for bool {
    #[inline(always)]
    fn from(variant: INT180_A) -> Self {
        variant as u8 != 0
    }
}
impl INT180_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT180_A {
        match self.bits {
            false => INT180_A::LEVEL,
            true => INT180_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT180_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT180_A::EDGE
    }
}
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type INT180_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT180_A>;
impl<'a, REG, const O: u8> INT180_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT180_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT180_A::EDGE)
    }
}
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type INT181_R = crate::BitReader<INT181_A>;
#[doc = "Interrupt 181\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT181_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT181_A> for bool {
    #[inline(always)]
    fn from(variant: INT181_A) -> Self {
        variant as u8 != 0
    }
}
impl INT181_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT181_A {
        match self.bits {
            false => INT181_A::LEVEL,
            true => INT181_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT181_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT181_A::EDGE
    }
}
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type INT181_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT181_A>;
impl<'a, REG, const O: u8> INT181_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT181_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT181_A::EDGE)
    }
}
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type INT182_R = crate::BitReader<INT182_A>;
#[doc = "Interrupt 182\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT182_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT182_A> for bool {
    #[inline(always)]
    fn from(variant: INT182_A) -> Self {
        variant as u8 != 0
    }
}
impl INT182_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT182_A {
        match self.bits {
            false => INT182_A::LEVEL,
            true => INT182_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT182_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT182_A::EDGE
    }
}
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type INT182_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT182_A>;
impl<'a, REG, const O: u8> INT182_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT182_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT182_A::EDGE)
    }
}
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type INT183_R = crate::BitReader<INT183_A>;
#[doc = "Interrupt 183\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT183_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT183_A> for bool {
    #[inline(always)]
    fn from(variant: INT183_A) -> Self {
        variant as u8 != 0
    }
}
impl INT183_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT183_A {
        match self.bits {
            false => INT183_A::LEVEL,
            true => INT183_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT183_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT183_A::EDGE
    }
}
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type INT183_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT183_A>;
impl<'a, REG, const O: u8> INT183_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT183_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT183_A::EDGE)
    }
}
#[doc = "Field `INT184` reader - Interrupt 184"]
pub type INT184_R = crate::BitReader<INT184_A>;
#[doc = "Interrupt 184\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT184_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT184_A> for bool {
    #[inline(always)]
    fn from(variant: INT184_A) -> Self {
        variant as u8 != 0
    }
}
impl INT184_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT184_A {
        match self.bits {
            false => INT184_A::LEVEL,
            true => INT184_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT184_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT184_A::EDGE
    }
}
#[doc = "Field `INT184` writer - Interrupt 184"]
pub type INT184_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT184_A>;
impl<'a, REG, const O: u8> INT184_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT184_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT184_A::EDGE)
    }
}
#[doc = "Field `INT185` reader - Interrupt 185"]
pub type INT185_R = crate::BitReader<INT185_A>;
#[doc = "Interrupt 185\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT185_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT185_A> for bool {
    #[inline(always)]
    fn from(variant: INT185_A) -> Self {
        variant as u8 != 0
    }
}
impl INT185_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT185_A {
        match self.bits {
            false => INT185_A::LEVEL,
            true => INT185_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT185_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT185_A::EDGE
    }
}
#[doc = "Field `INT185` writer - Interrupt 185"]
pub type INT185_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT185_A>;
impl<'a, REG, const O: u8> INT185_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT185_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT185_A::EDGE)
    }
}
#[doc = "Field `INT186` reader - Interrupt 186"]
pub type INT186_R = crate::BitReader<INT186_A>;
#[doc = "Interrupt 186\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT186_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT186_A> for bool {
    #[inline(always)]
    fn from(variant: INT186_A) -> Self {
        variant as u8 != 0
    }
}
impl INT186_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT186_A {
        match self.bits {
            false => INT186_A::LEVEL,
            true => INT186_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT186_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT186_A::EDGE
    }
}
#[doc = "Field `INT186` writer - Interrupt 186"]
pub type INT186_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT186_A>;
impl<'a, REG, const O: u8> INT186_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT186_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT186_A::EDGE)
    }
}
#[doc = "Field `INT187` reader - Interrupt 187"]
pub type INT187_R = crate::BitReader<INT187_A>;
#[doc = "Interrupt 187\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT187_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT187_A> for bool {
    #[inline(always)]
    fn from(variant: INT187_A) -> Self {
        variant as u8 != 0
    }
}
impl INT187_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT187_A {
        match self.bits {
            false => INT187_A::LEVEL,
            true => INT187_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT187_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT187_A::EDGE
    }
}
#[doc = "Field `INT187` writer - Interrupt 187"]
pub type INT187_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT187_A>;
impl<'a, REG, const O: u8> INT187_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT187_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT187_A::EDGE)
    }
}
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type INT188_R = crate::BitReader<INT188_A>;
#[doc = "Interrupt 188\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT188_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT188_A> for bool {
    #[inline(always)]
    fn from(variant: INT188_A) -> Self {
        variant as u8 != 0
    }
}
impl INT188_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT188_A {
        match self.bits {
            false => INT188_A::LEVEL,
            true => INT188_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT188_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT188_A::EDGE
    }
}
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type INT188_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT188_A>;
impl<'a, REG, const O: u8> INT188_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT188_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT188_A::EDGE)
    }
}
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type INT189_R = crate::BitReader<INT189_A>;
#[doc = "Interrupt 189\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT189_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT189_A> for bool {
    #[inline(always)]
    fn from(variant: INT189_A) -> Self {
        variant as u8 != 0
    }
}
impl INT189_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT189_A {
        match self.bits {
            false => INT189_A::LEVEL,
            true => INT189_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT189_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT189_A::EDGE
    }
}
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type INT189_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT189_A>;
impl<'a, REG, const O: u8> INT189_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT189_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT189_A::EDGE)
    }
}
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type INT190_R = crate::BitReader<INT190_A>;
#[doc = "Interrupt 190\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT190_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT190_A> for bool {
    #[inline(always)]
    fn from(variant: INT190_A) -> Self {
        variant as u8 != 0
    }
}
impl INT190_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT190_A {
        match self.bits {
            false => INT190_A::LEVEL,
            true => INT190_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT190_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT190_A::EDGE
    }
}
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type INT190_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT190_A>;
impl<'a, REG, const O: u8> INT190_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT190_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT190_A::EDGE)
    }
}
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type INT191_R = crate::BitReader<INT191_A>;
#[doc = "Interrupt 191\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT191_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT191_A> for bool {
    #[inline(always)]
    fn from(variant: INT191_A) -> Self {
        variant as u8 != 0
    }
}
impl INT191_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT191_A {
        match self.bits {
            false => INT191_A::LEVEL,
            true => INT191_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT191_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT191_A::EDGE
    }
}
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type INT191_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT191_A>;
impl<'a, REG, const O: u8> INT191_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT191_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT191_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> INT176_R {
        INT176_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> INT177_R {
        INT177_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> INT178_R {
        INT178_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> INT179_R {
        INT179_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> INT180_R {
        INT180_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> INT181_R {
        INT181_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> INT182_R {
        INT182_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> INT183_R {
        INT183_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 184"]
    #[inline(always)]
    pub fn int184(&self) -> INT184_R {
        INT184_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 185"]
    #[inline(always)]
    pub fn int185(&self) -> INT185_R {
        INT185_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 186"]
    #[inline(always)]
    pub fn int186(&self) -> INT186_R {
        INT186_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 187"]
    #[inline(always)]
    pub fn int187(&self) -> INT187_R {
        INT187_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> INT188_R {
        INT188_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> INT189_R {
        INT189_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> INT190_R {
        INT190_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> INT191_R {
        INT191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR44")
            .field("int176", &format_args!("{}", self.int176().bit()))
            .field("int177", &format_args!("{}", self.int177().bit()))
            .field("int178", &format_args!("{}", self.int178().bit()))
            .field("int179", &format_args!("{}", self.int179().bit()))
            .field("int180", &format_args!("{}", self.int180().bit()))
            .field("int181", &format_args!("{}", self.int181().bit()))
            .field("int182", &format_args!("{}", self.int182().bit()))
            .field("int183", &format_args!("{}", self.int183().bit()))
            .field("int184", &format_args!("{}", self.int184().bit()))
            .field("int185", &format_args!("{}", self.int185().bit()))
            .field("int186", &format_args!("{}", self.int186().bit()))
            .field("int187", &format_args!("{}", self.int187().bit()))
            .field("int188", &format_args!("{}", self.int188().bit()))
            .field("int189", &format_args!("{}", self.int189().bit()))
            .field("int190", &format_args!("{}", self.int190().bit()))
            .field("int191", &format_args!("{}", self.int191().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR44_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> INT176_W<GICD_ICFGR44_SPEC, 1> {
        INT176_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> INT177_W<GICD_ICFGR44_SPEC, 3> {
        INT177_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> INT178_W<GICD_ICFGR44_SPEC, 5> {
        INT178_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> INT179_W<GICD_ICFGR44_SPEC, 7> {
        INT179_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> INT180_W<GICD_ICFGR44_SPEC, 9> {
        INT180_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> INT181_W<GICD_ICFGR44_SPEC, 11> {
        INT181_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> INT182_W<GICD_ICFGR44_SPEC, 13> {
        INT182_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> INT183_W<GICD_ICFGR44_SPEC, 15> {
        INT183_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn int184(&mut self) -> INT184_W<GICD_ICFGR44_SPEC, 17> {
        INT184_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn int185(&mut self) -> INT185_W<GICD_ICFGR44_SPEC, 19> {
        INT185_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn int186(&mut self) -> INT186_W<GICD_ICFGR44_SPEC, 21> {
        INT186_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn int187(&mut self) -> INT187_W<GICD_ICFGR44_SPEC, 23> {
        INT187_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> INT188_W<GICD_ICFGR44_SPEC, 25> {
        INT188_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> INT189_W<GICD_ICFGR44_SPEC, 27> {
        INT189_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> INT190_W<GICD_ICFGR44_SPEC, 29> {
        INT190_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> INT191_W<GICD_ICFGR44_SPEC, 31> {
        INT191_W::new(self)
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
#[doc = "Interrupt Configuration 176 - 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR44_SPEC;
impl crate::RegisterSpec for GICD_ICFGR44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr44::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR44_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr44::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR44_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR44 to value 0"]
impl crate::Resettable for GICD_ICFGR44_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_ICFGR52` reader"]
pub type R = crate::R<GICD_ICFGR52_SPEC>;
#[doc = "Register `GICD_ICFGR52` writer"]
pub type W = crate::W<GICD_ICFGR52_SPEC>;
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type INT208_R = crate::BitReader<INT208_A>;
#[doc = "Interrupt 208\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT208_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT208_A> for bool {
    #[inline(always)]
    fn from(variant: INT208_A) -> Self {
        variant as u8 != 0
    }
}
impl INT208_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT208_A {
        match self.bits {
            false => INT208_A::LEVEL,
            true => INT208_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT208_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT208_A::EDGE
    }
}
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type INT208_W<'a, REG> = crate::BitWriter<'a, REG, INT208_A>;
impl<'a, REG> INT208_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT208_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT208_A::EDGE)
    }
}
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type INT209_R = crate::BitReader<INT209_A>;
#[doc = "Interrupt 209\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT209_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT209_A> for bool {
    #[inline(always)]
    fn from(variant: INT209_A) -> Self {
        variant as u8 != 0
    }
}
impl INT209_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT209_A {
        match self.bits {
            false => INT209_A::LEVEL,
            true => INT209_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT209_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT209_A::EDGE
    }
}
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type INT209_W<'a, REG> = crate::BitWriter<'a, REG, INT209_A>;
impl<'a, REG> INT209_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT209_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT209_A::EDGE)
    }
}
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type INT210_R = crate::BitReader<INT210_A>;
#[doc = "Interrupt 210\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT210_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT210_A> for bool {
    #[inline(always)]
    fn from(variant: INT210_A) -> Self {
        variant as u8 != 0
    }
}
impl INT210_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT210_A {
        match self.bits {
            false => INT210_A::LEVEL,
            true => INT210_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT210_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT210_A::EDGE
    }
}
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type INT210_W<'a, REG> = crate::BitWriter<'a, REG, INT210_A>;
impl<'a, REG> INT210_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT210_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT210_A::EDGE)
    }
}
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type INT211_R = crate::BitReader<INT211_A>;
#[doc = "Interrupt 211\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT211_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT211_A> for bool {
    #[inline(always)]
    fn from(variant: INT211_A) -> Self {
        variant as u8 != 0
    }
}
impl INT211_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT211_A {
        match self.bits {
            false => INT211_A::LEVEL,
            true => INT211_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT211_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT211_A::EDGE
    }
}
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type INT211_W<'a, REG> = crate::BitWriter<'a, REG, INT211_A>;
impl<'a, REG> INT211_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT211_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT211_A::EDGE)
    }
}
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type INT212_R = crate::BitReader<INT212_A>;
#[doc = "Interrupt 212\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT212_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT212_A> for bool {
    #[inline(always)]
    fn from(variant: INT212_A) -> Self {
        variant as u8 != 0
    }
}
impl INT212_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT212_A {
        match self.bits {
            false => INT212_A::LEVEL,
            true => INT212_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT212_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT212_A::EDGE
    }
}
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type INT212_W<'a, REG> = crate::BitWriter<'a, REG, INT212_A>;
impl<'a, REG> INT212_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT212_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT212_A::EDGE)
    }
}
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type INT213_R = crate::BitReader<INT213_A>;
#[doc = "Interrupt 213\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT213_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT213_A> for bool {
    #[inline(always)]
    fn from(variant: INT213_A) -> Self {
        variant as u8 != 0
    }
}
impl INT213_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT213_A {
        match self.bits {
            false => INT213_A::LEVEL,
            true => INT213_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT213_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT213_A::EDGE
    }
}
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type INT213_W<'a, REG> = crate::BitWriter<'a, REG, INT213_A>;
impl<'a, REG> INT213_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT213_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT213_A::EDGE)
    }
}
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type INT214_R = crate::BitReader<INT214_A>;
#[doc = "Interrupt 214\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT214_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT214_A> for bool {
    #[inline(always)]
    fn from(variant: INT214_A) -> Self {
        variant as u8 != 0
    }
}
impl INT214_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT214_A {
        match self.bits {
            false => INT214_A::LEVEL,
            true => INT214_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT214_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT214_A::EDGE
    }
}
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type INT214_W<'a, REG> = crate::BitWriter<'a, REG, INT214_A>;
impl<'a, REG> INT214_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT214_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT214_A::EDGE)
    }
}
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type INT215_R = crate::BitReader<INT215_A>;
#[doc = "Interrupt 215\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT215_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT215_A> for bool {
    #[inline(always)]
    fn from(variant: INT215_A) -> Self {
        variant as u8 != 0
    }
}
impl INT215_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT215_A {
        match self.bits {
            false => INT215_A::LEVEL,
            true => INT215_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT215_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT215_A::EDGE
    }
}
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type INT215_W<'a, REG> = crate::BitWriter<'a, REG, INT215_A>;
impl<'a, REG> INT215_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT215_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT215_A::EDGE)
    }
}
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type INT216_R = crate::BitReader<INT216_A>;
#[doc = "Interrupt 216\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT216_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT216_A> for bool {
    #[inline(always)]
    fn from(variant: INT216_A) -> Self {
        variant as u8 != 0
    }
}
impl INT216_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT216_A {
        match self.bits {
            false => INT216_A::LEVEL,
            true => INT216_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT216_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT216_A::EDGE
    }
}
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type INT216_W<'a, REG> = crate::BitWriter<'a, REG, INT216_A>;
impl<'a, REG> INT216_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT216_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT216_A::EDGE)
    }
}
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type INT217_R = crate::BitReader<INT217_A>;
#[doc = "Interrupt 217\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT217_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT217_A> for bool {
    #[inline(always)]
    fn from(variant: INT217_A) -> Self {
        variant as u8 != 0
    }
}
impl INT217_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT217_A {
        match self.bits {
            false => INT217_A::LEVEL,
            true => INT217_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT217_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT217_A::EDGE
    }
}
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type INT217_W<'a, REG> = crate::BitWriter<'a, REG, INT217_A>;
impl<'a, REG> INT217_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT217_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT217_A::EDGE)
    }
}
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type INT218_R = crate::BitReader<INT218_A>;
#[doc = "Interrupt 218\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT218_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT218_A> for bool {
    #[inline(always)]
    fn from(variant: INT218_A) -> Self {
        variant as u8 != 0
    }
}
impl INT218_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT218_A {
        match self.bits {
            false => INT218_A::LEVEL,
            true => INT218_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT218_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT218_A::EDGE
    }
}
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type INT218_W<'a, REG> = crate::BitWriter<'a, REG, INT218_A>;
impl<'a, REG> INT218_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT218_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT218_A::EDGE)
    }
}
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type INT219_R = crate::BitReader<INT219_A>;
#[doc = "Interrupt 219\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT219_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT219_A> for bool {
    #[inline(always)]
    fn from(variant: INT219_A) -> Self {
        variant as u8 != 0
    }
}
impl INT219_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT219_A {
        match self.bits {
            false => INT219_A::LEVEL,
            true => INT219_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT219_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT219_A::EDGE
    }
}
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type INT219_W<'a, REG> = crate::BitWriter<'a, REG, INT219_A>;
impl<'a, REG> INT219_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT219_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT219_A::EDGE)
    }
}
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type INT220_R = crate::BitReader<INT220_A>;
#[doc = "Interrupt 220\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT220_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT220_A> for bool {
    #[inline(always)]
    fn from(variant: INT220_A) -> Self {
        variant as u8 != 0
    }
}
impl INT220_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT220_A {
        match self.bits {
            false => INT220_A::LEVEL,
            true => INT220_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT220_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT220_A::EDGE
    }
}
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type INT220_W<'a, REG> = crate::BitWriter<'a, REG, INT220_A>;
impl<'a, REG> INT220_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT220_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT220_A::EDGE)
    }
}
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type INT221_R = crate::BitReader<INT221_A>;
#[doc = "Interrupt 221\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT221_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT221_A> for bool {
    #[inline(always)]
    fn from(variant: INT221_A) -> Self {
        variant as u8 != 0
    }
}
impl INT221_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT221_A {
        match self.bits {
            false => INT221_A::LEVEL,
            true => INT221_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT221_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT221_A::EDGE
    }
}
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type INT221_W<'a, REG> = crate::BitWriter<'a, REG, INT221_A>;
impl<'a, REG> INT221_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT221_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT221_A::EDGE)
    }
}
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type INT222_R = crate::BitReader<INT222_A>;
#[doc = "Interrupt 222\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT222_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT222_A> for bool {
    #[inline(always)]
    fn from(variant: INT222_A) -> Self {
        variant as u8 != 0
    }
}
impl INT222_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT222_A {
        match self.bits {
            false => INT222_A::LEVEL,
            true => INT222_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT222_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT222_A::EDGE
    }
}
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type INT222_W<'a, REG> = crate::BitWriter<'a, REG, INT222_A>;
impl<'a, REG> INT222_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT222_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT222_A::EDGE)
    }
}
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type INT223_R = crate::BitReader<INT223_A>;
#[doc = "Interrupt 223\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT223_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT223_A> for bool {
    #[inline(always)]
    fn from(variant: INT223_A) -> Self {
        variant as u8 != 0
    }
}
impl INT223_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT223_A {
        match self.bits {
            false => INT223_A::LEVEL,
            true => INT223_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT223_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT223_A::EDGE
    }
}
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type INT223_W<'a, REG> = crate::BitWriter<'a, REG, INT223_A>;
impl<'a, REG> INT223_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INT223_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INT223_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> INT208_R {
        INT208_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> INT209_R {
        INT209_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> INT210_R {
        INT210_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> INT211_R {
        INT211_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> INT212_R {
        INT212_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> INT213_R {
        INT213_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> INT214_R {
        INT214_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> INT215_R {
        INT215_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> INT216_R {
        INT216_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> INT217_R {
        INT217_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> INT218_R {
        INT218_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> INT219_R {
        INT219_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> INT220_R {
        INT220_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> INT221_R {
        INT221_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> INT222_R {
        INT222_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> INT223_R {
        INT223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR52")
            .field("int208", &format_args!("{}", self.int208().bit()))
            .field("int209", &format_args!("{}", self.int209().bit()))
            .field("int210", &format_args!("{}", self.int210().bit()))
            .field("int211", &format_args!("{}", self.int211().bit()))
            .field("int212", &format_args!("{}", self.int212().bit()))
            .field("int213", &format_args!("{}", self.int213().bit()))
            .field("int214", &format_args!("{}", self.int214().bit()))
            .field("int215", &format_args!("{}", self.int215().bit()))
            .field("int216", &format_args!("{}", self.int216().bit()))
            .field("int217", &format_args!("{}", self.int217().bit()))
            .field("int218", &format_args!("{}", self.int218().bit()))
            .field("int219", &format_args!("{}", self.int219().bit()))
            .field("int220", &format_args!("{}", self.int220().bit()))
            .field("int221", &format_args!("{}", self.int221().bit()))
            .field("int222", &format_args!("{}", self.int222().bit()))
            .field("int223", &format_args!("{}", self.int223().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR52_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> INT208_W<GICD_ICFGR52_SPEC> {
        INT208_W::new(self, 1)
    }
    #[doc = "Bit 3 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> INT209_W<GICD_ICFGR52_SPEC> {
        INT209_W::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> INT210_W<GICD_ICFGR52_SPEC> {
        INT210_W::new(self, 5)
    }
    #[doc = "Bit 7 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> INT211_W<GICD_ICFGR52_SPEC> {
        INT211_W::new(self, 7)
    }
    #[doc = "Bit 9 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> INT212_W<GICD_ICFGR52_SPEC> {
        INT212_W::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> INT213_W<GICD_ICFGR52_SPEC> {
        INT213_W::new(self, 11)
    }
    #[doc = "Bit 13 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> INT214_W<GICD_ICFGR52_SPEC> {
        INT214_W::new(self, 13)
    }
    #[doc = "Bit 15 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> INT215_W<GICD_ICFGR52_SPEC> {
        INT215_W::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> INT216_W<GICD_ICFGR52_SPEC> {
        INT216_W::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> INT217_W<GICD_ICFGR52_SPEC> {
        INT217_W::new(self, 19)
    }
    #[doc = "Bit 21 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> INT218_W<GICD_ICFGR52_SPEC> {
        INT218_W::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> INT219_W<GICD_ICFGR52_SPEC> {
        INT219_W::new(self, 23)
    }
    #[doc = "Bit 25 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> INT220_W<GICD_ICFGR52_SPEC> {
        INT220_W::new(self, 25)
    }
    #[doc = "Bit 27 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> INT221_W<GICD_ICFGR52_SPEC> {
        INT221_W::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> INT222_W<GICD_ICFGR52_SPEC> {
        INT222_W::new(self, 29)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> INT223_W<GICD_ICFGR52_SPEC> {
        INT223_W::new(self, 31)
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
#[doc = "Interrupt Configuration 208 - 223\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR52_SPEC;
impl crate::RegisterSpec for GICD_ICFGR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr52::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr52::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR52_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR52 to value 0"]
impl crate::Resettable for GICD_ICFGR52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

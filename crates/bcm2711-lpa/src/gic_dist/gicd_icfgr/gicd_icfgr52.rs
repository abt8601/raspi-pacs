#[doc = "Register `GICD_ICFGR52` reader"]
pub struct R(crate::R<GICD_ICFGR52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR52` writer"]
pub struct W(crate::W<GICD_ICFGR52_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR52_SPEC>;
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
impl From<crate::W<GICD_ICFGR52_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR52_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> INT208_A {
        match self.bits {
            false => INT208_A::LEVEL,
            true => INT208_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT208_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT208_A::EDGE
    }
}
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type INT208_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT208_A, O>;
impl<'a, const O: u8> INT208_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT208_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT209_A {
        match self.bits {
            false => INT209_A::LEVEL,
            true => INT209_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT209_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT209_A::EDGE
    }
}
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type INT209_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT209_A, O>;
impl<'a, const O: u8> INT209_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT209_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT210_A {
        match self.bits {
            false => INT210_A::LEVEL,
            true => INT210_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT210_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT210_A::EDGE
    }
}
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type INT210_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT210_A, O>;
impl<'a, const O: u8> INT210_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT210_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT211_A {
        match self.bits {
            false => INT211_A::LEVEL,
            true => INT211_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT211_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT211_A::EDGE
    }
}
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type INT211_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT211_A, O>;
impl<'a, const O: u8> INT211_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT211_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT212_A {
        match self.bits {
            false => INT212_A::LEVEL,
            true => INT212_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT212_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT212_A::EDGE
    }
}
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type INT212_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT212_A, O>;
impl<'a, const O: u8> INT212_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT212_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT213_A {
        match self.bits {
            false => INT213_A::LEVEL,
            true => INT213_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT213_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT213_A::EDGE
    }
}
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type INT213_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT213_A, O>;
impl<'a, const O: u8> INT213_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT213_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT214_A {
        match self.bits {
            false => INT214_A::LEVEL,
            true => INT214_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT214_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT214_A::EDGE
    }
}
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type INT214_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT214_A, O>;
impl<'a, const O: u8> INT214_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT214_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT215_A {
        match self.bits {
            false => INT215_A::LEVEL,
            true => INT215_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT215_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT215_A::EDGE
    }
}
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type INT215_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT215_A, O>;
impl<'a, const O: u8> INT215_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT215_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT216_A {
        match self.bits {
            false => INT216_A::LEVEL,
            true => INT216_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT216_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT216_A::EDGE
    }
}
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type INT216_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT216_A, O>;
impl<'a, const O: u8> INT216_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT216_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT217_A {
        match self.bits {
            false => INT217_A::LEVEL,
            true => INT217_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT217_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT217_A::EDGE
    }
}
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type INT217_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT217_A, O>;
impl<'a, const O: u8> INT217_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT217_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT218_A {
        match self.bits {
            false => INT218_A::LEVEL,
            true => INT218_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT218_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT218_A::EDGE
    }
}
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type INT218_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT218_A, O>;
impl<'a, const O: u8> INT218_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT218_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT219_A {
        match self.bits {
            false => INT219_A::LEVEL,
            true => INT219_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT219_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT219_A::EDGE
    }
}
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type INT219_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT219_A, O>;
impl<'a, const O: u8> INT219_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT219_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT220_A {
        match self.bits {
            false => INT220_A::LEVEL,
            true => INT220_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT220_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT220_A::EDGE
    }
}
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type INT220_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT220_A, O>;
impl<'a, const O: u8> INT220_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT220_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT221_A {
        match self.bits {
            false => INT221_A::LEVEL,
            true => INT221_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT221_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT221_A::EDGE
    }
}
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type INT221_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT221_A, O>;
impl<'a, const O: u8> INT221_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT221_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT222_A {
        match self.bits {
            false => INT222_A::LEVEL,
            true => INT222_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT222_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT222_A::EDGE
    }
}
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type INT222_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT222_A, O>;
impl<'a, const O: u8> INT222_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT222_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT223_A {
        match self.bits {
            false => INT223_A::LEVEL,
            true => INT223_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT223_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT223_A::EDGE
    }
}
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type INT223_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR52_SPEC, INT223_A, O>;
impl<'a, const O: u8> INT223_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT223_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
impl W {
    #[doc = "Bit 1 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> INT208_W<1> {
        INT208_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> INT209_W<3> {
        INT209_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> INT210_W<5> {
        INT210_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> INT211_W<7> {
        INT211_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> INT212_W<9> {
        INT212_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> INT213_W<11> {
        INT213_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> INT214_W<13> {
        INT214_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> INT215_W<15> {
        INT215_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> INT216_W<17> {
        INT216_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> INT217_W<19> {
        INT217_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> INT218_W<21> {
        INT218_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> INT219_W<23> {
        INT219_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> INT220_W<25> {
        INT220_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> INT221_W<27> {
        INT221_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> INT222_W<29> {
        INT222_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> INT223_W<31> {
        INT223_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 208 - 223\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr52](index.html) module"]
pub struct GICD_ICFGR52_SPEC;
impl crate::RegisterSpec for GICD_ICFGR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr52::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR52_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr52::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR52_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR52 to value 0"]
impl crate::Resettable for GICD_ICFGR52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_ICFGR4` reader"]
pub struct R(crate::R<GICD_ICFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR4` writer"]
pub struct W(crate::W<GICD_ICFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR4_SPEC>;
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
impl From<crate::W<GICD_ICFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR4_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> INT16_A {
        match self.bits {
            false => INT16_A::LEVEL,
            true => INT16_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT16_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT16_A::EDGE
    }
}
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type INT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT16_A, O>;
impl<'a, const O: u8> INT16_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT16_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT17_A {
        match self.bits {
            false => INT17_A::LEVEL,
            true => INT17_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT17_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT17_A::EDGE
    }
}
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type INT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT17_A, O>;
impl<'a, const O: u8> INT17_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT17_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT18_A {
        match self.bits {
            false => INT18_A::LEVEL,
            true => INT18_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT18_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT18_A::EDGE
    }
}
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type INT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT18_A, O>;
impl<'a, const O: u8> INT18_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT18_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT19_A {
        match self.bits {
            false => INT19_A::LEVEL,
            true => INT19_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT19_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT19_A::EDGE
    }
}
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type INT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT19_A, O>;
impl<'a, const O: u8> INT19_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT19_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT20_A {
        match self.bits {
            false => INT20_A::LEVEL,
            true => INT20_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT20_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT20_A::EDGE
    }
}
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type INT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT20_A, O>;
impl<'a, const O: u8> INT20_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT20_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT21_A {
        match self.bits {
            false => INT21_A::LEVEL,
            true => INT21_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT21_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT21_A::EDGE
    }
}
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type INT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT21_A, O>;
impl<'a, const O: u8> INT21_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT21_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT22_A {
        match self.bits {
            false => INT22_A::LEVEL,
            true => INT22_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT22_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT22_A::EDGE
    }
}
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type INT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT22_A, O>;
impl<'a, const O: u8> INT22_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT22_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT23_A {
        match self.bits {
            false => INT23_A::LEVEL,
            true => INT23_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT23_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT23_A::EDGE
    }
}
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type INT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT23_A, O>;
impl<'a, const O: u8> INT23_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT23_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT24_A {
        match self.bits {
            false => INT24_A::LEVEL,
            true => INT24_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT24_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT24_A::EDGE
    }
}
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type INT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT24_A, O>;
impl<'a, const O: u8> INT24_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT24_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT25_A {
        match self.bits {
            false => INT25_A::LEVEL,
            true => INT25_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT25_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT25_A::EDGE
    }
}
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type INT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT25_A, O>;
impl<'a, const O: u8> INT25_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT25_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT26_A {
        match self.bits {
            false => INT26_A::LEVEL,
            true => INT26_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT26_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT26_A::EDGE
    }
}
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type INT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT26_A, O>;
impl<'a, const O: u8> INT26_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT26_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT27_A {
        match self.bits {
            false => INT27_A::LEVEL,
            true => INT27_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT27_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT27_A::EDGE
    }
}
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type INT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT27_A, O>;
impl<'a, const O: u8> INT27_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT27_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT28_A {
        match self.bits {
            false => INT28_A::LEVEL,
            true => INT28_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT28_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT28_A::EDGE
    }
}
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type INT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT28_A, O>;
impl<'a, const O: u8> INT28_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT28_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT29_A {
        match self.bits {
            false => INT29_A::LEVEL,
            true => INT29_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT29_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT29_A::EDGE
    }
}
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type INT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT29_A, O>;
impl<'a, const O: u8> INT29_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT29_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT30_A {
        match self.bits {
            false => INT30_A::LEVEL,
            true => INT30_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT30_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT30_A::EDGE
    }
}
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type INT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT30_A, O>;
impl<'a, const O: u8> INT30_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT30_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
    pub fn variant(&self) -> INT31_A {
        match self.bits {
            false => INT31_A::LEVEL,
            true => INT31_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT31_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT31_A::EDGE
    }
}
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type INT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR4_SPEC, INT31_A, O>;
impl<'a, const O: u8> INT31_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT31_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
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
impl W {
    #[doc = "Bit 1 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<1> {
        INT16_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> INT17_W<3> {
        INT17_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> INT18_W<5> {
        INT18_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> INT19_W<7> {
        INT19_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> INT20_W<9> {
        INT20_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> INT21_W<11> {
        INT21_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> INT22_W<13> {
        INT22_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> INT23_W<15> {
        INT23_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> INT24_W<17> {
        INT24_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> INT25_W<19> {
        INT25_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> INT26_W<21> {
        INT26_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> INT27_W<23> {
        INT27_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> INT28_W<25> {
        INT28_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> INT29_W<27> {
        INT29_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> INT30_W<29> {
        INT30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> INT31_W<31> {
        INT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 16 - 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr4](index.html) module"]
pub struct GICD_ICFGR4_SPEC;
impl crate::RegisterSpec for GICD_ICFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr4::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr4::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR4 to value 0"]
impl crate::Resettable for GICD_ICFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

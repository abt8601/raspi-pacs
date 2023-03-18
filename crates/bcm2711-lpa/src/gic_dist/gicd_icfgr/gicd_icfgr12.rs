#[doc = "Register `GICD_ICFGR12` reader"]
pub struct R(crate::R<GICD_ICFGR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR12` writer"]
pub struct W(crate::W<GICD_ICFGR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR12_SPEC>;
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
impl From<crate::W<GICD_ICFGR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type INT48_R = crate::BitReader<INT48_A>;
#[doc = "Interrupt 48\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT48_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT48_A> for bool {
    #[inline(always)]
    fn from(variant: INT48_A) -> Self {
        variant as u8 != 0
    }
}
impl INT48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT48_A {
        match self.bits {
            false => INT48_A::LEVEL,
            true => INT48_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT48_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT48_A::EDGE
    }
}
#[doc = "Field `INT48` writer - Interrupt 48"]
pub type INT48_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT48_A, O>;
impl<'a, const O: u8> INT48_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT48_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT48_A::EDGE)
    }
}
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type INT49_R = crate::BitReader<INT49_A>;
#[doc = "Interrupt 49\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT49_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT49_A> for bool {
    #[inline(always)]
    fn from(variant: INT49_A) -> Self {
        variant as u8 != 0
    }
}
impl INT49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT49_A {
        match self.bits {
            false => INT49_A::LEVEL,
            true => INT49_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT49_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT49_A::EDGE
    }
}
#[doc = "Field `INT49` writer - Interrupt 49"]
pub type INT49_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT49_A, O>;
impl<'a, const O: u8> INT49_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT49_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT49_A::EDGE)
    }
}
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type INT50_R = crate::BitReader<INT50_A>;
#[doc = "Interrupt 50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT50_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT50_A> for bool {
    #[inline(always)]
    fn from(variant: INT50_A) -> Self {
        variant as u8 != 0
    }
}
impl INT50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT50_A {
        match self.bits {
            false => INT50_A::LEVEL,
            true => INT50_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT50_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT50_A::EDGE
    }
}
#[doc = "Field `INT50` writer - Interrupt 50"]
pub type INT50_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT50_A, O>;
impl<'a, const O: u8> INT50_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT50_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT50_A::EDGE)
    }
}
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type INT51_R = crate::BitReader<INT51_A>;
#[doc = "Interrupt 51\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT51_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT51_A> for bool {
    #[inline(always)]
    fn from(variant: INT51_A) -> Self {
        variant as u8 != 0
    }
}
impl INT51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT51_A {
        match self.bits {
            false => INT51_A::LEVEL,
            true => INT51_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT51_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT51_A::EDGE
    }
}
#[doc = "Field `INT51` writer - Interrupt 51"]
pub type INT51_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT51_A, O>;
impl<'a, const O: u8> INT51_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT51_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT51_A::EDGE)
    }
}
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type INT52_R = crate::BitReader<INT52_A>;
#[doc = "Interrupt 52\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT52_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT52_A> for bool {
    #[inline(always)]
    fn from(variant: INT52_A) -> Self {
        variant as u8 != 0
    }
}
impl INT52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT52_A {
        match self.bits {
            false => INT52_A::LEVEL,
            true => INT52_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT52_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT52_A::EDGE
    }
}
#[doc = "Field `INT52` writer - Interrupt 52"]
pub type INT52_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT52_A, O>;
impl<'a, const O: u8> INT52_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT52_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT52_A::EDGE)
    }
}
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type INT53_R = crate::BitReader<INT53_A>;
#[doc = "Interrupt 53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT53_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT53_A> for bool {
    #[inline(always)]
    fn from(variant: INT53_A) -> Self {
        variant as u8 != 0
    }
}
impl INT53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT53_A {
        match self.bits {
            false => INT53_A::LEVEL,
            true => INT53_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT53_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT53_A::EDGE
    }
}
#[doc = "Field `INT53` writer - Interrupt 53"]
pub type INT53_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT53_A, O>;
impl<'a, const O: u8> INT53_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT53_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT53_A::EDGE)
    }
}
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type INT54_R = crate::BitReader<INT54_A>;
#[doc = "Interrupt 54\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT54_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT54_A> for bool {
    #[inline(always)]
    fn from(variant: INT54_A) -> Self {
        variant as u8 != 0
    }
}
impl INT54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT54_A {
        match self.bits {
            false => INT54_A::LEVEL,
            true => INT54_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT54_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT54_A::EDGE
    }
}
#[doc = "Field `INT54` writer - Interrupt 54"]
pub type INT54_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT54_A, O>;
impl<'a, const O: u8> INT54_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT54_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT54_A::EDGE)
    }
}
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type INT55_R = crate::BitReader<INT55_A>;
#[doc = "Interrupt 55\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT55_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT55_A> for bool {
    #[inline(always)]
    fn from(variant: INT55_A) -> Self {
        variant as u8 != 0
    }
}
impl INT55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT55_A {
        match self.bits {
            false => INT55_A::LEVEL,
            true => INT55_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT55_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT55_A::EDGE
    }
}
#[doc = "Field `INT55` writer - Interrupt 55"]
pub type INT55_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT55_A, O>;
impl<'a, const O: u8> INT55_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT55_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT55_A::EDGE)
    }
}
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type INT56_R = crate::BitReader<INT56_A>;
#[doc = "Interrupt 56\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT56_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT56_A> for bool {
    #[inline(always)]
    fn from(variant: INT56_A) -> Self {
        variant as u8 != 0
    }
}
impl INT56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT56_A {
        match self.bits {
            false => INT56_A::LEVEL,
            true => INT56_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT56_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT56_A::EDGE
    }
}
#[doc = "Field `INT56` writer - Interrupt 56"]
pub type INT56_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT56_A, O>;
impl<'a, const O: u8> INT56_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT56_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT56_A::EDGE)
    }
}
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type INT57_R = crate::BitReader<INT57_A>;
#[doc = "Interrupt 57\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT57_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT57_A> for bool {
    #[inline(always)]
    fn from(variant: INT57_A) -> Self {
        variant as u8 != 0
    }
}
impl INT57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT57_A {
        match self.bits {
            false => INT57_A::LEVEL,
            true => INT57_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT57_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT57_A::EDGE
    }
}
#[doc = "Field `INT57` writer - Interrupt 57"]
pub type INT57_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT57_A, O>;
impl<'a, const O: u8> INT57_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT57_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT57_A::EDGE)
    }
}
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type INT58_R = crate::BitReader<INT58_A>;
#[doc = "Interrupt 58\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT58_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT58_A> for bool {
    #[inline(always)]
    fn from(variant: INT58_A) -> Self {
        variant as u8 != 0
    }
}
impl INT58_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT58_A {
        match self.bits {
            false => INT58_A::LEVEL,
            true => INT58_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT58_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT58_A::EDGE
    }
}
#[doc = "Field `INT58` writer - Interrupt 58"]
pub type INT58_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT58_A, O>;
impl<'a, const O: u8> INT58_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT58_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT58_A::EDGE)
    }
}
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type INT59_R = crate::BitReader<INT59_A>;
#[doc = "Interrupt 59\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT59_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT59_A> for bool {
    #[inline(always)]
    fn from(variant: INT59_A) -> Self {
        variant as u8 != 0
    }
}
impl INT59_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT59_A {
        match self.bits {
            false => INT59_A::LEVEL,
            true => INT59_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT59_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT59_A::EDGE
    }
}
#[doc = "Field `INT59` writer - Interrupt 59"]
pub type INT59_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT59_A, O>;
impl<'a, const O: u8> INT59_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT59_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT59_A::EDGE)
    }
}
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type INT60_R = crate::BitReader<INT60_A>;
#[doc = "Interrupt 60\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT60_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT60_A> for bool {
    #[inline(always)]
    fn from(variant: INT60_A) -> Self {
        variant as u8 != 0
    }
}
impl INT60_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT60_A {
        match self.bits {
            false => INT60_A::LEVEL,
            true => INT60_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT60_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT60_A::EDGE
    }
}
#[doc = "Field `INT60` writer - Interrupt 60"]
pub type INT60_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT60_A, O>;
impl<'a, const O: u8> INT60_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT60_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT60_A::EDGE)
    }
}
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type INT61_R = crate::BitReader<INT61_A>;
#[doc = "Interrupt 61\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT61_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT61_A> for bool {
    #[inline(always)]
    fn from(variant: INT61_A) -> Self {
        variant as u8 != 0
    }
}
impl INT61_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT61_A {
        match self.bits {
            false => INT61_A::LEVEL,
            true => INT61_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT61_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT61_A::EDGE
    }
}
#[doc = "Field `INT61` writer - Interrupt 61"]
pub type INT61_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT61_A, O>;
impl<'a, const O: u8> INT61_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT61_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT61_A::EDGE)
    }
}
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type INT62_R = crate::BitReader<INT62_A>;
#[doc = "Interrupt 62\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT62_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT62_A> for bool {
    #[inline(always)]
    fn from(variant: INT62_A) -> Self {
        variant as u8 != 0
    }
}
impl INT62_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT62_A {
        match self.bits {
            false => INT62_A::LEVEL,
            true => INT62_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT62_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT62_A::EDGE
    }
}
#[doc = "Field `INT62` writer - Interrupt 62"]
pub type INT62_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT62_A, O>;
impl<'a, const O: u8> INT62_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT62_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT62_A::EDGE)
    }
}
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type INT63_R = crate::BitReader<INT63_A>;
#[doc = "Interrupt 63\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT63_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT63_A> for bool {
    #[inline(always)]
    fn from(variant: INT63_A) -> Self {
        variant as u8 != 0
    }
}
impl INT63_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT63_A {
        match self.bits {
            false => INT63_A::LEVEL,
            true => INT63_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT63_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT63_A::EDGE
    }
}
#[doc = "Field `INT63` writer - Interrupt 63"]
pub type INT63_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR12_SPEC, INT63_A, O>;
impl<'a, const O: u8> INT63_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT63_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT63_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> INT48_R {
        INT48_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> INT49_R {
        INT49_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> INT50_R {
        INT50_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> INT51_R {
        INT51_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> INT52_R {
        INT52_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> INT53_R {
        INT53_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> INT54_R {
        INT54_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> INT55_R {
        INT55_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> INT56_R {
        INT56_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> INT57_R {
        INT57_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> INT58_R {
        INT58_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> INT59_R {
        INT59_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> INT60_R {
        INT60_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> INT61_R {
        INT61_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> INT62_R {
        INT62_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> INT63_R {
        INT63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn int48(&mut self) -> INT48_W<1> {
        INT48_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn int49(&mut self) -> INT49_W<3> {
        INT49_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn int50(&mut self) -> INT50_W<5> {
        INT50_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn int51(&mut self) -> INT51_W<7> {
        INT51_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn int52(&mut self) -> INT52_W<9> {
        INT52_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn int53(&mut self) -> INT53_W<11> {
        INT53_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn int54(&mut self) -> INT54_W<13> {
        INT54_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn int55(&mut self) -> INT55_W<15> {
        INT55_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn int56(&mut self) -> INT56_W<17> {
        INT56_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn int57(&mut self) -> INT57_W<19> {
        INT57_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn int58(&mut self) -> INT58_W<21> {
        INT58_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn int59(&mut self) -> INT59_W<23> {
        INT59_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 60"]
    #[inline(always)]
    #[must_use]
    pub fn int60(&mut self) -> INT60_W<25> {
        INT60_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 61"]
    #[inline(always)]
    #[must_use]
    pub fn int61(&mut self) -> INT61_W<27> {
        INT61_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 62"]
    #[inline(always)]
    #[must_use]
    pub fn int62(&mut self) -> INT62_W<29> {
        INT62_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 63"]
    #[inline(always)]
    #[must_use]
    pub fn int63(&mut self) -> INT63_W<31> {
        INT63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 48 - 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr12](index.html) module"]
pub struct GICD_ICFGR12_SPEC;
impl crate::RegisterSpec for GICD_ICFGR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr12::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr12::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR12 to value 0"]
impl crate::Resettable for GICD_ICFGR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

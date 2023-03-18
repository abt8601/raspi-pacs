#[doc = "Register `GICD_ICFGR0` reader"]
pub struct R(crate::R<GICD_ICFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR0` writer"]
pub struct W(crate::W<GICD_ICFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR0_SPEC>;
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
impl From<crate::W<GICD_ICFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type INT0_R = crate::BitReader<INT0_A>;
#[doc = "Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT0_A> for bool {
    #[inline(always)]
    fn from(variant: INT0_A) -> Self {
        variant as u8 != 0
    }
}
impl INT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT0_A {
        match self.bits {
            false => INT0_A::LEVEL,
            true => INT0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT0_A::EDGE
    }
}
#[doc = "Field `INT0` writer - Interrupt 0"]
pub type INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT0_A, O>;
impl<'a, const O: u8> INT0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT0_A::EDGE)
    }
}
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type INT1_R = crate::BitReader<INT1_A>;
#[doc = "Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
impl INT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::LEVEL,
            true => INT1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT1_A::EDGE
    }
}
#[doc = "Field `INT1` writer - Interrupt 1"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT1_A, O>;
impl<'a, const O: u8> INT1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT1_A::EDGE)
    }
}
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type INT2_R = crate::BitReader<INT2_A>;
#[doc = "Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
impl INT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::LEVEL,
            true => INT2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT2_A::EDGE
    }
}
#[doc = "Field `INT2` writer - Interrupt 2"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT2_A, O>;
impl<'a, const O: u8> INT2_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT2_A::EDGE)
    }
}
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type INT3_R = crate::BitReader<INT3_A>;
#[doc = "Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
impl INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::LEVEL,
            true => INT3_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT3_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT3_A::EDGE
    }
}
#[doc = "Field `INT3` writer - Interrupt 3"]
pub type INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT3_A, O>;
impl<'a, const O: u8> INT3_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT3_A::EDGE)
    }
}
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type INT4_R = crate::BitReader<INT4_A>;
#[doc = "Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT4_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
impl INT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::LEVEL,
            true => INT4_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT4_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT4_A::EDGE
    }
}
#[doc = "Field `INT4` writer - Interrupt 4"]
pub type INT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT4_A, O>;
impl<'a, const O: u8> INT4_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT4_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT4_A::EDGE)
    }
}
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type INT5_R = crate::BitReader<INT5_A>;
#[doc = "Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT5_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
impl INT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::LEVEL,
            true => INT5_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT5_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT5_A::EDGE
    }
}
#[doc = "Field `INT5` writer - Interrupt 5"]
pub type INT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT5_A, O>;
impl<'a, const O: u8> INT5_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT5_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT5_A::EDGE)
    }
}
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type INT6_R = crate::BitReader<INT6_A>;
#[doc = "Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT6_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
impl INT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::LEVEL,
            true => INT6_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT6_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT6_A::EDGE
    }
}
#[doc = "Field `INT6` writer - Interrupt 6"]
pub type INT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT6_A, O>;
impl<'a, const O: u8> INT6_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT6_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT6_A::EDGE)
    }
}
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type INT7_R = crate::BitReader<INT7_A>;
#[doc = "Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT7_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
impl INT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::LEVEL,
            true => INT7_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT7_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT7_A::EDGE
    }
}
#[doc = "Field `INT7` writer - Interrupt 7"]
pub type INT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT7_A, O>;
impl<'a, const O: u8> INT7_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT7_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT7_A::EDGE)
    }
}
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type INT8_R = crate::BitReader<INT8_A>;
#[doc = "Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT8_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT8_A> for bool {
    #[inline(always)]
    fn from(variant: INT8_A) -> Self {
        variant as u8 != 0
    }
}
impl INT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT8_A {
        match self.bits {
            false => INT8_A::LEVEL,
            true => INT8_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT8_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT8_A::EDGE
    }
}
#[doc = "Field `INT8` writer - Interrupt 8"]
pub type INT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT8_A, O>;
impl<'a, const O: u8> INT8_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT8_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT8_A::EDGE)
    }
}
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type INT9_R = crate::BitReader<INT9_A>;
#[doc = "Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT9_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT9_A> for bool {
    #[inline(always)]
    fn from(variant: INT9_A) -> Self {
        variant as u8 != 0
    }
}
impl INT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT9_A {
        match self.bits {
            false => INT9_A::LEVEL,
            true => INT9_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT9_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT9_A::EDGE
    }
}
#[doc = "Field `INT9` writer - Interrupt 9"]
pub type INT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT9_A, O>;
impl<'a, const O: u8> INT9_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT9_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT9_A::EDGE)
    }
}
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type INT10_R = crate::BitReader<INT10_A>;
#[doc = "Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT10_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT10_A> for bool {
    #[inline(always)]
    fn from(variant: INT10_A) -> Self {
        variant as u8 != 0
    }
}
impl INT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT10_A {
        match self.bits {
            false => INT10_A::LEVEL,
            true => INT10_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT10_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT10_A::EDGE
    }
}
#[doc = "Field `INT10` writer - Interrupt 10"]
pub type INT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT10_A, O>;
impl<'a, const O: u8> INT10_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT10_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT10_A::EDGE)
    }
}
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type INT11_R = crate::BitReader<INT11_A>;
#[doc = "Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT11_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT11_A> for bool {
    #[inline(always)]
    fn from(variant: INT11_A) -> Self {
        variant as u8 != 0
    }
}
impl INT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT11_A {
        match self.bits {
            false => INT11_A::LEVEL,
            true => INT11_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT11_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT11_A::EDGE
    }
}
#[doc = "Field `INT11` writer - Interrupt 11"]
pub type INT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT11_A, O>;
impl<'a, const O: u8> INT11_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT11_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT11_A::EDGE)
    }
}
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type INT12_R = crate::BitReader<INT12_A>;
#[doc = "Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT12_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT12_A> for bool {
    #[inline(always)]
    fn from(variant: INT12_A) -> Self {
        variant as u8 != 0
    }
}
impl INT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT12_A {
        match self.bits {
            false => INT12_A::LEVEL,
            true => INT12_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT12_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT12_A::EDGE
    }
}
#[doc = "Field `INT12` writer - Interrupt 12"]
pub type INT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT12_A, O>;
impl<'a, const O: u8> INT12_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT12_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT12_A::EDGE)
    }
}
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type INT13_R = crate::BitReader<INT13_A>;
#[doc = "Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT13_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT13_A> for bool {
    #[inline(always)]
    fn from(variant: INT13_A) -> Self {
        variant as u8 != 0
    }
}
impl INT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT13_A {
        match self.bits {
            false => INT13_A::LEVEL,
            true => INT13_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT13_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT13_A::EDGE
    }
}
#[doc = "Field `INT13` writer - Interrupt 13"]
pub type INT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT13_A, O>;
impl<'a, const O: u8> INT13_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT13_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT13_A::EDGE)
    }
}
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type INT14_R = crate::BitReader<INT14_A>;
#[doc = "Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT14_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT14_A> for bool {
    #[inline(always)]
    fn from(variant: INT14_A) -> Self {
        variant as u8 != 0
    }
}
impl INT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT14_A {
        match self.bits {
            false => INT14_A::LEVEL,
            true => INT14_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT14_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT14_A::EDGE
    }
}
#[doc = "Field `INT14` writer - Interrupt 14"]
pub type INT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT14_A, O>;
impl<'a, const O: u8> INT14_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT14_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT14_A::EDGE)
    }
}
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type INT15_R = crate::BitReader<INT15_A>;
#[doc = "Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT15_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT15_A> for bool {
    #[inline(always)]
    fn from(variant: INT15_A) -> Self {
        variant as u8 != 0
    }
}
impl INT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT15_A {
        match self.bits {
            false => INT15_A::LEVEL,
            true => INT15_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT15_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT15_A::EDGE
    }
}
#[doc = "Field `INT15` writer - Interrupt 15"]
pub type INT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR0_SPEC, INT15_A, O>;
impl<'a, const O: u8> INT15_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT15_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT15_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<1> {
        INT0_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<3> {
        INT1_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<5> {
        INT2_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<7> {
        INT3_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<9> {
        INT4_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<11> {
        INT5_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<13> {
        INT6_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<15> {
        INT7_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<17> {
        INT8_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<19> {
        INT9_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<21> {
        INT10_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<23> {
        INT11_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<25> {
        INT12_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<27> {
        INT13_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<29> {
        INT14_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<31> {
        INT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 0 - 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr0](index.html) module"]
pub struct GICD_ICFGR0_SPEC;
impl crate::RegisterSpec for GICD_ICFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr0::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr0::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR0 to value 0"]
impl crate::Resettable for GICD_ICFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

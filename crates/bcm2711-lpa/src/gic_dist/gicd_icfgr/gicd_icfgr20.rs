#[doc = "Register `GICD_ICFGR20` reader"]
pub struct R(crate::R<GICD_ICFGR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR20` writer"]
pub struct W(crate::W<GICD_ICFGR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR20_SPEC>;
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
impl From<crate::W<GICD_ICFGR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type INT80_R = crate::BitReader<INT80_A>;
#[doc = "Interrupt 80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT80_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT80_A> for bool {
    #[inline(always)]
    fn from(variant: INT80_A) -> Self {
        variant as u8 != 0
    }
}
impl INT80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT80_A {
        match self.bits {
            false => INT80_A::LEVEL,
            true => INT80_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT80_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT80_A::EDGE
    }
}
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type INT80_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT80_A, O>;
impl<'a, const O: u8> INT80_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT80_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT80_A::EDGE)
    }
}
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type INT81_R = crate::BitReader<INT81_A>;
#[doc = "Interrupt 81\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT81_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT81_A> for bool {
    #[inline(always)]
    fn from(variant: INT81_A) -> Self {
        variant as u8 != 0
    }
}
impl INT81_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT81_A {
        match self.bits {
            false => INT81_A::LEVEL,
            true => INT81_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT81_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT81_A::EDGE
    }
}
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type INT81_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT81_A, O>;
impl<'a, const O: u8> INT81_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT81_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT81_A::EDGE)
    }
}
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type INT82_R = crate::BitReader<INT82_A>;
#[doc = "Interrupt 82\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT82_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT82_A> for bool {
    #[inline(always)]
    fn from(variant: INT82_A) -> Self {
        variant as u8 != 0
    }
}
impl INT82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT82_A {
        match self.bits {
            false => INT82_A::LEVEL,
            true => INT82_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT82_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT82_A::EDGE
    }
}
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type INT82_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT82_A, O>;
impl<'a, const O: u8> INT82_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT82_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT82_A::EDGE)
    }
}
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type INT83_R = crate::BitReader<INT83_A>;
#[doc = "Interrupt 83\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT83_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT83_A> for bool {
    #[inline(always)]
    fn from(variant: INT83_A) -> Self {
        variant as u8 != 0
    }
}
impl INT83_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT83_A {
        match self.bits {
            false => INT83_A::LEVEL,
            true => INT83_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT83_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT83_A::EDGE
    }
}
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type INT83_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT83_A, O>;
impl<'a, const O: u8> INT83_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT83_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT83_A::EDGE)
    }
}
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type INT84_R = crate::BitReader<INT84_A>;
#[doc = "Interrupt 84\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT84_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT84_A> for bool {
    #[inline(always)]
    fn from(variant: INT84_A) -> Self {
        variant as u8 != 0
    }
}
impl INT84_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT84_A {
        match self.bits {
            false => INT84_A::LEVEL,
            true => INT84_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT84_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT84_A::EDGE
    }
}
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type INT84_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT84_A, O>;
impl<'a, const O: u8> INT84_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT84_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT84_A::EDGE)
    }
}
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type INT85_R = crate::BitReader<INT85_A>;
#[doc = "Interrupt 85\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT85_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT85_A> for bool {
    #[inline(always)]
    fn from(variant: INT85_A) -> Self {
        variant as u8 != 0
    }
}
impl INT85_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT85_A {
        match self.bits {
            false => INT85_A::LEVEL,
            true => INT85_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT85_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT85_A::EDGE
    }
}
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type INT85_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT85_A, O>;
impl<'a, const O: u8> INT85_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT85_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT85_A::EDGE)
    }
}
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type INT86_R = crate::BitReader<INT86_A>;
#[doc = "Interrupt 86\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT86_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT86_A> for bool {
    #[inline(always)]
    fn from(variant: INT86_A) -> Self {
        variant as u8 != 0
    }
}
impl INT86_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT86_A {
        match self.bits {
            false => INT86_A::LEVEL,
            true => INT86_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT86_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT86_A::EDGE
    }
}
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type INT86_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT86_A, O>;
impl<'a, const O: u8> INT86_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT86_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT86_A::EDGE)
    }
}
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type INT87_R = crate::BitReader<INT87_A>;
#[doc = "Interrupt 87\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT87_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT87_A> for bool {
    #[inline(always)]
    fn from(variant: INT87_A) -> Self {
        variant as u8 != 0
    }
}
impl INT87_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT87_A {
        match self.bits {
            false => INT87_A::LEVEL,
            true => INT87_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT87_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT87_A::EDGE
    }
}
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type INT87_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT87_A, O>;
impl<'a, const O: u8> INT87_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT87_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT87_A::EDGE)
    }
}
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type INT88_R = crate::BitReader<INT88_A>;
#[doc = "Interrupt 88\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT88_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT88_A> for bool {
    #[inline(always)]
    fn from(variant: INT88_A) -> Self {
        variant as u8 != 0
    }
}
impl INT88_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT88_A {
        match self.bits {
            false => INT88_A::LEVEL,
            true => INT88_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT88_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT88_A::EDGE
    }
}
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type INT88_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT88_A, O>;
impl<'a, const O: u8> INT88_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT88_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT88_A::EDGE)
    }
}
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type INT89_R = crate::BitReader<INT89_A>;
#[doc = "Interrupt 89\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT89_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT89_A> for bool {
    #[inline(always)]
    fn from(variant: INT89_A) -> Self {
        variant as u8 != 0
    }
}
impl INT89_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT89_A {
        match self.bits {
            false => INT89_A::LEVEL,
            true => INT89_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT89_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT89_A::EDGE
    }
}
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type INT89_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT89_A, O>;
impl<'a, const O: u8> INT89_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT89_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT89_A::EDGE)
    }
}
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type INT90_R = crate::BitReader<INT90_A>;
#[doc = "Interrupt 90\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT90_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT90_A> for bool {
    #[inline(always)]
    fn from(variant: INT90_A) -> Self {
        variant as u8 != 0
    }
}
impl INT90_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT90_A {
        match self.bits {
            false => INT90_A::LEVEL,
            true => INT90_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT90_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT90_A::EDGE
    }
}
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type INT90_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT90_A, O>;
impl<'a, const O: u8> INT90_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT90_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT90_A::EDGE)
    }
}
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type INT91_R = crate::BitReader<INT91_A>;
#[doc = "Interrupt 91\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT91_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT91_A> for bool {
    #[inline(always)]
    fn from(variant: INT91_A) -> Self {
        variant as u8 != 0
    }
}
impl INT91_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT91_A {
        match self.bits {
            false => INT91_A::LEVEL,
            true => INT91_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT91_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT91_A::EDGE
    }
}
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type INT91_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT91_A, O>;
impl<'a, const O: u8> INT91_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT91_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT91_A::EDGE)
    }
}
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type INT92_R = crate::BitReader<INT92_A>;
#[doc = "Interrupt 92\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT92_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT92_A> for bool {
    #[inline(always)]
    fn from(variant: INT92_A) -> Self {
        variant as u8 != 0
    }
}
impl INT92_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT92_A {
        match self.bits {
            false => INT92_A::LEVEL,
            true => INT92_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT92_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT92_A::EDGE
    }
}
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type INT92_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT92_A, O>;
impl<'a, const O: u8> INT92_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT92_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT92_A::EDGE)
    }
}
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type INT93_R = crate::BitReader<INT93_A>;
#[doc = "Interrupt 93\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT93_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT93_A> for bool {
    #[inline(always)]
    fn from(variant: INT93_A) -> Self {
        variant as u8 != 0
    }
}
impl INT93_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT93_A {
        match self.bits {
            false => INT93_A::LEVEL,
            true => INT93_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT93_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT93_A::EDGE
    }
}
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type INT93_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT93_A, O>;
impl<'a, const O: u8> INT93_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT93_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT93_A::EDGE)
    }
}
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type INT94_R = crate::BitReader<INT94_A>;
#[doc = "Interrupt 94\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT94_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT94_A> for bool {
    #[inline(always)]
    fn from(variant: INT94_A) -> Self {
        variant as u8 != 0
    }
}
impl INT94_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT94_A {
        match self.bits {
            false => INT94_A::LEVEL,
            true => INT94_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT94_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT94_A::EDGE
    }
}
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type INT94_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT94_A, O>;
impl<'a, const O: u8> INT94_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT94_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT94_A::EDGE)
    }
}
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type INT95_R = crate::BitReader<INT95_A>;
#[doc = "Interrupt 95\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT95_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT95_A> for bool {
    #[inline(always)]
    fn from(variant: INT95_A) -> Self {
        variant as u8 != 0
    }
}
impl INT95_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT95_A {
        match self.bits {
            false => INT95_A::LEVEL,
            true => INT95_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT95_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT95_A::EDGE
    }
}
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type INT95_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR20_SPEC, INT95_A, O>;
impl<'a, const O: u8> INT95_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT95_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT95_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> INT80_R {
        INT80_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> INT81_R {
        INT81_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> INT82_R {
        INT82_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> INT83_R {
        INT83_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> INT84_R {
        INT84_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> INT85_R {
        INT85_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> INT86_R {
        INT86_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> INT87_R {
        INT87_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> INT88_R {
        INT88_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> INT89_R {
        INT89_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> INT90_R {
        INT90_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> INT91_R {
        INT91_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> INT92_R {
        INT92_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> INT93_R {
        INT93_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> INT94_R {
        INT94_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> INT95_R {
        INT95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> INT80_W<1> {
        INT80_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> INT81_W<3> {
        INT81_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> INT82_W<5> {
        INT82_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> INT83_W<7> {
        INT83_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> INT84_W<9> {
        INT84_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> INT85_W<11> {
        INT85_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> INT86_W<13> {
        INT86_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> INT87_W<15> {
        INT87_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> INT88_W<17> {
        INT88_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> INT89_W<19> {
        INT89_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> INT90_W<21> {
        INT90_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> INT91_W<23> {
        INT91_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> INT92_W<25> {
        INT92_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> INT93_W<27> {
        INT93_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> INT94_W<29> {
        INT94_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> INT95_W<31> {
        INT95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 80 - 95\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr20](index.html) module"]
pub struct GICD_ICFGR20_SPEC;
impl crate::RegisterSpec for GICD_ICFGR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr20::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr20::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR20 to value 0"]
impl crate::Resettable for GICD_ICFGR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

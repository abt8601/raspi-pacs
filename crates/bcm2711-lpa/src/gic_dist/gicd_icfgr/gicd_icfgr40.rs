#[doc = "Register `GICD_ICFGR40` reader"]
pub struct R(crate::R<GICD_ICFGR40_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR40_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR40_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR40_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR40` writer"]
pub struct W(crate::W<GICD_ICFGR40_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR40_SPEC>;
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
impl From<crate::W<GICD_ICFGR40_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR40_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type INT160_R = crate::BitReader<INT160_A>;
#[doc = "Interrupt 160\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT160_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT160_A> for bool {
    #[inline(always)]
    fn from(variant: INT160_A) -> Self {
        variant as u8 != 0
    }
}
impl INT160_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT160_A {
        match self.bits {
            false => INT160_A::LEVEL,
            true => INT160_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT160_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT160_A::EDGE
    }
}
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type INT160_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT160_A, O>;
impl<'a, const O: u8> INT160_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT160_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT160_A::EDGE)
    }
}
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type INT161_R = crate::BitReader<INT161_A>;
#[doc = "Interrupt 161\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT161_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT161_A> for bool {
    #[inline(always)]
    fn from(variant: INT161_A) -> Self {
        variant as u8 != 0
    }
}
impl INT161_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT161_A {
        match self.bits {
            false => INT161_A::LEVEL,
            true => INT161_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT161_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT161_A::EDGE
    }
}
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type INT161_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT161_A, O>;
impl<'a, const O: u8> INT161_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT161_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT161_A::EDGE)
    }
}
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type INT162_R = crate::BitReader<INT162_A>;
#[doc = "Interrupt 162\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT162_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT162_A> for bool {
    #[inline(always)]
    fn from(variant: INT162_A) -> Self {
        variant as u8 != 0
    }
}
impl INT162_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT162_A {
        match self.bits {
            false => INT162_A::LEVEL,
            true => INT162_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT162_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT162_A::EDGE
    }
}
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type INT162_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT162_A, O>;
impl<'a, const O: u8> INT162_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT162_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT162_A::EDGE)
    }
}
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type INT163_R = crate::BitReader<INT163_A>;
#[doc = "Interrupt 163\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT163_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT163_A> for bool {
    #[inline(always)]
    fn from(variant: INT163_A) -> Self {
        variant as u8 != 0
    }
}
impl INT163_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT163_A {
        match self.bits {
            false => INT163_A::LEVEL,
            true => INT163_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT163_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT163_A::EDGE
    }
}
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type INT163_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT163_A, O>;
impl<'a, const O: u8> INT163_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT163_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT163_A::EDGE)
    }
}
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type INT164_R = crate::BitReader<INT164_A>;
#[doc = "Interrupt 164\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT164_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT164_A> for bool {
    #[inline(always)]
    fn from(variant: INT164_A) -> Self {
        variant as u8 != 0
    }
}
impl INT164_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT164_A {
        match self.bits {
            false => INT164_A::LEVEL,
            true => INT164_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT164_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT164_A::EDGE
    }
}
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type INT164_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT164_A, O>;
impl<'a, const O: u8> INT164_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT164_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT164_A::EDGE)
    }
}
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type INT165_R = crate::BitReader<INT165_A>;
#[doc = "Interrupt 165\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT165_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT165_A> for bool {
    #[inline(always)]
    fn from(variant: INT165_A) -> Self {
        variant as u8 != 0
    }
}
impl INT165_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT165_A {
        match self.bits {
            false => INT165_A::LEVEL,
            true => INT165_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT165_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT165_A::EDGE
    }
}
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type INT165_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT165_A, O>;
impl<'a, const O: u8> INT165_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT165_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT165_A::EDGE)
    }
}
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type INT166_R = crate::BitReader<INT166_A>;
#[doc = "Interrupt 166\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT166_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT166_A> for bool {
    #[inline(always)]
    fn from(variant: INT166_A) -> Self {
        variant as u8 != 0
    }
}
impl INT166_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT166_A {
        match self.bits {
            false => INT166_A::LEVEL,
            true => INT166_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT166_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT166_A::EDGE
    }
}
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type INT166_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT166_A, O>;
impl<'a, const O: u8> INT166_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT166_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT166_A::EDGE)
    }
}
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type INT167_R = crate::BitReader<INT167_A>;
#[doc = "Interrupt 167\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT167_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT167_A> for bool {
    #[inline(always)]
    fn from(variant: INT167_A) -> Self {
        variant as u8 != 0
    }
}
impl INT167_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT167_A {
        match self.bits {
            false => INT167_A::LEVEL,
            true => INT167_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT167_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT167_A::EDGE
    }
}
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type INT167_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT167_A, O>;
impl<'a, const O: u8> INT167_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT167_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT167_A::EDGE)
    }
}
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type INT168_R = crate::BitReader<INT168_A>;
#[doc = "Interrupt 168\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT168_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT168_A> for bool {
    #[inline(always)]
    fn from(variant: INT168_A) -> Self {
        variant as u8 != 0
    }
}
impl INT168_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT168_A {
        match self.bits {
            false => INT168_A::LEVEL,
            true => INT168_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT168_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT168_A::EDGE
    }
}
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type INT168_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT168_A, O>;
impl<'a, const O: u8> INT168_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT168_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT168_A::EDGE)
    }
}
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type INT169_R = crate::BitReader<INT169_A>;
#[doc = "Interrupt 169\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT169_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT169_A> for bool {
    #[inline(always)]
    fn from(variant: INT169_A) -> Self {
        variant as u8 != 0
    }
}
impl INT169_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT169_A {
        match self.bits {
            false => INT169_A::LEVEL,
            true => INT169_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT169_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT169_A::EDGE
    }
}
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type INT169_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT169_A, O>;
impl<'a, const O: u8> INT169_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT169_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT169_A::EDGE)
    }
}
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type INT170_R = crate::BitReader<INT170_A>;
#[doc = "Interrupt 170\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT170_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT170_A> for bool {
    #[inline(always)]
    fn from(variant: INT170_A) -> Self {
        variant as u8 != 0
    }
}
impl INT170_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT170_A {
        match self.bits {
            false => INT170_A::LEVEL,
            true => INT170_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT170_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT170_A::EDGE
    }
}
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type INT170_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT170_A, O>;
impl<'a, const O: u8> INT170_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT170_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT170_A::EDGE)
    }
}
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type INT171_R = crate::BitReader<INT171_A>;
#[doc = "Interrupt 171\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT171_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT171_A> for bool {
    #[inline(always)]
    fn from(variant: INT171_A) -> Self {
        variant as u8 != 0
    }
}
impl INT171_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT171_A {
        match self.bits {
            false => INT171_A::LEVEL,
            true => INT171_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT171_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT171_A::EDGE
    }
}
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type INT171_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT171_A, O>;
impl<'a, const O: u8> INT171_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT171_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT171_A::EDGE)
    }
}
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type INT172_R = crate::BitReader<INT172_A>;
#[doc = "Interrupt 172\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT172_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT172_A> for bool {
    #[inline(always)]
    fn from(variant: INT172_A) -> Self {
        variant as u8 != 0
    }
}
impl INT172_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT172_A {
        match self.bits {
            false => INT172_A::LEVEL,
            true => INT172_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT172_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT172_A::EDGE
    }
}
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type INT172_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT172_A, O>;
impl<'a, const O: u8> INT172_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT172_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT172_A::EDGE)
    }
}
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type INT173_R = crate::BitReader<INT173_A>;
#[doc = "Interrupt 173\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT173_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT173_A> for bool {
    #[inline(always)]
    fn from(variant: INT173_A) -> Self {
        variant as u8 != 0
    }
}
impl INT173_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT173_A {
        match self.bits {
            false => INT173_A::LEVEL,
            true => INT173_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT173_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT173_A::EDGE
    }
}
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type INT173_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT173_A, O>;
impl<'a, const O: u8> INT173_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT173_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT173_A::EDGE)
    }
}
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type INT174_R = crate::BitReader<INT174_A>;
#[doc = "Interrupt 174\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT174_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT174_A> for bool {
    #[inline(always)]
    fn from(variant: INT174_A) -> Self {
        variant as u8 != 0
    }
}
impl INT174_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT174_A {
        match self.bits {
            false => INT174_A::LEVEL,
            true => INT174_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT174_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT174_A::EDGE
    }
}
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type INT174_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT174_A, O>;
impl<'a, const O: u8> INT174_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT174_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT174_A::EDGE)
    }
}
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type INT175_R = crate::BitReader<INT175_A>;
#[doc = "Interrupt 175\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT175_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<INT175_A> for bool {
    #[inline(always)]
    fn from(variant: INT175_A) -> Self {
        variant as u8 != 0
    }
}
impl INT175_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT175_A {
        match self.bits {
            false => INT175_A::LEVEL,
            true => INT175_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INT175_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INT175_A::EDGE
    }
}
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type INT175_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR40_SPEC, INT175_A, O>;
impl<'a, const O: u8> INT175_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(INT175_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(INT175_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> INT160_R {
        INT160_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> INT161_R {
        INT161_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> INT162_R {
        INT162_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> INT163_R {
        INT163_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> INT164_R {
        INT164_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> INT165_R {
        INT165_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> INT166_R {
        INT166_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> INT167_R {
        INT167_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> INT168_R {
        INT168_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> INT169_R {
        INT169_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> INT170_R {
        INT170_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> INT171_R {
        INT171_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> INT172_R {
        INT172_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> INT173_R {
        INT173_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> INT174_R {
        INT174_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> INT175_R {
        INT175_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> INT160_W<1> {
        INT160_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> INT161_W<3> {
        INT161_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> INT162_W<5> {
        INT162_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> INT163_W<7> {
        INT163_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> INT164_W<9> {
        INT164_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> INT165_W<11> {
        INT165_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> INT166_W<13> {
        INT166_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> INT167_W<15> {
        INT167_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> INT168_W<17> {
        INT168_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> INT169_W<19> {
        INT169_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> INT170_W<21> {
        INT170_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> INT171_W<23> {
        INT171_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> INT172_W<25> {
        INT172_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> INT173_W<27> {
        INT173_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> INT174_W<29> {
        INT174_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> INT175_W<31> {
        INT175_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 160 - 175\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr40](index.html) module"]
pub struct GICD_ICFGR40_SPEC;
impl crate::RegisterSpec for GICD_ICFGR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr40::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR40_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr40::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR40_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR40 to value 0"]
impl crate::Resettable for GICD_ICFGR40_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

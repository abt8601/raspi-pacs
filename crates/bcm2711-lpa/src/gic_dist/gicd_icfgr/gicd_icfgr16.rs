#[doc = "Register `GICD_ICFGR16` reader"]
pub struct R(crate::R<GICD_ICFGR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICFGR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICFGR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICFGR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICFGR16` writer"]
pub struct W(crate::W<GICD_ICFGR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICFGR16_SPEC>;
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
impl From<crate::W<GICD_ICFGR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICFGR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader<TIMER_A>;
#[doc = "ARMC Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_A {
        match self.bits {
            false => TIMER_A::LEVEL,
            true => TIMER_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TIMER_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TIMER_A::EDGE
    }
}
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, TIMER_A, O>;
impl<'a, const O: u8> TIMER_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(TIMER_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(TIMER_A::EDGE)
    }
}
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader<MAILBOX_A>;
#[doc = "Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAILBOX_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<MAILBOX_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_A) -> Self {
        variant as u8 != 0
    }
}
impl MAILBOX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_A {
        match self.bits {
            false => MAILBOX_A::LEVEL,
            true => MAILBOX_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MAILBOX_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MAILBOX_A::EDGE
    }
}
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, MAILBOX_A, O>;
impl<'a, const O: u8> MAILBOX_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(MAILBOX_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(MAILBOX_A::EDGE)
    }
}
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader<DOORBELL0_A>;
#[doc = "Doorbell 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOORBELL0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DOORBELL0_A> for bool {
    #[inline(always)]
    fn from(variant: DOORBELL0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOORBELL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOORBELL0_A {
        match self.bits {
            false => DOORBELL0_A::LEVEL,
            true => DOORBELL0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DOORBELL0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DOORBELL0_A::EDGE
    }
}
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, DOORBELL0_A, O>;
impl<'a, const O: u8> DOORBELL0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DOORBELL0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(DOORBELL0_A::EDGE)
    }
}
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader<DOORBELL1_A>;
#[doc = "Doorbell 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOORBELL1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<DOORBELL1_A> for bool {
    #[inline(always)]
    fn from(variant: DOORBELL1_A) -> Self {
        variant as u8 != 0
    }
}
impl DOORBELL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOORBELL1_A {
        match self.bits {
            false => DOORBELL1_A::LEVEL,
            true => DOORBELL1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DOORBELL1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DOORBELL1_A::EDGE
    }
}
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, DOORBELL1_A, O>;
impl<'a, const O: u8> DOORBELL1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DOORBELL1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(DOORBELL1_A::EDGE)
    }
}
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader<VPU0_HALTED_A>;
#[doc = "VPU0 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPU0_HALTED_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<VPU0_HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: VPU0_HALTED_A) -> Self {
        variant as u8 != 0
    }
}
impl VPU0_HALTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPU0_HALTED_A {
        match self.bits {
            false => VPU0_HALTED_A::LEVEL,
            true => VPU0_HALTED_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == VPU0_HALTED_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == VPU0_HALTED_A::EDGE
    }
}
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, VPU0_HALTED_A, O>;
impl<'a, const O: u8> VPU0_HALTED_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(VPU0_HALTED_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(VPU0_HALTED_A::EDGE)
    }
}
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader<VPU1_HALTED_A>;
#[doc = "VPU1 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPU1_HALTED_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<VPU1_HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: VPU1_HALTED_A) -> Self {
        variant as u8 != 0
    }
}
impl VPU1_HALTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPU1_HALTED_A {
        match self.bits {
            false => VPU1_HALTED_A::LEVEL,
            true => VPU1_HALTED_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == VPU1_HALTED_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == VPU1_HALTED_A::EDGE
    }
}
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, VPU1_HALTED_A, O>;
impl<'a, const O: u8> VPU1_HALTED_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(VPU1_HALTED_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(VPU1_HALTED_A::EDGE)
    }
}
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader<ARM_ADDRESS_ERROR_A>;
#[doc = "ARM address error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_ADDRESS_ERROR_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ARM_ADDRESS_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_ADDRESS_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_ADDRESS_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_ADDRESS_ERROR_A {
        match self.bits {
            false => ARM_ADDRESS_ERROR_A::LEVEL,
            true => ARM_ADDRESS_ERROR_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ARM_ADDRESS_ERROR_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ARM_ADDRESS_ERROR_A::EDGE
    }
}
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, ARM_ADDRESS_ERROR_A, O>;
impl<'a, const O: u8> ARM_ADDRESS_ERROR_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ARM_ADDRESS_ERROR_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ARM_ADDRESS_ERROR_A::EDGE)
    }
}
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader<ARM_AXI_ERROR_A>;
#[doc = "ARM AXI error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_AXI_ERROR_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ARM_AXI_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_AXI_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_AXI_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_AXI_ERROR_A {
        match self.bits {
            false => ARM_AXI_ERROR_A::LEVEL,
            true => ARM_AXI_ERROR_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ARM_AXI_ERROR_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ARM_AXI_ERROR_A::EDGE
    }
}
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, ARM_AXI_ERROR_A, O>;
impl<'a, const O: u8> ARM_AXI_ERROR_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ARM_AXI_ERROR_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ARM_AXI_ERROR_A::EDGE)
    }
}
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::BitReader<SWI0_A>;
#[doc = "Software interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI0_A> for bool {
    #[inline(always)]
    fn from(variant: SWI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI0_A {
        match self.bits {
            false => SWI0_A::LEVEL,
            true => SWI0_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI0_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI0_A::EDGE
    }
}
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI0_A, O>;
impl<'a, const O: u8> SWI0_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI0_A::EDGE)
    }
}
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::BitReader<SWI1_A>;
#[doc = "Software interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI1_A> for bool {
    #[inline(always)]
    fn from(variant: SWI1_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI1_A {
        match self.bits {
            false => SWI1_A::LEVEL,
            true => SWI1_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI1_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI1_A::EDGE
    }
}
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI1_A, O>;
impl<'a, const O: u8> SWI1_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI1_A::EDGE)
    }
}
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::BitReader<SWI2_A>;
#[doc = "Software interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI2_A> for bool {
    #[inline(always)]
    fn from(variant: SWI2_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI2_A {
        match self.bits {
            false => SWI2_A::LEVEL,
            true => SWI2_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI2_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI2_A::EDGE
    }
}
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI2_A, O>;
impl<'a, const O: u8> SWI2_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI2_A::EDGE)
    }
}
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::BitReader<SWI3_A>;
#[doc = "Software interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI3_A> for bool {
    #[inline(always)]
    fn from(variant: SWI3_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI3_A {
        match self.bits {
            false => SWI3_A::LEVEL,
            true => SWI3_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI3_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI3_A::EDGE
    }
}
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI3_A, O>;
impl<'a, const O: u8> SWI3_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI3_A::EDGE)
    }
}
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::BitReader<SWI4_A>;
#[doc = "Software interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI4_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI4_A> for bool {
    #[inline(always)]
    fn from(variant: SWI4_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI4_A {
        match self.bits {
            false => SWI4_A::LEVEL,
            true => SWI4_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI4_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI4_A::EDGE
    }
}
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI4_A, O>;
impl<'a, const O: u8> SWI4_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI4_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI4_A::EDGE)
    }
}
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::BitReader<SWI5_A>;
#[doc = "Software interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI5_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI5_A> for bool {
    #[inline(always)]
    fn from(variant: SWI5_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI5_A {
        match self.bits {
            false => SWI5_A::LEVEL,
            true => SWI5_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI5_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI5_A::EDGE
    }
}
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI5_A, O>;
impl<'a, const O: u8> SWI5_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI5_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI5_A::EDGE)
    }
}
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::BitReader<SWI6_A>;
#[doc = "Software interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI6_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI6_A> for bool {
    #[inline(always)]
    fn from(variant: SWI6_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI6_A {
        match self.bits {
            false => SWI6_A::LEVEL,
            true => SWI6_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI6_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI6_A::EDGE
    }
}
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI6_A, O>;
impl<'a, const O: u8> SWI6_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI6_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI6_A::EDGE)
    }
}
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::BitReader<SWI7_A>;
#[doc = "Software interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI7_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<SWI7_A> for bool {
    #[inline(always)]
    fn from(variant: SWI7_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWI7_A {
        match self.bits {
            false => SWI7_A::LEVEL,
            true => SWI7_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SWI7_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == SWI7_A::EDGE
    }
}
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_ICFGR16_SPEC, SWI7_A, O>;
impl<'a, const O: u8> SWI7_W<'a, O> {
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SWI7_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(SWI7_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<1> {
        TIMER_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<3> {
        MAILBOX_W::new(self)
    }
    #[doc = "Bit 5 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<5> {
        DOORBELL0_W::new(self)
    }
    #[doc = "Bit 7 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<7> {
        DOORBELL1_W::new(self)
    }
    #[doc = "Bit 9 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<9> {
        VPU0_HALTED_W::new(self)
    }
    #[doc = "Bit 11 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<11> {
        VPU1_HALTED_W::new(self)
    }
    #[doc = "Bit 13 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<13> {
        ARM_ADDRESS_ERROR_W::new(self)
    }
    #[doc = "Bit 15 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<15> {
        ARM_AXI_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<17> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 19 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<19> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 21 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<21> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 23 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<23> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 25 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<25> {
        SWI4_W::new(self)
    }
    #[doc = "Bit 27 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<27> {
        SWI5_W::new(self)
    }
    #[doc = "Bit 29 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<29> {
        SWI6_W::new(self)
    }
    #[doc = "Bit 31 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<31> {
        SWI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration 64 - 79\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr16](index.html) module"]
pub struct GICD_ICFGR16_SPEC;
impl crate::RegisterSpec for GICD_ICFGR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icfgr16::R](R) reader structure"]
impl crate::Readable for GICD_ICFGR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr16::W](W) writer structure"]
impl crate::Writable for GICD_ICFGR16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR16 to value 0"]
impl crate::Resettable for GICD_ICFGR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

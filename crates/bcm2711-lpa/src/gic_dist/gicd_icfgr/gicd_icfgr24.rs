#[doc = "Register `GICD_ICFGR24` reader"]
pub type R = crate::R<GICD_ICFGR24_SPEC>;
#[doc = "Register `GICD_ICFGR24` writer"]
pub type W = crate::W<GICD_ICFGR24_SPEC>;
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type TIMER_0_R = crate::BitReader<TIMER_0_A>;
#[doc = "Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TIMER_0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_0_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_0_A {
        match self.bits {
            false => TIMER_0_A::LEVEL,
            true => TIMER_0_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TIMER_0_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TIMER_0_A::EDGE
    }
}
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type TIMER_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER_0_A>;
impl<'a, REG, const O: u8> TIMER_0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_0_A::EDGE)
    }
}
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type TIMER_1_R = crate::BitReader<TIMER_1_A>;
#[doc = "Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TIMER_1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_1_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_1_A {
        match self.bits {
            false => TIMER_1_A::LEVEL,
            true => TIMER_1_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TIMER_1_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TIMER_1_A::EDGE
    }
}
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type TIMER_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER_1_A>;
impl<'a, REG, const O: u8> TIMER_1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_1_A::EDGE)
    }
}
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type TIMER_2_R = crate::BitReader<TIMER_2_A>;
#[doc = "Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TIMER_2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_2_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_2_A {
        match self.bits {
            false => TIMER_2_A::LEVEL,
            true => TIMER_2_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TIMER_2_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TIMER_2_A::EDGE
    }
}
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type TIMER_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER_2_A>;
impl<'a, REG, const O: u8> TIMER_2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_2_A::EDGE)
    }
}
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type TIMER_3_R = crate::BitReader<TIMER_3_A>;
#[doc = "Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMER_3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TIMER_3_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_3_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMER_3_A {
        match self.bits {
            false => TIMER_3_A::LEVEL,
            true => TIMER_3_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TIMER_3_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TIMER_3_A::EDGE
    }
}
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type TIMER_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TIMER_3_A>;
impl<'a, REG, const O: u8> TIMER_3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_3_A::EDGE)
    }
}
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0_R = crate::BitReader<H264_0_A>;
#[doc = "H264 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<H264_0_A> for bool {
    #[inline(always)]
    fn from(variant: H264_0_A) -> Self {
        variant as u8 != 0
    }
}
impl H264_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_0_A {
        match self.bits {
            false => H264_0_A::LEVEL,
            true => H264_0_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_0_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_0_A::EDGE
    }
}
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, H264_0_A>;
impl<'a, REG, const O: u8> H264_0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_0_A::EDGE)
    }
}
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1_R = crate::BitReader<H264_1_A>;
#[doc = "H264 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<H264_1_A> for bool {
    #[inline(always)]
    fn from(variant: H264_1_A) -> Self {
        variant as u8 != 0
    }
}
impl H264_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_1_A {
        match self.bits {
            false => H264_1_A::LEVEL,
            true => H264_1_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_1_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_1_A::EDGE
    }
}
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, H264_1_A>;
impl<'a, REG, const O: u8> H264_1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_1_A::EDGE)
    }
}
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2_R = crate::BitReader<H264_2_A>;
#[doc = "H264 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<H264_2_A> for bool {
    #[inline(always)]
    fn from(variant: H264_2_A) -> Self {
        variant as u8 != 0
    }
}
impl H264_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264_2_A {
        match self.bits {
            false => H264_2_A::LEVEL,
            true => H264_2_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == H264_2_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == H264_2_A::EDGE
    }
}
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, H264_2_A>;
impl<'a, REG, const O: u8> H264_2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(H264_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(H264_2_A::EDGE)
    }
}
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader<JPEG_A>;
#[doc = "JPEG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JPEG_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<JPEG_A> for bool {
    #[inline(always)]
    fn from(variant: JPEG_A) -> Self {
        variant as u8 != 0
    }
}
impl JPEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JPEG_A {
        match self.bits {
            false => JPEG_A::LEVEL,
            true => JPEG_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == JPEG_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == JPEG_A::EDGE
    }
}
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, JPEG_A>;
impl<'a, REG, const O: u8> JPEG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(JPEG_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(JPEG_A::EDGE)
    }
}
#[doc = "Field `ISP` reader - ISP"]
pub type ISP_R = crate::BitReader<ISP_A>;
#[doc = "ISP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISP_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<ISP_A> for bool {
    #[inline(always)]
    fn from(variant: ISP_A) -> Self {
        variant as u8 != 0
    }
}
impl ISP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISP_A {
        match self.bits {
            false => ISP_A::LEVEL,
            true => ISP_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISP_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ISP_A::EDGE
    }
}
#[doc = "Field `ISP` writer - ISP"]
pub type ISP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ISP_A>;
impl<'a, REG, const O: u8> ISP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ISP_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ISP_A::EDGE)
    }
}
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::BitReader<USB_A>;
#[doc = "USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::LEVEL,
            true => USB_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == USB_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == USB_A::EDGE
    }
}
#[doc = "Field `USB` writer - USB"]
pub type USB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USB_A>;
impl<'a, REG, const O: u8> USB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(USB_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(USB_A::EDGE)
    }
}
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::BitReader<V3D_A>;
#[doc = "V3D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V3D_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<V3D_A> for bool {
    #[inline(always)]
    fn from(variant: V3D_A) -> Self {
        variant as u8 != 0
    }
}
impl V3D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V3D_A {
        match self.bits {
            false => V3D_A::LEVEL,
            true => V3D_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == V3D_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == V3D_A::EDGE
    }
}
#[doc = "Field `V3D` writer - V3D"]
pub type V3D_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, V3D_A>;
impl<'a, REG, const O: u8> V3D_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(V3D_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(V3D_A::EDGE)
    }
}
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TRANSPOSER_R = crate::BitReader<TRANSPOSER_A>;
#[doc = "Transposer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANSPOSER_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<TRANSPOSER_A> for bool {
    #[inline(always)]
    fn from(variant: TRANSPOSER_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANSPOSER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRANSPOSER_A {
        match self.bits {
            false => TRANSPOSER_A::LEVEL,
            true => TRANSPOSER_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == TRANSPOSER_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == TRANSPOSER_A::EDGE
    }
}
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TRANSPOSER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRANSPOSER_A>;
impl<'a, REG, const O: u8> TRANSPOSER_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(TRANSPOSER_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRANSPOSER_A::EDGE)
    }
}
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_R = crate::BitReader<MULTICORE_SYNC_0_A>;
#[doc = "Multicore Sync 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULTICORE_SYNC_0_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<MULTICORE_SYNC_0_A> for bool {
    #[inline(always)]
    fn from(variant: MULTICORE_SYNC_0_A) -> Self {
        variant as u8 != 0
    }
}
impl MULTICORE_SYNC_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MULTICORE_SYNC_0_A {
        match self.bits {
            false => MULTICORE_SYNC_0_A::LEVEL,
            true => MULTICORE_SYNC_0_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MULTICORE_SYNC_0_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MULTICORE_SYNC_0_A::EDGE
    }
}
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, MULTICORE_SYNC_0_A>;
impl<'a, REG, const O: u8> MULTICORE_SYNC_0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_0_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_0_A::EDGE)
    }
}
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_R = crate::BitReader<MULTICORE_SYNC_1_A>;
#[doc = "Multicore Sync 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULTICORE_SYNC_1_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<MULTICORE_SYNC_1_A> for bool {
    #[inline(always)]
    fn from(variant: MULTICORE_SYNC_1_A) -> Self {
        variant as u8 != 0
    }
}
impl MULTICORE_SYNC_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MULTICORE_SYNC_1_A {
        match self.bits {
            false => MULTICORE_SYNC_1_A::LEVEL,
            true => MULTICORE_SYNC_1_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MULTICORE_SYNC_1_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MULTICORE_SYNC_1_A::EDGE
    }
}
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, MULTICORE_SYNC_1_A>;
impl<'a, REG, const O: u8> MULTICORE_SYNC_1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_1_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_1_A::EDGE)
    }
}
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_R = crate::BitReader<MULTICORE_SYNC_2_A>;
#[doc = "Multicore Sync 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULTICORE_SYNC_2_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<MULTICORE_SYNC_2_A> for bool {
    #[inline(always)]
    fn from(variant: MULTICORE_SYNC_2_A) -> Self {
        variant as u8 != 0
    }
}
impl MULTICORE_SYNC_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MULTICORE_SYNC_2_A {
        match self.bits {
            false => MULTICORE_SYNC_2_A::LEVEL,
            true => MULTICORE_SYNC_2_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MULTICORE_SYNC_2_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MULTICORE_SYNC_2_A::EDGE
    }
}
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, MULTICORE_SYNC_2_A>;
impl<'a, REG, const O: u8> MULTICORE_SYNC_2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_2_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_2_A::EDGE)
    }
}
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_R = crate::BitReader<MULTICORE_SYNC_3_A>;
#[doc = "Multicore Sync 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULTICORE_SYNC_3_A {
    #[doc = "0: Level sensitive"]
    LEVEL = 0,
    #[doc = "1: Edge triggered"]
    EDGE = 1,
}
impl From<MULTICORE_SYNC_3_A> for bool {
    #[inline(always)]
    fn from(variant: MULTICORE_SYNC_3_A) -> Self {
        variant as u8 != 0
    }
}
impl MULTICORE_SYNC_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MULTICORE_SYNC_3_A {
        match self.bits {
            false => MULTICORE_SYNC_3_A::LEVEL,
            true => MULTICORE_SYNC_3_A::EDGE,
        }
    }
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == MULTICORE_SYNC_3_A::LEVEL
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == MULTICORE_SYNC_3_A::EDGE
    }
}
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, MULTICORE_SYNC_3_A>;
impl<'a, REG, const O: u8> MULTICORE_SYNC_3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_3_A::LEVEL)
    }
    #[doc = "Edge triggered"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(MULTICORE_SYNC_3_A::EDGE)
    }
}
impl R {
    #[doc = "Bit 1 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> TIMER_0_R {
        TIMER_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> TIMER_1_R {
        TIMER_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> TIMER_2_R {
        TIMER_2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> TIMER_3_R {
        TIMER_3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0_R {
        H264_0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1_R {
        H264_1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2_R {
        H264_2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> ISP_R {
        ISP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TRANSPOSER_R {
        TRANSPOSER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MULTICORE_SYNC_0_R {
        MULTICORE_SYNC_0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MULTICORE_SYNC_1_R {
        MULTICORE_SYNC_1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MULTICORE_SYNC_2_R {
        MULTICORE_SYNC_2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MULTICORE_SYNC_3_R {
        MULTICORE_SYNC_3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICFGR24")
            .field("timer_0", &format_args!("{}", self.timer_0().bit()))
            .field("timer_1", &format_args!("{}", self.timer_1().bit()))
            .field("timer_2", &format_args!("{}", self.timer_2().bit()))
            .field("timer_3", &format_args!("{}", self.timer_3().bit()))
            .field("h264_0", &format_args!("{}", self.h264_0().bit()))
            .field("h264_1", &format_args!("{}", self.h264_1().bit()))
            .field("h264_2", &format_args!("{}", self.h264_2().bit()))
            .field("jpeg", &format_args!("{}", self.jpeg().bit()))
            .field("isp", &format_args!("{}", self.isp().bit()))
            .field("usb", &format_args!("{}", self.usb().bit()))
            .field("v3d", &format_args!("{}", self.v3d().bit()))
            .field("transposer", &format_args!("{}", self.transposer().bit()))
            .field(
                "multicore_sync_0",
                &format_args!("{}", self.multicore_sync_0().bit()),
            )
            .field(
                "multicore_sync_1",
                &format_args!("{}", self.multicore_sync_1().bit()),
            )
            .field(
                "multicore_sync_2",
                &format_args!("{}", self.multicore_sync_2().bit()),
            )
            .field(
                "multicore_sync_3",
                &format_args!("{}", self.multicore_sync_3().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ICFGR24_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> TIMER_0_W<GICD_ICFGR24_SPEC, 1> {
        TIMER_0_W::new(self)
    }
    #[doc = "Bit 3 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> TIMER_1_W<GICD_ICFGR24_SPEC, 3> {
        TIMER_1_W::new(self)
    }
    #[doc = "Bit 5 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> TIMER_2_W<GICD_ICFGR24_SPEC, 5> {
        TIMER_2_W::new(self)
    }
    #[doc = "Bit 7 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> TIMER_3_W<GICD_ICFGR24_SPEC, 7> {
        TIMER_3_W::new(self)
    }
    #[doc = "Bit 9 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0_W<GICD_ICFGR24_SPEC, 9> {
        H264_0_W::new(self)
    }
    #[doc = "Bit 11 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1_W<GICD_ICFGR24_SPEC, 11> {
        H264_1_W::new(self)
    }
    #[doc = "Bit 13 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2_W<GICD_ICFGR24_SPEC, 13> {
        H264_2_W::new(self)
    }
    #[doc = "Bit 15 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<GICD_ICFGR24_SPEC, 15> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 17 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> ISP_W<GICD_ICFGR24_SPEC, 17> {
        ISP_W::new(self)
    }
    #[doc = "Bit 19 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<GICD_ICFGR24_SPEC, 19> {
        USB_W::new(self)
    }
    #[doc = "Bit 21 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3D_W<GICD_ICFGR24_SPEC, 21> {
        V3D_W::new(self)
    }
    #[doc = "Bit 23 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TRANSPOSER_W<GICD_ICFGR24_SPEC, 23> {
        TRANSPOSER_W::new(self)
    }
    #[doc = "Bit 25 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MULTICORE_SYNC_0_W<GICD_ICFGR24_SPEC, 25> {
        MULTICORE_SYNC_0_W::new(self)
    }
    #[doc = "Bit 27 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MULTICORE_SYNC_1_W<GICD_ICFGR24_SPEC, 27> {
        MULTICORE_SYNC_1_W::new(self)
    }
    #[doc = "Bit 29 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MULTICORE_SYNC_2_W<GICD_ICFGR24_SPEC, 29> {
        MULTICORE_SYNC_2_W::new(self)
    }
    #[doc = "Bit 31 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MULTICORE_SYNC_3_W<GICD_ICFGR24_SPEC, 31> {
        MULTICORE_SYNC_3_W::new(self)
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
#[doc = "Interrupt Configuration 96 - 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICFGR24_SPEC;
impl crate::RegisterSpec for GICD_ICFGR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icfgr24::R`](R) reader structure"]
impl crate::Readable for GICD_ICFGR24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_icfgr24::W`](W) writer structure"]
impl crate::Writable for GICD_ICFGR24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICFGR24 to value 0"]
impl crate::Resettable for GICD_ICFGR24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

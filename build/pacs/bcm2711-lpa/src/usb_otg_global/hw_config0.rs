#[doc = "Register `HW_CONFIG0` reader"]
pub type R = crate::R<HW_CONFIG0_SPEC>;
#[doc = "Field `OPERATING_MODE` reader - Operating Mode"]
pub type OPERATING_MODE_R = crate::FieldReader<OPERATING_MODE_A>;
#[doc = "Operating Mode"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPERATING_MODE_A {
    #[doc = "0: `0`"]
    HNP_SRP_CAPABLE = 0,
    #[doc = "1: `1`"]
    SRP_ONLY_CAPABLE = 1,
    #[doc = "2: `10`"]
    NO_HNP_SRP_CAPABLE = 2,
    #[doc = "3: `11`"]
    SRP_CAPABLE_DEVICE = 3,
    #[doc = "4: `100`"]
    NO_SRP_CAPABLE_DEVICE = 4,
    #[doc = "5: `101`"]
    SRP_CAPABLE_HOST = 5,
    #[doc = "6: `110`"]
    NO_SRP_CAPABLE_HOST = 6,
}
impl From<OPERATING_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPERATING_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPERATING_MODE_A {
    type Ux = u8;
}
impl OPERATING_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPERATING_MODE_A> {
        match self.bits {
            0 => Some(OPERATING_MODE_A::HNP_SRP_CAPABLE),
            1 => Some(OPERATING_MODE_A::SRP_ONLY_CAPABLE),
            2 => Some(OPERATING_MODE_A::NO_HNP_SRP_CAPABLE),
            3 => Some(OPERATING_MODE_A::SRP_CAPABLE_DEVICE),
            4 => Some(OPERATING_MODE_A::NO_SRP_CAPABLE_DEVICE),
            5 => Some(OPERATING_MODE_A::SRP_CAPABLE_HOST),
            6 => Some(OPERATING_MODE_A::NO_SRP_CAPABLE_HOST),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hnp_srp_capable(&self) -> bool {
        *self == OPERATING_MODE_A::HNP_SRP_CAPABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_srp_only_capable(&self) -> bool {
        *self == OPERATING_MODE_A::SRP_ONLY_CAPABLE
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_no_hnp_srp_capable(&self) -> bool {
        *self == OPERATING_MODE_A::NO_HNP_SRP_CAPABLE
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_srp_capable_device(&self) -> bool {
        *self == OPERATING_MODE_A::SRP_CAPABLE_DEVICE
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_no_srp_capable_device(&self) -> bool {
        *self == OPERATING_MODE_A::NO_SRP_CAPABLE_DEVICE
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_srp_capable_host(&self) -> bool {
        *self == OPERATING_MODE_A::SRP_CAPABLE_HOST
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_no_srp_capable_host(&self) -> bool {
        *self == OPERATING_MODE_A::NO_SRP_CAPABLE_HOST
    }
}
#[doc = "Field `ARCHITECTURE` reader - Architecture"]
pub type ARCHITECTURE_R = crate::FieldReader<ARCHITECTURE_A>;
#[doc = "Architecture"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCHITECTURE_A {
    #[doc = "0: `0`"]
    SLAVE_ONLY = 0,
    #[doc = "1: `1`"]
    EXTERNAL_DMA = 1,
    #[doc = "2: `10`"]
    INTERNAL_DMA = 2,
}
impl From<ARCHITECTURE_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCHITECTURE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCHITECTURE_A {
    type Ux = u8;
}
impl ARCHITECTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARCHITECTURE_A> {
        match self.bits {
            0 => Some(ARCHITECTURE_A::SLAVE_ONLY),
            1 => Some(ARCHITECTURE_A::EXTERNAL_DMA),
            2 => Some(ARCHITECTURE_A::INTERNAL_DMA),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_slave_only(&self) -> bool {
        *self == ARCHITECTURE_A::SLAVE_ONLY
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_external_dma(&self) -> bool {
        *self == ARCHITECTURE_A::EXTERNAL_DMA
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_internal_dma(&self) -> bool {
        *self == ARCHITECTURE_A::INTERNAL_DMA
    }
}
#[doc = "Field `POINT_TO_POINT` reader - Point to Point"]
pub type POINT_TO_POINT_R = crate::BitReader;
#[doc = "Field `HIGH_SPEED_PHY` reader - High Speed Physical"]
pub type HIGH_SPEED_PHY_R = crate::FieldReader<HIGH_SPEED_PHY_A>;
#[doc = "High Speed Physical"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIGH_SPEED_PHY_A {
    #[doc = "0: `0`"]
    NOT_SUPPORTED = 0,
    #[doc = "1: `1`"]
    UTMI = 1,
    #[doc = "2: `10`"]
    ULPI = 2,
    #[doc = "3: `11`"]
    UTMI_ULPI = 3,
}
impl From<HIGH_SPEED_PHY_A> for u8 {
    #[inline(always)]
    fn from(variant: HIGH_SPEED_PHY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HIGH_SPEED_PHY_A {
    type Ux = u8;
}
impl HIGH_SPEED_PHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIGH_SPEED_PHY_A {
        match self.bits {
            0 => HIGH_SPEED_PHY_A::NOT_SUPPORTED,
            1 => HIGH_SPEED_PHY_A::UTMI,
            2 => HIGH_SPEED_PHY_A::ULPI,
            3 => HIGH_SPEED_PHY_A::UTMI_ULPI,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == HIGH_SPEED_PHY_A::NOT_SUPPORTED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == HIGH_SPEED_PHY_A::UTMI
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == HIGH_SPEED_PHY_A::ULPI
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_utmi_ulpi(&self) -> bool {
        *self == HIGH_SPEED_PHY_A::UTMI_ULPI
    }
}
#[doc = "Field `FULL_SPEED_PHY` reader - Full Speed Physical"]
pub type FULL_SPEED_PHY_R = crate::FieldReader<FULL_SPEED_PHY_A>;
#[doc = "Full Speed Physical"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FULL_SPEED_PHY_A {
    #[doc = "0: `0`"]
    PHY0 = 0,
    #[doc = "1: `1`"]
    DEDICATED = 1,
    #[doc = "2: `10`"]
    PHY2 = 2,
    #[doc = "3: `11`"]
    PHY3 = 3,
}
impl From<FULL_SPEED_PHY_A> for u8 {
    #[inline(always)]
    fn from(variant: FULL_SPEED_PHY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FULL_SPEED_PHY_A {
    type Ux = u8;
}
impl FULL_SPEED_PHY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FULL_SPEED_PHY_A {
        match self.bits {
            0 => FULL_SPEED_PHY_A::PHY0,
            1 => FULL_SPEED_PHY_A::DEDICATED,
            2 => FULL_SPEED_PHY_A::PHY2,
            3 => FULL_SPEED_PHY_A::PHY3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_phy0(&self) -> bool {
        *self == FULL_SPEED_PHY_A::PHY0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_dedicated(&self) -> bool {
        *self == FULL_SPEED_PHY_A::DEDICATED
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_phy2(&self) -> bool {
        *self == FULL_SPEED_PHY_A::PHY2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_phy3(&self) -> bool {
        *self == FULL_SPEED_PHY_A::PHY3
    }
}
#[doc = "Field `DEVICE_END_POINT_COUNT` reader - Device end point count"]
pub type DEVICE_END_POINT_COUNT_R = crate::FieldReader;
#[doc = "Field `HOST_CHANNEL_COUNT` reader - Host channel count"]
pub type HOST_CHANNEL_COUNT_R = crate::FieldReader;
#[doc = "Field `SUPPORTS_PERIODIC_ENDPOINTS` reader - Supports periodic endpoints"]
pub type SUPPORTS_PERIODIC_ENDPOINTS_R = crate::BitReader;
#[doc = "Field `DYNAMIC_FIFO` reader - Dynamic FIFO"]
pub type DYNAMIC_FIFO_R = crate::BitReader;
#[doc = "Field `MULTI_PROC_INT` reader - Multi proc int"]
pub type MULTI_PROC_INT_R = crate::BitReader;
#[doc = "Field `NON_PERIODIC_QUEUE_DEPTH` reader - Non periodic queue depth"]
pub type NON_PERIODIC_QUEUE_DEPTH_R = crate::FieldReader;
#[doc = "Field `HOST_PERIODIC_QUEUE_DEPTH` reader - Host periodic queue depth"]
pub type HOST_PERIODIC_QUEUE_DEPTH_R = crate::FieldReader;
#[doc = "Field `DEVICE_TOKEN_QUEUE_DEPTH` reader - Device token queue depth"]
pub type DEVICE_TOKEN_QUEUE_DEPTH_R = crate::FieldReader;
#[doc = "Field `ENABLE_IC_USB` reader - Enable IC USB"]
pub type ENABLE_IC_USB_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Operating Mode"]
    #[inline(always)]
    pub fn operating_mode(&self) -> OPERATING_MODE_R {
        OPERATING_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Architecture"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Point to Point"]
    #[inline(always)]
    pub fn point_to_point(&self) -> POINT_TO_POINT_R {
        POINT_TO_POINT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - High Speed Physical"]
    #[inline(always)]
    pub fn high_speed_phy(&self) -> HIGH_SPEED_PHY_R {
        HIGH_SPEED_PHY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Full Speed Physical"]
    #[inline(always)]
    pub fn full_speed_phy(&self) -> FULL_SPEED_PHY_R {
        FULL_SPEED_PHY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Device end point count"]
    #[inline(always)]
    pub fn device_end_point_count(&self) -> DEVICE_END_POINT_COUNT_R {
        DEVICE_END_POINT_COUNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Host channel count"]
    #[inline(always)]
    pub fn host_channel_count(&self) -> HOST_CHANNEL_COUNT_R {
        HOST_CHANNEL_COUNT_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Supports periodic endpoints"]
    #[inline(always)]
    pub fn supports_periodic_endpoints(&self) -> SUPPORTS_PERIODIC_ENDPOINTS_R {
        SUPPORTS_PERIODIC_ENDPOINTS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dynamic FIFO"]
    #[inline(always)]
    pub fn dynamic_fifo(&self) -> DYNAMIC_FIFO_R {
        DYNAMIC_FIFO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi proc int"]
    #[inline(always)]
    pub fn multi_proc_int(&self) -> MULTI_PROC_INT_R {
        MULTI_PROC_INT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Non periodic queue depth"]
    #[inline(always)]
    pub fn non_periodic_queue_depth(&self) -> NON_PERIODIC_QUEUE_DEPTH_R {
        NON_PERIODIC_QUEUE_DEPTH_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Host periodic queue depth"]
    #[inline(always)]
    pub fn host_periodic_queue_depth(&self) -> HOST_PERIODIC_QUEUE_DEPTH_R {
        HOST_PERIODIC_QUEUE_DEPTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - Device token queue depth"]
    #[inline(always)]
    pub fn device_token_queue_depth(&self) -> DEVICE_TOKEN_QUEUE_DEPTH_R {
        DEVICE_TOKEN_QUEUE_DEPTH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable IC USB"]
    #[inline(always)]
    pub fn enable_ic_usb(&self) -> ENABLE_IC_USB_R {
        ENABLE_IC_USB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CONFIG0")
            .field(
                "operating_mode",
                &format_args!("{}", self.operating_mode().bits()),
            )
            .field(
                "architecture",
                &format_args!("{}", self.architecture().bits()),
            )
            .field(
                "point_to_point",
                &format_args!("{}", self.point_to_point().bit()),
            )
            .field(
                "high_speed_phy",
                &format_args!("{}", self.high_speed_phy().bits()),
            )
            .field(
                "full_speed_phy",
                &format_args!("{}", self.full_speed_phy().bits()),
            )
            .field(
                "device_end_point_count",
                &format_args!("{}", self.device_end_point_count().bits()),
            )
            .field(
                "host_channel_count",
                &format_args!("{}", self.host_channel_count().bits()),
            )
            .field(
                "supports_periodic_endpoints",
                &format_args!("{}", self.supports_periodic_endpoints().bit()),
            )
            .field(
                "dynamic_fifo",
                &format_args!("{}", self.dynamic_fifo().bit()),
            )
            .field(
                "multi_proc_int",
                &format_args!("{}", self.multi_proc_int().bit()),
            )
            .field(
                "non_periodic_queue_depth",
                &format_args!("{}", self.non_periodic_queue_depth().bits()),
            )
            .field(
                "host_periodic_queue_depth",
                &format_args!("{}", self.host_periodic_queue_depth().bits()),
            )
            .field(
                "device_token_queue_depth",
                &format_args!("{}", self.device_token_queue_depth().bits()),
            )
            .field(
                "enable_ic_usb",
                &format_args!("{}", self.enable_ic_usb().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HW_CONFIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Hardware Config 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_config0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_CONFIG0_SPEC;
impl crate::RegisterSpec for HW_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_config0::R`](R) reader structure"]
impl crate::Readable for HW_CONFIG0_SPEC {}

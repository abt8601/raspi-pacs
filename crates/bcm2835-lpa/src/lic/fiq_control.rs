#[doc = "Register `FIQ_CONTROL` reader"]
pub type R = crate::R<FIQ_CONTROL_SPEC>;
#[doc = "Register `FIQ_CONTROL` writer"]
pub type W = crate::W<FIQ_CONTROL_SPEC>;
#[doc = "Field `SOURCE` reader - FIQ Source"]
pub type SOURCE_R = crate::FieldReader<SOURCE_A>;
#[doc = "FIQ Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Timer 0"]
    TIMER_0 = 0,
    #[doc = "1: Timer 1"]
    TIMER_1 = 1,
    #[doc = "2: Timer 2"]
    TIMER_2 = 2,
    #[doc = "3: Timer 3"]
    TIMER_3 = 3,
    #[doc = "4: H264 0"]
    H264_0 = 4,
    #[doc = "5: H264 1"]
    H264_1 = 5,
    #[doc = "6: H264 2"]
    H264_2 = 6,
    #[doc = "7: JPEG"]
    JPEG = 7,
    #[doc = "8: ISP"]
    ISP = 8,
    #[doc = "9: USB"]
    USB = 9,
    #[doc = "10: V3D"]
    V3D = 10,
    #[doc = "11: Transposer"]
    TRANSPOSER = 11,
    #[doc = "12: Multicore Sync 0"]
    MULTICORE_SYNC_0 = 12,
    #[doc = "13: Multicore Sync 1"]
    MULTICORE_SYNC_1 = 13,
    #[doc = "14: Multicore Sync 2"]
    MULTICORE_SYNC_2 = 14,
    #[doc = "15: Multicore Sync 3"]
    MULTICORE_SYNC_3 = 15,
    #[doc = "16: DMA 0"]
    DMA_0 = 16,
    #[doc = "17: DMA 1"]
    DMA_1 = 17,
    #[doc = "18: DMA 2"]
    DMA_2 = 18,
    #[doc = "19: DMA 3"]
    DMA_3 = 19,
    #[doc = "20: DMA 4"]
    DMA_4 = 20,
    #[doc = "21: DMA 5"]
    DMA_5 = 21,
    #[doc = "22: DMA 6"]
    DMA_6 = 22,
    #[doc = "23: OR of DMA 7 and 8"]
    DMA_7_8 = 23,
    #[doc = "24: OR of DMA 9 and 10"]
    DMA_9_10 = 24,
    #[doc = "25: DMA 11"]
    DMA_11 = 25,
    #[doc = "26: DMA 12"]
    DMA_12 = 26,
    #[doc = "27: DMA 13"]
    DMA_13 = 27,
    #[doc = "28: DMA 14"]
    DMA_14 = 28,
    #[doc = "29: OR of UART1, SPI1 and SPI2"]
    AUX = 29,
    #[doc = "30: ARM"]
    ARM = 30,
    #[doc = "31: DMA 15"]
    DMA_15 = 31,
    #[doc = "32: HDMI CEC"]
    HDMI_CEC = 32,
    #[doc = "33: HVS"]
    HVS = 33,
    #[doc = "34: RPIVID"]
    RPIVID = 34,
    #[doc = "35: SDC"]
    SDC = 35,
    #[doc = "36: DSI 0"]
    DSI_0 = 36,
    #[doc = "37: Pixel Valve 2"]
    PIXEL_VALVE_2 = 37,
    #[doc = "38: Camera 0"]
    CAMERA_0 = 38,
    #[doc = "39: Camera 1"]
    CAMERA_1 = 39,
    #[doc = "40: HDMI 0"]
    HDMI_0 = 40,
    #[doc = "41: HDMI 1"]
    HDMI_1 = 41,
    #[doc = "42: Pixel Valve 3"]
    PIXEL_VALVE_3 = 42,
    #[doc = "43: SPI/BSC Slave"]
    SPI_BSC_SLAVE = 43,
    #[doc = "44: DSI 1"]
    DSI_1 = 44,
    #[doc = "45: Pixel Valve 0"]
    PIXEL_VALVE_0 = 45,
    #[doc = "46: OR of Pixel Valve 1 and 2"]
    PIXEL_VALVE_1_2 = 46,
    #[doc = "47: CPR"]
    CPR = 47,
    #[doc = "48: SMI"]
    SMI = 48,
    #[doc = "49: GPIO 0"]
    GPIO_0 = 49,
    #[doc = "50: GPIO 1"]
    GPIO_1 = 50,
    #[doc = "51: GPIO 2"]
    GPIO_2 = 51,
    #[doc = "52: GPIO 3"]
    GPIO_3 = 52,
    #[doc = "53: OR of all I2C"]
    I2C = 53,
    #[doc = "54: OR of all SPI"]
    SPI = 54,
    #[doc = "55: PCM/I2S"]
    PCM_I2S = 55,
    #[doc = "56: SDHOST"]
    SDHOST = 56,
    #[doc = "57: OR of all PL011 UARTs"]
    UART = 57,
    #[doc = "58: OR of all ETH_PCIe L2"]
    ETH_PCIE = 58,
    #[doc = "59: VEC"]
    VEC = 59,
    #[doc = "60: CPG"]
    CPG = 60,
    #[doc = "61: RNG"]
    RNG = 61,
    #[doc = "62: OR of EMMC and EMMC2"]
    EMMC = 62,
    #[doc = "63: ETH_PCIe secure"]
    ETH_PCIE_SECURE = 63,
    #[doc = "64: ARMC Timer"]
    TIMER = 64,
    #[doc = "65: Mailbox"]
    MAILBOX = 65,
    #[doc = "66: Doorbell 0"]
    DOORBELL0 = 66,
    #[doc = "67: Doorbell 1"]
    DOORBELL1 = 67,
    #[doc = "68: VPU0 halted"]
    VPU0_HALTED = 68,
    #[doc = "69: VPU1 halted"]
    VPU1_HALTED = 69,
    #[doc = "70: ARM address error"]
    ARM_ADDRESS_ERROR = 70,
    #[doc = "71: ARM AXI error"]
    ARM_AXI_ERROR = 71,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOURCE_A {
    type Ux = u8;
}
impl SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOURCE_A> {
        match self.bits {
            0 => Some(SOURCE_A::TIMER_0),
            1 => Some(SOURCE_A::TIMER_1),
            2 => Some(SOURCE_A::TIMER_2),
            3 => Some(SOURCE_A::TIMER_3),
            4 => Some(SOURCE_A::H264_0),
            5 => Some(SOURCE_A::H264_1),
            6 => Some(SOURCE_A::H264_2),
            7 => Some(SOURCE_A::JPEG),
            8 => Some(SOURCE_A::ISP),
            9 => Some(SOURCE_A::USB),
            10 => Some(SOURCE_A::V3D),
            11 => Some(SOURCE_A::TRANSPOSER),
            12 => Some(SOURCE_A::MULTICORE_SYNC_0),
            13 => Some(SOURCE_A::MULTICORE_SYNC_1),
            14 => Some(SOURCE_A::MULTICORE_SYNC_2),
            15 => Some(SOURCE_A::MULTICORE_SYNC_3),
            16 => Some(SOURCE_A::DMA_0),
            17 => Some(SOURCE_A::DMA_1),
            18 => Some(SOURCE_A::DMA_2),
            19 => Some(SOURCE_A::DMA_3),
            20 => Some(SOURCE_A::DMA_4),
            21 => Some(SOURCE_A::DMA_5),
            22 => Some(SOURCE_A::DMA_6),
            23 => Some(SOURCE_A::DMA_7_8),
            24 => Some(SOURCE_A::DMA_9_10),
            25 => Some(SOURCE_A::DMA_11),
            26 => Some(SOURCE_A::DMA_12),
            27 => Some(SOURCE_A::DMA_13),
            28 => Some(SOURCE_A::DMA_14),
            29 => Some(SOURCE_A::AUX),
            30 => Some(SOURCE_A::ARM),
            31 => Some(SOURCE_A::DMA_15),
            32 => Some(SOURCE_A::HDMI_CEC),
            33 => Some(SOURCE_A::HVS),
            34 => Some(SOURCE_A::RPIVID),
            35 => Some(SOURCE_A::SDC),
            36 => Some(SOURCE_A::DSI_0),
            37 => Some(SOURCE_A::PIXEL_VALVE_2),
            38 => Some(SOURCE_A::CAMERA_0),
            39 => Some(SOURCE_A::CAMERA_1),
            40 => Some(SOURCE_A::HDMI_0),
            41 => Some(SOURCE_A::HDMI_1),
            42 => Some(SOURCE_A::PIXEL_VALVE_3),
            43 => Some(SOURCE_A::SPI_BSC_SLAVE),
            44 => Some(SOURCE_A::DSI_1),
            45 => Some(SOURCE_A::PIXEL_VALVE_0),
            46 => Some(SOURCE_A::PIXEL_VALVE_1_2),
            47 => Some(SOURCE_A::CPR),
            48 => Some(SOURCE_A::SMI),
            49 => Some(SOURCE_A::GPIO_0),
            50 => Some(SOURCE_A::GPIO_1),
            51 => Some(SOURCE_A::GPIO_2),
            52 => Some(SOURCE_A::GPIO_3),
            53 => Some(SOURCE_A::I2C),
            54 => Some(SOURCE_A::SPI),
            55 => Some(SOURCE_A::PCM_I2S),
            56 => Some(SOURCE_A::SDHOST),
            57 => Some(SOURCE_A::UART),
            58 => Some(SOURCE_A::ETH_PCIE),
            59 => Some(SOURCE_A::VEC),
            60 => Some(SOURCE_A::CPG),
            61 => Some(SOURCE_A::RNG),
            62 => Some(SOURCE_A::EMMC),
            63 => Some(SOURCE_A::ETH_PCIE_SECURE),
            64 => Some(SOURCE_A::TIMER),
            65 => Some(SOURCE_A::MAILBOX),
            66 => Some(SOURCE_A::DOORBELL0),
            67 => Some(SOURCE_A::DOORBELL1),
            68 => Some(SOURCE_A::VPU0_HALTED),
            69 => Some(SOURCE_A::VPU1_HALTED),
            70 => Some(SOURCE_A::ARM_ADDRESS_ERROR),
            71 => Some(SOURCE_A::ARM_AXI_ERROR),
            _ => None,
        }
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn is_timer_0(&self) -> bool {
        *self == SOURCE_A::TIMER_0
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn is_timer_1(&self) -> bool {
        *self == SOURCE_A::TIMER_1
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn is_timer_2(&self) -> bool {
        *self == SOURCE_A::TIMER_2
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn is_timer_3(&self) -> bool {
        *self == SOURCE_A::TIMER_3
    }
    #[doc = "H264 0"]
    #[inline(always)]
    pub fn is_h264_0(&self) -> bool {
        *self == SOURCE_A::H264_0
    }
    #[doc = "H264 1"]
    #[inline(always)]
    pub fn is_h264_1(&self) -> bool {
        *self == SOURCE_A::H264_1
    }
    #[doc = "H264 2"]
    #[inline(always)]
    pub fn is_h264_2(&self) -> bool {
        *self == SOURCE_A::H264_2
    }
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn is_jpeg(&self) -> bool {
        *self == SOURCE_A::JPEG
    }
    #[doc = "ISP"]
    #[inline(always)]
    pub fn is_isp(&self) -> bool {
        *self == SOURCE_A::ISP
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCE_A::USB
    }
    #[doc = "V3D"]
    #[inline(always)]
    pub fn is_v3d(&self) -> bool {
        *self == SOURCE_A::V3D
    }
    #[doc = "Transposer"]
    #[inline(always)]
    pub fn is_transposer(&self) -> bool {
        *self == SOURCE_A::TRANSPOSER
    }
    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn is_multicore_sync_0(&self) -> bool {
        *self == SOURCE_A::MULTICORE_SYNC_0
    }
    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn is_multicore_sync_1(&self) -> bool {
        *self == SOURCE_A::MULTICORE_SYNC_1
    }
    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn is_multicore_sync_2(&self) -> bool {
        *self == SOURCE_A::MULTICORE_SYNC_2
    }
    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn is_multicore_sync_3(&self) -> bool {
        *self == SOURCE_A::MULTICORE_SYNC_3
    }
    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn is_dma_0(&self) -> bool {
        *self == SOURCE_A::DMA_0
    }
    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn is_dma_1(&self) -> bool {
        *self == SOURCE_A::DMA_1
    }
    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn is_dma_2(&self) -> bool {
        *self == SOURCE_A::DMA_2
    }
    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn is_dma_3(&self) -> bool {
        *self == SOURCE_A::DMA_3
    }
    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn is_dma_4(&self) -> bool {
        *self == SOURCE_A::DMA_4
    }
    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn is_dma_5(&self) -> bool {
        *self == SOURCE_A::DMA_5
    }
    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn is_dma_6(&self) -> bool {
        *self == SOURCE_A::DMA_6
    }
    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn is_dma_7_8(&self) -> bool {
        *self == SOURCE_A::DMA_7_8
    }
    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn is_dma_9_10(&self) -> bool {
        *self == SOURCE_A::DMA_9_10
    }
    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn is_dma_11(&self) -> bool {
        *self == SOURCE_A::DMA_11
    }
    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn is_dma_12(&self) -> bool {
        *self == SOURCE_A::DMA_12
    }
    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn is_dma_13(&self) -> bool {
        *self == SOURCE_A::DMA_13
    }
    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn is_dma_14(&self) -> bool {
        *self == SOURCE_A::DMA_14
    }
    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn is_aux(&self) -> bool {
        *self == SOURCE_A::AUX
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == SOURCE_A::ARM
    }
    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn is_dma_15(&self) -> bool {
        *self == SOURCE_A::DMA_15
    }
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn is_hdmi_cec(&self) -> bool {
        *self == SOURCE_A::HDMI_CEC
    }
    #[doc = "HVS"]
    #[inline(always)]
    pub fn is_hvs(&self) -> bool {
        *self == SOURCE_A::HVS
    }
    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn is_rpivid(&self) -> bool {
        *self == SOURCE_A::RPIVID
    }
    #[doc = "SDC"]
    #[inline(always)]
    pub fn is_sdc(&self) -> bool {
        *self == SOURCE_A::SDC
    }
    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn is_dsi_0(&self) -> bool {
        *self == SOURCE_A::DSI_0
    }
    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn is_pixel_valve_2(&self) -> bool {
        *self == SOURCE_A::PIXEL_VALVE_2
    }
    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn is_camera_0(&self) -> bool {
        *self == SOURCE_A::CAMERA_0
    }
    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn is_camera_1(&self) -> bool {
        *self == SOURCE_A::CAMERA_1
    }
    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn is_hdmi_0(&self) -> bool {
        *self == SOURCE_A::HDMI_0
    }
    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn is_hdmi_1(&self) -> bool {
        *self == SOURCE_A::HDMI_1
    }
    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn is_pixel_valve_3(&self) -> bool {
        *self == SOURCE_A::PIXEL_VALVE_3
    }
    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn is_spi_bsc_slave(&self) -> bool {
        *self == SOURCE_A::SPI_BSC_SLAVE
    }
    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn is_dsi_1(&self) -> bool {
        *self == SOURCE_A::DSI_1
    }
    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn is_pixel_valve_0(&self) -> bool {
        *self == SOURCE_A::PIXEL_VALVE_0
    }
    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn is_pixel_valve_1_2(&self) -> bool {
        *self == SOURCE_A::PIXEL_VALVE_1_2
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn is_cpr(&self) -> bool {
        *self == SOURCE_A::CPR
    }
    #[doc = "SMI"]
    #[inline(always)]
    pub fn is_smi(&self) -> bool {
        *self == SOURCE_A::SMI
    }
    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn is_gpio_0(&self) -> bool {
        *self == SOURCE_A::GPIO_0
    }
    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn is_gpio_1(&self) -> bool {
        *self == SOURCE_A::GPIO_1
    }
    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn is_gpio_2(&self) -> bool {
        *self == SOURCE_A::GPIO_2
    }
    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn is_gpio_3(&self) -> bool {
        *self == SOURCE_A::GPIO_3
    }
    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SOURCE_A::I2C
    }
    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == SOURCE_A::SPI
    }
    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn is_pcm_i2s(&self) -> bool {
        *self == SOURCE_A::PCM_I2S
    }
    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn is_sdhost(&self) -> bool {
        *self == SOURCE_A::SDHOST
    }
    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == SOURCE_A::UART
    }
    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn is_eth_pcie(&self) -> bool {
        *self == SOURCE_A::ETH_PCIE
    }
    #[doc = "VEC"]
    #[inline(always)]
    pub fn is_vec(&self) -> bool {
        *self == SOURCE_A::VEC
    }
    #[doc = "CPG"]
    #[inline(always)]
    pub fn is_cpg(&self) -> bool {
        *self == SOURCE_A::CPG
    }
    #[doc = "RNG"]
    #[inline(always)]
    pub fn is_rng(&self) -> bool {
        *self == SOURCE_A::RNG
    }
    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn is_emmc(&self) -> bool {
        *self == SOURCE_A::EMMC
    }
    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn is_eth_pcie_secure(&self) -> bool {
        *self == SOURCE_A::ETH_PCIE_SECURE
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == SOURCE_A::TIMER
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn is_mailbox(&self) -> bool {
        *self == SOURCE_A::MAILBOX
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn is_doorbell0(&self) -> bool {
        *self == SOURCE_A::DOORBELL0
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn is_doorbell1(&self) -> bool {
        *self == SOURCE_A::DOORBELL1
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn is_vpu0_halted(&self) -> bool {
        *self == SOURCE_A::VPU0_HALTED
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn is_vpu1_halted(&self) -> bool {
        *self == SOURCE_A::VPU1_HALTED
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn is_arm_address_error(&self) -> bool {
        *self == SOURCE_A::ARM_ADDRESS_ERROR
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn is_arm_axi_error(&self) -> bool {
        *self == SOURCE_A::ARM_AXI_ERROR
    }
}
#[doc = "Field `SOURCE` writer - FIQ Source"]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 7, SOURCE_A>;
impl<'a, REG> SOURCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER_0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER_1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER_2)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer_3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER_3)
    }
    #[doc = "H264 0"]
    #[inline(always)]
    pub fn h264_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::H264_0)
    }
    #[doc = "H264 1"]
    #[inline(always)]
    pub fn h264_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::H264_1)
    }
    #[doc = "H264 2"]
    #[inline(always)]
    pub fn h264_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::H264_2)
    }
    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::JPEG)
    }
    #[doc = "ISP"]
    #[inline(always)]
    pub fn isp(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ISP)
    }
    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::USB)
    }
    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::V3D)
    }
    #[doc = "Transposer"]
    #[inline(always)]
    pub fn transposer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TRANSPOSER)
    }
    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MULTICORE_SYNC_0)
    }
    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MULTICORE_SYNC_1)
    }
    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MULTICORE_SYNC_2)
    }
    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MULTICORE_SYNC_3)
    }
    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn dma_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_0)
    }
    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn dma_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_1)
    }
    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_2)
    }
    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_3)
    }
    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn dma_4(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_4)
    }
    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn dma_5(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_5)
    }
    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn dma_6(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_6)
    }
    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_7_8)
    }
    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_9_10)
    }
    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn dma_11(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_11)
    }
    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn dma_12(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_12)
    }
    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn dma_13(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_13)
    }
    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn dma_14(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_14)
    }
    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::AUX)
    }
    #[doc = "ARM"]
    #[inline(always)]
    pub fn arm(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ARM)
    }
    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn dma_15(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DMA_15)
    }
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::HDMI_CEC)
    }
    #[doc = "HVS"]
    #[inline(always)]
    pub fn hvs(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::HVS)
    }
    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn rpivid(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::RPIVID)
    }
    #[doc = "SDC"]
    #[inline(always)]
    pub fn sdc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::SDC)
    }
    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn dsi_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DSI_0)
    }
    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::PIXEL_VALVE_2)
    }
    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn camera_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::CAMERA_0)
    }
    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn camera_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::CAMERA_1)
    }
    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::HDMI_0)
    }
    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::HDMI_1)
    }
    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::PIXEL_VALVE_3)
    }
    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::SPI_BSC_SLAVE)
    }
    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn dsi_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DSI_1)
    }
    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::PIXEL_VALVE_0)
    }
    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::PIXEL_VALVE_1_2)
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::CPR)
    }
    #[doc = "SMI"]
    #[inline(always)]
    pub fn smi(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::SMI)
    }
    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::GPIO_0)
    }
    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::GPIO_1)
    }
    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::GPIO_2)
    }
    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::GPIO_3)
    }
    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::I2C)
    }
    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::SPI)
    }
    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::PCM_I2S)
    }
    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::SDHOST)
    }
    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::UART)
    }
    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ETH_PCIE)
    }
    #[doc = "VEC"]
    #[inline(always)]
    pub fn vec(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::VEC)
    }
    #[doc = "CPG"]
    #[inline(always)]
    pub fn cpg(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::CPG)
    }
    #[doc = "RNG"]
    #[inline(always)]
    pub fn rng(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::RNG)
    }
    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::EMMC)
    }
    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ETH_PCIE_SECURE)
    }
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::TIMER)
    }
    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::MAILBOX)
    }
    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DOORBELL0)
    }
    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::DOORBELL1)
    }
    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::VPU0_HALTED)
    }
    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::VPU1_HALTED)
    }
    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ARM_ADDRESS_ERROR)
    }
    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(self) -> &'a mut crate::W<REG> {
        self.variant(SOURCE_A::ARM_AXI_ERROR)
    }
}
#[doc = "Field `ENABLE` reader - FIQ Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - FIQ Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIQ_CONTROL")
            .field("enable", &format_args!("{}", self.enable().bit()))
            .field("source", &format_args!("{}", self.source().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIQ_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - FIQ Source"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<FIQ_CONTROL_SPEC> {
        SOURCE_W::new(self, 0)
    }
    #[doc = "Bit 7 - FIQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<FIQ_CONTROL_SPEC> {
        ENABLE_W::new(self, 7)
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
#[doc = "FIQ control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fiq_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fiq_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIQ_CONTROL_SPEC;
impl crate::RegisterSpec for FIQ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiq_control::R`](R) reader structure"]
impl crate::Readable for FIQ_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fiq_control::W`](W) writer structure"]
impl crate::Writable for FIQ_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIQ_CONTROL to value 0"]
impl crate::Resettable for FIQ_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

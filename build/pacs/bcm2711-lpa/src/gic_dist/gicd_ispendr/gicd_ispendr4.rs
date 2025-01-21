#[doc = "Register `GICD_ISPENDR4` reader"]
pub type R = crate::R<GICD_ISPENDR4_SPEC>;
#[doc = "Register `GICD_ISPENDR4` writer"]
pub type W = crate::W<GICD_ISPENDR4_SPEC>;
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HDMI_CEC_R = crate::BitReader;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HDMI_CEC_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HVS` reader - HVS"]
pub type HVS_R = crate::BitReader;
#[doc = "Field `HVS` writer - HVS"]
pub type HVS_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RPIVID_R = crate::BitReader;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RPIVID_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SDC` reader - SDC"]
pub type SDC_R = crate::BitReader;
#[doc = "Field `SDC` writer - SDC"]
pub type SDC_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type DSI_0_R = crate::BitReader;
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type DSI_0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PIXEL_VALVE_2_R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PIXEL_VALVE_2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type CAMERA_0_R = crate::BitReader;
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type CAMERA_0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type CAMERA_1_R = crate::BitReader;
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type CAMERA_1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::BitReader;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::BitReader;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::BitReader;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type DSI_1_R = crate::BitReader;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type DSI_1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PIXEL_VALVE_0_R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PIXEL_VALVE_0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_R = crate::BitReader;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CPR` reader - CPR"]
pub type CPR_R = crate::BitReader;
#[doc = "Field `CPR` writer - CPR"]
pub type CPR_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SMI` reader - SMI"]
pub type SMI_R = crate::BitReader;
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type GPIO_0_R = crate::BitReader;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type GPIO_1_R = crate::BitReader;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type GPIO_2_R = crate::BitReader;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type GPIO_3_R = crate::BitReader;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::BitReader;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::BitReader;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::BitReader;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::BitReader;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::BitReader;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_R = crate::BitReader;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VEC` reader - VEC"]
pub type VEC_R = crate::BitReader;
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CPG` reader - CPG"]
pub type CPG_R = crate::BitReader;
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader;
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::BitReader;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_R = crate::BitReader;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HDMI_CEC_R {
        HDMI_CEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HVS_R {
        HVS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RPIVID_R {
        RPIVID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SDC_R {
        SDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> DSI_0_R {
        DSI_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PIXEL_VALVE_2_R {
        PIXEL_VALVE_2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> CAMERA_0_R {
        CAMERA_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> CAMERA_1_R {
        CAMERA_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> HDMI_0_R {
        HDMI_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> HDMI_1_R {
        HDMI_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PIXEL_VALVE_3_R {
        PIXEL_VALVE_3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SPI_BSC_SLAVE_R {
        SPI_BSC_SLAVE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> DSI_1_R {
        DSI_1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PIXEL_VALVE_0_R {
        PIXEL_VALVE_0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PIXEL_VALVE_1_2_R {
        PIXEL_VALVE_1_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SMI_R {
        SMI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> GPIO_0_R {
        GPIO_0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> GPIO_1_R {
        GPIO_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> GPIO_2_R {
        GPIO_2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(&self) -> GPIO_3_R {
        GPIO_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PCM_I2S_R {
        PCM_I2S_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SDHOST_R {
        SDHOST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> ETH_PCIE_R {
        ETH_PCIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CPG_R {
        CPG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EMMC_R {
        EMMC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> ETH_PCIE_SECURE_R {
        ETH_PCIE_SECURE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISPENDR4")
            .field("hdmi_cec", &format_args!("{}", self.hdmi_cec().bit()))
            .field("hvs", &format_args!("{}", self.hvs().bit()))
            .field("rpivid", &format_args!("{}", self.rpivid().bit()))
            .field("sdc", &format_args!("{}", self.sdc().bit()))
            .field("dsi_0", &format_args!("{}", self.dsi_0().bit()))
            .field(
                "pixel_valve_2",
                &format_args!("{}", self.pixel_valve_2().bit()),
            )
            .field("camera_0", &format_args!("{}", self.camera_0().bit()))
            .field("camera_1", &format_args!("{}", self.camera_1().bit()))
            .field("hdmi_0", &format_args!("{}", self.hdmi_0().bit()))
            .field("hdmi_1", &format_args!("{}", self.hdmi_1().bit()))
            .field(
                "pixel_valve_3",
                &format_args!("{}", self.pixel_valve_3().bit()),
            )
            .field(
                "spi_bsc_slave",
                &format_args!("{}", self.spi_bsc_slave().bit()),
            )
            .field("dsi_1", &format_args!("{}", self.dsi_1().bit()))
            .field(
                "pixel_valve_0",
                &format_args!("{}", self.pixel_valve_0().bit()),
            )
            .field(
                "pixel_valve_1_2",
                &format_args!("{}", self.pixel_valve_1_2().bit()),
            )
            .field("cpr", &format_args!("{}", self.cpr().bit()))
            .field("smi", &format_args!("{}", self.smi().bit()))
            .field("gpio_0", &format_args!("{}", self.gpio_0().bit()))
            .field("gpio_1", &format_args!("{}", self.gpio_1().bit()))
            .field("gpio_2", &format_args!("{}", self.gpio_2().bit()))
            .field("gpio_3", &format_args!("{}", self.gpio_3().bit()))
            .field("i2c", &format_args!("{}", self.i2c().bit()))
            .field("spi", &format_args!("{}", self.spi().bit()))
            .field("pcm_i2s", &format_args!("{}", self.pcm_i2s().bit()))
            .field("sdhost", &format_args!("{}", self.sdhost().bit()))
            .field("uart", &format_args!("{}", self.uart().bit()))
            .field("eth_pcie", &format_args!("{}", self.eth_pcie().bit()))
            .field("vec", &format_args!("{}", self.vec().bit()))
            .field("cpg", &format_args!("{}", self.cpg().bit()))
            .field("rng", &format_args!("{}", self.rng().bit()))
            .field("emmc", &format_args!("{}", self.emmc().bit()))
            .field(
                "eth_pcie_secure",
                &format_args!("{}", self.eth_pcie_secure().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ISPENDR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HDMI_CEC_W<GICD_ISPENDR4_SPEC> {
        HDMI_CEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HVS_W<GICD_ISPENDR4_SPEC> {
        HVS_W::new(self, 1)
    }
    #[doc = "Bit 2 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RPIVID_W<GICD_ISPENDR4_SPEC> {
        RPIVID_W::new(self, 2)
    }
    #[doc = "Bit 3 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<GICD_ISPENDR4_SPEC> {
        SDC_W::new(self, 3)
    }
    #[doc = "Bit 4 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> DSI_0_W<GICD_ISPENDR4_SPEC> {
        DSI_0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PIXEL_VALVE_2_W<GICD_ISPENDR4_SPEC> {
        PIXEL_VALVE_2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> CAMERA_0_W<GICD_ISPENDR4_SPEC> {
        CAMERA_0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> CAMERA_1_W<GICD_ISPENDR4_SPEC> {
        CAMERA_1_W::new(self, 7)
    }
    #[doc = "Bit 8 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<GICD_ISPENDR4_SPEC> {
        HDMI_0_W::new(self, 8)
    }
    #[doc = "Bit 9 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<GICD_ISPENDR4_SPEC> {
        HDMI_1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<GICD_ISPENDR4_SPEC> {
        PIXEL_VALVE_3_W::new(self, 10)
    }
    #[doc = "Bit 11 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<GICD_ISPENDR4_SPEC> {
        SPI_BSC_SLAVE_W::new(self, 11)
    }
    #[doc = "Bit 12 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> DSI_1_W<GICD_ISPENDR4_SPEC> {
        DSI_1_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PIXEL_VALVE_0_W<GICD_ISPENDR4_SPEC> {
        PIXEL_VALVE_0_W::new(self, 13)
    }
    #[doc = "Bit 14 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PIXEL_VALVE_1_2_W<GICD_ISPENDR4_SPEC> {
        PIXEL_VALVE_1_2_W::new(self, 14)
    }
    #[doc = "Bit 15 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<GICD_ISPENDR4_SPEC> {
        CPR_W::new(self, 15)
    }
    #[doc = "Bit 16 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<GICD_ISPENDR4_SPEC> {
        SMI_W::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<GICD_ISPENDR4_SPEC> {
        GPIO_0_W::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<GICD_ISPENDR4_SPEC> {
        GPIO_1_W::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<GICD_ISPENDR4_SPEC> {
        GPIO_2_W::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<GICD_ISPENDR4_SPEC> {
        GPIO_3_W::new(self, 20)
    }
    #[doc = "Bit 21 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<GICD_ISPENDR4_SPEC> {
        I2C_W::new(self, 21)
    }
    #[doc = "Bit 22 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<GICD_ISPENDR4_SPEC> {
        SPI_W::new(self, 22)
    }
    #[doc = "Bit 23 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<GICD_ISPENDR4_SPEC> {
        PCM_I2S_W::new(self, 23)
    }
    #[doc = "Bit 24 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<GICD_ISPENDR4_SPEC> {
        SDHOST_W::new(self, 24)
    }
    #[doc = "Bit 25 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<GICD_ISPENDR4_SPEC> {
        UART_W::new(self, 25)
    }
    #[doc = "Bit 26 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<GICD_ISPENDR4_SPEC> {
        ETH_PCIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<GICD_ISPENDR4_SPEC> {
        VEC_W::new(self, 27)
    }
    #[doc = "Bit 28 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<GICD_ISPENDR4_SPEC> {
        CPG_W::new(self, 28)
    }
    #[doc = "Bit 29 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<GICD_ISPENDR4_SPEC> {
        RNG_W::new(self, 29)
    }
    #[doc = "Bit 30 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<GICD_ISPENDR4_SPEC> {
        EMMC_W::new(self, 30)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<GICD_ISPENDR4_SPEC> {
        ETH_PCIE_SECURE_W::new(self, 31)
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
#[doc = "Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR4_SPEC;
impl crate::RegisterSpec for GICD_ISPENDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr4::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr4::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISPENDR4 to value 0"]
impl crate::Resettable for GICD_ISPENDR4_SPEC {
    const RESET_VALUE: u32 = 0;
}

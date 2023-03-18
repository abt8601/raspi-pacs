#[doc = "Register `GICD_IGROUPR4` reader"]
pub struct R(crate::R<GICD_IGROUPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IGROUPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IGROUPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IGROUPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IGROUPR4` writer"]
pub struct W(crate::W<GICD_IGROUPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IGROUPR4_SPEC>;
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
impl From<crate::W<GICD_IGROUPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IGROUPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HDMI_CEC_R = crate::BitReader<bool>;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HDMI_CEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `HVS` reader - HVS"]
pub type HVS_R = crate::BitReader<bool>;
#[doc = "Field `HVS` writer - HVS"]
pub type HVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RPIVID_R = crate::BitReader<bool>;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RPIVID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `SDC` reader - SDC"]
pub type SDC_R = crate::BitReader<bool>;
#[doc = "Field `SDC` writer - SDC"]
pub type SDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type DSI_0_R = crate::BitReader<bool>;
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type DSI_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PIXEL_VALVE_2_R = crate::BitReader<bool>;
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PIXEL_VALVE_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type CAMERA_0_R = crate::BitReader<bool>;
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type CAMERA_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type CAMERA_1_R = crate::BitReader<bool>;
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type CAMERA_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::BitReader<bool>;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::BitReader<bool>;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::BitReader<bool>;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type DSI_1_R = crate::BitReader<bool>;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type DSI_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PIXEL_VALVE_0_R = crate::BitReader<bool>;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PIXEL_VALVE_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_R = crate::BitReader<bool>;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `CPR` reader - CPR"]
pub type CPR_R = crate::BitReader<bool>;
#[doc = "Field `CPR` writer - CPR"]
pub type CPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `SMI` reader - SMI"]
pub type SMI_R = crate::BitReader<bool>;
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type GPIO_0_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type GPIO_1_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type GPIO_2_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `GPIO_3` reader - GPIO 3"]
pub type GPIO_3_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_3` writer - GPIO 3"]
pub type GPIO_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::BitReader<bool>;
#[doc = "Field `I2C` writer - OR of all I2C"]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::BitReader<bool>;
#[doc = "Field `SPI` writer - OR of all SPI"]
pub type SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::BitReader<bool>;
#[doc = "Field `PCM_I2S` writer - PCM/I2S"]
pub type PCM_I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::BitReader<bool>;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::BitReader<bool>;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_R = crate::BitReader<bool>;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `VEC` reader - VEC"]
pub type VEC_R = crate::BitReader<bool>;
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `CPG` reader - CPG"]
pub type CPG_R = crate::BitReader<bool>;
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::BitReader<bool>;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_R = crate::BitReader<bool>;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GICD_IGROUPR4_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HDMI_CEC_W<0> {
        HDMI_CEC_W::new(self)
    }
    #[doc = "Bit 1 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HVS_W<1> {
        HVS_W::new(self)
    }
    #[doc = "Bit 2 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RPIVID_W<2> {
        RPIVID_W::new(self)
    }
    #[doc = "Bit 3 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<3> {
        SDC_W::new(self)
    }
    #[doc = "Bit 4 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> DSI_0_W<4> {
        DSI_0_W::new(self)
    }
    #[doc = "Bit 5 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PIXEL_VALVE_2_W<5> {
        PIXEL_VALVE_2_W::new(self)
    }
    #[doc = "Bit 6 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> CAMERA_0_W<6> {
        CAMERA_0_W::new(self)
    }
    #[doc = "Bit 7 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> CAMERA_1_W<7> {
        CAMERA_1_W::new(self)
    }
    #[doc = "Bit 8 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<8> {
        HDMI_0_W::new(self)
    }
    #[doc = "Bit 9 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<9> {
        HDMI_1_W::new(self)
    }
    #[doc = "Bit 10 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<10> {
        PIXEL_VALVE_3_W::new(self)
    }
    #[doc = "Bit 11 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<11> {
        SPI_BSC_SLAVE_W::new(self)
    }
    #[doc = "Bit 12 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> DSI_1_W<12> {
        DSI_1_W::new(self)
    }
    #[doc = "Bit 13 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PIXEL_VALVE_0_W<13> {
        PIXEL_VALVE_0_W::new(self)
    }
    #[doc = "Bit 14 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PIXEL_VALVE_1_2_W<14> {
        PIXEL_VALVE_1_2_W::new(self)
    }
    #[doc = "Bit 15 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<15> {
        CPR_W::new(self)
    }
    #[doc = "Bit 16 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<16> {
        SMI_W::new(self)
    }
    #[doc = "Bit 17 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<17> {
        GPIO_0_W::new(self)
    }
    #[doc = "Bit 18 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<18> {
        GPIO_1_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<19> {
        GPIO_2_W::new(self)
    }
    #[doc = "Bit 20 - GPIO 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_3(&mut self) -> GPIO_3_W<20> {
        GPIO_3_W::new(self)
    }
    #[doc = "Bit 21 - OR of all I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<21> {
        I2C_W::new(self)
    }
    #[doc = "Bit 22 - OR of all SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<22> {
        SPI_W::new(self)
    }
    #[doc = "Bit 23 - PCM/I2S"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_i2s(&mut self) -> PCM_I2S_W<23> {
        PCM_I2S_W::new(self)
    }
    #[doc = "Bit 24 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<24> {
        SDHOST_W::new(self)
    }
    #[doc = "Bit 25 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<25> {
        UART_W::new(self)
    }
    #[doc = "Bit 26 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<26> {
        ETH_PCIE_W::new(self)
    }
    #[doc = "Bit 27 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<27> {
        VEC_W::new(self)
    }
    #[doc = "Bit 28 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<28> {
        CPG_W::new(self)
    }
    #[doc = "Bit 29 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<29> {
        RNG_W::new(self)
    }
    #[doc = "Bit 30 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<30> {
        EMMC_W::new(self)
    }
    #[doc = "Bit 31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<31> {
        ETH_PCIE_SECURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Group\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr4](index.html) module"]
pub struct GICD_IGROUPR4_SPEC;
impl crate::RegisterSpec for GICD_IGROUPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_igroupr4::R](R) reader structure"]
impl crate::Readable for GICD_IGROUPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr4::W](W) writer structure"]
impl crate::Writable for GICD_IGROUPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR4 to value 0"]
impl crate::Resettable for GICD_IGROUPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

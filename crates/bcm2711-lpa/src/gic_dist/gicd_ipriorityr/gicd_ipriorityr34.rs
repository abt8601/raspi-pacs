#[doc = "Register `GICD_IPRIORITYR34` reader"]
pub type R = crate::R<GICD_IPRIORITYR34_SPEC>;
#[doc = "Register `GICD_IPRIORITYR34` writer"]
pub type W = crate::W<GICD_IPRIORITYR34_SPEC>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::FieldReader;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::FieldReader;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::FieldReader;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(&self) -> HDMI_0_R {
        HDMI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(&self) -> HDMI_1_R {
        HDMI_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(&self) -> PIXEL_VALVE_3_R {
        PIXEL_VALVE_3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(&self) -> SPI_BSC_SLAVE_R {
        SPI_BSC_SLAVE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR34")
            .field("hdmi_0", &format_args!("{}", self.hdmi_0().bits()))
            .field("hdmi_1", &format_args!("{}", self.hdmi_1().bits()))
            .field(
                "pixel_valve_3",
                &format_args!("{}", self.pixel_valve_3().bits()),
            )
            .field(
                "spi_bsc_slave",
                &format_args!("{}", self.spi_bsc_slave().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR34_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<GICD_IPRIORITYR34_SPEC, 0> {
        HDMI_0_W::new(self)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<GICD_IPRIORITYR34_SPEC, 8> {
        HDMI_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<GICD_IPRIORITYR34_SPEC, 16> {
        PIXEL_VALVE_3_W::new(self)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<GICD_IPRIORITYR34_SPEC, 24> {
        SPI_BSC_SLAVE_W::new(self)
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
#[doc = "Interrupt Priority 136 - 139 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR34_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr34::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR34_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr34::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR34_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR34 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

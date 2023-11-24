#[doc = "Register `GICD_ITARGETSR34` reader"]
pub type R = crate::R<GICD_ITARGETSR34_SPEC>;
#[doc = "Register `GICD_ITARGETSR34` writer"]
pub type W = crate::W<GICD_ITARGETSR34_SPEC>;
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::FieldReader;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::FieldReader;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::FieldReader;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
        f.debug_struct("GICD_ITARGETSR34")
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
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR34_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<GICD_ITARGETSR34_SPEC> {
        HDMI_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<GICD_ITARGETSR34_SPEC> {
        HDMI_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<GICD_ITARGETSR34_SPEC> {
        PIXEL_VALVE_3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<GICD_ITARGETSR34_SPEC> {
        SPI_BSC_SLAVE_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 136 - 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR34_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr34::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR34_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr34::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR34_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR34 to value 0"]
impl crate::Resettable for GICD_ITARGETSR34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

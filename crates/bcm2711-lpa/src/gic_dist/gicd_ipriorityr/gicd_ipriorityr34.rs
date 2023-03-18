#[doc = "Register `GICD_IPRIORITYR34` reader"]
pub struct R(crate::R<GICD_IPRIORITYR34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR34` writer"]
pub struct W(crate::W<GICD_IPRIORITYR34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR34_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDMI_0` reader - HDMI 0"]
pub type HDMI_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDMI_0` writer - HDMI 0"]
pub type HDMI_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR34_SPEC, u8, u8, 8, O>;
#[doc = "Field `HDMI_1` reader - HDMI 1"]
pub type HDMI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDMI_1` writer - HDMI 1"]
pub type HDMI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR34_SPEC, u8, u8, 8, O>;
#[doc = "Field `PIXEL_VALVE_3` reader - Pixel Valve 3"]
pub type PIXEL_VALVE_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXEL_VALVE_3` writer - Pixel Valve 3"]
pub type PIXEL_VALVE_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR34_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPI_BSC_SLAVE` reader - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_BSC_SLAVE` writer - SPI/BSC Slave"]
pub type SPI_BSC_SLAVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR34_SPEC, u8, u8, 8, O>;
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
impl W {
    #[doc = "Bits 0:7 - HDMI 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_0(&mut self) -> HDMI_0_W<0> {
        HDMI_0_W::new(self)
    }
    #[doc = "Bits 8:15 - HDMI 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_1(&mut self) -> HDMI_1_W<8> {
        HDMI_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Pixel Valve 3"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_3(&mut self) -> PIXEL_VALVE_3_W<16> {
        PIXEL_VALVE_3_W::new(self)
    }
    #[doc = "Bits 24:31 - SPI/BSC Slave"]
    #[inline(always)]
    #[must_use]
    pub fn spi_bsc_slave(&mut self) -> SPI_BSC_SLAVE_W<24> {
        SPI_BSC_SLAVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 136 - 139 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr34](index.html) module"]
pub struct GICD_IPRIORITYR34_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr34::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr34::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR34_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR34 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

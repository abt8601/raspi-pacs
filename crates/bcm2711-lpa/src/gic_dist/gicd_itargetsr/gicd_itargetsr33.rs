#[doc = "Register `GICD_ITARGETSR33` reader"]
pub type R = crate::R<GICD_ITARGETSR33_SPEC>;
#[doc = "Register `GICD_ITARGETSR33` writer"]
pub type W = crate::W<GICD_ITARGETSR33_SPEC>;
#[doc = "Field `DSI_0` reader - DSI 0"]
pub type DSI_0_R = crate::FieldReader;
#[doc = "Field `DSI_0` writer - DSI 0"]
pub type DSI_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_2` reader - Pixel Valve 2"]
pub type PIXEL_VALVE_2_R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_2` writer - Pixel Valve 2"]
pub type PIXEL_VALVE_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAMERA_0` reader - Camera 0"]
pub type CAMERA_0_R = crate::FieldReader;
#[doc = "Field `CAMERA_0` writer - Camera 0"]
pub type CAMERA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAMERA_1` reader - Camera 1"]
pub type CAMERA_1_R = crate::FieldReader;
#[doc = "Field `CAMERA_1` writer - Camera 1"]
pub type CAMERA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DSI 0"]
    #[inline(always)]
    pub fn dsi_0(&self) -> DSI_0_R {
        DSI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(&self) -> PIXEL_VALVE_2_R {
        PIXEL_VALVE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Camera 0"]
    #[inline(always)]
    pub fn camera_0(&self) -> CAMERA_0_R {
        CAMERA_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Camera 1"]
    #[inline(always)]
    pub fn camera_1(&self) -> CAMERA_1_R {
        CAMERA_1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR33")
            .field("dsi_0", &format_args!("{}", self.dsi_0().bits()))
            .field(
                "pixel_valve_2",
                &format_args!("{}", self.pixel_valve_2().bits()),
            )
            .field("camera_0", &format_args!("{}", self.camera_0().bits()))
            .field("camera_1", &format_args!("{}", self.camera_1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR33_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DSI 0"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_0(&mut self) -> DSI_0_W<GICD_ITARGETSR33_SPEC> {
        DSI_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pixel Valve 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_2(&mut self) -> PIXEL_VALVE_2_W<GICD_ITARGETSR33_SPEC> {
        PIXEL_VALVE_2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Camera 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_0(&mut self) -> CAMERA_0_W<GICD_ITARGETSR33_SPEC> {
        CAMERA_0_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Camera 1"]
    #[inline(always)]
    #[must_use]
    pub fn camera_1(&mut self) -> CAMERA_1_W<GICD_ITARGETSR33_SPEC> {
        CAMERA_1_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 132 - 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR33_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr33::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr33::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR33_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR33 to value 0"]
impl crate::Resettable for GICD_ITARGETSR33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

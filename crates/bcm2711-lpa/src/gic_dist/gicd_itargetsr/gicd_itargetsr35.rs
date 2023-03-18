#[doc = "Register `GICD_ITARGETSR35` reader"]
pub struct R(crate::R<GICD_ITARGETSR35_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR35_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR35_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR35_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR35` writer"]
pub struct W(crate::W<GICD_ITARGETSR35_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR35_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR35_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR35_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type DSI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type DSI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR35_SPEC, u8, u8, 8, O>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PIXEL_VALVE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PIXEL_VALVE_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR35_SPEC, u8, u8, 8, O>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR35_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPR` reader - CPR"]
pub type CPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPR` writer - CPR"]
pub type CPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR35_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DSI 1"]
    #[inline(always)]
    pub fn dsi_1(&self) -> DSI_1_R {
        DSI_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(&self) -> PIXEL_VALVE_0_R {
        PIXEL_VALVE_0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(&self) -> PIXEL_VALVE_1_2_R {
        PIXEL_VALVE_1_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CPR"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> DSI_1_W<0> {
        DSI_1_W::new(self)
    }
    #[doc = "Bits 8:15 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PIXEL_VALVE_0_W<8> {
        PIXEL_VALVE_0_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PIXEL_VALVE_1_2_W<16> {
        PIXEL_VALVE_1_2_W::new(self)
    }
    #[doc = "Bits 24:31 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<24> {
        CPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 140 - 143\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr35](index.html) module"]
pub struct GICD_ITARGETSR35_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr35::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR35_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr35::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR35_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR35 to value 0"]
impl crate::Resettable for GICD_ITARGETSR35_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

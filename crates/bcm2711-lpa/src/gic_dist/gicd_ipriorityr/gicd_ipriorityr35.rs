#[doc = "Register `GICD_IPRIORITYR35` reader"]
pub type R = crate::R<GICD_IPRIORITYR35_SPEC>;
#[doc = "Register `GICD_IPRIORITYR35` writer"]
pub type W = crate::W<GICD_IPRIORITYR35_SPEC>;
#[doc = "Field `DSI_1` reader - DSI 1"]
pub type DSI_1_R = crate::FieldReader;
#[doc = "Field `DSI_1` writer - DSI 1"]
pub type DSI_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_0` reader - Pixel Valve 0"]
pub type PIXEL_VALVE_0_R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_0` writer - Pixel Valve 0"]
pub type PIXEL_VALVE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIXEL_VALVE_1_2` reader - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_R = crate::FieldReader;
#[doc = "Field `PIXEL_VALVE_1_2` writer - OR of Pixel Valve 1 and 2"]
pub type PIXEL_VALVE_1_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPR` reader - CPR"]
pub type CPR_R = crate::FieldReader;
#[doc = "Field `CPR` writer - CPR"]
pub type CPR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR35")
            .field("dsi_1", &format_args!("{}", self.dsi_1().bits()))
            .field(
                "pixel_valve_0",
                &format_args!("{}", self.pixel_valve_0().bits()),
            )
            .field(
                "pixel_valve_1_2",
                &format_args!("{}", self.pixel_valve_1_2().bits()),
            )
            .field("cpr", &format_args!("{}", self.cpr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR35_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DSI 1"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_1(&mut self) -> DSI_1_W<GICD_IPRIORITYR35_SPEC> {
        DSI_1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pixel Valve 0"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_0(&mut self) -> PIXEL_VALVE_0_W<GICD_IPRIORITYR35_SPEC> {
        PIXEL_VALVE_0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_valve_1_2(&mut self) -> PIXEL_VALVE_1_2_W<GICD_IPRIORITYR35_SPEC> {
        PIXEL_VALVE_1_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CPR"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<GICD_IPRIORITYR35_SPEC> {
        CPR_W::new(self, 24)
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
#[doc = "Interrupt Priority 140 - 143 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR35_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr35::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR35_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr35::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR35_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR35 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR35_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

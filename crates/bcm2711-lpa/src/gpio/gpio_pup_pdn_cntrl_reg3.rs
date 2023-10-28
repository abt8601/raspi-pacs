#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` reader"]
pub type R = crate::R<GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG3` writer"]
pub type W = crate::W<GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
#[doc = "Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::BP_PULL_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` reader - Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL48_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` reader - Resistor select for 49"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL49_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` reader - Resistor select for 50"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL50_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` reader - Resistor select for 51"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL51_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` reader - Resistor select for 52"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL52_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` reader - Resistor select for 53"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL53_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL54` reader - Resistor select for 54"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL54_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL55` reader - Resistor select for 55"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL55_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL56` reader - Resistor select for 56"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL56_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL57` reader - Resistor select for 57"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL57_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL48` writer - Resistor select for 48"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL48_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL49` writer - Resistor select for 49"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL49_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL50` writer - Resistor select for 50"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL50_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL51` writer - Resistor select for 51"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL51_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL52` writer - Resistor select for 52"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL52_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL53` writer - Resistor select for 53"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL53_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL54` writer - Resistor select for 54"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL54_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL55` writer - Resistor select for 55"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL55_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL56` writer - Resistor select for 56"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL56_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL57` writer - Resistor select for 57"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL57_W;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl48(&self) -> GPIO_PUP_PDN_CNTRL48_R {
        GPIO_PUP_PDN_CNTRL48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl49(&self) -> GPIO_PUP_PDN_CNTRL49_R {
        GPIO_PUP_PDN_CNTRL49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl50(&self) -> GPIO_PUP_PDN_CNTRL50_R {
        GPIO_PUP_PDN_CNTRL50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl51(&self) -> GPIO_PUP_PDN_CNTRL51_R {
        GPIO_PUP_PDN_CNTRL51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl52(&self) -> GPIO_PUP_PDN_CNTRL52_R {
        GPIO_PUP_PDN_CNTRL52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl53(&self) -> GPIO_PUP_PDN_CNTRL53_R {
        GPIO_PUP_PDN_CNTRL53_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 54"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl54(&self) -> GPIO_PUP_PDN_CNTRL54_R {
        GPIO_PUP_PDN_CNTRL54_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 55"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl55(&self) -> GPIO_PUP_PDN_CNTRL55_R {
        GPIO_PUP_PDN_CNTRL55_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 56"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl56(&self) -> GPIO_PUP_PDN_CNTRL56_R {
        GPIO_PUP_PDN_CNTRL56_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 57"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl57(&self) -> GPIO_PUP_PDN_CNTRL57_R {
        GPIO_PUP_PDN_CNTRL57_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PUP_PDN_CNTRL_REG3")
            .field(
                "gpio_pup_pdn_cntrl48",
                &format_args!("{}", self.gpio_pup_pdn_cntrl48().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl49",
                &format_args!("{}", self.gpio_pup_pdn_cntrl49().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl50",
                &format_args!("{}", self.gpio_pup_pdn_cntrl50().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl51",
                &format_args!("{}", self.gpio_pup_pdn_cntrl51().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl52",
                &format_args!("{}", self.gpio_pup_pdn_cntrl52().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl53",
                &format_args!("{}", self.gpio_pup_pdn_cntrl53().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl54",
                &format_args!("{}", self.gpio_pup_pdn_cntrl54().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl55",
                &format_args!("{}", self.gpio_pup_pdn_cntrl55().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl56",
                &format_args!("{}", self.gpio_pup_pdn_cntrl56().bits()),
            )
            .field(
                "gpio_pup_pdn_cntrl57",
                &format_args!("{}", self.gpio_pup_pdn_cntrl57().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPIO_PUP_PDN_CNTRL_REG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 48"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl48(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL48_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 0> {
        GPIO_PUP_PDN_CNTRL48_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 49"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl49(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL49_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 2> {
        GPIO_PUP_PDN_CNTRL49_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 50"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl50(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL50_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 4> {
        GPIO_PUP_PDN_CNTRL50_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 51"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl51(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL51_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 6> {
        GPIO_PUP_PDN_CNTRL51_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 52"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl52(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL52_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 8> {
        GPIO_PUP_PDN_CNTRL52_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 53"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl53(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL53_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 10> {
        GPIO_PUP_PDN_CNTRL53_W::new(self)
    }
    #[doc = "Bits 12:13 - Resistor select for 54"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl54(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL54_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 12> {
        GPIO_PUP_PDN_CNTRL54_W::new(self)
    }
    #[doc = "Bits 14:15 - Resistor select for 55"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl55(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL55_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 14> {
        GPIO_PUP_PDN_CNTRL55_W::new(self)
    }
    #[doc = "Bits 16:17 - Resistor select for 56"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl56(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL56_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 16> {
        GPIO_PUP_PDN_CNTRL56_W::new(self)
    }
    #[doc = "Bits 18:19 - Resistor select for 57"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl57(
        &mut self,
    ) -> GPIO_PUP_PDN_CNTRL57_W<GPIO_PUP_PDN_CNTRL_REG3_SPEC, 18> {
        GPIO_PUP_PDN_CNTRL57_W::new(self)
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
#[doc = "GPIO Pull-up / Pull-down Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg3::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_PUP_PDN_CNTRL_REG3_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pup_pdn_cntrl_reg3::R`](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_pup_pdn_cntrl_reg3::W`](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

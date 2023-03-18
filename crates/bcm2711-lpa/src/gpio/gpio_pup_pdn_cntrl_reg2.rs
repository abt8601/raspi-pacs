#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG2` reader"]
pub struct R(crate::R<GPIO_PUP_PDN_CNTRL_REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PUP_PDN_CNTRL_REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PUP_PDN_CNTRL_REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PUP_PDN_CNTRL_REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG2` writer"]
pub struct W(crate::W<GPIO_PUP_PDN_CNTRL_REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PUP_PDN_CNTRL_REG2_SPEC>;
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
impl From<crate::W<GPIO_PUP_PDN_CNTRL_REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PUP_PDN_CNTRL_REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Resistor select for 32"]
pub use super::gpio_pup_pdn_cntrl_reg0::BP_PULL_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL32` reader - Resistor select for 32"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL32_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL32` writer - Resistor select for 32"]
pub type GPIO_PUP_PDN_CNTRL32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL33` reader - Resistor select for 33"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL33_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL33` writer - Resistor select for 33"]
pub type GPIO_PUP_PDN_CNTRL33_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL34` reader - Resistor select for 34"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL34_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL34` writer - Resistor select for 34"]
pub type GPIO_PUP_PDN_CNTRL34_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL35` reader - Resistor select for 35"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL35_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL35` writer - Resistor select for 35"]
pub type GPIO_PUP_PDN_CNTRL35_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL36` reader - Resistor select for 36"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL36_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL36` writer - Resistor select for 36"]
pub type GPIO_PUP_PDN_CNTRL36_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL37` reader - Resistor select for 37"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL37_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL37` writer - Resistor select for 37"]
pub type GPIO_PUP_PDN_CNTRL37_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL38` reader - Resistor select for 38"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL38_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL38` writer - Resistor select for 38"]
pub type GPIO_PUP_PDN_CNTRL38_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL39` reader - Resistor select for 39"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL39_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL39` writer - Resistor select for 39"]
pub type GPIO_PUP_PDN_CNTRL39_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL40` reader - Resistor select for 40"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL40_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL40` writer - Resistor select for 40"]
pub type GPIO_PUP_PDN_CNTRL40_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL41` reader - Resistor select for 41"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL41_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL41` writer - Resistor select for 41"]
pub type GPIO_PUP_PDN_CNTRL41_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL42` reader - Resistor select for 42"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL42_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL42` writer - Resistor select for 42"]
pub type GPIO_PUP_PDN_CNTRL42_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL43` reader - Resistor select for 43"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL43_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL43` writer - Resistor select for 43"]
pub type GPIO_PUP_PDN_CNTRL43_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL44` reader - Resistor select for 44"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL44_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL44` writer - Resistor select for 44"]
pub type GPIO_PUP_PDN_CNTRL44_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL45` reader - Resistor select for 45"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL45_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL45` writer - Resistor select for 45"]
pub type GPIO_PUP_PDN_CNTRL45_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL46` reader - Resistor select for 46"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL46_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL46` writer - Resistor select for 46"]
pub type GPIO_PUP_PDN_CNTRL46_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL47` reader - Resistor select for 47"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL47_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL47` writer - Resistor select for 47"]
pub type GPIO_PUP_PDN_CNTRL47_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG2_SPEC, u8, BP_PULL_A, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 32"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl32(&self) -> GPIO_PUP_PDN_CNTRL32_R {
        GPIO_PUP_PDN_CNTRL32_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 33"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl33(&self) -> GPIO_PUP_PDN_CNTRL33_R {
        GPIO_PUP_PDN_CNTRL33_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 34"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl34(&self) -> GPIO_PUP_PDN_CNTRL34_R {
        GPIO_PUP_PDN_CNTRL34_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 35"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl35(&self) -> GPIO_PUP_PDN_CNTRL35_R {
        GPIO_PUP_PDN_CNTRL35_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 36"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl36(&self) -> GPIO_PUP_PDN_CNTRL36_R {
        GPIO_PUP_PDN_CNTRL36_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 37"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl37(&self) -> GPIO_PUP_PDN_CNTRL37_R {
        GPIO_PUP_PDN_CNTRL37_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 38"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl38(&self) -> GPIO_PUP_PDN_CNTRL38_R {
        GPIO_PUP_PDN_CNTRL38_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 39"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl39(&self) -> GPIO_PUP_PDN_CNTRL39_R {
        GPIO_PUP_PDN_CNTRL39_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 40"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl40(&self) -> GPIO_PUP_PDN_CNTRL40_R {
        GPIO_PUP_PDN_CNTRL40_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 41"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl41(&self) -> GPIO_PUP_PDN_CNTRL41_R {
        GPIO_PUP_PDN_CNTRL41_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Resistor select for 42"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl42(&self) -> GPIO_PUP_PDN_CNTRL42_R {
        GPIO_PUP_PDN_CNTRL42_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Resistor select for 43"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl43(&self) -> GPIO_PUP_PDN_CNTRL43_R {
        GPIO_PUP_PDN_CNTRL43_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor select for 44"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl44(&self) -> GPIO_PUP_PDN_CNTRL44_R {
        GPIO_PUP_PDN_CNTRL44_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Resistor select for 45"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl45(&self) -> GPIO_PUP_PDN_CNTRL45_R {
        GPIO_PUP_PDN_CNTRL45_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Resistor select for 46"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl46(&self) -> GPIO_PUP_PDN_CNTRL46_R {
        GPIO_PUP_PDN_CNTRL46_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Resistor select for 47"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl47(&self) -> GPIO_PUP_PDN_CNTRL47_R {
        GPIO_PUP_PDN_CNTRL47_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 32"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl32(&mut self) -> GPIO_PUP_PDN_CNTRL32_W<0> {
        GPIO_PUP_PDN_CNTRL32_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 33"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl33(&mut self) -> GPIO_PUP_PDN_CNTRL33_W<2> {
        GPIO_PUP_PDN_CNTRL33_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 34"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl34(&mut self) -> GPIO_PUP_PDN_CNTRL34_W<4> {
        GPIO_PUP_PDN_CNTRL34_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 35"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl35(&mut self) -> GPIO_PUP_PDN_CNTRL35_W<6> {
        GPIO_PUP_PDN_CNTRL35_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 36"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl36(&mut self) -> GPIO_PUP_PDN_CNTRL36_W<8> {
        GPIO_PUP_PDN_CNTRL36_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 37"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl37(&mut self) -> GPIO_PUP_PDN_CNTRL37_W<10> {
        GPIO_PUP_PDN_CNTRL37_W::new(self)
    }
    #[doc = "Bits 12:13 - Resistor select for 38"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl38(&mut self) -> GPIO_PUP_PDN_CNTRL38_W<12> {
        GPIO_PUP_PDN_CNTRL38_W::new(self)
    }
    #[doc = "Bits 14:15 - Resistor select for 39"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl39(&mut self) -> GPIO_PUP_PDN_CNTRL39_W<14> {
        GPIO_PUP_PDN_CNTRL39_W::new(self)
    }
    #[doc = "Bits 16:17 - Resistor select for 40"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl40(&mut self) -> GPIO_PUP_PDN_CNTRL40_W<16> {
        GPIO_PUP_PDN_CNTRL40_W::new(self)
    }
    #[doc = "Bits 18:19 - Resistor select for 41"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl41(&mut self) -> GPIO_PUP_PDN_CNTRL41_W<18> {
        GPIO_PUP_PDN_CNTRL41_W::new(self)
    }
    #[doc = "Bits 20:21 - Resistor select for 42"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl42(&mut self) -> GPIO_PUP_PDN_CNTRL42_W<20> {
        GPIO_PUP_PDN_CNTRL42_W::new(self)
    }
    #[doc = "Bits 22:23 - Resistor select for 43"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl43(&mut self) -> GPIO_PUP_PDN_CNTRL43_W<22> {
        GPIO_PUP_PDN_CNTRL43_W::new(self)
    }
    #[doc = "Bits 24:25 - Resistor select for 44"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl44(&mut self) -> GPIO_PUP_PDN_CNTRL44_W<24> {
        GPIO_PUP_PDN_CNTRL44_W::new(self)
    }
    #[doc = "Bits 26:27 - Resistor select for 45"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl45(&mut self) -> GPIO_PUP_PDN_CNTRL45_W<26> {
        GPIO_PUP_PDN_CNTRL45_W::new(self)
    }
    #[doc = "Bits 28:29 - Resistor select for 46"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl46(&mut self) -> GPIO_PUP_PDN_CNTRL46_W<28> {
        GPIO_PUP_PDN_CNTRL46_W::new(self)
    }
    #[doc = "Bits 30:31 - Resistor select for 47"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl47(&mut self) -> GPIO_PUP_PDN_CNTRL47_W<30> {
        GPIO_PUP_PDN_CNTRL47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pup_pdn_cntrl_reg2](index.html) module"]
pub struct GPIO_PUP_PDN_CNTRL_REG2_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pup_pdn_cntrl_reg2::R](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pup_pdn_cntrl_reg2::W](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG1` reader"]
pub struct R(crate::R<GPIO_PUP_PDN_CNTRL_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PUP_PDN_CNTRL_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PUP_PDN_CNTRL_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PUP_PDN_CNTRL_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG1` writer"]
pub struct W(crate::W<GPIO_PUP_PDN_CNTRL_REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PUP_PDN_CNTRL_REG1_SPEC>;
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
impl From<crate::W<GPIO_PUP_PDN_CNTRL_REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PUP_PDN_CNTRL_REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Resistor select for 16"]
pub use super::gpio_pup_pdn_cntrl_reg0::BP_PULL_A;
#[doc = "Field `GPIO_PUP_PDN_CNTRL16` reader - Resistor select for 16"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL16_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL16` writer - Resistor select for 16"]
pub type GPIO_PUP_PDN_CNTRL16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL17` reader - Resistor select for 17"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL17_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL17` writer - Resistor select for 17"]
pub type GPIO_PUP_PDN_CNTRL17_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL18` reader - Resistor select for 18"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL18_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL18` writer - Resistor select for 18"]
pub type GPIO_PUP_PDN_CNTRL18_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL19` reader - Resistor select for 19"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL19_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL19` writer - Resistor select for 19"]
pub type GPIO_PUP_PDN_CNTRL19_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL20` reader - Resistor select for 20"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL20_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL20` writer - Resistor select for 20"]
pub type GPIO_PUP_PDN_CNTRL20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL21` reader - Resistor select for 21"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL21_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL21` writer - Resistor select for 21"]
pub type GPIO_PUP_PDN_CNTRL21_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL22` reader - Resistor select for 22"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL22_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL22` writer - Resistor select for 22"]
pub type GPIO_PUP_PDN_CNTRL22_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL23` reader - Resistor select for 23"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL23_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL23` writer - Resistor select for 23"]
pub type GPIO_PUP_PDN_CNTRL23_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL24` reader - Resistor select for 24"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL24_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL24` writer - Resistor select for 24"]
pub type GPIO_PUP_PDN_CNTRL24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL25` reader - Resistor select for 25"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL25_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL25` writer - Resistor select for 25"]
pub type GPIO_PUP_PDN_CNTRL25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL26` reader - Resistor select for 26"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL26_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL26` writer - Resistor select for 26"]
pub type GPIO_PUP_PDN_CNTRL26_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL27` reader - Resistor select for 27"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL27_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL27` writer - Resistor select for 27"]
pub type GPIO_PUP_PDN_CNTRL27_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL28` reader - Resistor select for 28"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL28_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL28` writer - Resistor select for 28"]
pub type GPIO_PUP_PDN_CNTRL28_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL29` reader - Resistor select for 29"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL29_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL29` writer - Resistor select for 29"]
pub type GPIO_PUP_PDN_CNTRL29_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL30` reader - Resistor select for 30"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL30_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL30` writer - Resistor select for 30"]
pub type GPIO_PUP_PDN_CNTRL30_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
#[doc = "Field `GPIO_PUP_PDN_CNTRL31` reader - Resistor select for 31"]
pub use super::gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL31_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL31` writer - Resistor select for 31"]
pub type GPIO_PUP_PDN_CNTRL31_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG1_SPEC, u8, BP_PULL_A, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 16"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl16(&self) -> GPIO_PUP_PDN_CNTRL16_R {
        GPIO_PUP_PDN_CNTRL16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 17"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl17(&self) -> GPIO_PUP_PDN_CNTRL17_R {
        GPIO_PUP_PDN_CNTRL17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 18"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl18(&self) -> GPIO_PUP_PDN_CNTRL18_R {
        GPIO_PUP_PDN_CNTRL18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 19"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl19(&self) -> GPIO_PUP_PDN_CNTRL19_R {
        GPIO_PUP_PDN_CNTRL19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 20"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl20(&self) -> GPIO_PUP_PDN_CNTRL20_R {
        GPIO_PUP_PDN_CNTRL20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 21"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl21(&self) -> GPIO_PUP_PDN_CNTRL21_R {
        GPIO_PUP_PDN_CNTRL21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 22"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl22(&self) -> GPIO_PUP_PDN_CNTRL22_R {
        GPIO_PUP_PDN_CNTRL22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 23"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl23(&self) -> GPIO_PUP_PDN_CNTRL23_R {
        GPIO_PUP_PDN_CNTRL23_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 24"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl24(&self) -> GPIO_PUP_PDN_CNTRL24_R {
        GPIO_PUP_PDN_CNTRL24_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 25"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl25(&self) -> GPIO_PUP_PDN_CNTRL25_R {
        GPIO_PUP_PDN_CNTRL25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Resistor select for 26"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl26(&self) -> GPIO_PUP_PDN_CNTRL26_R {
        GPIO_PUP_PDN_CNTRL26_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Resistor select for 27"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl27(&self) -> GPIO_PUP_PDN_CNTRL27_R {
        GPIO_PUP_PDN_CNTRL27_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor select for 28"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl28(&self) -> GPIO_PUP_PDN_CNTRL28_R {
        GPIO_PUP_PDN_CNTRL28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Resistor select for 29"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl29(&self) -> GPIO_PUP_PDN_CNTRL29_R {
        GPIO_PUP_PDN_CNTRL29_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Resistor select for 30"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl30(&self) -> GPIO_PUP_PDN_CNTRL30_R {
        GPIO_PUP_PDN_CNTRL30_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Resistor select for 31"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl31(&self) -> GPIO_PUP_PDN_CNTRL31_R {
        GPIO_PUP_PDN_CNTRL31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl16(&mut self) -> GPIO_PUP_PDN_CNTRL16_W<0> {
        GPIO_PUP_PDN_CNTRL16_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl17(&mut self) -> GPIO_PUP_PDN_CNTRL17_W<2> {
        GPIO_PUP_PDN_CNTRL17_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl18(&mut self) -> GPIO_PUP_PDN_CNTRL18_W<4> {
        GPIO_PUP_PDN_CNTRL18_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl19(&mut self) -> GPIO_PUP_PDN_CNTRL19_W<6> {
        GPIO_PUP_PDN_CNTRL19_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 20"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl20(&mut self) -> GPIO_PUP_PDN_CNTRL20_W<8> {
        GPIO_PUP_PDN_CNTRL20_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 21"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl21(&mut self) -> GPIO_PUP_PDN_CNTRL21_W<10> {
        GPIO_PUP_PDN_CNTRL21_W::new(self)
    }
    #[doc = "Bits 12:13 - Resistor select for 22"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl22(&mut self) -> GPIO_PUP_PDN_CNTRL22_W<12> {
        GPIO_PUP_PDN_CNTRL22_W::new(self)
    }
    #[doc = "Bits 14:15 - Resistor select for 23"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl23(&mut self) -> GPIO_PUP_PDN_CNTRL23_W<14> {
        GPIO_PUP_PDN_CNTRL23_W::new(self)
    }
    #[doc = "Bits 16:17 - Resistor select for 24"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl24(&mut self) -> GPIO_PUP_PDN_CNTRL24_W<16> {
        GPIO_PUP_PDN_CNTRL24_W::new(self)
    }
    #[doc = "Bits 18:19 - Resistor select for 25"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl25(&mut self) -> GPIO_PUP_PDN_CNTRL25_W<18> {
        GPIO_PUP_PDN_CNTRL25_W::new(self)
    }
    #[doc = "Bits 20:21 - Resistor select for 26"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl26(&mut self) -> GPIO_PUP_PDN_CNTRL26_W<20> {
        GPIO_PUP_PDN_CNTRL26_W::new(self)
    }
    #[doc = "Bits 22:23 - Resistor select for 27"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl27(&mut self) -> GPIO_PUP_PDN_CNTRL27_W<22> {
        GPIO_PUP_PDN_CNTRL27_W::new(self)
    }
    #[doc = "Bits 24:25 - Resistor select for 28"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl28(&mut self) -> GPIO_PUP_PDN_CNTRL28_W<24> {
        GPIO_PUP_PDN_CNTRL28_W::new(self)
    }
    #[doc = "Bits 26:27 - Resistor select for 29"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl29(&mut self) -> GPIO_PUP_PDN_CNTRL29_W<26> {
        GPIO_PUP_PDN_CNTRL29_W::new(self)
    }
    #[doc = "Bits 28:29 - Resistor select for 30"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl30(&mut self) -> GPIO_PUP_PDN_CNTRL30_W<28> {
        GPIO_PUP_PDN_CNTRL30_W::new(self)
    }
    #[doc = "Bits 30:31 - Resistor select for 31"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl31(&mut self) -> GPIO_PUP_PDN_CNTRL31_W<30> {
        GPIO_PUP_PDN_CNTRL31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pup_pdn_cntrl_reg1](index.html) module"]
pub struct GPIO_PUP_PDN_CNTRL_REG1_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pup_pdn_cntrl_reg1::R](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pup_pdn_cntrl_reg1::W](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

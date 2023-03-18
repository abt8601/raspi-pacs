#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` reader"]
pub struct R(crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PUP_PDN_CNTRL_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PUP_PDN_CNTRL_REG0` writer"]
pub struct W(crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
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
impl From<crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PUP_PDN_CNTRL_REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` reader - Resistor select for 0"]
pub type GPIO_PUP_PDN_CNTRL0_R = crate::FieldReader<u8, BP_PULL_A>;
#[doc = "Resistor select for 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BP_PULL_A {
    #[doc = "0: No pull"]
    NONE = 0,
    #[doc = "1: Pull up"]
    UP = 1,
    #[doc = "2: Pull down"]
    DOWN = 2,
}
impl From<BP_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: BP_PULL_A) -> Self {
        variant as _
    }
}
impl GPIO_PUP_PDN_CNTRL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BP_PULL_A> {
        match self.bits {
            0 => Some(BP_PULL_A::NONE),
            1 => Some(BP_PULL_A::UP),
            2 => Some(BP_PULL_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BP_PULL_A::NONE
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BP_PULL_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == BP_PULL_A::DOWN
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL0` writer - Resistor select for 0"]
pub type GPIO_PUP_PDN_CNTRL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_PUP_PDN_CNTRL_REG0_SPEC, u8, BP_PULL_A, 2, O>;
impl<'a, const O: u8> GPIO_PUP_PDN_CNTRL0_W<'a, O> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BP_PULL_A::NONE)
    }
    #[doc = "Pull up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BP_PULL_A::UP)
    }
    #[doc = "Pull down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(BP_PULL_A::DOWN)
    }
}
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` reader - Resistor select for 1"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL1_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` reader - Resistor select for 2"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL2_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` reader - Resistor select for 3"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL3_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` reader - Resistor select for 4"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL4_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` reader - Resistor select for 5"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL5_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` reader - Resistor select for 6"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL6_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` reader - Resistor select for 7"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL7_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` reader - Resistor select for 8"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL8_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` reader - Resistor select for 9"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL9_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` reader - Resistor select for 10"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL10_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` reader - Resistor select for 11"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL11_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` reader - Resistor select for 12"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL12_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` reader - Resistor select for 13"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL13_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` reader - Resistor select for 14"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL14_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` reader - Resistor select for 15"]
pub use GPIO_PUP_PDN_CNTRL0_R as GPIO_PUP_PDN_CNTRL15_R;
#[doc = "Field `GPIO_PUP_PDN_CNTRL1` writer - Resistor select for 1"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL1_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL2` writer - Resistor select for 2"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL2_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL3` writer - Resistor select for 3"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL3_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL4` writer - Resistor select for 4"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL4_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL5` writer - Resistor select for 5"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL5_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL6` writer - Resistor select for 6"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL6_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL7` writer - Resistor select for 7"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL7_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL8` writer - Resistor select for 8"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL8_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL9` writer - Resistor select for 9"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL9_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL10` writer - Resistor select for 10"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL10_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL11` writer - Resistor select for 11"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL11_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL12` writer - Resistor select for 12"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL12_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL13` writer - Resistor select for 13"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL13_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL14` writer - Resistor select for 14"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL14_W;
#[doc = "Field `GPIO_PUP_PDN_CNTRL15` writer - Resistor select for 15"]
pub use GPIO_PUP_PDN_CNTRL0_W as GPIO_PUP_PDN_CNTRL15_W;
impl R {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl0(&self) -> GPIO_PUP_PDN_CNTRL0_R {
        GPIO_PUP_PDN_CNTRL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl1(&self) -> GPIO_PUP_PDN_CNTRL1_R {
        GPIO_PUP_PDN_CNTRL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl2(&self) -> GPIO_PUP_PDN_CNTRL2_R {
        GPIO_PUP_PDN_CNTRL2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl3(&self) -> GPIO_PUP_PDN_CNTRL3_R {
        GPIO_PUP_PDN_CNTRL3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl4(&self) -> GPIO_PUP_PDN_CNTRL4_R {
        GPIO_PUP_PDN_CNTRL4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl5(&self) -> GPIO_PUP_PDN_CNTRL5_R {
        GPIO_PUP_PDN_CNTRL5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl6(&self) -> GPIO_PUP_PDN_CNTRL6_R {
        GPIO_PUP_PDN_CNTRL6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl7(&self) -> GPIO_PUP_PDN_CNTRL7_R {
        GPIO_PUP_PDN_CNTRL7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl8(&self) -> GPIO_PUP_PDN_CNTRL8_R {
        GPIO_PUP_PDN_CNTRL8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl9(&self) -> GPIO_PUP_PDN_CNTRL9_R {
        GPIO_PUP_PDN_CNTRL9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl10(&self) -> GPIO_PUP_PDN_CNTRL10_R {
        GPIO_PUP_PDN_CNTRL10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl11(&self) -> GPIO_PUP_PDN_CNTRL11_R {
        GPIO_PUP_PDN_CNTRL11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl12(&self) -> GPIO_PUP_PDN_CNTRL12_R {
        GPIO_PUP_PDN_CNTRL12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl13(&self) -> GPIO_PUP_PDN_CNTRL13_R {
        GPIO_PUP_PDN_CNTRL13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl14(&self) -> GPIO_PUP_PDN_CNTRL14_R {
        GPIO_PUP_PDN_CNTRL14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    pub fn gpio_pup_pdn_cntrl15(&self) -> GPIO_PUP_PDN_CNTRL15_R {
        GPIO_PUP_PDN_CNTRL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Resistor select for 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl0(&mut self) -> GPIO_PUP_PDN_CNTRL0_W<0> {
        GPIO_PUP_PDN_CNTRL0_W::new(self)
    }
    #[doc = "Bits 2:3 - Resistor select for 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl1(&mut self) -> GPIO_PUP_PDN_CNTRL1_W<2> {
        GPIO_PUP_PDN_CNTRL1_W::new(self)
    }
    #[doc = "Bits 4:5 - Resistor select for 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl2(&mut self) -> GPIO_PUP_PDN_CNTRL2_W<4> {
        GPIO_PUP_PDN_CNTRL2_W::new(self)
    }
    #[doc = "Bits 6:7 - Resistor select for 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl3(&mut self) -> GPIO_PUP_PDN_CNTRL3_W<6> {
        GPIO_PUP_PDN_CNTRL3_W::new(self)
    }
    #[doc = "Bits 8:9 - Resistor select for 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl4(&mut self) -> GPIO_PUP_PDN_CNTRL4_W<8> {
        GPIO_PUP_PDN_CNTRL4_W::new(self)
    }
    #[doc = "Bits 10:11 - Resistor select for 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl5(&mut self) -> GPIO_PUP_PDN_CNTRL5_W<10> {
        GPIO_PUP_PDN_CNTRL5_W::new(self)
    }
    #[doc = "Bits 12:13 - Resistor select for 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl6(&mut self) -> GPIO_PUP_PDN_CNTRL6_W<12> {
        GPIO_PUP_PDN_CNTRL6_W::new(self)
    }
    #[doc = "Bits 14:15 - Resistor select for 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl7(&mut self) -> GPIO_PUP_PDN_CNTRL7_W<14> {
        GPIO_PUP_PDN_CNTRL7_W::new(self)
    }
    #[doc = "Bits 16:17 - Resistor select for 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl8(&mut self) -> GPIO_PUP_PDN_CNTRL8_W<16> {
        GPIO_PUP_PDN_CNTRL8_W::new(self)
    }
    #[doc = "Bits 18:19 - Resistor select for 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl9(&mut self) -> GPIO_PUP_PDN_CNTRL9_W<18> {
        GPIO_PUP_PDN_CNTRL9_W::new(self)
    }
    #[doc = "Bits 20:21 - Resistor select for 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl10(&mut self) -> GPIO_PUP_PDN_CNTRL10_W<20> {
        GPIO_PUP_PDN_CNTRL10_W::new(self)
    }
    #[doc = "Bits 22:23 - Resistor select for 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl11(&mut self) -> GPIO_PUP_PDN_CNTRL11_W<22> {
        GPIO_PUP_PDN_CNTRL11_W::new(self)
    }
    #[doc = "Bits 24:25 - Resistor select for 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl12(&mut self) -> GPIO_PUP_PDN_CNTRL12_W<24> {
        GPIO_PUP_PDN_CNTRL12_W::new(self)
    }
    #[doc = "Bits 26:27 - Resistor select for 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl13(&mut self) -> GPIO_PUP_PDN_CNTRL13_W<26> {
        GPIO_PUP_PDN_CNTRL13_W::new(self)
    }
    #[doc = "Bits 28:29 - Resistor select for 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl14(&mut self) -> GPIO_PUP_PDN_CNTRL14_W<28> {
        GPIO_PUP_PDN_CNTRL14_W::new(self)
    }
    #[doc = "Bits 30:31 - Resistor select for 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pup_pdn_cntrl15(&mut self) -> GPIO_PUP_PDN_CNTRL15_W<30> {
        GPIO_PUP_PDN_CNTRL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pull-up / Pull-down Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pup_pdn_cntrl_reg0](index.html) module"]
pub struct GPIO_PUP_PDN_CNTRL_REG0_SPEC;
impl crate::RegisterSpec for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pup_pdn_cntrl_reg0::R](R) reader structure"]
impl crate::Readable for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pup_pdn_cntrl_reg0::W](W) writer structure"]
impl crate::Writable for GPIO_PUP_PDN_CNTRL_REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

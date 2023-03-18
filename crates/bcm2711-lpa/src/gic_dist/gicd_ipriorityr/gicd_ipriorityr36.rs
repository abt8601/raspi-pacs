#[doc = "Register `GICD_IPRIORITYR36` reader"]
pub struct R(crate::R<GICD_IPRIORITYR36_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR36_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR36_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR36_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR36` writer"]
pub struct W(crate::W<GICD_IPRIORITYR36_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR36_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR36_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR36_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMI` reader - SMI"]
pub type SMI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR36_SPEC, u8, u8, 8, O>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type GPIO_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR36_SPEC, u8, u8, 8, O>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type GPIO_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR36_SPEC, u8, u8, 8, O>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type GPIO_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR36_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SMI"]
    #[inline(always)]
    pub fn smi(&self) -> SMI_R {
        SMI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> GPIO_0_R {
        GPIO_0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> GPIO_1_R {
        GPIO_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> GPIO_2_R {
        GPIO_2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<0> {
        SMI_W::new(self)
    }
    #[doc = "Bits 8:15 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<8> {
        GPIO_0_W::new(self)
    }
    #[doc = "Bits 16:23 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<16> {
        GPIO_1_W::new(self)
    }
    #[doc = "Bits 24:31 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<24> {
        GPIO_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 144 - 147 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr36](index.html) module"]
pub struct GICD_IPRIORITYR36_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr36::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR36_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr36::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR36_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR36 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

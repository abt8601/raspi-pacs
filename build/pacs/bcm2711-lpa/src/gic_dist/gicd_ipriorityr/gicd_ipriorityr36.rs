#[doc = "Register `GICD_IPRIORITYR36` reader"]
pub type R = crate::R<GICD_IPRIORITYR36_SPEC>;
#[doc = "Register `GICD_IPRIORITYR36` writer"]
pub type W = crate::W<GICD_IPRIORITYR36_SPEC>;
#[doc = "Field `SMI` reader - SMI"]
pub type SMI_R = crate::FieldReader;
#[doc = "Field `SMI` writer - SMI"]
pub type SMI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_0` reader - GPIO 0"]
pub type GPIO_0_R = crate::FieldReader;
#[doc = "Field `GPIO_0` writer - GPIO 0"]
pub type GPIO_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_1` reader - GPIO 1"]
pub type GPIO_1_R = crate::FieldReader;
#[doc = "Field `GPIO_1` writer - GPIO 1"]
pub type GPIO_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO_2` reader - GPIO 2"]
pub type GPIO_2_R = crate::FieldReader;
#[doc = "Field `GPIO_2` writer - GPIO 2"]
pub type GPIO_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR36")
            .field("smi", &format_args!("{}", self.smi().bits()))
            .field("gpio_0", &format_args!("{}", self.gpio_0().bits()))
            .field("gpio_1", &format_args!("{}", self.gpio_1().bits()))
            .field("gpio_2", &format_args!("{}", self.gpio_2().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR36_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SMI"]
    #[inline(always)]
    #[must_use]
    pub fn smi(&mut self) -> SMI_W<GICD_IPRIORITYR36_SPEC> {
        SMI_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_0(&mut self) -> GPIO_0_W<GICD_IPRIORITYR36_SPEC> {
        GPIO_0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_1(&mut self) -> GPIO_1_W<GICD_IPRIORITYR36_SPEC> {
        GPIO_1_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_2(&mut self) -> GPIO_2_W<GICD_IPRIORITYR36_SPEC> {
        GPIO_2_W::new(self, 24)
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
#[doc = "Interrupt Priority 144 - 147 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR36_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr36::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr36::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR36 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR36_SPEC {
    const RESET_VALUE: u32 = 0;
}

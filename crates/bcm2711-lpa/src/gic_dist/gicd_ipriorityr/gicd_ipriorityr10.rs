#[doc = "Register `GICD_IPRIORITYR10` reader"]
pub type R = crate::R<GICD_IPRIORITYR10_SPEC>;
#[doc = "Register `GICD_IPRIORITYR10` writer"]
pub type W = crate::W<GICD_IPRIORITYR10_SPEC>;
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type INT40_R = crate::FieldReader;
#[doc = "Field `INT40` writer - Interrupt 40"]
pub type INT40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type INT41_R = crate::FieldReader;
#[doc = "Field `INT41` writer - Interrupt 41"]
pub type INT41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type INT42_R = crate::FieldReader;
#[doc = "Field `INT42` writer - Interrupt 42"]
pub type INT42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type INT43_R = crate::FieldReader;
#[doc = "Field `INT43` writer - Interrupt 43"]
pub type INT43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> INT40_R {
        INT40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> INT41_R {
        INT41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> INT42_R {
        INT42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> INT43_R {
        INT43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR10")
            .field("int40", &format_args!("{}", self.int40().bits()))
            .field("int41", &format_args!("{}", self.int41().bits()))
            .field("int42", &format_args!("{}", self.int42().bits()))
            .field("int43", &format_args!("{}", self.int43().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn int40(&mut self) -> INT40_W<GICD_IPRIORITYR10_SPEC> {
        INT40_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn int41(&mut self) -> INT41_W<GICD_IPRIORITYR10_SPEC> {
        INT41_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn int42(&mut self) -> INT42_W<GICD_IPRIORITYR10_SPEC> {
        INT42_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn int43(&mut self) -> INT43_W<GICD_IPRIORITYR10_SPEC> {
        INT43_W::new(self, 24)
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
#[doc = "Interrupt Priority 40 - 43 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR10_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr10::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr10::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR10 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

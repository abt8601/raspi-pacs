#[doc = "Register `GICD_IPRIORITYR8` reader"]
pub type R = crate::R<GICD_IPRIORITYR8_SPEC>;
#[doc = "Register `GICD_IPRIORITYR8` writer"]
pub type W = crate::W<GICD_IPRIORITYR8_SPEC>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type INT32_R = crate::FieldReader;
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type INT32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type INT33_R = crate::FieldReader;
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type INT33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type INT34_R = crate::FieldReader;
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type INT34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type INT35_R = crate::FieldReader;
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type INT35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> INT32_R {
        INT32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> INT33_R {
        INT33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> INT34_R {
        INT34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> INT35_R {
        INT35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR8")
            .field("int32", &format_args!("{}", self.int32().bits()))
            .field("int33", &format_args!("{}", self.int33().bits()))
            .field("int34", &format_args!("{}", self.int34().bits()))
            .field("int35", &format_args!("{}", self.int35().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> INT32_W<GICD_IPRIORITYR8_SPEC> {
        INT32_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> INT33_W<GICD_IPRIORITYR8_SPEC> {
        INT33_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> INT34_W<GICD_IPRIORITYR8_SPEC> {
        INT34_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> INT35_W<GICD_IPRIORITYR8_SPEC> {
        INT35_W::new(self, 24)
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
#[doc = "Interrupt Priority 32 - 35 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR8_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr8::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr8::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR8 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_IPRIORITYR4` reader"]
pub type R = crate::R<GICD_IPRIORITYR4_SPEC>;
#[doc = "Register `GICD_IPRIORITYR4` writer"]
pub type W = crate::W<GICD_IPRIORITYR4_SPEC>;
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type INT16_R = crate::FieldReader;
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type INT16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type INT17_R = crate::FieldReader;
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type INT17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type INT18_R = crate::FieldReader;
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type INT18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type INT19_R = crate::FieldReader;
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type INT19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR4")
            .field("int16", &format_args!("{}", self.int16().bits()))
            .field("int17", &format_args!("{}", self.int17().bits()))
            .field("int18", &format_args!("{}", self.int18().bits()))
            .field("int19", &format_args!("{}", self.int19().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<GICD_IPRIORITYR4_SPEC> {
        INT16_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> INT17_W<GICD_IPRIORITYR4_SPEC> {
        INT17_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> INT18_W<GICD_IPRIORITYR4_SPEC> {
        INT18_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> INT19_W<GICD_IPRIORITYR4_SPEC> {
        INT19_W::new(self, 24)
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
#[doc = "Interrupt Priority 16 - 19 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR4_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr4::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr4::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR4 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

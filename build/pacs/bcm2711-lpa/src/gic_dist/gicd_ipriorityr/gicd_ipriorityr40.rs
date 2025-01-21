#[doc = "Register `GICD_IPRIORITYR40` reader"]
pub type R = crate::R<GICD_IPRIORITYR40_SPEC>;
#[doc = "Register `GICD_IPRIORITYR40` writer"]
pub type W = crate::W<GICD_IPRIORITYR40_SPEC>;
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type INT160_R = crate::FieldReader;
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type INT160_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type INT161_R = crate::FieldReader;
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type INT161_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type INT162_R = crate::FieldReader;
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type INT162_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type INT163_R = crate::FieldReader;
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type INT163_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> INT160_R {
        INT160_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> INT161_R {
        INT161_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> INT162_R {
        INT162_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> INT163_R {
        INT163_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR40")
            .field("int160", &format_args!("{}", self.int160().bits()))
            .field("int161", &format_args!("{}", self.int161().bits()))
            .field("int162", &format_args!("{}", self.int162().bits()))
            .field("int163", &format_args!("{}", self.int163().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR40_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> INT160_W<GICD_IPRIORITYR40_SPEC> {
        INT160_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> INT161_W<GICD_IPRIORITYR40_SPEC> {
        INT161_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> INT162_W<GICD_IPRIORITYR40_SPEC> {
        INT162_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> INT163_W<GICD_IPRIORITYR40_SPEC> {
        INT163_W::new(self, 24)
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
#[doc = "Interrupt Priority 160 - 163 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR40_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr40::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR40_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr40::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR40_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR40 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR40_SPEC {
    const RESET_VALUE: u32 = 0;
}

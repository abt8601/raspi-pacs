#[doc = "Register `GICD_IPRIORITYR9` reader"]
pub type R = crate::R<GICD_IPRIORITYR9_SPEC>;
#[doc = "Register `GICD_IPRIORITYR9` writer"]
pub type W = crate::W<GICD_IPRIORITYR9_SPEC>;
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type INT36_R = crate::FieldReader;
#[doc = "Field `INT36` writer - Interrupt 36"]
pub type INT36_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type INT37_R = crate::FieldReader;
#[doc = "Field `INT37` writer - Interrupt 37"]
pub type INT37_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type INT38_R = crate::FieldReader;
#[doc = "Field `INT38` writer - Interrupt 38"]
pub type INT38_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type INT39_R = crate::FieldReader;
#[doc = "Field `INT39` writer - Interrupt 39"]
pub type INT39_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> INT36_R {
        INT36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> INT37_R {
        INT37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> INT38_R {
        INT38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> INT39_R {
        INT39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR9")
            .field("int36", &format_args!("{}", self.int36().bits()))
            .field("int37", &format_args!("{}", self.int37().bits()))
            .field("int38", &format_args!("{}", self.int38().bits()))
            .field("int39", &format_args!("{}", self.int39().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn int36(&mut self) -> INT36_W<GICD_IPRIORITYR9_SPEC> {
        INT36_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn int37(&mut self) -> INT37_W<GICD_IPRIORITYR9_SPEC> {
        INT37_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn int38(&mut self) -> INT38_W<GICD_IPRIORITYR9_SPEC> {
        INT38_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn int39(&mut self) -> INT39_W<GICD_IPRIORITYR9_SPEC> {
        INT39_W::new(self, 24)
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
#[doc = "Interrupt Priority 36 - 39 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR9_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr9::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr9::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR9 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR9_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `GICD_IPRIORITYR42` reader"]
pub type R = crate::R<GICD_IPRIORITYR42_SPEC>;
#[doc = "Register `GICD_IPRIORITYR42` writer"]
pub type W = crate::W<GICD_IPRIORITYR42_SPEC>;
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type INT168_R = crate::FieldReader;
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type INT168_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type INT169_R = crate::FieldReader;
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type INT169_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type INT170_R = crate::FieldReader;
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type INT170_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type INT171_R = crate::FieldReader;
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type INT171_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> INT168_R {
        INT168_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> INT169_R {
        INT169_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> INT170_R {
        INT170_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> INT171_R {
        INT171_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR42")
            .field("int168", &format_args!("{}", self.int168().bits()))
            .field("int169", &format_args!("{}", self.int169().bits()))
            .field("int170", &format_args!("{}", self.int170().bits()))
            .field("int171", &format_args!("{}", self.int171().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR42_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> INT168_W<GICD_IPRIORITYR42_SPEC> {
        INT168_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> INT169_W<GICD_IPRIORITYR42_SPEC> {
        INT169_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> INT170_W<GICD_IPRIORITYR42_SPEC> {
        INT170_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> INT171_W<GICD_IPRIORITYR42_SPEC> {
        INT171_W::new(self, 24)
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
#[doc = "Interrupt Priority 168 - 171 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR42_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR42_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr42::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR42_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr42::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR42_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR42 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR42_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

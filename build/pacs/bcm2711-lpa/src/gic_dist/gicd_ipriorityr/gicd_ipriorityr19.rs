#[doc = "Register `GICD_IPRIORITYR19` reader"]
pub type R = crate::R<GICD_IPRIORITYR19_SPEC>;
#[doc = "Register `GICD_IPRIORITYR19` writer"]
pub type W = crate::W<GICD_IPRIORITYR19_SPEC>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::FieldReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::FieldReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::FieldReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::FieldReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR19")
            .field("swi4", &format_args!("{}", self.swi4().bits()))
            .field("swi5", &format_args!("{}", self.swi5().bits()))
            .field("swi6", &format_args!("{}", self.swi6().bits()))
            .field("swi7", &format_args!("{}", self.swi7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR19_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<GICD_IPRIORITYR19_SPEC> {
        SWI4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<GICD_IPRIORITYR19_SPEC> {
        SWI5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<GICD_IPRIORITYR19_SPEC> {
        SWI6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<GICD_IPRIORITYR19_SPEC> {
        SWI7_W::new(self, 24)
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
#[doc = "Interrupt Priority 76 - 79 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR19_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr19::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr19::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR19 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR19_SPEC {
    const RESET_VALUE: u32 = 0;
}

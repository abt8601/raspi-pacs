#[doc = "Register `GICD_ITARGETSR13` reader"]
pub type R = crate::R<GICD_ITARGETSR13_SPEC>;
#[doc = "Register `GICD_ITARGETSR13` writer"]
pub type W = crate::W<GICD_ITARGETSR13_SPEC>;
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type INT52_R = crate::FieldReader;
#[doc = "Field `INT52` writer - Interrupt 52"]
pub type INT52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type INT53_R = crate::FieldReader;
#[doc = "Field `INT53` writer - Interrupt 53"]
pub type INT53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type INT54_R = crate::FieldReader;
#[doc = "Field `INT54` writer - Interrupt 54"]
pub type INT54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type INT55_R = crate::FieldReader;
#[doc = "Field `INT55` writer - Interrupt 55"]
pub type INT55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> INT52_R {
        INT52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> INT53_R {
        INT53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> INT54_R {
        INT54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> INT55_R {
        INT55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR13")
            .field("int52", &format_args!("{}", self.int52().bits()))
            .field("int53", &format_args!("{}", self.int53().bits()))
            .field("int54", &format_args!("{}", self.int54().bits()))
            .field("int55", &format_args!("{}", self.int55().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn int52(&mut self) -> INT52_W<GICD_ITARGETSR13_SPEC> {
        INT52_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn int53(&mut self) -> INT53_W<GICD_ITARGETSR13_SPEC> {
        INT53_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn int54(&mut self) -> INT54_W<GICD_ITARGETSR13_SPEC> {
        INT54_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn int55(&mut self) -> INT55_W<GICD_ITARGETSR13_SPEC> {
        INT55_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 52 - 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR13_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr13::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr13::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR13 to value 0"]
impl crate::Resettable for GICD_ITARGETSR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

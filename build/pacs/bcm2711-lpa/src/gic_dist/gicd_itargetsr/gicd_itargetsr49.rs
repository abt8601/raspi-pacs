#[doc = "Register `GICD_ITARGETSR49` reader"]
pub type R = crate::R<GICD_ITARGETSR49_SPEC>;
#[doc = "Register `GICD_ITARGETSR49` writer"]
pub type W = crate::W<GICD_ITARGETSR49_SPEC>;
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type INT196_R = crate::FieldReader;
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type INT196_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type INT197_R = crate::FieldReader;
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type INT197_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type INT198_R = crate::FieldReader;
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type INT198_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type INT199_R = crate::FieldReader;
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type INT199_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> INT196_R {
        INT196_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> INT197_R {
        INT197_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> INT198_R {
        INT198_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> INT199_R {
        INT199_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR49")
            .field("int196", &format_args!("{}", self.int196().bits()))
            .field("int197", &format_args!("{}", self.int197().bits()))
            .field("int198", &format_args!("{}", self.int198().bits()))
            .field("int199", &format_args!("{}", self.int199().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR49_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> INT196_W<GICD_ITARGETSR49_SPEC> {
        INT196_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> INT197_W<GICD_ITARGETSR49_SPEC> {
        INT197_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> INT198_W<GICD_ITARGETSR49_SPEC> {
        INT198_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> INT199_W<GICD_ITARGETSR49_SPEC> {
        INT199_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 196 - 199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR49_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr49::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR49_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr49::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR49_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR49 to value 0"]
impl crate::Resettable for GICD_ITARGETSR49_SPEC {
    const RESET_VALUE: u32 = 0;
}

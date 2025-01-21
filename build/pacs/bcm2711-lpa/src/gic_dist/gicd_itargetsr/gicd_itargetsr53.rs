#[doc = "Register `GICD_ITARGETSR53` reader"]
pub type R = crate::R<GICD_ITARGETSR53_SPEC>;
#[doc = "Register `GICD_ITARGETSR53` writer"]
pub type W = crate::W<GICD_ITARGETSR53_SPEC>;
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type INT212_R = crate::FieldReader;
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type INT212_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type INT213_R = crate::FieldReader;
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type INT213_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type INT214_R = crate::FieldReader;
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type INT214_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type INT215_R = crate::FieldReader;
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type INT215_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> INT212_R {
        INT212_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> INT213_R {
        INT213_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> INT214_R {
        INT214_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> INT215_R {
        INT215_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR53")
            .field("int212", &format_args!("{}", self.int212().bits()))
            .field("int213", &format_args!("{}", self.int213().bits()))
            .field("int214", &format_args!("{}", self.int214().bits()))
            .field("int215", &format_args!("{}", self.int215().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR53_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> INT212_W<GICD_ITARGETSR53_SPEC> {
        INT212_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> INT213_W<GICD_ITARGETSR53_SPEC> {
        INT213_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> INT214_W<GICD_ITARGETSR53_SPEC> {
        INT214_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> INT215_W<GICD_ITARGETSR53_SPEC> {
        INT215_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 212 - 215\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR53_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR53_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr53::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR53_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr53::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR53_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR53 to value 0"]
impl crate::Resettable for GICD_ITARGETSR53_SPEC {
    const RESET_VALUE: u32 = 0;
}

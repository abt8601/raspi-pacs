#[doc = "Register `GICD_ITARGETSR5` reader"]
pub type R = crate::R<GICD_ITARGETSR5_SPEC>;
#[doc = "Register `GICD_ITARGETSR5` writer"]
pub type W = crate::W<GICD_ITARGETSR5_SPEC>;
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type INT20_R = crate::FieldReader;
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type INT20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type INT21_R = crate::FieldReader;
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type INT21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type INT22_R = crate::FieldReader;
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type INT22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type INT23_R = crate::FieldReader;
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type INT23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR5")
            .field("int20", &format_args!("{}", self.int20().bits()))
            .field("int21", &format_args!("{}", self.int21().bits()))
            .field("int22", &format_args!("{}", self.int22().bits()))
            .field("int23", &format_args!("{}", self.int23().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> INT20_W<GICD_ITARGETSR5_SPEC> {
        INT20_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> INT21_W<GICD_ITARGETSR5_SPEC> {
        INT21_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> INT22_W<GICD_ITARGETSR5_SPEC> {
        INT22_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> INT23_W<GICD_ITARGETSR5_SPEC> {
        INT23_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 20 - 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR5_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr5::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr5::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR5 to value 0"]
impl crate::Resettable for GICD_ITARGETSR5_SPEC {
    const RESET_VALUE: u32 = 0;
}

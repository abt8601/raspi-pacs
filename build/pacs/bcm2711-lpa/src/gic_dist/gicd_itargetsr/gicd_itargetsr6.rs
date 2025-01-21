#[doc = "Register `GICD_ITARGETSR6` reader"]
pub type R = crate::R<GICD_ITARGETSR6_SPEC>;
#[doc = "Register `GICD_ITARGETSR6` writer"]
pub type W = crate::W<GICD_ITARGETSR6_SPEC>;
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type INT24_R = crate::FieldReader;
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type INT24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type INT25_R = crate::FieldReader;
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type INT25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type INT26_R = crate::FieldReader;
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type INT26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type INT27_R = crate::FieldReader;
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type INT27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR6")
            .field("int24", &format_args!("{}", self.int24().bits()))
            .field("int25", &format_args!("{}", self.int25().bits()))
            .field("int26", &format_args!("{}", self.int26().bits()))
            .field("int27", &format_args!("{}", self.int27().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> INT24_W<GICD_ITARGETSR6_SPEC> {
        INT24_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> INT25_W<GICD_ITARGETSR6_SPEC> {
        INT25_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> INT26_W<GICD_ITARGETSR6_SPEC> {
        INT26_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> INT27_W<GICD_ITARGETSR6_SPEC> {
        INT27_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 24 - 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR6_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr6::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr6::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR6 to value 0"]
impl crate::Resettable for GICD_ITARGETSR6_SPEC {
    const RESET_VALUE: u32 = 0;
}

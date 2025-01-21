#[doc = "Register `GICD_ITARGETSR48` reader"]
pub type R = crate::R<GICD_ITARGETSR48_SPEC>;
#[doc = "Register `GICD_ITARGETSR48` writer"]
pub type W = crate::W<GICD_ITARGETSR48_SPEC>;
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type INT192_R = crate::FieldReader;
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type INT192_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type INT193_R = crate::FieldReader;
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type INT193_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type INT194_R = crate::FieldReader;
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type INT194_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type INT195_R = crate::FieldReader;
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type INT195_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> INT192_R {
        INT192_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> INT193_R {
        INT193_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> INT194_R {
        INT194_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> INT195_R {
        INT195_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR48")
            .field("int192", &format_args!("{}", self.int192().bits()))
            .field("int193", &format_args!("{}", self.int193().bits()))
            .field("int194", &format_args!("{}", self.int194().bits()))
            .field("int195", &format_args!("{}", self.int195().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR48_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> INT192_W<GICD_ITARGETSR48_SPEC> {
        INT192_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> INT193_W<GICD_ITARGETSR48_SPEC> {
        INT193_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> INT194_W<GICD_ITARGETSR48_SPEC> {
        INT194_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> INT195_W<GICD_ITARGETSR48_SPEC> {
        INT195_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 192 - 195\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR48_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr48::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr48::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR48 to value 0"]
impl crate::Resettable for GICD_ITARGETSR48_SPEC {
    const RESET_VALUE: u32 = 0;
}

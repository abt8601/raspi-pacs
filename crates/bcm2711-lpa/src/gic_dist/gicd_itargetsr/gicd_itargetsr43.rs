#[doc = "Register `GICD_ITARGETSR43` reader"]
pub type R = crate::R<GICD_ITARGETSR43_SPEC>;
#[doc = "Register `GICD_ITARGETSR43` writer"]
pub type W = crate::W<GICD_ITARGETSR43_SPEC>;
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type INT172_R = crate::FieldReader;
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type INT172_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type INT173_R = crate::FieldReader;
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type INT173_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type INT174_R = crate::FieldReader;
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type INT174_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type INT175_R = crate::FieldReader;
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type INT175_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> INT172_R {
        INT172_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> INT173_R {
        INT173_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> INT174_R {
        INT174_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> INT175_R {
        INT175_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR43")
            .field("int172", &format_args!("{}", self.int172().bits()))
            .field("int173", &format_args!("{}", self.int173().bits()))
            .field("int174", &format_args!("{}", self.int174().bits()))
            .field("int175", &format_args!("{}", self.int175().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR43_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> INT172_W<GICD_ITARGETSR43_SPEC> {
        INT172_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> INT173_W<GICD_ITARGETSR43_SPEC> {
        INT173_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> INT174_W<GICD_ITARGETSR43_SPEC> {
        INT174_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> INT175_W<GICD_ITARGETSR43_SPEC> {
        INT175_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 172 - 175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR43_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr43::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR43_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr43::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR43_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR43 to value 0"]
impl crate::Resettable for GICD_ITARGETSR43_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

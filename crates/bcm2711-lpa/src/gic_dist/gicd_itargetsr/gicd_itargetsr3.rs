#[doc = "Register `GICD_ITARGETSR3` reader"]
pub type R = crate::R<GICD_ITARGETSR3_SPEC>;
#[doc = "Register `GICD_ITARGETSR3` writer"]
pub type W = crate::W<GICD_ITARGETSR3_SPEC>;
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type INT12_R = crate::FieldReader;
#[doc = "Field `INT12` writer - Interrupt 12"]
pub type INT12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type INT13_R = crate::FieldReader;
#[doc = "Field `INT13` writer - Interrupt 13"]
pub type INT13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type INT14_R = crate::FieldReader;
#[doc = "Field `INT14` writer - Interrupt 14"]
pub type INT14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type INT15_R = crate::FieldReader;
#[doc = "Field `INT15` writer - Interrupt 15"]
pub type INT15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR3")
            .field("int12", &format_args!("{}", self.int12().bits()))
            .field("int13", &format_args!("{}", self.int13().bits()))
            .field("int14", &format_args!("{}", self.int14().bits()))
            .field("int15", &format_args!("{}", self.int15().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<GICD_ITARGETSR3_SPEC> {
        INT12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<GICD_ITARGETSR3_SPEC> {
        INT13_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<GICD_ITARGETSR3_SPEC> {
        INT14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<GICD_ITARGETSR3_SPEC> {
        INT15_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 12 - 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR3_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr3::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr3::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR3 to value 0"]
impl crate::Resettable for GICD_ITARGETSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

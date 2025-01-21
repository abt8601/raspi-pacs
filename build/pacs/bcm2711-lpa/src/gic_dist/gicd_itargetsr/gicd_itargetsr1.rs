#[doc = "Register `GICD_ITARGETSR1` reader"]
pub type R = crate::R<GICD_ITARGETSR1_SPEC>;
#[doc = "Register `GICD_ITARGETSR1` writer"]
pub type W = crate::W<GICD_ITARGETSR1_SPEC>;
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type INT4_R = crate::FieldReader;
#[doc = "Field `INT4` writer - Interrupt 4"]
pub type INT4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type INT5_R = crate::FieldReader;
#[doc = "Field `INT5` writer - Interrupt 5"]
pub type INT5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type INT6_R = crate::FieldReader;
#[doc = "Field `INT6` writer - Interrupt 6"]
pub type INT6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type INT7_R = crate::FieldReader;
#[doc = "Field `INT7` writer - Interrupt 7"]
pub type INT7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR1")
            .field("int4", &format_args!("{}", self.int4().bits()))
            .field("int5", &format_args!("{}", self.int5().bits()))
            .field("int6", &format_args!("{}", self.int6().bits()))
            .field("int7", &format_args!("{}", self.int7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<GICD_ITARGETSR1_SPEC> {
        INT4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<GICD_ITARGETSR1_SPEC> {
        INT5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<GICD_ITARGETSR1_SPEC> {
        INT6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<GICD_ITARGETSR1_SPEC> {
        INT7_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 4 - 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR1_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr1::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr1::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR1 to value 0"]
impl crate::Resettable for GICD_ITARGETSR1_SPEC {
    const RESET_VALUE: u32 = 0;
}

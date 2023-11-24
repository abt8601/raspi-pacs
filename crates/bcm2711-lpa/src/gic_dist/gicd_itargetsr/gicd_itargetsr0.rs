#[doc = "Register `GICD_ITARGETSR0` reader"]
pub type R = crate::R<GICD_ITARGETSR0_SPEC>;
#[doc = "Register `GICD_ITARGETSR0` writer"]
pub type W = crate::W<GICD_ITARGETSR0_SPEC>;
#[doc = "Field `INT0` reader - Interrupt 0"]
pub type INT0_R = crate::FieldReader;
#[doc = "Field `INT0` writer - Interrupt 0"]
pub type INT0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT1` reader - Interrupt 1"]
pub type INT1_R = crate::FieldReader;
#[doc = "Field `INT1` writer - Interrupt 1"]
pub type INT1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT2` reader - Interrupt 2"]
pub type INT2_R = crate::FieldReader;
#[doc = "Field `INT2` writer - Interrupt 2"]
pub type INT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT3` reader - Interrupt 3"]
pub type INT3_R = crate::FieldReader;
#[doc = "Field `INT3` writer - Interrupt 3"]
pub type INT3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR0")
            .field("int0", &format_args!("{}", self.int0().bits()))
            .field("int1", &format_args!("{}", self.int1().bits()))
            .field("int2", &format_args!("{}", self.int2().bits()))
            .field("int3", &format_args!("{}", self.int3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<GICD_ITARGETSR0_SPEC> {
        INT0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<GICD_ITARGETSR0_SPEC> {
        INT1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<GICD_ITARGETSR0_SPEC> {
        INT2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<GICD_ITARGETSR0_SPEC> {
        INT3_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 0 - 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR0_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr0::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr0::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR0 to value 0"]
impl crate::Resettable for GICD_ITARGETSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

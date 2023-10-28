#[doc = "Register `GICD_ITARGETSR18` reader"]
pub type R = crate::R<GICD_ITARGETSR18_SPEC>;
#[doc = "Register `GICD_ITARGETSR18` writer"]
pub type W = crate::W<GICD_ITARGETSR18_SPEC>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::FieldReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::FieldReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::FieldReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::FieldReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR18")
            .field("swi0", &format_args!("{}", self.swi0().bits()))
            .field("swi1", &format_args!("{}", self.swi1().bits()))
            .field("swi2", &format_args!("{}", self.swi2().bits()))
            .field("swi3", &format_args!("{}", self.swi3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR18_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<GICD_ITARGETSR18_SPEC, 0> {
        SWI0_W::new(self)
    }
    #[doc = "Bits 8:15 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<GICD_ITARGETSR18_SPEC, 8> {
        SWI1_W::new(self)
    }
    #[doc = "Bits 16:23 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<GICD_ITARGETSR18_SPEC, 16> {
        SWI2_W::new(self)
    }
    #[doc = "Bits 24:31 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<GICD_ITARGETSR18_SPEC, 24> {
        SWI3_W::new(self)
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
#[doc = "Interrupt Processor Target 72 - 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR18_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr18::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr18::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR18_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR18 to value 0"]
impl crate::Resettable for GICD_ITARGETSR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_ITARGETSR20` reader"]
pub type R = crate::R<GICD_ITARGETSR20_SPEC>;
#[doc = "Register `GICD_ITARGETSR20` writer"]
pub type W = crate::W<GICD_ITARGETSR20_SPEC>;
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type INT80_R = crate::FieldReader;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type INT80_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type INT81_R = crate::FieldReader;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type INT81_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type INT82_R = crate::FieldReader;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type INT82_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type INT83_R = crate::FieldReader;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type INT83_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> INT80_R {
        INT80_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> INT81_R {
        INT81_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> INT82_R {
        INT82_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> INT83_R {
        INT83_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR20")
            .field("int80", &format_args!("{}", self.int80().bits()))
            .field("int81", &format_args!("{}", self.int81().bits()))
            .field("int82", &format_args!("{}", self.int82().bits()))
            .field("int83", &format_args!("{}", self.int83().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR20_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> INT80_W<GICD_ITARGETSR20_SPEC, 0> {
        INT80_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> INT81_W<GICD_ITARGETSR20_SPEC, 8> {
        INT81_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> INT82_W<GICD_ITARGETSR20_SPEC, 16> {
        INT82_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> INT83_W<GICD_ITARGETSR20_SPEC, 24> {
        INT83_W::new(self)
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
#[doc = "Interrupt Processor Target 80 - 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR20_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr20::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr20::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR20 to value 0"]
impl crate::Resettable for GICD_ITARGETSR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

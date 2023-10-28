#[doc = "Register `GICD_ITARGETSR45` reader"]
pub type R = crate::R<GICD_ITARGETSR45_SPEC>;
#[doc = "Register `GICD_ITARGETSR45` writer"]
pub type W = crate::W<GICD_ITARGETSR45_SPEC>;
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type INT180_R = crate::FieldReader;
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type INT180_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type INT181_R = crate::FieldReader;
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type INT181_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type INT182_R = crate::FieldReader;
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type INT182_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type INT183_R = crate::FieldReader;
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type INT183_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> INT180_R {
        INT180_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> INT181_R {
        INT181_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> INT182_R {
        INT182_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> INT183_R {
        INT183_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR45")
            .field("int180", &format_args!("{}", self.int180().bits()))
            .field("int181", &format_args!("{}", self.int181().bits()))
            .field("int182", &format_args!("{}", self.int182().bits()))
            .field("int183", &format_args!("{}", self.int183().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR45_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> INT180_W<GICD_ITARGETSR45_SPEC, 0> {
        INT180_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> INT181_W<GICD_ITARGETSR45_SPEC, 8> {
        INT181_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> INT182_W<GICD_ITARGETSR45_SPEC, 16> {
        INT182_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> INT183_W<GICD_ITARGETSR45_SPEC, 24> {
        INT183_W::new(self)
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
#[doc = "Interrupt Processor Target 180 - 183\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR45_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr45::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR45_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr45::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR45_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR45 to value 0"]
impl crate::Resettable for GICD_ITARGETSR45_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

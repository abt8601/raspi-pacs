#[doc = "Register `GICD_ITARGETSR15` reader"]
pub type R = crate::R<GICD_ITARGETSR15_SPEC>;
#[doc = "Register `GICD_ITARGETSR15` writer"]
pub type W = crate::W<GICD_ITARGETSR15_SPEC>;
#[doc = "Field `INT60` reader - Interrupt 60"]
pub type INT60_R = crate::FieldReader;
#[doc = "Field `INT60` writer - Interrupt 60"]
pub type INT60_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT61` reader - Interrupt 61"]
pub type INT61_R = crate::FieldReader;
#[doc = "Field `INT61` writer - Interrupt 61"]
pub type INT61_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type INT62_R = crate::FieldReader;
#[doc = "Field `INT62` writer - Interrupt 62"]
pub type INT62_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT63` reader - Interrupt 63"]
pub type INT63_R = crate::FieldReader;
#[doc = "Field `INT63` writer - Interrupt 63"]
pub type INT63_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 60"]
    #[inline(always)]
    pub fn int60(&self) -> INT60_R {
        INT60_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 61"]
    #[inline(always)]
    pub fn int61(&self) -> INT61_R {
        INT61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> INT62_R {
        INT62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 63"]
    #[inline(always)]
    pub fn int63(&self) -> INT63_R {
        INT63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR15")
            .field("int60", &format_args!("{}", self.int60().bits()))
            .field("int61", &format_args!("{}", self.int61().bits()))
            .field("int62", &format_args!("{}", self.int62().bits()))
            .field("int63", &format_args!("{}", self.int63().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 60"]
    #[inline(always)]
    #[must_use]
    pub fn int60(&mut self) -> INT60_W<GICD_ITARGETSR15_SPEC, 0> {
        INT60_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 61"]
    #[inline(always)]
    #[must_use]
    pub fn int61(&mut self) -> INT61_W<GICD_ITARGETSR15_SPEC, 8> {
        INT61_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 62"]
    #[inline(always)]
    #[must_use]
    pub fn int62(&mut self) -> INT62_W<GICD_ITARGETSR15_SPEC, 16> {
        INT62_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 63"]
    #[inline(always)]
    #[must_use]
    pub fn int63(&mut self) -> INT63_W<GICD_ITARGETSR15_SPEC, 24> {
        INT63_W::new(self)
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
#[doc = "Interrupt Processor Target 60 - 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR15_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr15::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr15::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR15 to value 0"]
impl crate::Resettable for GICD_ITARGETSR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
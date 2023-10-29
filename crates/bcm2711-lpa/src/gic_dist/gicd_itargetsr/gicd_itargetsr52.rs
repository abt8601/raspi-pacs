#[doc = "Register `GICD_ITARGETSR52` reader"]
pub type R = crate::R<GICD_ITARGETSR52_SPEC>;
#[doc = "Register `GICD_ITARGETSR52` writer"]
pub type W = crate::W<GICD_ITARGETSR52_SPEC>;
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type INT208_R = crate::FieldReader;
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type INT208_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type INT209_R = crate::FieldReader;
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type INT209_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type INT210_R = crate::FieldReader;
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type INT210_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type INT211_R = crate::FieldReader;
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type INT211_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> INT208_R {
        INT208_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> INT209_R {
        INT209_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> INT210_R {
        INT210_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> INT211_R {
        INT211_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR52")
            .field("int208", &format_args!("{}", self.int208().bits()))
            .field("int209", &format_args!("{}", self.int209().bits()))
            .field("int210", &format_args!("{}", self.int210().bits()))
            .field("int211", &format_args!("{}", self.int211().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR52_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> INT208_W<GICD_ITARGETSR52_SPEC, 0> {
        INT208_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> INT209_W<GICD_ITARGETSR52_SPEC, 8> {
        INT209_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> INT210_W<GICD_ITARGETSR52_SPEC, 16> {
        INT210_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> INT211_W<GICD_ITARGETSR52_SPEC, 24> {
        INT211_W::new(self)
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
#[doc = "Interrupt Processor Target 208 - 211\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR52_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr52::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr52::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR52_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR52 to value 0"]
impl crate::Resettable for GICD_ITARGETSR52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

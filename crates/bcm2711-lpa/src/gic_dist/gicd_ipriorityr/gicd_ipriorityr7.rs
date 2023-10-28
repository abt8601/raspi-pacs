#[doc = "Register `GICD_IPRIORITYR7` reader"]
pub type R = crate::R<GICD_IPRIORITYR7_SPEC>;
#[doc = "Register `GICD_IPRIORITYR7` writer"]
pub type W = crate::W<GICD_IPRIORITYR7_SPEC>;
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type INT28_R = crate::FieldReader;
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type INT28_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type INT29_R = crate::FieldReader;
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type INT29_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type INT30_R = crate::FieldReader;
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type INT30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type INT31_R = crate::FieldReader;
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type INT31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR7")
            .field("int28", &format_args!("{}", self.int28().bits()))
            .field("int29", &format_args!("{}", self.int29().bits()))
            .field("int30", &format_args!("{}", self.int30().bits()))
            .field("int31", &format_args!("{}", self.int31().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> INT28_W<GICD_IPRIORITYR7_SPEC, 0> {
        INT28_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> INT29_W<GICD_IPRIORITYR7_SPEC, 8> {
        INT29_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> INT30_W<GICD_IPRIORITYR7_SPEC, 16> {
        INT30_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> INT31_W<GICD_IPRIORITYR7_SPEC, 24> {
        INT31_W::new(self)
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
#[doc = "Interrupt Priority 28 - 31 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR7_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr7::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr7::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR7 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

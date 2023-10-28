#[doc = "Register `GICD_IPRIORITYR55` reader"]
pub type R = crate::R<GICD_IPRIORITYR55_SPEC>;
#[doc = "Register `GICD_IPRIORITYR55` writer"]
pub type W = crate::W<GICD_IPRIORITYR55_SPEC>;
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type INT220_R = crate::FieldReader;
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type INT220_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type INT221_R = crate::FieldReader;
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type INT221_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type INT222_R = crate::FieldReader;
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type INT222_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type INT223_R = crate::FieldReader;
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type INT223_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> INT220_R {
        INT220_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> INT221_R {
        INT221_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> INT222_R {
        INT222_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> INT223_R {
        INT223_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR55")
            .field("int220", &format_args!("{}", self.int220().bits()))
            .field("int221", &format_args!("{}", self.int221().bits()))
            .field("int222", &format_args!("{}", self.int222().bits()))
            .field("int223", &format_args!("{}", self.int223().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR55_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> INT220_W<GICD_IPRIORITYR55_SPEC, 0> {
        INT220_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> INT221_W<GICD_IPRIORITYR55_SPEC, 8> {
        INT221_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> INT222_W<GICD_IPRIORITYR55_SPEC, 16> {
        INT222_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> INT223_W<GICD_IPRIORITYR55_SPEC, 24> {
        INT223_W::new(self)
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
#[doc = "Interrupt Priority 220 - 223 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR55_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr55::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR55_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr55::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR55_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR55 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR55_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_IPRIORITYR14` reader"]
pub type R = crate::R<GICD_IPRIORITYR14_SPEC>;
#[doc = "Register `GICD_IPRIORITYR14` writer"]
pub type W = crate::W<GICD_IPRIORITYR14_SPEC>;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type INT56_R = crate::FieldReader;
#[doc = "Field `INT56` writer - Interrupt 56"]
pub type INT56_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type INT57_R = crate::FieldReader;
#[doc = "Field `INT57` writer - Interrupt 57"]
pub type INT57_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type INT58_R = crate::FieldReader;
#[doc = "Field `INT58` writer - Interrupt 58"]
pub type INT58_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type INT59_R = crate::FieldReader;
#[doc = "Field `INT59` writer - Interrupt 59"]
pub type INT59_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> INT56_R {
        INT56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> INT57_R {
        INT57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> INT58_R {
        INT58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> INT59_R {
        INT59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR14")
            .field("int56", &format_args!("{}", self.int56().bits()))
            .field("int57", &format_args!("{}", self.int57().bits()))
            .field("int58", &format_args!("{}", self.int58().bits()))
            .field("int59", &format_args!("{}", self.int59().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn int56(&mut self) -> INT56_W<GICD_IPRIORITYR14_SPEC, 0> {
        INT56_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn int57(&mut self) -> INT57_W<GICD_IPRIORITYR14_SPEC, 8> {
        INT57_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn int58(&mut self) -> INT58_W<GICD_IPRIORITYR14_SPEC, 16> {
        INT58_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn int59(&mut self) -> INT59_W<GICD_IPRIORITYR14_SPEC, 24> {
        INT59_W::new(self)
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
#[doc = "Interrupt Priority 56 - 59 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR14_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr14::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr14::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR14 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
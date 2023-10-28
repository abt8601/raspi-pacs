#[doc = "Register `GICD_IPRIORITYR21` reader"]
pub type R = crate::R<GICD_IPRIORITYR21_SPEC>;
#[doc = "Register `GICD_IPRIORITYR21` writer"]
pub type W = crate::W<GICD_IPRIORITYR21_SPEC>;
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type INT84_R = crate::FieldReader;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type INT84_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type INT85_R = crate::FieldReader;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type INT85_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type INT86_R = crate::FieldReader;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type INT86_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type INT87_R = crate::FieldReader;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type INT87_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> INT84_R {
        INT84_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> INT85_R {
        INT85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> INT86_R {
        INT86_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> INT87_R {
        INT87_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR21")
            .field("int84", &format_args!("{}", self.int84().bits()))
            .field("int85", &format_args!("{}", self.int85().bits()))
            .field("int86", &format_args!("{}", self.int86().bits()))
            .field("int87", &format_args!("{}", self.int87().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR21_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> INT84_W<GICD_IPRIORITYR21_SPEC, 0> {
        INT84_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> INT85_W<GICD_IPRIORITYR21_SPEC, 8> {
        INT85_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> INT86_W<GICD_IPRIORITYR21_SPEC, 16> {
        INT86_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> INT87_W<GICD_IPRIORITYR21_SPEC, 24> {
        INT87_W::new(self)
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
#[doc = "Interrupt Priority 84 - 87 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR21_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr21::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr21::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR21_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR21 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

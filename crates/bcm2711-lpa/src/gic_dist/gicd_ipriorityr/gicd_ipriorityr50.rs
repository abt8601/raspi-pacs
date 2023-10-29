#[doc = "Register `GICD_IPRIORITYR50` reader"]
pub type R = crate::R<GICD_IPRIORITYR50_SPEC>;
#[doc = "Register `GICD_IPRIORITYR50` writer"]
pub type W = crate::W<GICD_IPRIORITYR50_SPEC>;
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type INT200_R = crate::FieldReader;
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type INT200_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type INT201_R = crate::FieldReader;
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type INT201_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type INT202_R = crate::FieldReader;
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type INT202_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type INT203_R = crate::FieldReader;
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type INT203_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> INT200_R {
        INT200_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> INT201_R {
        INT201_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> INT202_R {
        INT202_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> INT203_R {
        INT203_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR50")
            .field("int200", &format_args!("{}", self.int200().bits()))
            .field("int201", &format_args!("{}", self.int201().bits()))
            .field("int202", &format_args!("{}", self.int202().bits()))
            .field("int203", &format_args!("{}", self.int203().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR50_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> INT200_W<GICD_IPRIORITYR50_SPEC, 0> {
        INT200_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> INT201_W<GICD_IPRIORITYR50_SPEC, 8> {
        INT201_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> INT202_W<GICD_IPRIORITYR50_SPEC, 16> {
        INT202_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> INT203_W<GICD_IPRIORITYR50_SPEC, 24> {
        INT203_W::new(self)
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
#[doc = "Interrupt Priority 200 - 203 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR50_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR50_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr50::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR50_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr50::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR50_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR50 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR50_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

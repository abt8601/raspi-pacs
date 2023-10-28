#[doc = "Register `GICD_IPRIORITYR46` reader"]
pub type R = crate::R<GICD_IPRIORITYR46_SPEC>;
#[doc = "Register `GICD_IPRIORITYR46` writer"]
pub type W = crate::W<GICD_IPRIORITYR46_SPEC>;
#[doc = "Field `INT184` reader - Interrupt 184"]
pub type INT184_R = crate::FieldReader;
#[doc = "Field `INT184` writer - Interrupt 184"]
pub type INT184_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT185` reader - Interrupt 185"]
pub type INT185_R = crate::FieldReader;
#[doc = "Field `INT185` writer - Interrupt 185"]
pub type INT185_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT186` reader - Interrupt 186"]
pub type INT186_R = crate::FieldReader;
#[doc = "Field `INT186` writer - Interrupt 186"]
pub type INT186_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT187` reader - Interrupt 187"]
pub type INT187_R = crate::FieldReader;
#[doc = "Field `INT187` writer - Interrupt 187"]
pub type INT187_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 184"]
    #[inline(always)]
    pub fn int184(&self) -> INT184_R {
        INT184_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 185"]
    #[inline(always)]
    pub fn int185(&self) -> INT185_R {
        INT185_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 186"]
    #[inline(always)]
    pub fn int186(&self) -> INT186_R {
        INT186_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 187"]
    #[inline(always)]
    pub fn int187(&self) -> INT187_R {
        INT187_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR46")
            .field("int184", &format_args!("{}", self.int184().bits()))
            .field("int185", &format_args!("{}", self.int185().bits()))
            .field("int186", &format_args!("{}", self.int186().bits()))
            .field("int187", &format_args!("{}", self.int187().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR46_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn int184(&mut self) -> INT184_W<GICD_IPRIORITYR46_SPEC, 0> {
        INT184_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn int185(&mut self) -> INT185_W<GICD_IPRIORITYR46_SPEC, 8> {
        INT185_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn int186(&mut self) -> INT186_W<GICD_IPRIORITYR46_SPEC, 16> {
        INT186_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn int187(&mut self) -> INT187_W<GICD_IPRIORITYR46_SPEC, 24> {
        INT187_W::new(self)
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
#[doc = "Interrupt Priority 184 - 187 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR46_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR46_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr46::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR46_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr46::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR46_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR46 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR46_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

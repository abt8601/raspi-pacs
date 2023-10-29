#[doc = "Register `GICD_IPRIORITYR41` reader"]
pub type R = crate::R<GICD_IPRIORITYR41_SPEC>;
#[doc = "Register `GICD_IPRIORITYR41` writer"]
pub type W = crate::W<GICD_IPRIORITYR41_SPEC>;
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type INT164_R = crate::FieldReader;
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type INT164_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type INT165_R = crate::FieldReader;
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type INT165_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type INT166_R = crate::FieldReader;
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type INT166_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type INT167_R = crate::FieldReader;
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type INT167_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> INT164_R {
        INT164_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> INT165_R {
        INT165_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> INT166_R {
        INT166_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> INT167_R {
        INT167_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR41")
            .field("int164", &format_args!("{}", self.int164().bits()))
            .field("int165", &format_args!("{}", self.int165().bits()))
            .field("int166", &format_args!("{}", self.int166().bits()))
            .field("int167", &format_args!("{}", self.int167().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR41_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> INT164_W<GICD_IPRIORITYR41_SPEC, 0> {
        INT164_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> INT165_W<GICD_IPRIORITYR41_SPEC, 8> {
        INT165_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> INT166_W<GICD_IPRIORITYR41_SPEC, 16> {
        INT166_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> INT167_W<GICD_IPRIORITYR41_SPEC, 24> {
        INT167_W::new(self)
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
#[doc = "Interrupt Priority 164 - 167 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR41_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR41_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr41::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR41_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr41::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR41_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR41 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR41_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `GICD_IPRIORITYR23` reader"]
pub type R = crate::R<GICD_IPRIORITYR23_SPEC>;
#[doc = "Register `GICD_IPRIORITYR23` writer"]
pub type W = crate::W<GICD_IPRIORITYR23_SPEC>;
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type INT92_R = crate::FieldReader;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type INT92_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type INT93_R = crate::FieldReader;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type INT93_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type INT94_R = crate::FieldReader;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type INT94_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type INT95_R = crate::FieldReader;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type INT95_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> INT92_R {
        INT92_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> INT93_R {
        INT93_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> INT94_R {
        INT94_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> INT95_R {
        INT95_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR23")
            .field("int92", &format_args!("{}", self.int92().bits()))
            .field("int93", &format_args!("{}", self.int93().bits()))
            .field("int94", &format_args!("{}", self.int94().bits()))
            .field("int95", &format_args!("{}", self.int95().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR23_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> INT92_W<GICD_IPRIORITYR23_SPEC> {
        INT92_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> INT93_W<GICD_IPRIORITYR23_SPEC> {
        INT93_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> INT94_W<GICD_IPRIORITYR23_SPEC> {
        INT94_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> INT95_W<GICD_IPRIORITYR23_SPEC> {
        INT95_W::new(self, 24)
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
#[doc = "Interrupt Priority 92 - 95 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR23_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr23::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr23::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR23 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR23_SPEC {
    const RESET_VALUE: u32 = 0;
}

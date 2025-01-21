#[doc = "Register `GICD_IPRIORITYR51` reader"]
pub type R = crate::R<GICD_IPRIORITYR51_SPEC>;
#[doc = "Register `GICD_IPRIORITYR51` writer"]
pub type W = crate::W<GICD_IPRIORITYR51_SPEC>;
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type INT204_R = crate::FieldReader;
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type INT204_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type INT205_R = crate::FieldReader;
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type INT205_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type INT206_R = crate::FieldReader;
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type INT206_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type INT207_R = crate::FieldReader;
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type INT207_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> INT204_R {
        INT204_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> INT205_R {
        INT205_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> INT206_R {
        INT206_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> INT207_R {
        INT207_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR51")
            .field("int204", &format_args!("{}", self.int204().bits()))
            .field("int205", &format_args!("{}", self.int205().bits()))
            .field("int206", &format_args!("{}", self.int206().bits()))
            .field("int207", &format_args!("{}", self.int207().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR51_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> INT204_W<GICD_IPRIORITYR51_SPEC> {
        INT204_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> INT205_W<GICD_IPRIORITYR51_SPEC> {
        INT205_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> INT206_W<GICD_IPRIORITYR51_SPEC> {
        INT206_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> INT207_W<GICD_IPRIORITYR51_SPEC> {
        INT207_W::new(self, 24)
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
#[doc = "Interrupt Priority 204 - 207 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR51_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr51::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR51_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr51::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR51_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR51 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR51_SPEC {
    const RESET_VALUE: u32 = 0;
}

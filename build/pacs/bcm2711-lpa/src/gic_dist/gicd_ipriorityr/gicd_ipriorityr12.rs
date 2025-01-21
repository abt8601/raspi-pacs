#[doc = "Register `GICD_IPRIORITYR12` reader"]
pub type R = crate::R<GICD_IPRIORITYR12_SPEC>;
#[doc = "Register `GICD_IPRIORITYR12` writer"]
pub type W = crate::W<GICD_IPRIORITYR12_SPEC>;
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type INT48_R = crate::FieldReader;
#[doc = "Field `INT48` writer - Interrupt 48"]
pub type INT48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type INT49_R = crate::FieldReader;
#[doc = "Field `INT49` writer - Interrupt 49"]
pub type INT49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type INT50_R = crate::FieldReader;
#[doc = "Field `INT50` writer - Interrupt 50"]
pub type INT50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type INT51_R = crate::FieldReader;
#[doc = "Field `INT51` writer - Interrupt 51"]
pub type INT51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> INT48_R {
        INT48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> INT49_R {
        INT49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> INT50_R {
        INT50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> INT51_R {
        INT51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR12")
            .field("int48", &format_args!("{}", self.int48().bits()))
            .field("int49", &format_args!("{}", self.int49().bits()))
            .field("int50", &format_args!("{}", self.int50().bits()))
            .field("int51", &format_args!("{}", self.int51().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn int48(&mut self) -> INT48_W<GICD_IPRIORITYR12_SPEC> {
        INT48_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn int49(&mut self) -> INT49_W<GICD_IPRIORITYR12_SPEC> {
        INT49_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn int50(&mut self) -> INT50_W<GICD_IPRIORITYR12_SPEC> {
        INT50_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn int51(&mut self) -> INT51_W<GICD_IPRIORITYR12_SPEC> {
        INT51_W::new(self, 24)
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
#[doc = "Interrupt Priority 48 - 51 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR12_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr12::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr12::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR12 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR12_SPEC {
    const RESET_VALUE: u32 = 0;
}

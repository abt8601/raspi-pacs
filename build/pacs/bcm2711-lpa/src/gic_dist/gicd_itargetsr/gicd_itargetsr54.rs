#[doc = "Register `GICD_ITARGETSR54` reader"]
pub type R = crate::R<GICD_ITARGETSR54_SPEC>;
#[doc = "Register `GICD_ITARGETSR54` writer"]
pub type W = crate::W<GICD_ITARGETSR54_SPEC>;
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type INT216_R = crate::FieldReader;
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type INT216_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type INT217_R = crate::FieldReader;
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type INT217_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type INT218_R = crate::FieldReader;
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type INT218_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type INT219_R = crate::FieldReader;
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type INT219_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> INT216_R {
        INT216_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> INT217_R {
        INT217_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> INT218_R {
        INT218_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> INT219_R {
        INT219_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR54")
            .field("int216", &format_args!("{}", self.int216().bits()))
            .field("int217", &format_args!("{}", self.int217().bits()))
            .field("int218", &format_args!("{}", self.int218().bits()))
            .field("int219", &format_args!("{}", self.int219().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR54_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> INT216_W<GICD_ITARGETSR54_SPEC> {
        INT216_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> INT217_W<GICD_ITARGETSR54_SPEC> {
        INT217_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> INT218_W<GICD_ITARGETSR54_SPEC> {
        INT218_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> INT219_W<GICD_ITARGETSR54_SPEC> {
        INT219_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 216 - 219\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR54_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR54_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr54::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR54_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr54::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR54_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR54 to value 0"]
impl crate::Resettable for GICD_ITARGETSR54_SPEC {
    const RESET_VALUE: u32 = 0;
}

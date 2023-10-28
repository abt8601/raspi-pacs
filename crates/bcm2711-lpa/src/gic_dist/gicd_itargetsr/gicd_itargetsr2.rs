#[doc = "Register `GICD_ITARGETSR2` reader"]
pub type R = crate::R<GICD_ITARGETSR2_SPEC>;
#[doc = "Register `GICD_ITARGETSR2` writer"]
pub type W = crate::W<GICD_ITARGETSR2_SPEC>;
#[doc = "Field `INT8` reader - Interrupt 8"]
pub type INT8_R = crate::FieldReader;
#[doc = "Field `INT8` writer - Interrupt 8"]
pub type INT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type INT9_R = crate::FieldReader;
#[doc = "Field `INT9` writer - Interrupt 9"]
pub type INT9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type INT10_R = crate::FieldReader;
#[doc = "Field `INT10` writer - Interrupt 10"]
pub type INT10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT11` reader - Interrupt 11"]
pub type INT11_R = crate::FieldReader;
#[doc = "Field `INT11` writer - Interrupt 11"]
pub type INT11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR2")
            .field("int8", &format_args!("{}", self.int8().bits()))
            .field("int9", &format_args!("{}", self.int9().bits()))
            .field("int10", &format_args!("{}", self.int10().bits()))
            .field("int11", &format_args!("{}", self.int11().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<GICD_ITARGETSR2_SPEC, 0> {
        INT8_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<GICD_ITARGETSR2_SPEC, 8> {
        INT9_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<GICD_ITARGETSR2_SPEC, 16> {
        INT10_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<GICD_ITARGETSR2_SPEC, 24> {
        INT11_W::new(self)
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
#[doc = "Interrupt Processor Target 8 - 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR2_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr2::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr2::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR2 to value 0"]
impl crate::Resettable for GICD_ITARGETSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

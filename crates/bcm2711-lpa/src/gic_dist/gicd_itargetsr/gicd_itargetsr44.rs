#[doc = "Register `GICD_ITARGETSR44` reader"]
pub type R = crate::R<GICD_ITARGETSR44_SPEC>;
#[doc = "Register `GICD_ITARGETSR44` writer"]
pub type W = crate::W<GICD_ITARGETSR44_SPEC>;
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type INT176_R = crate::FieldReader;
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type INT176_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type INT177_R = crate::FieldReader;
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type INT177_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type INT178_R = crate::FieldReader;
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type INT178_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type INT179_R = crate::FieldReader;
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type INT179_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> INT176_R {
        INT176_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> INT177_R {
        INT177_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> INT178_R {
        INT178_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> INT179_R {
        INT179_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR44")
            .field("int176", &format_args!("{}", self.int176().bits()))
            .field("int177", &format_args!("{}", self.int177().bits()))
            .field("int178", &format_args!("{}", self.int178().bits()))
            .field("int179", &format_args!("{}", self.int179().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR44_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> INT176_W<GICD_ITARGETSR44_SPEC, 0> {
        INT176_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> INT177_W<GICD_ITARGETSR44_SPEC, 8> {
        INT177_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> INT178_W<GICD_ITARGETSR44_SPEC, 16> {
        INT178_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> INT179_W<GICD_ITARGETSR44_SPEC, 24> {
        INT179_W::new(self)
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
#[doc = "Interrupt Processor Target 176 - 179\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR44_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr44::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR44_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr44::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR44_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR44 to value 0"]
impl crate::Resettable for GICD_ITARGETSR44_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

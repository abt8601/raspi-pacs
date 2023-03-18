#[doc = "Register `GICD_IPRIORITYR16` reader"]
pub struct R(crate::R<GICD_IPRIORITYR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR16` writer"]
pub struct W(crate::W<GICD_IPRIORITYR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GICD_IPRIORITYR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR16_SPEC, u8, u8, 8, O>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR16_SPEC, u8, u8, 8, O>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR16_SPEC, u8, u8, 8, O>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR16_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<0> {
        TIMER_W::new(self)
    }
    #[doc = "Bits 8:15 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<8> {
        MAILBOX_W::new(self)
    }
    #[doc = "Bits 16:23 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<16> {
        DOORBELL0_W::new(self)
    }
    #[doc = "Bits 24:31 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<24> {
        DOORBELL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 64 - 67 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr16](index.html) module"]
pub struct GICD_IPRIORITYR16_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr16::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr16::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR16 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

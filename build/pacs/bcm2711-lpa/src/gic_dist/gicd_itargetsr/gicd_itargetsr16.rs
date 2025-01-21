#[doc = "Register `GICD_ITARGETSR16` reader"]
pub type R = crate::R<GICD_ITARGETSR16_SPEC>;
#[doc = "Register `GICD_ITARGETSR16` writer"]
pub type W = crate::W<GICD_ITARGETSR16_SPEC>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::FieldReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::FieldReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::FieldReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::FieldReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR16")
            .field("timer", &format_args!("{}", self.timer().bits()))
            .field("mailbox", &format_args!("{}", self.mailbox().bits()))
            .field("doorbell0", &format_args!("{}", self.doorbell0().bits()))
            .field("doorbell1", &format_args!("{}", self.doorbell1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR16_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<GICD_ITARGETSR16_SPEC> {
        TIMER_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<GICD_ITARGETSR16_SPEC> {
        MAILBOX_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<GICD_ITARGETSR16_SPEC> {
        DOORBELL0_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<GICD_ITARGETSR16_SPEC> {
        DOORBELL1_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 64 - 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR16_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr16::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr16::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR16 to value 0"]
impl crate::Resettable for GICD_ITARGETSR16_SPEC {
    const RESET_VALUE: u32 = 0;
}

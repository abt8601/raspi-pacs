#[doc = "Register `GICC_CTLR` reader"]
pub struct R(crate::R<GICC_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_CTLR` writer"]
pub struct W(crate::W<GICC_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_CTLR_SPEC>;
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
impl From<crate::W<GICC_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_GROUP_0` reader - Enable signaling of group 0"]
pub type ENABLE_GROUP_0_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_GROUP_0` writer - Enable signaling of group 0"]
pub type ENABLE_GROUP_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_GROUP_1` reader - Enable signaling of group 1"]
pub type ENABLE_GROUP_1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_GROUP_1` writer - Enable signaling of group 1"]
pub type ENABLE_GROUP_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `ACKCTL` reader - Whether a read of IAR acknowledges the interrupt"]
pub type ACKCTL_R = crate::BitReader<bool>;
#[doc = "Field `ACKCTL` writer - Whether a read of IAR acknowledges the interrupt"]
pub type ACKCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `FIQEN` reader - Group 0 triggers FIQ"]
pub type FIQEN_R = crate::BitReader<bool>;
#[doc = "Field `FIQEN` writer - Group 0 triggers FIQ"]
pub type FIQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `CBPR` reader - Common control of interrupts through GICC_BPR"]
pub type CBPR_R = crate::BitReader<bool>;
#[doc = "Field `CBPR` writer - Common control of interrupts through GICC_BPR"]
pub type CBPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `FIQBYPDISGRP0` reader - Bypass FIQ is not signaled to processor"]
pub type FIQBYPDISGRP0_R = crate::BitReader<bool>;
#[doc = "Field `FIQBYPDISGRP0` writer - Bypass FIQ is not signaled to processor"]
pub type FIQBYPDISGRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `IRQBYPDISGRP0` reader - Bypass IRQ is not signaled to processor"]
pub type IRQBYPDISGRP0_R = crate::BitReader<bool>;
#[doc = "Field `IRQBYPDISGRP0` writer - Bypass IRQ is not signaled to processor"]
pub type IRQBYPDISGRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `FIQBYPDISGRP1` reader - Alias of group 1 FIQ bypass disable"]
pub type FIQBYPDISGRP1_R = crate::BitReader<bool>;
#[doc = "Field `FIQBYPDISGRP1` writer - Alias of group 1 FIQ bypass disable"]
pub type FIQBYPDISGRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `IRQBYPDISGRP1` reader - Alias of group 1 IRQ bypass disable"]
pub type IRQBYPDISGRP1_R = crate::BitReader<bool>;
#[doc = "Field `IRQBYPDISGRP1` writer - Alias of group 1 IRQ bypass disable"]
pub type IRQBYPDISGRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `EOIMODES` reader - Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODES_R = crate::BitReader<bool>;
#[doc = "Field `EOIMODES` writer - Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODES_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
#[doc = "Field `EOIMODENS` reader - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODENS_R = crate::BitReader<bool>;
#[doc = "Field `EOIMODENS` writer - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICC_CTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable signaling of group 0"]
    #[inline(always)]
    pub fn enable_group_0(&self) -> ENABLE_GROUP_0_R {
        ENABLE_GROUP_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable signaling of group 1"]
    #[inline(always)]
    pub fn enable_group_1(&self) -> ENABLE_GROUP_1_R {
        ENABLE_GROUP_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Group 0 triggers FIQ"]
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> FIQBYPDISGRP0_R {
        FIQBYPDISGRP0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> IRQBYPDISGRP0_R {
        IRQBYPDISGRP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodes(&self) -> EOIMODES_R {
        EOIMODES_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signaling of group 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_0(&mut self) -> ENABLE_GROUP_0_W<0> {
        ENABLE_GROUP_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable signaling of group 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_1(&mut self) -> ENABLE_GROUP_1_W<1> {
        ENABLE_GROUP_1_W::new(self)
    }
    #[doc = "Bit 2 - Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> ACKCTL_W<2> {
        ACKCTL_W::new(self)
    }
    #[doc = "Bit 3 - Group 0 triggers FIQ"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FIQEN_W<3> {
        FIQEN_W::new(self)
    }
    #[doc = "Bit 4 - Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CBPR_W<4> {
        CBPR_W::new(self)
    }
    #[doc = "Bit 5 - Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W<5> {
        FIQBYPDISGRP0_W::new(self)
    }
    #[doc = "Bit 6 - Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W<6> {
        IRQBYPDISGRP0_W::new(self)
    }
    #[doc = "Bit 7 - Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W<7> {
        FIQBYPDISGRP1_W::new(self)
    }
    #[doc = "Bit 8 - Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W<8> {
        IRQBYPDISGRP1_W::new(self)
    }
    #[doc = "Bit 9 - Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodes(&mut self) -> EOIMODES_W<9> {
        EOIMODES_W::new(self)
    }
    #[doc = "Bit 10 - Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodens(&mut self) -> EOIMODENS_W<10> {
        EOIMODENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Interface Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_ctlr](index.html) module"]
pub struct GICC_CTLR_SPEC;
impl crate::RegisterSpec for GICC_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_ctlr::R](R) reader structure"]
impl crate::Readable for GICC_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_ctlr::W](W) writer structure"]
impl crate::Writable for GICC_CTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_CTLR to value 0"]
impl crate::Resettable for GICC_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

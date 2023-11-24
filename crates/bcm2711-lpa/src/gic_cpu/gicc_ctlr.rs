#[doc = "Register `GICC_CTLR` reader"]
pub type R = crate::R<GICC_CTLR_SPEC>;
#[doc = "Register `GICC_CTLR` writer"]
pub type W = crate::W<GICC_CTLR_SPEC>;
#[doc = "Field `ENABLE_GROUP_0` reader - Enable signaling of group 0"]
pub type ENABLE_GROUP_0_R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP_0` writer - Enable signaling of group 0"]
pub type ENABLE_GROUP_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_GROUP_1` reader - Enable signaling of group 1"]
pub type ENABLE_GROUP_1_R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP_1` writer - Enable signaling of group 1"]
pub type ENABLE_GROUP_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKCTL` reader - Whether a read of IAR acknowledges the interrupt"]
pub type ACKCTL_R = crate::BitReader;
#[doc = "Field `ACKCTL` writer - Whether a read of IAR acknowledges the interrupt"]
pub type ACKCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQEN` reader - Group 0 triggers FIQ"]
pub type FIQEN_R = crate::BitReader;
#[doc = "Field `FIQEN` writer - Group 0 triggers FIQ"]
pub type FIQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBPR` reader - Common control of interrupts through GICC_BPR"]
pub type CBPR_R = crate::BitReader;
#[doc = "Field `CBPR` writer - Common control of interrupts through GICC_BPR"]
pub type CBPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQBYPDISGRP0` reader - Bypass FIQ is not signaled to processor"]
pub type FIQBYPDISGRP0_R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP0` writer - Bypass FIQ is not signaled to processor"]
pub type FIQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP0` reader - Bypass IRQ is not signaled to processor"]
pub type IRQBYPDISGRP0_R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP0` writer - Bypass IRQ is not signaled to processor"]
pub type IRQBYPDISGRP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIQBYPDISGRP1` reader - Alias of group 1 FIQ bypass disable"]
pub type FIQBYPDISGRP1_R = crate::BitReader;
#[doc = "Field `FIQBYPDISGRP1` writer - Alias of group 1 FIQ bypass disable"]
pub type FIQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQBYPDISGRP1` reader - Alias of group 1 IRQ bypass disable"]
pub type IRQBYPDISGRP1_R = crate::BitReader;
#[doc = "Field `IRQBYPDISGRP1` writer - Alias of group 1 IRQ bypass disable"]
pub type IRQBYPDISGRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODES` reader - Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODES_R = crate::BitReader;
#[doc = "Field `EOIMODES` writer - Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIMODENS` reader - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODENS_R = crate::BitReader;
#[doc = "Field `EOIMODENS` writer - Non-Secure EOIR does priority drop. DIR does deactivate."]
pub type EOIMODENS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_CTLR")
            .field(
                "enable_group_0",
                &format_args!("{}", self.enable_group_0().bit()),
            )
            .field(
                "enable_group_1",
                &format_args!("{}", self.enable_group_1().bit()),
            )
            .field("ackctl", &format_args!("{}", self.ackctl().bit()))
            .field("fiqen", &format_args!("{}", self.fiqen().bit()))
            .field("cbpr", &format_args!("{}", self.cbpr().bit()))
            .field(
                "fiqbypdisgrp0",
                &format_args!("{}", self.fiqbypdisgrp0().bit()),
            )
            .field(
                "irqbypdisgrp0",
                &format_args!("{}", self.irqbypdisgrp0().bit()),
            )
            .field(
                "fiqbypdisgrp1",
                &format_args!("{}", self.fiqbypdisgrp1().bit()),
            )
            .field(
                "irqbypdisgrp1",
                &format_args!("{}", self.irqbypdisgrp1().bit()),
            )
            .field("eoimodes", &format_args!("{}", self.eoimodes().bit()))
            .field("eoimodens", &format_args!("{}", self.eoimodens().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_CTLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signaling of group 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_0(&mut self) -> ENABLE_GROUP_0_W<GICC_CTLR_SPEC> {
        ENABLE_GROUP_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable signaling of group 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group_1(&mut self) -> ENABLE_GROUP_1_W<GICC_CTLR_SPEC> {
        ENABLE_GROUP_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> ACKCTL_W<GICC_CTLR_SPEC> {
        ACKCTL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Group 0 triggers FIQ"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FIQEN_W<GICC_CTLR_SPEC> {
        FIQEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CBPR_W<GICC_CTLR_SPEC> {
        CBPR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W<GICC_CTLR_SPEC> {
        FIQBYPDISGRP0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W<GICC_CTLR_SPEC> {
        IRQBYPDISGRP0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W<GICC_CTLR_SPEC> {
        FIQBYPDISGRP1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    #[must_use]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W<GICC_CTLR_SPEC> {
        IRQBYPDISGRP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodes(&mut self) -> EOIMODES_W<GICC_CTLR_SPEC> {
        EOIMODES_W::new(self, 9)
    }
    #[doc = "Bit 10 - Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn eoimodens(&mut self) -> EOIMODENS_W<GICC_CTLR_SPEC> {
        EOIMODENS_W::new(self, 10)
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
#[doc = "CPU Interface Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_CTLR_SPEC;
impl crate::RegisterSpec for GICC_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_ctlr::R`](R) reader structure"]
impl crate::Readable for GICC_CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicc_ctlr::W`](W) writer structure"]
impl crate::Writable for GICC_CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_CTLR to value 0"]
impl crate::Resettable for GICC_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

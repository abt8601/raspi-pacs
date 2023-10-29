#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRCLMP` reader - Power clamp"]
pub type PWRCLMP_R = crate::BitReader;
#[doc = "Field `PWRCLMP` writer - Power clamp"]
pub type PWRCLMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTPDWNMODULE` reader - Power down modules"]
pub type RSTPDWNMODULE_R = crate::BitReader;
#[doc = "Field `RSTPDWNMODULE` writer - Power down modules"]
pub type RSTPDWNMODULE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PHYSUSP_R = crate::BitReader;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PHYSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE_L1GATING` reader - Enable sleep clock gating"]
pub type ENABLE_L1GATING_R = crate::BitReader;
#[doc = "Field `ENABLE_L1GATING` writer - Enable sleep clock gating"]
pub type ENABLE_L1GATING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PHYSLEEP` reader - PHY is in sleep mode"]
pub type PHYSLEEP_R = crate::BitReader;
#[doc = "Field `PHYSLEEP` writer - PHY is in sleep mode"]
pub type PHYSLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEEPSLEEP` reader - PHY is in deep sleep"]
pub type DEEPSLEEP_R = crate::BitReader;
#[doc = "Field `DEEPSLEEP` writer - PHY is in deep sleep"]
pub type DEEPSLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETAFTERSUSP` reader - Reset after suspend"]
pub type RESETAFTERSUSP_R = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` writer - Reset after suspend"]
pub type RESETAFTERSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTOREMODE` reader - Restore mode"]
pub type RESTOREMODE_R = crate::BitReader;
#[doc = "Field `RESTOREMODE` writer - Restore mode"]
pub type RESTOREMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENEXTNDEDHIBER` reader - Enable extended hibernation"]
pub type ENEXTNDEDHIBER_R = crate::BitReader;
#[doc = "Field `ENEXTNDEDHIBER` writer - Enable extended hibernation"]
pub type ENEXTNDEDHIBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` reader - Extended hibernation clamp"]
pub type EXTNDEDHIBERNATIONCLAMP_R = crate::BitReader;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` writer - Extended hibernation clamp"]
pub type EXTNDEDHIBERNATIONCLAMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` reader - Extended hibernation switch"]
pub type EXTNDEDHIBERNATIONSWITCH_R = crate::BitReader;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` writer - Extended hibernation switch"]
pub type EXTNDEDHIBERNATIONSWITCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESSREGRESTORED` reader - Essential register values restored"]
pub type ESSREGRESTORED_R = crate::BitReader;
#[doc = "Field `ESSREGRESTORED` writer - Essential register values restored"]
pub type ESSREGRESTORED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTORE_VALUE` reader - Restore value"]
pub type RESTORE_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `RESTORE_VALUE` writer - Restore value"]
pub type RESTORE_VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power clamp"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PWRCLMP_R {
        PWRCLMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RSTPDWNMODULE_R {
        RSTPDWNMODULE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    pub fn enable_l1gating(&self) -> ENABLE_L1GATING_R {
        ENABLE_L1GATING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY is in sleep mode"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PHY is in deep sleep"]
    #[inline(always)]
    pub fn deepsleep(&self) -> DEEPSLEEP_R {
        DEEPSLEEP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> RESETAFTERSUSP_R {
        RESETAFTERSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Restore mode"]
    #[inline(always)]
    pub fn restoremode(&self) -> RESTOREMODE_R {
        RESTOREMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable extended hibernation"]
    #[inline(always)]
    pub fn enextndedhiber(&self) -> ENEXTNDEDHIBER_R {
        ENEXTNDEDHIBER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Extended hibernation clamp"]
    #[inline(always)]
    pub fn extndedhibernationclamp(&self) -> EXTNDEDHIBERNATIONCLAMP_R {
        EXTNDEDHIBERNATIONCLAMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Extended hibernation switch"]
    #[inline(always)]
    pub fn extndedhibernationswitch(&self) -> EXTNDEDHIBERNATIONSWITCH_R {
        EXTNDEDHIBERNATIONSWITCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Essential register values restored"]
    #[inline(always)]
    pub fn essregrestored(&self) -> ESSREGRESTORED_R {
        ESSREGRESTORED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Restore value"]
    #[inline(always)]
    pub fn restore_value(&self) -> RESTORE_VALUE_R {
        RESTORE_VALUE_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stppclk", &format_args!("{}", self.stppclk().bit()))
            .field("gatehclk", &format_args!("{}", self.gatehclk().bit()))
            .field("pwrclmp", &format_args!("{}", self.pwrclmp().bit()))
            .field(
                "rstpdwnmodule",
                &format_args!("{}", self.rstpdwnmodule().bit()),
            )
            .field("physusp", &format_args!("{}", self.physusp().bit()))
            .field(
                "enable_l1gating",
                &format_args!("{}", self.enable_l1gating().bit()),
            )
            .field("physleep", &format_args!("{}", self.physleep().bit()))
            .field("deepsleep", &format_args!("{}", self.deepsleep().bit()))
            .field(
                "resetaftersusp",
                &format_args!("{}", self.resetaftersusp().bit()),
            )
            .field("restoremode", &format_args!("{}", self.restoremode().bit()))
            .field(
                "enextndedhiber",
                &format_args!("{}", self.enextndedhiber().bit()),
            )
            .field(
                "extndedhibernationclamp",
                &format_args!("{}", self.extndedhibernationclamp().bit()),
            )
            .field(
                "extndedhibernationswitch",
                &format_args!("{}", self.extndedhibernationswitch().bit()),
            )
            .field(
                "essregrestored",
                &format_args!("{}", self.essregrestored().bit()),
            )
            .field(
                "restore_value",
                &format_args!("{}", self.restore_value().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PCGCCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<PCGCCTL_SPEC, 0> {
        STPPCLK_W::new(self)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<PCGCCTL_SPEC, 1> {
        GATEHCLK_W::new(self)
    }
    #[doc = "Bit 2 - Power clamp"]
    #[inline(always)]
    #[must_use]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W<PCGCCTL_SPEC, 2> {
        PWRCLMP_W::new(self)
    }
    #[doc = "Bit 3 - Power down modules"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W<PCGCCTL_SPEC, 3> {
        RSTPDWNMODULE_W::new(self)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PHYSUSP_W<PCGCCTL_SPEC, 4> {
        PHYSUSP_W::new(self)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn enable_l1gating(&mut self) -> ENABLE_L1GATING_W<PCGCCTL_SPEC, 5> {
        ENABLE_L1GATING_W::new(self)
    }
    #[doc = "Bit 6 - PHY is in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn physleep(&mut self) -> PHYSLEEP_W<PCGCCTL_SPEC, 6> {
        PHYSLEEP_W::new(self)
    }
    #[doc = "Bit 7 - PHY is in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep(&mut self) -> DEEPSLEEP_W<PCGCCTL_SPEC, 7> {
        DEEPSLEEP_W::new(self)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    #[must_use]
    pub fn resetaftersusp(&mut self) -> RESETAFTERSUSP_W<PCGCCTL_SPEC, 8> {
        RESETAFTERSUSP_W::new(self)
    }
    #[doc = "Bit 9 - Restore mode"]
    #[inline(always)]
    #[must_use]
    pub fn restoremode(&mut self) -> RESTOREMODE_W<PCGCCTL_SPEC, 9> {
        RESTOREMODE_W::new(self)
    }
    #[doc = "Bit 10 - Enable extended hibernation"]
    #[inline(always)]
    #[must_use]
    pub fn enextndedhiber(&mut self) -> ENEXTNDEDHIBER_W<PCGCCTL_SPEC, 10> {
        ENEXTNDEDHIBER_W::new(self)
    }
    #[doc = "Bit 11 - Extended hibernation clamp"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationclamp(&mut self) -> EXTNDEDHIBERNATIONCLAMP_W<PCGCCTL_SPEC, 11> {
        EXTNDEDHIBERNATIONCLAMP_W::new(self)
    }
    #[doc = "Bit 12 - Extended hibernation switch"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationswitch(&mut self) -> EXTNDEDHIBERNATIONSWITCH_W<PCGCCTL_SPEC, 12> {
        EXTNDEDHIBERNATIONSWITCH_W::new(self)
    }
    #[doc = "Bit 13 - Essential register values restored"]
    #[inline(always)]
    #[must_use]
    pub fn essregrestored(&mut self) -> ESSREGRESTORED_W<PCGCCTL_SPEC, 13> {
        ESSREGRESTORED_W::new(self)
    }
    #[doc = "Bits 14:31 - Restore value"]
    #[inline(always)]
    #[must_use]
    pub fn restore_value(&mut self) -> RESTORE_VALUE_W<PCGCCTL_SPEC, 14> {
        RESTORE_VALUE_W::new(self)
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
#[doc = "power and clock gating control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x200b_8000;
}

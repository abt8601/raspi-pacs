#[doc = "Register `PCGCCTL` reader"]
pub struct R(crate::R<PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCGCCTL` writer"]
pub struct W(crate::W<PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCGCCTL_SPEC>;
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
impl From<crate::W<PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader<bool>;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader<bool>;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PWRCLMP` reader - Power clamp"]
pub type PWRCLMP_R = crate::BitReader<bool>;
#[doc = "Field `PWRCLMP` writer - Power clamp"]
pub type PWRCLMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `RSTPDWNMODULE` reader - Power down modules"]
pub type RSTPDWNMODULE_R = crate::BitReader<bool>;
#[doc = "Field `RSTPDWNMODULE` writer - Power down modules"]
pub type RSTPDWNMODULE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PHYSUSP_R = crate::BitReader<bool>;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PHYSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `ENABLE_L1GATING` reader - Enable sleep clock gating"]
pub type ENABLE_L1GATING_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_L1GATING` writer - Enable sleep clock gating"]
pub type ENABLE_L1GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PHYSLEEP` reader - PHY is in sleep mode"]
pub type PHYSLEEP_R = crate::BitReader<bool>;
#[doc = "Field `PHYSLEEP` writer - PHY is in sleep mode"]
pub type PHYSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `DEEPSLEEP` reader - PHY is in deep sleep"]
pub type DEEPSLEEP_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP` writer - PHY is in deep sleep"]
pub type DEEPSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `RESETAFTERSUSP` reader - Reset after suspend"]
pub type RESETAFTERSUSP_R = crate::BitReader<bool>;
#[doc = "Field `RESETAFTERSUSP` writer - Reset after suspend"]
pub type RESETAFTERSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `RESTOREMODE` reader - Restore mode"]
pub type RESTOREMODE_R = crate::BitReader<bool>;
#[doc = "Field `RESTOREMODE` writer - Restore mode"]
pub type RESTOREMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `ENEXTNDEDHIBER` reader - Enable extended hibernation"]
pub type ENEXTNDEDHIBER_R = crate::BitReader<bool>;
#[doc = "Field `ENEXTNDEDHIBER` writer - Enable extended hibernation"]
pub type ENEXTNDEDHIBER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` reader - Extended hibernation clamp"]
pub type EXTNDEDHIBERNATIONCLAMP_R = crate::BitReader<bool>;
#[doc = "Field `EXTNDEDHIBERNATIONCLAMP` writer - Extended hibernation clamp"]
pub type EXTNDEDHIBERNATIONCLAMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` reader - Extended hibernation switch"]
pub type EXTNDEDHIBERNATIONSWITCH_R = crate::BitReader<bool>;
#[doc = "Field `EXTNDEDHIBERNATIONSWITCH` writer - Extended hibernation switch"]
pub type EXTNDEDHIBERNATIONSWITCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `ESSREGRESTORED` reader - Essential register values restored"]
pub type ESSREGRESTORED_R = crate::BitReader<bool>;
#[doc = "Field `ESSREGRESTORED` writer - Essential register values restored"]
pub type ESSREGRESTORED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
#[doc = "Field `RESTORE_VALUE` reader - Restore value"]
pub type RESTORE_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESTORE_VALUE` writer - Restore value"]
pub type RESTORE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCGCCTL_SPEC, u32, u32, 18, O>;
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
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<0> {
        STPPCLK_W::new(self)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<1> {
        GATEHCLK_W::new(self)
    }
    #[doc = "Bit 2 - Power clamp"]
    #[inline(always)]
    #[must_use]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W<2> {
        PWRCLMP_W::new(self)
    }
    #[doc = "Bit 3 - Power down modules"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W<3> {
        RSTPDWNMODULE_W::new(self)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PHYSUSP_W<4> {
        PHYSUSP_W::new(self)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn enable_l1gating(&mut self) -> ENABLE_L1GATING_W<5> {
        ENABLE_L1GATING_W::new(self)
    }
    #[doc = "Bit 6 - PHY is in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn physleep(&mut self) -> PHYSLEEP_W<6> {
        PHYSLEEP_W::new(self)
    }
    #[doc = "Bit 7 - PHY is in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep(&mut self) -> DEEPSLEEP_W<7> {
        DEEPSLEEP_W::new(self)
    }
    #[doc = "Bit 8 - Reset after suspend"]
    #[inline(always)]
    #[must_use]
    pub fn resetaftersusp(&mut self) -> RESETAFTERSUSP_W<8> {
        RESETAFTERSUSP_W::new(self)
    }
    #[doc = "Bit 9 - Restore mode"]
    #[inline(always)]
    #[must_use]
    pub fn restoremode(&mut self) -> RESTOREMODE_W<9> {
        RESTOREMODE_W::new(self)
    }
    #[doc = "Bit 10 - Enable extended hibernation"]
    #[inline(always)]
    #[must_use]
    pub fn enextndedhiber(&mut self) -> ENEXTNDEDHIBER_W<10> {
        ENEXTNDEDHIBER_W::new(self)
    }
    #[doc = "Bit 11 - Extended hibernation clamp"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationclamp(&mut self) -> EXTNDEDHIBERNATIONCLAMP_W<11> {
        EXTNDEDHIBERNATIONCLAMP_W::new(self)
    }
    #[doc = "Bit 12 - Extended hibernation switch"]
    #[inline(always)]
    #[must_use]
    pub fn extndedhibernationswitch(&mut self) -> EXTNDEDHIBERNATIONSWITCH_W<12> {
        EXTNDEDHIBERNATIONSWITCH_W::new(self)
    }
    #[doc = "Bit 13 - Essential register values restored"]
    #[inline(always)]
    #[must_use]
    pub fn essregrestored(&mut self) -> ESSREGRESTORED_W<13> {
        ESSREGRESTORED_W::new(self)
    }
    #[doc = "Bits 14:31 - Restore value"]
    #[inline(always)]
    #[must_use]
    pub fn restore_value(&mut self) -> RESTORE_VALUE_W<14> {
        RESTORE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power and clock gating control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgcctl](index.html) module"]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcgcctl::R](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x200b_8000;
}

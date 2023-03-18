#[doc = "Register `RSTC` reader"]
pub struct R(crate::R<RSTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTC` writer"]
pub struct W(crate::W<RSTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTC_SPEC>;
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
impl From<crate::W<RSTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRCFG` reader - Watchdog reset config"]
pub type WRCFG_R = crate::FieldReader<u8, WRCFG_A>;
#[doc = "Watchdog reset config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRCFG_A {
    #[doc = "2: `10`"]
    FULL_RESET = 2,
}
impl From<WRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: WRCFG_A) -> Self {
        variant as _
    }
}
impl WRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRCFG_A> {
        match self.bits {
            2 => Some(WRCFG_A::FULL_RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_RESET`"]
    #[inline(always)]
    pub fn is_full_reset(&self) -> bool {
        *self == WRCFG_A::FULL_RESET
    }
}
#[doc = "Field `WRCFG` writer - Watchdog reset config"]
pub type WRCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSTC_SPEC, u8, WRCFG_A, 2, O>;
impl<'a, const O: u8> WRCFG_W<'a, O> {
    #[doc = "`10`"]
    #[inline(always)]
    pub fn full_reset(self) -> &'a mut W {
        self.variant(WRCFG_A::FULL_RESET)
    }
}
#[doc = "Password. Always 0x5a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PASSWD_AW {
    #[doc = "90: `1011010`"]
    PASSWD = 90,
}
impl From<PASSWD_AW> for u8 {
    #[inline(always)]
    fn from(variant: PASSWD_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PASSWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSTC_SPEC, u8, PASSWD_AW, 8, O>;
impl<'a, const O: u8> PASSWD_W<'a, O> {
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(PASSWD_AW::PASSWD)
    }
}
impl R {
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    pub fn wrcfg(&self) -> WRCFG_R {
        WRCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    #[must_use]
    pub fn wrcfg(&mut self) -> WRCFG_W<4> {
        WRCFG_W::new(self)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<24> {
        PASSWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc](index.html) module"]
pub struct RSTC_SPEC;
impl crate::RegisterSpec for RSTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstc::R](R) reader structure"]
impl crate::Readable for RSTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstc::W](W) writer structure"]
impl crate::Writable for RSTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTC to value 0x0102"]
impl crate::Resettable for RSTC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102;
}

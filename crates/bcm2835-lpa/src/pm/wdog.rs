#[doc = "Register `WDOG` reader"]
pub struct R(crate::R<WDOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOG` writer"]
pub struct W(crate::W<WDOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_SPEC>;
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
impl From<crate::W<WDOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - Time until watchdog alarm"]
pub type TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIME` writer - Time until watchdog alarm"]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDOG_SPEC, u32, u32, 20, O>;
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
pub type PASSWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDOG_SPEC, u8, PASSWD_AW, 8, O>;
impl<'a, const O: u8> PASSWD_W<'a, O> {
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(PASSWD_AW::PASSWD)
    }
}
impl R {
    #[doc = "Bits 0:19 - Time until watchdog alarm"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time until watchdog alarm"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
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
#[doc = "Watchdog control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog](index.html) module"]
pub struct WDOG_SPEC;
impl crate::RegisterSpec for WDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdog::R](R) reader structure"]
impl crate::Readable for WDOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog::W](W) writer structure"]
impl crate::Writable for WDOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDOG to value 0"]
impl crate::Resettable for WDOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

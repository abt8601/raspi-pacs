#[doc = "Register `WDOG` reader"]
pub type R = crate::R<WDOG_SPEC>;
#[doc = "Register `WDOG` writer"]
pub type W = crate::W<WDOG_SPEC>;
#[doc = "Field `TIME` reader - Time until watchdog alarm"]
pub type TIME_R = crate::FieldReader<u32>;
#[doc = "Field `TIME` writer - Time until watchdog alarm"]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Password. Always 0x5a\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
impl crate::FieldSpec for PASSWD_AW {
    type Ux = u8;
}
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PASSWD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PASSWD_AW>;
impl<'a, REG> PASSWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDOG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time until watchdog alarm"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<WDOG_SPEC> {
        TIME_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<WDOG_SPEC> {
        PASSWD_W::new(self, 24)
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
#[doc = "Watchdog control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_SPEC;
impl crate::RegisterSpec for WDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog::R`](R) reader structure"]
impl crate::Readable for WDOG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog::W`](W) writer structure"]
impl crate::Writable for WDOG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDOG to value 0"]
impl crate::Resettable for WDOG_SPEC {
    const RESET_VALUE: u32 = 0;
}

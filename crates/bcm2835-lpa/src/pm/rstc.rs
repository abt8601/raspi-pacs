#[doc = "Register `RSTC` reader"]
pub type R = crate::R<RSTC_SPEC>;
#[doc = "Register `RSTC` writer"]
pub type W = crate::W<RSTC_SPEC>;
#[doc = "Field `WRCFG` reader - Watchdog reset config"]
pub type WRCFG_R = crate::FieldReader<WRCFG_A>;
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
impl crate::FieldSpec for WRCFG_A {
    type Ux = u8;
}
impl WRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRCFG_A> {
        match self.bits {
            2 => Some(WRCFG_A::FULL_RESET),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_full_reset(&self) -> bool {
        *self == WRCFG_A::FULL_RESET
    }
}
#[doc = "Field `WRCFG` writer - Watchdog reset config"]
pub type WRCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WRCFG_A>;
impl<'a, REG> WRCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn full_reset(self) -> &'a mut crate::W<REG> {
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
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    pub fn wrcfg(&self) -> WRCFG_R {
        WRCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTC")
            .field("wrcfg", &format_args!("{}", self.wrcfg().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 4:5 - Watchdog reset config"]
    #[inline(always)]
    #[must_use]
    pub fn wrcfg(&mut self) -> WRCFG_W<RSTC_SPEC> {
        WRCFG_W::new(self, 4)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<RSTC_SPEC> {
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
#[doc = "Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTC_SPEC;
impl crate::RegisterSpec for RSTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstc::R`](R) reader structure"]
impl crate::Readable for RSTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstc::W`](W) writer structure"]
impl crate::Writable for RSTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTC to value 0x0102"]
impl crate::Resettable for RSTC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102;
}

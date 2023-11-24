#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RWUSIG_R = crate::BitReader;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SDIS_R = crate::BitReader;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GINSTS_R = crate::BitReader;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GONSTS_R = crate::BitReader;
#[doc = "Field `TCTL` reader - Test control"]
pub type TCTL_R = crate::FieldReader;
#[doc = "Field `TCTL` writer - Test control"]
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type POPRGDNE_R = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwusig", &format_args!("{}", self.rwusig().bit()))
            .field("sdis", &format_args!("{}", self.sdis().bit()))
            .field("ginsts", &format_args!("{}", self.ginsts().bit()))
            .field("gonsts", &format_args!("{}", self.gonsts().bit()))
            .field("tctl", &format_args!("{}", self.tctl().bits()))
            .field("poprgdne", &format_args!("{}", self.poprgdne().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rwusig(&mut self) -> RWUSIG_W<DCTL_SPEC> {
        RWUSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<DCTL_SPEC> {
        SDIS_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    #[must_use]
    pub fn tctl(&mut self) -> TCTL_W<DCTL_SPEC> {
        TCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<DCTL_SPEC> {
        SGINAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<DCTL_SPEC> {
        CGINAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<DCTL_SPEC> {
        SGONAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<DCTL_SPEC> {
        CGONAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    #[must_use]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<DCTL_SPEC> {
        POPRGDNE_W::new(self, 11)
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
#[doc = "OTG_HS device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DOEPCTL` reader"]
pub type R = crate::R<DOEPCTL_SPEC>;
#[doc = "Register `DOEPCTL` writer"]
pub type W = crate::W<DOEPCTL_SPEC>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MPSIZ_R = crate::FieldReader;
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub type USBAEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EPTYP_R = crate::FieldReader;
#[doc = "Field `SNPM` reader - Snoop mode"]
pub type SNPM_R = crate::BitReader;
#[doc = "Field `SNPM` writer - Snoop mode"]
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `Stall` writer - STALL handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint enable"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL")
            .field("mpsiz", &format_args!("{}", self.mpsiz().bits()))
            .field("usbaep", &format_args!("{}", self.usbaep().bit()))
            .field("naksts", &format_args!("{}", self.naksts().bit()))
            .field("eptyp", &format_args!("{}", self.eptyp().bits()))
            .field("snpm", &format_args!("{}", self.snpm().bit()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("epdis", &format_args!("{}", self.epdis().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snpm(&mut self) -> SNPM_W<DOEPCTL_SPEC> {
        SNPM_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL_SPEC> {
        EPENA_W::new(self, 31)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL_SPEC;
impl crate::RegisterSpec for DOEPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl::R`](R) reader structure"]
impl crate::Readable for DOEPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl::W`](W) writer structure"]
impl crate::Writable for DOEPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL to value 0x8000"]
impl crate::Resettable for DOEPCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}

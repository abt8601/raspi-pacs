#[doc = "Register `HCCHAR` reader"]
pub type R = crate::R<HCCHAR_SPEC>;
#[doc = "Register `HCCHAR` writer"]
pub type W = crate::W<HCCHAR_SPEC>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MPSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDEV` reader - Low-speed device"]
pub type LSDEV_R = crate::BitReader;
#[doc = "Field `LSDEV` writer - Low-speed device"]
pub type LSDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EPTYP_R = crate::FieldReader;
#[doc = "Field `EPTYP` writer - Endpoint type"]
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MC` reader - Multi Count (MC) / Error Count (EC)"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multi Count (MC) / Error Count (EC)"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAD` reader - Device address"]
pub type DAD_R = crate::FieldReader;
#[doc = "Field `DAD` writer - Device address"]
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR")
            .field("mpsiz", &format_args!("{}", self.mpsiz().bits()))
            .field("epnum", &format_args!("{}", self.epnum().bits()))
            .field("epdir", &format_args!("{}", self.epdir().bit()))
            .field("lsdev", &format_args!("{}", self.lsdev().bit()))
            .field("eptyp", &format_args!("{}", self.eptyp().bits()))
            .field("mc", &format_args!("{}", self.mc().bits()))
            .field("dad", &format_args!("{}", self.dad().bits()))
            .field("oddfrm", &format_args!("{}", self.oddfrm().bit()))
            .field("chdis", &format_args!("{}", self.chdis().bit()))
            .field("chena", &format_args!("{}", self.chena().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCCHAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<HCCHAR_SPEC> {
        MPSIZ_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<HCCHAR_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<HCCHAR_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lsdev(&mut self) -> LSDEV_W<HCCHAR_SPEC> {
        LSDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<HCCHAR_SPEC> {
        EPTYP_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<HCCHAR_SPEC> {
        MC_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<HCCHAR_SPEC> {
        DAD_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HCCHAR_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<HCCHAR_SPEC> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<HCCHAR_SPEC> {
        CHENA_W::new(self, 31)
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
#[doc = "Characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR_SPEC;
impl crate::RegisterSpec for HCCHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar::R`](R) reader structure"]
impl crate::Readable for HCCHAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar::W`](W) writer structure"]
impl crate::Writable for HCCHAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCCHAR to value 0"]
impl crate::Resettable for HCCHAR_SPEC {
    const RESET_VALUE: u32 = 0;
}

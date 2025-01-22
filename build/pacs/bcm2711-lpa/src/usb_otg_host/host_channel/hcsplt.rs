#[doc = "Register `HCSPLT` reader"]
pub type R = crate::R<HCSPLT_SPEC>;
#[doc = "Register `HCSPLT` writer"]
pub type W = crate::W<HCSPLT_SPEC>;
#[doc = "Field `PRTADDR` reader - Port address"]
pub type PRTADDR_R = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port address"]
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub address"]
pub type HUBADDR_R = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub address"]
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XACTPOS_R = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub type COMPLSPLT_R = crate::BitReader;
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub type COMPLSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLITEN` reader - Split enable"]
pub type SPLITEN_R = crate::BitReader;
#[doc = "Field `SPLITEN` writer - Split enable"]
pub type SPLITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT")
            .field("prtaddr", &format_args!("{}", self.prtaddr().bits()))
            .field("hubaddr", &format_args!("{}", self.hubaddr().bits()))
            .field("xactpos", &format_args!("{}", self.xactpos().bits()))
            .field("complsplt", &format_args!("{}", self.complsplt().bit()))
            .field("spliten", &format_args!("{}", self.spliten().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCSPLT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<HCSPLT_SPEC> {
        PRTADDR_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HUBADDR_W<HCSPLT_SPEC> {
        HUBADDR_W::new(self, 7)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XACTPOS_W<HCSPLT_SPEC> {
        XACTPOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<HCSPLT_SPEC> {
        COMPLSPLT_W::new(self, 16)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    #[must_use]
    pub fn spliten(&mut self) -> SPLITEN_W<HCSPLT_SPEC> {
        SPLITEN_W::new(self, 31)
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
#[doc = "Split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCSPLT_SPEC;
impl crate::RegisterSpec for HCSPLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsplt::R`](R) reader structure"]
impl crate::Readable for HCSPLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcsplt::W`](W) writer structure"]
impl crate::Writable for HCSPLT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCSPLT to value 0"]
impl crate::Resettable for HCSPLT_SPEC {
    const RESET_VALUE: u32 = 0;
}

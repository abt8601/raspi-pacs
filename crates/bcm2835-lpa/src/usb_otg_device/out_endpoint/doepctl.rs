#[doc = "Register `DOEPCTL` reader"]
pub struct R(crate::R<DOEPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL` writer"]
pub struct W(crate::W<DOEPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL_SPEC>;
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
impl From<crate::W<DOEPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNPM` reader - Snoop mode"]
pub type SNPM_R = crate::BitReader<bool>;
#[doc = "Field `SNPM` writer - Snoop mode"]
pub type SNPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL_SPEC, bool, O>;
#[doc = "Field `Stall` reader - STALL handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `Stall` writer - STALL handshake"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - Endpoint enable"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snpm(&mut self) -> SNPM_W<20> {
        SNPM_W::new(self)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl](index.html) module"]
pub struct DOEPCTL_SPEC;
impl crate::RegisterSpec for DOEPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl::R](R) reader structure"]
impl crate::Readable for DOEPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl::W](W) writer structure"]
impl crate::Writable for DOEPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL to value 0x8000"]
impl crate::Resettable for DOEPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}

#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMIC` writer - RIMIC"]
pub type RIMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTSMIC` writer - CTSMIC"]
pub type CTSMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DCDMIC` writer - DCDMIC"]
pub type DCDMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DSRMIC` writer - DSRMIC"]
pub type DSRMIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXIC` writer - RXIC"]
pub type RXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXIC` writer - TXIC"]
pub type TXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RTIC` writer - RTIC"]
pub type RTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FEIC` writer - FEIC"]
pub type FEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `PEIC` writer - PEIC"]
pub type PEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BEIC` writer - BEIC"]
pub type BEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `OEIC` writer - OEIC"]
pub type OEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RIMIC"]
    #[inline(always)]
    #[must_use]
    pub fn rimic(&mut self) -> RIMIC_W<0> {
        RIMIC_W::new(self)
    }
    #[doc = "Bit 1 - CTSMIC"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CTSMIC_W<1> {
        CTSMIC_W::new(self)
    }
    #[doc = "Bit 2 - DCDMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmic(&mut self) -> DCDMIC_W<2> {
        DCDMIC_W::new(self)
    }
    #[doc = "Bit 3 - DSRMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmic(&mut self) -> DSRMIC_W<3> {
        DSRMIC_W::new(self)
    }
    #[doc = "Bit 4 - RXIC"]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RXIC_W<4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - TXIC"]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TXIC_W<5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - RTIC"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - FEIC"]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FEIC_W<7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - PEIC"]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PEIC_W<8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - BEIC"]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BEIC_W<9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - OEIC"]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OEIC_W<10> {
        OEIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

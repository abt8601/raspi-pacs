#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `RIMIC` writer - RIMIC"]
pub type RIMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSMIC` writer - CTSMIC"]
pub type CTSMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDMIC` writer - DCDMIC"]
pub type DCDMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSRMIC` writer - DSRMIC"]
pub type DSRMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXIC` writer - RXIC"]
pub type RXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXIC` writer - TXIC"]
pub type TXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIC` writer - RTIC"]
pub type RTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEIC` writer - FEIC"]
pub type FEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEIC` writer - PEIC"]
pub type PEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEIC` writer - BEIC"]
pub type BEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEIC` writer - OEIC"]
pub type OEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<ICR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - RIMIC"]
    #[inline(always)]
    #[must_use]
    pub fn rimic(&mut self) -> RIMIC_W<ICR_SPEC, 0> {
        RIMIC_W::new(self)
    }
    #[doc = "Bit 1 - CTSMIC"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CTSMIC_W<ICR_SPEC, 1> {
        CTSMIC_W::new(self)
    }
    #[doc = "Bit 2 - DCDMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmic(&mut self) -> DCDMIC_W<ICR_SPEC, 2> {
        DCDMIC_W::new(self)
    }
    #[doc = "Bit 3 - DSRMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmic(&mut self) -> DSRMIC_W<ICR_SPEC, 3> {
        DSRMIC_W::new(self)
    }
    #[doc = "Bit 4 - RXIC"]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RXIC_W<ICR_SPEC, 4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - TXIC"]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TXIC_W<ICR_SPEC, 5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - RTIC"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<ICR_SPEC, 6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - FEIC"]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FEIC_W<ICR_SPEC, 7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - PEIC"]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PEIC_W<ICR_SPEC, 8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - BEIC"]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BEIC_W<ICR_SPEC, 9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - OEIC"]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OEIC_W<ICR_SPEC, 10> {
        OEIC_W::new(self)
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
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

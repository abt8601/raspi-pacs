#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `UARTEN` reader - UARTEN"]
pub type UARTEN_R = crate::BitReader;
#[doc = "Field `UARTEN` writer - UARTEN"]
pub type UARTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIREN` reader - SIREN"]
pub type SIREN_R = crate::BitReader;
#[doc = "Field `SIREN` writer - SIREN"]
pub type SIREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - SIRLP"]
pub type SIRLP_R = crate::BitReader;
#[doc = "Field `SIRLP` writer - SIRLP"]
pub type SIRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - TXE"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - TXE"]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - RXE"]
pub type RXE_R = crate::BitReader;
#[doc = "Field `RXE` writer - RXE"]
pub type RXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR` reader - DTR"]
pub type DTR_R = crate::BitReader;
#[doc = "Field `DTR` writer - DTR"]
pub type DTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - RTS"]
pub type RTS_R = crate::BitReader;
#[doc = "Field `RTS` writer - RTS"]
pub type RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTSEN"]
pub type RTSEN_R = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTSEN"]
pub type RTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTSEN"]
pub type CTSEN_R = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTSEN"]
pub type CTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("uarten", &format_args!("{}", self.uarten().bit()))
            .field("siren", &format_args!("{}", self.siren().bit()))
            .field("sirlp", &format_args!("{}", self.sirlp().bit()))
            .field("txe", &format_args!("{}", self.txe().bit()))
            .field("rxe", &format_args!("{}", self.rxe().bit()))
            .field("dtr", &format_args!("{}", self.dtr().bit()))
            .field("rts", &format_args!("{}", self.rts().bit()))
            .field("rtsen", &format_args!("{}", self.rtsen().bit()))
            .field("ctsen", &format_args!("{}", self.ctsen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UARTEN_W<CR_SPEC> {
        UARTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SIREN_W<CR_SPEC> {
        SIREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SIRLP_W<CR_SPEC> {
        SIRLP_W::new(self, 2)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<CR_SPEC> {
        TXE_W::new(self, 8)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<CR_SPEC> {
        RXE_W::new(self, 9)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<CR_SPEC> {
        DTR_W::new(self, 10)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<CR_SPEC> {
        RTS_W::new(self, 11)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<CR_SPEC> {
        RTSEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<CR_SPEC> {
        CTSEN_W::new(self, 15)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

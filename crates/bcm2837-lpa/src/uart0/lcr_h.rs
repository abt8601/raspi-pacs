#[doc = "Register `LCR_H` reader"]
pub type R = crate::R<LCR_H_SPEC>;
#[doc = "Register `LCR_H` writer"]
pub type W = crate::W<LCR_H_SPEC>;
#[doc = "Field `BRK` reader - BRK"]
pub type BRK_R = crate::BitReader;
#[doc = "Field `BRK` writer - BRK"]
pub type BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - PEN"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - PEN"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - EPS"]
pub type EPS_R = crate::BitReader;
#[doc = "Field `EPS` writer - EPS"]
pub type EPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - STP2"]
pub type STP2_R = crate::BitReader;
#[doc = "Field `STP2` writer - STP2"]
pub type STP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - FEN"]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - FEN"]
pub type FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - WLEN"]
pub type WLEN_R = crate::FieldReader;
#[doc = "Field `WLEN` writer - WLEN"]
pub type WLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - SPS"]
pub type SPS_R = crate::BitReader;
#[doc = "Field `SPS` writer - SPS"]
pub type SPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR_H")
            .field("brk", &format_args!("{}", self.brk().bit()))
            .field("pen", &format_args!("{}", self.pen().bit()))
            .field("eps", &format_args!("{}", self.eps().bit()))
            .field("stp2", &format_args!("{}", self.stp2().bit()))
            .field("fen", &format_args!("{}", self.fen().bit()))
            .field("wlen", &format_args!("{}", self.wlen().bits()))
            .field("sps", &format_args!("{}", self.sps().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LCR_H_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - BRK"]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BRK_W<LCR_H_SPEC> {
        BRK_W::new(self, 0)
    }
    #[doc = "Bit 1 - PEN"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<LCR_H_SPEC> {
        PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - EPS"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EPS_W<LCR_H_SPEC> {
        EPS_W::new(self, 2)
    }
    #[doc = "Bit 3 - STP2"]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> STP2_W<LCR_H_SPEC> {
        STP2_W::new(self, 3)
    }
    #[doc = "Bit 4 - FEN"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<LCR_H_SPEC> {
        FEN_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - WLEN"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WLEN_W<LCR_H_SPEC> {
        WLEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - SPS"]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SPS_W<LCR_H_SPEC> {
        SPS_W::new(self, 7)
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
#[doc = "Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCR_H_SPEC;
impl crate::RegisterSpec for LCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr_h::R`](R) reader structure"]
impl crate::Readable for LCR_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcr_h::W`](W) writer structure"]
impl crate::Writable for LCR_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR_H to value 0"]
impl crate::Resettable for LCR_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

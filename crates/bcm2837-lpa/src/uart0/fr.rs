#[doc = "Register `FR` reader"]
pub type R = crate::R<FR_SPEC>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FR_SPEC>;
#[doc = "Field `CTS` reader - CTS"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - CTS"]
pub type CTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR` reader - DSR"]
pub type DSR_R = crate::BitReader;
#[doc = "Field `DSR` writer - DSR"]
pub type DSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` reader - DCD"]
pub type DCD_R = crate::BitReader;
#[doc = "Field `DCD` writer - DCD"]
pub type DCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFE` reader - RXFE"]
pub type RXFE_R = crate::BitReader;
#[doc = "Field `RXFE` writer - RXFE"]
pub type RXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFF` reader - TXFF"]
pub type TXFF_R = crate::BitReader;
#[doc = "Field `TXFF` writer - TXFF"]
pub type TXFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFF` reader - RXFF"]
pub type RXFF_R = crate::BitReader;
#[doc = "Field `RXFF` writer - RXFF"]
pub type RXFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `TXFE` writer - TXFE"]
pub type TXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - RI"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - RI"]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FR")
            .field("cts", &format_args!("{}", self.cts().bit()))
            .field("dsr", &format_args!("{}", self.dsr().bit()))
            .field("dcd", &format_args!("{}", self.dcd().bit()))
            .field("busy", &format_args!("{}", self.busy().bit()))
            .field("rxfe", &format_args!("{}", self.rxfe().bit()))
            .field("txff", &format_args!("{}", self.txff().bit()))
            .field("rxff", &format_args!("{}", self.rxff().bit()))
            .field("txfe", &format_args!("{}", self.txfe().bit()))
            .field("ri", &format_args!("{}", self.ri().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<FR_SPEC> {
        CTS_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DSR_W<FR_SPEC> {
        DSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DCD_W<FR_SPEC> {
        DCD_W::new(self, 2)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<FR_SPEC> {
        BUSY_W::new(self, 3)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RXFE_W<FR_SPEC> {
        RXFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TXFF_W<FR_SPEC> {
        TXFF_W::new(self, 5)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    #[must_use]
    pub fn rxff(&mut self) -> RXFF_W<FR_SPEC> {
        RXFF_W::new(self, 6)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TXFE_W<FR_SPEC> {
        TXFE_W::new(self, 7)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<FR_SPEC> {
        RI_W::new(self, 8)
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
#[doc = "Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

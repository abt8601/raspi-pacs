#[doc = "Register `STA` reader"]
pub type R = crate::R<STA_SPEC>;
#[doc = "Register `STA` writer"]
pub type W = crate::W<STA_SPEC>;
#[doc = "Field `FULL1` reader - FIFO full"]
pub type FULL1_R = crate::BitReader;
#[doc = "Field `FULL1` writer - FIFO full"]
pub type FULL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPT1` reader - FIFO empty"]
pub type EMPT1_R = crate::BitReader;
#[doc = "Field `EMPT1` writer - FIFO empty"]
pub type EMPT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WERR1` reader - FIFO write error"]
pub type WERR1_R = crate::BitReader;
#[doc = "Field `WERR1` writer - FIFO write error"]
pub type WERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR1` reader - FIFO read error"]
pub type RERR1_R = crate::BitReader;
#[doc = "Field `RERR1` writer - FIFO read error"]
pub type RERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO1` reader - Channel 1 gap occurred"]
pub type GAPO1_R = crate::BitReader;
#[doc = "Field `GAPO1` writer - Channel 1 gap occurred"]
pub type GAPO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO2` reader - Channel 2 gap occurred"]
pub type GAPO2_R = crate::BitReader;
#[doc = "Field `GAPO2` writer - Channel 2 gap occurred"]
pub type GAPO2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO3` reader - Channel 3 gap occurred"]
pub type GAPO3_R = crate::BitReader;
#[doc = "Field `GAPO3` writer - Channel 3 gap occurred"]
pub type GAPO3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO4` reader - Channel 4 gap occurred"]
pub type GAPO4_R = crate::BitReader;
#[doc = "Field `GAPO4` writer - Channel 4 gap occurred"]
pub type GAPO4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA1` reader - Channel 1 state"]
pub type STA1_R = crate::BitReader;
#[doc = "Field `STA1` writer - Channel 1 state"]
pub type STA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA2` reader - Channel 2 state"]
pub type STA2_R = crate::BitReader;
#[doc = "Field `STA2` writer - Channel 2 state"]
pub type STA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA3` reader - Channel 3 state"]
pub type STA3_R = crate::BitReader;
#[doc = "Field `STA3` writer - Channel 3 state"]
pub type STA3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA4` reader - Channel 4 state"]
pub type STA4_R = crate::BitReader;
#[doc = "Field `STA4` writer - Channel 4 state"]
pub type STA4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    pub fn empt1(&self) -> EMPT1_R {
        EMPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    pub fn werr1(&self) -> WERR1_R {
        WERR1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    pub fn rerr1(&self) -> RERR1_R {
        RERR1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    pub fn gapo1(&self) -> GAPO1_R {
        GAPO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    pub fn gapo2(&self) -> GAPO2_R {
        GAPO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    pub fn gapo3(&self) -> GAPO3_R {
        GAPO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    pub fn gapo4(&self) -> GAPO4_R {
        GAPO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    pub fn sta1(&self) -> STA1_R {
        STA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    pub fn sta2(&self) -> STA2_R {
        STA2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    pub fn sta3(&self) -> STA3_R {
        STA3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    pub fn sta4(&self) -> STA4_R {
        STA4_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STA")
            .field("sta4", &format_args!("{}", self.sta4().bit()))
            .field("sta3", &format_args!("{}", self.sta3().bit()))
            .field("sta2", &format_args!("{}", self.sta2().bit()))
            .field("sta1", &format_args!("{}", self.sta1().bit()))
            .field("berr", &format_args!("{}", self.berr().bit()))
            .field("gapo4", &format_args!("{}", self.gapo4().bit()))
            .field("gapo3", &format_args!("{}", self.gapo3().bit()))
            .field("gapo2", &format_args!("{}", self.gapo2().bit()))
            .field("gapo1", &format_args!("{}", self.gapo1().bit()))
            .field("rerr1", &format_args!("{}", self.rerr1().bit()))
            .field("werr1", &format_args!("{}", self.werr1().bit()))
            .field("empt1", &format_args!("{}", self.empt1().bit()))
            .field("full1", &format_args!("{}", self.full1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> FULL1_W<STA_SPEC> {
        FULL1_W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn empt1(&mut self) -> EMPT1_W<STA_SPEC> {
        EMPT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    #[must_use]
    pub fn werr1(&mut self) -> WERR1_W<STA_SPEC> {
        WERR1_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    #[must_use]
    pub fn rerr1(&mut self) -> RERR1_W<STA_SPEC> {
        RERR1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo1(&mut self) -> GAPO1_W<STA_SPEC> {
        GAPO1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo2(&mut self) -> GAPO2_W<STA_SPEC> {
        GAPO2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo3(&mut self) -> GAPO3_W<STA_SPEC> {
        GAPO3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo4(&mut self) -> GAPO4_W<STA_SPEC> {
        GAPO4_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<STA_SPEC> {
        BERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta1(&mut self) -> STA1_W<STA_SPEC> {
        STA1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta2(&mut self) -> STA2_W<STA_SPEC> {
        STA2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta3(&mut self) -> STA3_W<STA_SPEC> {
        STA3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta4(&mut self) -> STA4_W<STA_SPEC> {
        STA4_W::new(self, 12)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sta::W`](W) writer structure"]
impl crate::Writable for STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: u32 = 0;
}

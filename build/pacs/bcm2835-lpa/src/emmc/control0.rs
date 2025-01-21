#[doc = "Register `CONTROL0` reader"]
pub type R = crate::R<CONTROL0_SPEC>;
#[doc = "Register `CONTROL0` writer"]
pub type W = crate::W<CONTROL0_SPEC>;
#[doc = "Field `HCTL_DWIDTH` reader - Use 4 data lines"]
pub type HCTL_DWIDTH_R = crate::BitReader;
#[doc = "Field `HCTL_DWIDTH` writer - Use 4 data lines"]
pub type HCTL_DWIDTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCTL_HS_EN` reader - Enable high speed mode"]
pub type HCTL_HS_EN_R = crate::BitReader;
#[doc = "Field `HCTL_HS_EN` writer - Enable high speed mode"]
pub type HCTL_HS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCTL_8BIT` reader - Use 8 data lines"]
pub type HCTL_8BIT_R = crate::BitReader;
#[doc = "Field `HCTL_8BIT` writer - Use 8 data lines"]
pub type HCTL_8BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_STOP` reader - Stop the current transaction at the next block gap"]
pub type GAP_STOP_R = crate::BitReader;
#[doc = "Field `GAP_STOP` writer - Stop the current transaction at the next block gap"]
pub type GAP_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_RESTART` reader - Restart a transaction stopped by GAP_STOP"]
pub type GAP_RESTART_R = crate::BitReader;
#[doc = "Field `GAP_RESTART` writer - Restart a transaction stopped by GAP_STOP"]
pub type GAP_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READWAIT_EN` reader - Use DAT2 read/wait protocol"]
pub type READWAIT_EN_R = crate::BitReader;
#[doc = "Field `READWAIT_EN` writer - Use DAT2 read/wait protocol"]
pub type READWAIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAP_IEN` reader - Enable interrupt on block gap"]
pub type GAP_IEN_R = crate::BitReader;
#[doc = "Field `GAP_IEN` writer - Enable interrupt on block gap"]
pub type GAP_IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MODE` reader - Enable SPI mode"]
pub type SPI_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MODE` writer - Enable SPI mode"]
pub type SPI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_EN` reader - Boot mode enabled"]
pub type BOOT_EN_R = crate::BitReader;
#[doc = "Field `BOOT_EN` writer - Boot mode enabled"]
pub type BOOT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALT_BOOT_EN` reader - Enable alternate boot mode"]
pub type ALT_BOOT_EN_R = crate::BitReader;
#[doc = "Field `ALT_BOOT_EN` writer - Enable alternate boot mode"]
pub type ALT_BOOT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    pub fn hctl_dwidth(&self) -> HCTL_DWIDTH_R {
        HCTL_DWIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    pub fn hctl_hs_en(&self) -> HCTL_HS_EN_R {
        HCTL_HS_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    pub fn hctl_8bit(&self) -> HCTL_8BIT_R {
        HCTL_8BIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    pub fn gap_stop(&self) -> GAP_STOP_R {
        GAP_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    pub fn gap_restart(&self) -> GAP_RESTART_R {
        GAP_RESTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    pub fn readwait_en(&self) -> READWAIT_EN_R {
        READWAIT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    pub fn gap_ien(&self) -> GAP_IEN_R {
        GAP_IEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    pub fn boot_en(&self) -> BOOT_EN_R {
        BOOT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    pub fn alt_boot_en(&self) -> ALT_BOOT_EN_R {
        ALT_BOOT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL0")
            .field("alt_boot_en", &format_args!("{}", self.alt_boot_en().bit()))
            .field("boot_en", &format_args!("{}", self.boot_en().bit()))
            .field("spi_mode", &format_args!("{}", self.spi_mode().bit()))
            .field("gap_ien", &format_args!("{}", self.gap_ien().bit()))
            .field("readwait_en", &format_args!("{}", self.readwait_en().bit()))
            .field("gap_restart", &format_args!("{}", self.gap_restart().bit()))
            .field("gap_stop", &format_args!("{}", self.gap_stop().bit()))
            .field("hctl_8bit", &format_args!("{}", self.hctl_8bit().bit()))
            .field("hctl_hs_en", &format_args!("{}", self.hctl_hs_en().bit()))
            .field("hctl_dwidth", &format_args!("{}", self.hctl_dwidth().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CONTROL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Use 4 data lines"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_dwidth(&mut self) -> HCTL_DWIDTH_W<CONTROL0_SPEC> {
        HCTL_DWIDTH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable high speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_hs_en(&mut self) -> HCTL_HS_EN_W<CONTROL0_SPEC> {
        HCTL_HS_EN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Use 8 data lines"]
    #[inline(always)]
    #[must_use]
    pub fn hctl_8bit(&mut self) -> HCTL_8BIT_W<CONTROL0_SPEC> {
        HCTL_8BIT_W::new(self, 5)
    }
    #[doc = "Bit 16 - Stop the current transaction at the next block gap"]
    #[inline(always)]
    #[must_use]
    pub fn gap_stop(&mut self) -> GAP_STOP_W<CONTROL0_SPEC> {
        GAP_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn gap_restart(&mut self) -> GAP_RESTART_W<CONTROL0_SPEC> {
        GAP_RESTART_W::new(self, 17)
    }
    #[doc = "Bit 18 - Use DAT2 read/wait protocol"]
    #[inline(always)]
    #[must_use]
    pub fn readwait_en(&mut self) -> READWAIT_EN_W<CONTROL0_SPEC> {
        READWAIT_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable interrupt on block gap"]
    #[inline(always)]
    #[must_use]
    pub fn gap_ien(&mut self) -> GAP_IEN_W<CONTROL0_SPEC> {
        GAP_IEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable SPI mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mode(&mut self) -> SPI_MODE_W<CONTROL0_SPEC> {
        SPI_MODE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Boot mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_en(&mut self) -> BOOT_EN_W<CONTROL0_SPEC> {
        BOOT_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable alternate boot mode"]
    #[inline(always)]
    #[must_use]
    pub fn alt_boot_en(&mut self) -> ALT_BOOT_EN_W<CONTROL0_SPEC> {
        ALT_BOOT_EN_W::new(self, 22)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL0_SPEC;
impl crate::RegisterSpec for CONTROL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control0::R`](R) reader structure"]
impl crate::Readable for CONTROL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control0::W`](W) writer structure"]
impl crate::Writable for CONTROL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL0 to value 0"]
impl crate::Resettable for CONTROL0_SPEC {
    const RESET_VALUE: u32 = 0;
}

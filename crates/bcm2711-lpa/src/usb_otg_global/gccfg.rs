#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFG_SPEC>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFG_SPEC>;
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CPADEN` reader - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2CPADEN_R = crate::BitReader;
#[doc = "Field `I2CPADEN` writer - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2CPADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSASEN` reader - Enable the VBUS sensing device"]
pub type VBUSASEN_R = crate::BitReader;
#[doc = "Field `VBUSASEN` writer - Enable the VBUS sensing device"]
pub type VBUSASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSBSEN` reader - Enable the VBUS sensing device"]
pub type VBUSBSEN_R = crate::BitReader;
#[doc = "Field `VBUSBSEN` writer - Enable the VBUS sensing device"]
pub type VBUSBSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOVBUSSENS` reader - VBUS sensing disable option"]
pub type NOVBUSSENS_R = crate::BitReader;
#[doc = "Field `NOVBUSSENS` writer - VBUS sensing disable option"]
pub type NOVBUSSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface"]
    #[inline(always)]
    pub fn i2cpaden(&self) -> I2CPADEN_R {
        I2CPADEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusasen(&self) -> VBUSASEN_R {
        VBUSASEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusbsen(&self) -> VBUSBSEN_R {
        VBUSBSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS sensing disable option"]
    #[inline(always)]
    pub fn novbussens(&self) -> NOVBUSSENS_R {
        NOVBUSSENS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdwn", &format_args!("{}", self.pwrdwn().bit()))
            .field("i2cpaden", &format_args!("{}", self.i2cpaden().bit()))
            .field("vbusasen", &format_args!("{}", self.vbusasen().bit()))
            .field("vbusbsen", &format_args!("{}", self.vbusbsen().bit()))
            .field("sofouten", &format_args!("{}", self.sofouten().bit()))
            .field("novbussens", &format_args!("{}", self.novbussens().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GCCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<GCCFG_SPEC> {
        PWRDWN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface"]
    #[inline(always)]
    #[must_use]
    pub fn i2cpaden(&mut self) -> I2CPADEN_W<GCCFG_SPEC> {
        I2CPADEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<GCCFG_SPEC> {
        VBUSASEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<GCCFG_SPEC> {
        VBUSBSEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<GCCFG_SPEC> {
        SOFOUTEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS sensing disable option"]
    #[inline(always)]
    #[must_use]
    pub fn novbussens(&mut self) -> NOVBUSSENS_W<GCCFG_SPEC> {
        NOVBUSSENS_W::new(self, 21)
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
#[doc = "OTG_HS general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

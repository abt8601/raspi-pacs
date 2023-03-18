#[doc = "Register `GCCFG` reader"]
pub struct R(crate::R<GCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCFG` writer"]
pub struct W(crate::W<GCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCFG_SPEC>;
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
impl From<crate::W<GCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
#[doc = "Field `I2CPADEN` reader - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2CPADEN_R = crate::BitReader<bool>;
#[doc = "Field `I2CPADEN` writer - Enable I2C bus connection for the external I2C PHY interface"]
pub type I2CPADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
#[doc = "Field `VBUSASEN` reader - Enable the VBUS sensing device"]
pub type VBUSASEN_R = crate::BitReader<bool>;
#[doc = "Field `VBUSASEN` writer - Enable the VBUS sensing device"]
pub type VBUSASEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
#[doc = "Field `VBUSBSEN` reader - Enable the VBUS sensing device"]
pub type VBUSBSEN_R = crate::BitReader<bool>;
#[doc = "Field `VBUSBSEN` writer - Enable the VBUS sensing device"]
pub type VBUSBSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
#[doc = "Field `NOVBUSSENS` reader - VBUS sensing disable option"]
pub type NOVBUSSENS_R = crate::BitReader<bool>;
#[doc = "Field `NOVBUSSENS` writer - VBUS sensing disable option"]
pub type NOVBUSSENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<16> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 17 - Enable I2C bus connection for the external I2C PHY interface"]
    #[inline(always)]
    #[must_use]
    pub fn i2cpaden(&mut self) -> I2CPADEN_W<17> {
        I2CPADEN_W::new(self)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<18> {
        VBUSASEN_W::new(self)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<19> {
        VBUSBSEN_W::new(self)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<20> {
        SOFOUTEN_W::new(self)
    }
    #[doc = "Bit 21 - VBUS sensing disable option"]
    #[inline(always)]
    #[must_use]
    pub fn novbussens(&mut self) -> NOVBUSSENS_W<21> {
        NOVBUSSENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS general core configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gccfg](index.html) module"]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gccfg::R](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gccfg::W](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

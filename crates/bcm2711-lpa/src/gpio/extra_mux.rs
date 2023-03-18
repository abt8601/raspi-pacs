#[doc = "Register `EXTRA_MUX` reader"]
pub struct R(crate::R<EXTRA_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTRA_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTRA_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTRA_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTRA_MUX` writer"]
pub struct W(crate::W<EXTRA_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTRA_MUX_SPEC>;
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
impl From<crate::W<EXTRA_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTRA_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO` reader - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
pub type SDIO_R = crate::BitReader<SDIO_A>;
#[doc = "Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO_A {
    #[doc = "0: Connect the newer SD host"]
    SDHOST = 0,
    #[doc = "1: Connect Arasan SD/EMMC host"]
    ARASAN = 1,
}
impl From<SDIO_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_A {
        match self.bits {
            false => SDIO_A::SDHOST,
            true => SDIO_A::ARASAN,
        }
    }
    #[doc = "Checks if the value of the field is `SDHOST`"]
    #[inline(always)]
    pub fn is_sdhost(&self) -> bool {
        *self == SDIO_A::SDHOST
    }
    #[doc = "Checks if the value of the field is `ARASAN`"]
    #[inline(always)]
    pub fn is_arasan(&self) -> bool {
        *self == SDIO_A::ARASAN
    }
}
#[doc = "Field `SDIO` writer - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
pub type SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTRA_MUX_SPEC, SDIO_A, O>;
impl<'a, const O: u8> SDIO_W<'a, O> {
    #[doc = "Connect the newer SD host"]
    #[inline(always)]
    pub fn sdhost(self) -> &'a mut W {
        self.variant(SDIO_A::SDHOST)
    }
    #[doc = "Connect Arasan SD/EMMC host"]
    #[inline(always)]
    pub fn arasan(self) -> &'a mut W {
        self.variant(SDIO_A::ARASAN)
    }
}
impl R {
    #[doc = "Bit 1 - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<1> {
        SDIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Undocumented multiplexing bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extra_mux](index.html) module"]
pub struct EXTRA_MUX_SPEC;
impl crate::RegisterSpec for EXTRA_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extra_mux::R](R) reader structure"]
impl crate::Readable for EXTRA_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extra_mux::W](W) writer structure"]
impl crate::Writable for EXTRA_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

#[doc = "Register `EXTRA_MUX` reader"]
pub type R = crate::R<EXTRA_MUX_SPEC>;
#[doc = "Register `EXTRA_MUX` writer"]
pub type W = crate::W<EXTRA_MUX_SPEC>;
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
    pub const fn variant(&self) -> SDIO_A {
        match self.bits {
            false => SDIO_A::SDHOST,
            true => SDIO_A::ARASAN,
        }
    }
    #[doc = "Connect the newer SD host"]
    #[inline(always)]
    pub fn is_sdhost(&self) -> bool {
        *self == SDIO_A::SDHOST
    }
    #[doc = "Connect Arasan SD/EMMC host"]
    #[inline(always)]
    pub fn is_arasan(&self) -> bool {
        *self == SDIO_A::ARASAN
    }
}
#[doc = "Field `SDIO` writer - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
pub type SDIO_W<'a, REG> = crate::BitWriter<'a, REG, SDIO_A>;
impl<'a, REG> SDIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect the newer SD host"]
    #[inline(always)]
    pub fn sdhost(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO_A::SDHOST)
    }
    #[doc = "Connect Arasan SD/EMMC host"]
    #[inline(always)]
    pub fn arasan(self) -> &'a mut crate::W<REG> {
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTRA_MUX")
            .field("sdio", &format_args!("{}", self.sdio().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXTRA_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<EXTRA_MUX_SPEC> {
        SDIO_W::new(self, 1)
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
#[doc = "Undocumented multiplexing bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extra_mux::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extra_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTRA_MUX_SPEC;
impl crate::RegisterSpec for EXTRA_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extra_mux::R`](R) reader structure"]
impl crate::Readable for EXTRA_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extra_mux::W`](W) writer structure"]
impl crate::Writable for EXTRA_MUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

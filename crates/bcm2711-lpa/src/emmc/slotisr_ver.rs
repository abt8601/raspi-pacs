#[doc = "Register `SLOTISR_VER` reader"]
pub type R = crate::R<SLOTISR_VER_SPEC>;
#[doc = "Register `SLOTISR_VER` writer"]
pub type W = crate::W<SLOTISR_VER_SPEC>;
#[doc = "Field `SLOT_STATUS` reader - OR of interrupt and wakeup signals for each slot"]
pub type SLOT_STATUS_R = crate::FieldReader;
#[doc = "Field `SLOT_STATUS` writer - OR of interrupt and wakeup signals for each slot"]
pub type SLOT_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDVERSION` reader - Host controller specification version"]
pub type SDVERSION_R = crate::FieldReader;
#[doc = "Field `SDVERSION` writer - Host controller specification version"]
pub type SDVERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VENDOR` reader - Vendor version number"]
pub type VENDOR_R = crate::FieldReader;
#[doc = "Field `VENDOR` writer - Vendor version number"]
pub type VENDOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    pub fn slot_status(&self) -> SLOT_STATUS_R {
        SLOT_STATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    pub fn sdversion(&self) -> SDVERSION_R {
        SDVERSION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    pub fn vendor(&self) -> VENDOR_R {
        VENDOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLOTISR_VER")
            .field("vendor", &format_args!("{}", self.vendor().bits()))
            .field("sdversion", &format_args!("{}", self.sdversion().bits()))
            .field(
                "slot_status",
                &format_args!("{}", self.slot_status().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLOTISR_VER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot_status(&mut self) -> SLOT_STATUS_W<SLOTISR_VER_SPEC> {
        SLOT_STATUS_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Host controller specification version"]
    #[inline(always)]
    #[must_use]
    pub fn sdversion(&mut self) -> SDVERSION_W<SLOTISR_VER_SPEC> {
        SDVERSION_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Vendor version number"]
    #[inline(always)]
    #[must_use]
    pub fn vendor(&mut self) -> VENDOR_W<SLOTISR_VER_SPEC> {
        VENDOR_W::new(self, 24)
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
#[doc = "Version information and slot interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotisr_ver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slotisr_ver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLOTISR_VER_SPEC;
impl crate::RegisterSpec for SLOTISR_VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slotisr_ver::R`](R) reader structure"]
impl crate::Readable for SLOTISR_VER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slotisr_ver::W`](W) writer structure"]
impl crate::Writable for SLOTISR_VER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOTISR_VER to value 0"]
impl crate::Resettable for SLOTISR_VER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

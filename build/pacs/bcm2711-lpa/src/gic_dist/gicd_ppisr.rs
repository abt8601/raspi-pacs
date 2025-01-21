#[doc = "Register `GICD_PPISR` reader"]
pub type R = crate::R<GICD_PPISR_SPEC>;
#[doc = "Register `GICD_PPISR` writer"]
pub type W = crate::W<GICD_PPISR_SPEC>;
#[doc = "Field `ID25` reader - Virtual maintenance interrupt"]
pub type ID25_R = crate::BitReader;
#[doc = "Field `ID25` writer - Virtual maintenance interrupt"]
pub type ID25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID26` reader - Hypervisor timer event"]
pub type ID26_R = crate::BitReader;
#[doc = "Field `ID26` writer - Hypervisor timer event"]
pub type ID26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID27` reader - Virtual timer event"]
pub type ID27_R = crate::BitReader;
#[doc = "Field `ID27` writer - Virtual timer event"]
pub type ID27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID28` reader - nLEGACYFIQ signal"]
pub type ID28_R = crate::BitReader;
#[doc = "Field `ID28` writer - nLEGACYFIQ signal"]
pub type ID28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID29` reader - Secure physical timer event"]
pub type ID29_R = crate::BitReader;
#[doc = "Field `ID29` writer - Secure physical timer event"]
pub type ID29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID30` reader - Non-secure physical timer event"]
pub type ID30_R = crate::BitReader;
#[doc = "Field `ID30` writer - Non-secure physical timer event"]
pub type ID30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID31` reader - nLEGACYIRQ signal"]
pub type ID31_R = crate::BitReader;
#[doc = "Field `ID31` writer - nLEGACYIRQ signal"]
pub type ID31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    pub fn id25(&self) -> ID25_R {
        ID25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    pub fn id26(&self) -> ID26_R {
        ID26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    pub fn id27(&self) -> ID27_R {
        ID27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    pub fn id28(&self) -> ID28_R {
        ID28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    pub fn id29(&self) -> ID29_R {
        ID29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    pub fn id30(&self) -> ID30_R {
        ID30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    pub fn id31(&self) -> ID31_R {
        ID31_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PPISR")
            .field("id31", &format_args!("{}", self.id31().bit()))
            .field("id30", &format_args!("{}", self.id30().bit()))
            .field("id29", &format_args!("{}", self.id29().bit()))
            .field("id28", &format_args!("{}", self.id28().bit()))
            .field("id27", &format_args!("{}", self.id27().bit()))
            .field("id26", &format_args!("{}", self.id26().bit()))
            .field("id25", &format_args!("{}", self.id25().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PPISR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn id25(&mut self) -> ID25_W<GICD_PPISR_SPEC> {
        ID25_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id26(&mut self) -> ID26_W<GICD_PPISR_SPEC> {
        ID26_W::new(self, 10)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id27(&mut self) -> ID27_W<GICD_PPISR_SPEC> {
        ID27_W::new(self, 11)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id28(&mut self) -> ID28_W<GICD_PPISR_SPEC> {
        ID28_W::new(self, 12)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id29(&mut self) -> ID29_W<GICD_PPISR_SPEC> {
        ID29_W::new(self, 13)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id30(&mut self) -> ID30_W<GICD_PPISR_SPEC> {
        ID30_W::new(self, 14)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id31(&mut self) -> ID31_W<GICD_PPISR_SPEC> {
        ID31_W::new(self, 15)
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
#[doc = "Private Peripheral Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ppisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ppisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PPISR_SPEC;
impl crate::RegisterSpec for GICD_PPISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ppisr::R`](R) reader structure"]
impl crate::Readable for GICD_PPISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ppisr::W`](W) writer structure"]
impl crate::Writable for GICD_PPISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_PPISR to value 0"]
impl crate::Resettable for GICD_PPISR_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CNTL1` reader"]
pub type R = crate::R<CNTL1_SPEC>;
#[doc = "Register `CNTL1` writer"]
pub type W = crate::W<CNTL1_SPEC>;
#[doc = "Field `KEEP_INPUT` reader - Don't clear the RX shift register before a new transaction"]
pub type KEEP_INPUT_R = crate::BitReader;
#[doc = "Field `KEEP_INPUT` writer - Don't clear the RX shift register before a new transaction"]
pub type KEEP_INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB_FIRST` reader - Shift the most significant bit first (MSB)"]
pub type MSB_FIRST_R = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Shift the most significant bit first (MSB)"]
pub type MSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_ENABLE` reader - Enable DONE interrupt"]
pub type DONE_ENABLE_R = crate::BitReader;
#[doc = "Field `DONE_ENABLE` writer - Enable DONE interrupt"]
pub type DONE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE_ENABLE` reader - Enable TX empty interrupt"]
pub type TXE_ENABLE_R = crate::BitReader;
#[doc = "Field `TXE_ENABLE` writer - Enable TX empty interrupt"]
pub type TXE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HIGH_TIME` reader - Additional SPI clock cycles where CS is high"]
pub type CS_HIGH_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HIGH_TIME` writer - Additional SPI clock cycles where CS is high"]
pub type CS_HIGH_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    pub fn keep_input(&self) -> KEEP_INPUT_R {
        KEEP_INPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    pub fn done_enable(&self) -> DONE_ENABLE_R {
        DONE_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    pub fn txe_enable(&self) -> TXE_ENABLE_R {
        TXE_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    pub fn cs_high_time(&self) -> CS_HIGH_TIME_R {
        CS_HIGH_TIME_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL1")
            .field(
                "cs_high_time",
                &format_args!("{}", self.cs_high_time().bits()),
            )
            .field("txe_enable", &format_args!("{}", self.txe_enable().bit()))
            .field("done_enable", &format_args!("{}", self.done_enable().bit()))
            .field("msb_first", &format_args!("{}", self.msb_first().bit()))
            .field("keep_input", &format_args!("{}", self.keep_input().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CNTL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    #[must_use]
    pub fn keep_input(&mut self) -> KEEP_INPUT_W<CNTL1_SPEC> {
        KEEP_INPUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<CNTL1_SPEC> {
        MSB_FIRST_W::new(self, 1)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn done_enable(&mut self) -> DONE_ENABLE_W<CNTL1_SPEC> {
        DONE_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txe_enable(&mut self) -> TXE_ENABLE_W<CNTL1_SPEC> {
        TXE_ENABLE_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    #[must_use]
    pub fn cs_high_time(&mut self) -> CS_HIGH_TIME_W<CNTL1_SPEC> {
        CS_HIGH_TIME_W::new(self, 8)
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
#[doc = "Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL1_SPEC;
impl crate::RegisterSpec for CNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl1::R`](R) reader structure"]
impl crate::Readable for CNTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl1::W`](W) writer structure"]
impl crate::Writable for CNTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL1 to value 0"]
impl crate::Resettable for CNTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

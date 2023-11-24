#[doc = "Register `CONTROL2` reader"]
pub type R = crate::R<CONTROL2_SPEC>;
#[doc = "Register `CONTROL2` writer"]
pub type W = crate::W<CONTROL2_SPEC>;
#[doc = "Field `ACNOX_ERR` reader - Auto command not executed due to an error"]
pub type ACNOX_ERR_R = crate::BitReader;
#[doc = "Field `ACTO_ERR` reader - Auto command timeout"]
pub type ACTO_ERR_R = crate::BitReader;
#[doc = "Field `ACCRC_ERR` reader - Command CRC error during auto command"]
pub type ACCRC_ERR_R = crate::BitReader;
#[doc = "Field `ACEND_ERR` reader - End bit is not 1 during auto command"]
pub type ACEND_ERR_R = crate::BitReader;
#[doc = "Field `ACBAD_ERR` reader - Command index error during auto command"]
pub type ACBAD_ERR_R = crate::BitReader;
#[doc = "Field `NOTC12_ERR` reader - Error during auto CMD12"]
pub type NOTC12_ERR_R = crate::BitReader;
#[doc = "Field `UHSMODE` reader - Select the speed of the SD card"]
pub type UHSMODE_R = crate::FieldReader<UHSMODE_A>;
#[doc = "Select the speed of the SD card\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UHSMODE_A {
    #[doc = "0: `0`"]
    SDR12 = 0,
    #[doc = "1: `1`"]
    SDR25 = 1,
    #[doc = "2: `10`"]
    SDR50 = 2,
    #[doc = "3: `11`"]
    SDR104 = 3,
    #[doc = "4: `100`"]
    DDR50 = 4,
}
impl From<UHSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UHSMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UHSMODE_A {
    type Ux = u8;
}
impl UHSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UHSMODE_A> {
        match self.bits {
            0 => Some(UHSMODE_A::SDR12),
            1 => Some(UHSMODE_A::SDR25),
            2 => Some(UHSMODE_A::SDR50),
            3 => Some(UHSMODE_A::SDR104),
            4 => Some(UHSMODE_A::DDR50),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMODE_A::SDR12
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMODE_A::SDR25
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMODE_A::SDR50
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMODE_A::SDR104
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMODE_A::DDR50
    }
}
#[doc = "Field `UHSMODE` writer - Select the speed of the SD card"]
pub type UHSMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, UHSMODE_A>;
impl<'a, REG> UHSMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODE_A::SDR12)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODE_A::SDR25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODE_A::SDR50)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODE_A::SDR104)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODE_A::DDR50)
    }
}
#[doc = "Field `TUNEON` reader - SD Clock tune in progress"]
pub type TUNEON_R = crate::BitReader;
#[doc = "Field `TUNEON` writer - SD Clock tune in progress"]
pub type TUNEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNED` reader - Tuned clock is used for sampling data"]
pub type TUNED_R = crate::BitReader;
#[doc = "Field `TUNED` writer - Tuned clock is used for sampling data"]
pub type TUNED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto command not executed due to an error"]
    #[inline(always)]
    pub fn acnox_err(&self) -> ACNOX_ERR_R {
        ACNOX_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto command timeout"]
    #[inline(always)]
    pub fn acto_err(&self) -> ACTO_ERR_R {
        ACTO_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command CRC error during auto command"]
    #[inline(always)]
    pub fn accrc_err(&self) -> ACCRC_ERR_R {
        ACCRC_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End bit is not 1 during auto command"]
    #[inline(always)]
    pub fn acend_err(&self) -> ACEND_ERR_R {
        ACEND_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command index error during auto command"]
    #[inline(always)]
    pub fn acbad_err(&self) -> ACBAD_ERR_R {
        ACBAD_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Error during auto CMD12"]
    #[inline(always)]
    pub fn notc12_err(&self) -> NOTC12_ERR_R {
        NOTC12_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select the speed of the SD card"]
    #[inline(always)]
    pub fn uhsmode(&self) -> UHSMODE_R {
        UHSMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - SD Clock tune in progress"]
    #[inline(always)]
    pub fn tuneon(&self) -> TUNEON_R {
        TUNEON_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tuned clock is used for sampling data"]
    #[inline(always)]
    pub fn tuned(&self) -> TUNED_R {
        TUNED_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL2")
            .field("tuned", &format_args!("{}", self.tuned().bit()))
            .field("tuneon", &format_args!("{}", self.tuneon().bit()))
            .field("uhsmode", &format_args!("{}", self.uhsmode().bits()))
            .field("notc12_err", &format_args!("{}", self.notc12_err().bit()))
            .field("acbad_err", &format_args!("{}", self.acbad_err().bit()))
            .field("acend_err", &format_args!("{}", self.acend_err().bit()))
            .field("accrc_err", &format_args!("{}", self.accrc_err().bit()))
            .field("acto_err", &format_args!("{}", self.acto_err().bit()))
            .field("acnox_err", &format_args!("{}", self.acnox_err().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CONTROL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 16:18 - Select the speed of the SD card"]
    #[inline(always)]
    #[must_use]
    pub fn uhsmode(&mut self) -> UHSMODE_W<CONTROL2_SPEC> {
        UHSMODE_W::new(self, 16)
    }
    #[doc = "Bit 22 - SD Clock tune in progress"]
    #[inline(always)]
    #[must_use]
    pub fn tuneon(&mut self) -> TUNEON_W<CONTROL2_SPEC> {
        TUNEON_W::new(self, 22)
    }
    #[doc = "Bit 23 - Tuned clock is used for sampling data"]
    #[inline(always)]
    #[must_use]
    pub fn tuned(&mut self) -> TUNED_W<CONTROL2_SPEC> {
        TUNED_W::new(self, 23)
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
#[doc = "Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL2_SPEC;
impl crate::RegisterSpec for CONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control2::R`](R) reader structure"]
impl crate::Readable for CONTROL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control2::W`](W) writer structure"]
impl crate::Writable for CONTROL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL2 to value 0"]
impl crate::Resettable for CONTROL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

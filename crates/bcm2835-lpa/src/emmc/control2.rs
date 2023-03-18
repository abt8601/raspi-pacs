#[doc = "Register `CONTROL2` reader"]
pub struct R(crate::R<CONTROL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL2` writer"]
pub struct W(crate::W<CONTROL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL2_SPEC>;
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
impl From<crate::W<CONTROL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACNOX_ERR` reader - Auto command not executed due to an error"]
pub type ACNOX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACTO_ERR` reader - Auto command timeout"]
pub type ACTO_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACCRC_ERR` reader - Command CRC error during auto command"]
pub type ACCRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACEND_ERR` reader - End bit is not 1 during auto command"]
pub type ACEND_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACBAD_ERR` reader - Command index error during auto command"]
pub type ACBAD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `NOTC12_ERR` reader - Error during auto CMD12"]
pub type NOTC12_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UHSMODE` reader - Select the speed of the SD card"]
pub type UHSMODE_R = crate::FieldReader<u8, UHSMODE_A>;
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
impl UHSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UHSMODE_A> {
        match self.bits {
            0 => Some(UHSMODE_A::SDR12),
            1 => Some(UHSMODE_A::SDR25),
            2 => Some(UHSMODE_A::SDR50),
            3 => Some(UHSMODE_A::SDR104),
            4 => Some(UHSMODE_A::DDR50),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMODE_A::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMODE_A::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMODE_A::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMODE_A::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMODE_A::DDR50
    }
}
#[doc = "Field `UHSMODE` writer - Select the speed of the SD card"]
pub type UHSMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL2_SPEC, u8, UHSMODE_A, 3, O>;
impl<'a, const O: u8> UHSMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(UHSMODE_A::SDR12)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(UHSMODE_A::SDR25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(UHSMODE_A::SDR50)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(UHSMODE_A::SDR104)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(UHSMODE_A::DDR50)
    }
}
#[doc = "Field `TUNEON` reader - SD Clock tune in progress"]
pub type TUNEON_R = crate::BitReader<bool>;
#[doc = "Field `TUNEON` writer - SD Clock tune in progress"]
pub type TUNEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, bool, O>;
#[doc = "Field `TUNED` reader - Tuned clock is used for sampling data"]
pub type TUNED_R = crate::BitReader<bool>;
#[doc = "Field `TUNED` writer - Tuned clock is used for sampling data"]
pub type TUNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL2_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 16:18 - Select the speed of the SD card"]
    #[inline(always)]
    #[must_use]
    pub fn uhsmode(&mut self) -> UHSMODE_W<16> {
        UHSMODE_W::new(self)
    }
    #[doc = "Bit 22 - SD Clock tune in progress"]
    #[inline(always)]
    #[must_use]
    pub fn tuneon(&mut self) -> TUNEON_W<22> {
        TUNEON_W::new(self)
    }
    #[doc = "Bit 23 - Tuned clock is used for sampling data"]
    #[inline(always)]
    #[must_use]
    pub fn tuned(&mut self) -> TUNED_W<23> {
        TUNED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control2](index.html) module"]
pub struct CONTROL2_SPEC;
impl crate::RegisterSpec for CONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control2::R](R) reader structure"]
impl crate::Readable for CONTROL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control2::W](W) writer structure"]
impl crate::Writable for CONTROL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL2 to value 0"]
impl crate::Resettable for CONTROL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CMDTM` reader"]
pub struct R(crate::R<CMDTM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDTM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDTM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDTM` writer"]
pub struct W(crate::W<CMDTM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDTM_SPEC>;
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
impl From<crate::W<CMDTM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDTM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TM_BLKCNT_EN` reader - Enable block counter"]
pub type TM_BLKCNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TM_BLKCNT_EN` writer - Enable block counter"]
pub type TM_BLKCNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTM_SPEC, bool, O>;
#[doc = "Field `TM_AUTO_CMD_EN` reader - Command after completion"]
pub type TM_AUTO_CMD_EN_R = crate::FieldReader<u8, TM_AUTO_CMD_EN_A>;
#[doc = "Command after completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TM_AUTO_CMD_EN_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    CMD12 = 1,
    #[doc = "2: `10`"]
    CMD23 = 2,
}
impl From<TM_AUTO_CMD_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: TM_AUTO_CMD_EN_A) -> Self {
        variant as _
    }
}
impl TM_AUTO_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TM_AUTO_CMD_EN_A> {
        match self.bits {
            0 => Some(TM_AUTO_CMD_EN_A::NONE),
            1 => Some(TM_AUTO_CMD_EN_A::CMD12),
            2 => Some(TM_AUTO_CMD_EN_A::CMD23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::NONE
    }
    #[doc = "Checks if the value of the field is `CMD12`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::CMD12
    }
    #[doc = "Checks if the value of the field is `CMD23`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::CMD23
    }
}
#[doc = "Field `TM_AUTO_CMD_EN` writer - Command after completion"]
pub type TM_AUTO_CMD_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMDTM_SPEC, u8, TM_AUTO_CMD_EN_A, 2, O>;
impl<'a, const O: u8> TM_AUTO_CMD_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TM_AUTO_CMD_EN_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut W {
        self.variant(TM_AUTO_CMD_EN_A::CMD12)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut W {
        self.variant(TM_AUTO_CMD_EN_A::CMD23)
    }
}
#[doc = "Field `TM_DAT_DIR` reader - Direction of data transfer"]
pub type TM_DAT_DIR_R = crate::BitReader<TM_DAT_DIR_A>;
#[doc = "Direction of data transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM_DAT_DIR_A {
    #[doc = "0: `0`"]
    HOST_TO_CARD = 0,
    #[doc = "1: `1`"]
    CARD_TO_HOST = 1,
}
impl From<TM_DAT_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: TM_DAT_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TM_DAT_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM_DAT_DIR_A {
        match self.bits {
            false => TM_DAT_DIR_A::HOST_TO_CARD,
            true => TM_DAT_DIR_A::CARD_TO_HOST,
        }
    }
    #[doc = "Checks if the value of the field is `HOST_TO_CARD`"]
    #[inline(always)]
    pub fn is_host_to_card(&self) -> bool {
        *self == TM_DAT_DIR_A::HOST_TO_CARD
    }
    #[doc = "Checks if the value of the field is `CARD_TO_HOST`"]
    #[inline(always)]
    pub fn is_card_to_host(&self) -> bool {
        *self == TM_DAT_DIR_A::CARD_TO_HOST
    }
}
#[doc = "Field `TM_DAT_DIR` writer - Direction of data transfer"]
pub type TM_DAT_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTM_SPEC, TM_DAT_DIR_A, O>;
impl<'a, const O: u8> TM_DAT_DIR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn host_to_card(self) -> &'a mut W {
        self.variant(TM_DAT_DIR_A::HOST_TO_CARD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn card_to_host(self) -> &'a mut W {
        self.variant(TM_DAT_DIR_A::CARD_TO_HOST)
    }
}
#[doc = "Field `TM_MULTI_BLOCK` reader - Type of data transfer"]
pub type TM_MULTI_BLOCK_R = crate::BitReader<TM_MULTI_BLOCK_A>;
#[doc = "Type of data transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM_MULTI_BLOCK_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    MULTIPLE = 1,
}
impl From<TM_MULTI_BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: TM_MULTI_BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl TM_MULTI_BLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM_MULTI_BLOCK_A {
        match self.bits {
            false => TM_MULTI_BLOCK_A::SINGLE,
            true => TM_MULTI_BLOCK_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TM_MULTI_BLOCK_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == TM_MULTI_BLOCK_A::MULTIPLE
    }
}
#[doc = "Field `TM_MULTI_BLOCK` writer - Type of data transfer"]
pub type TM_MULTI_BLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMDTM_SPEC, TM_MULTI_BLOCK_A, O>;
impl<'a, const O: u8> TM_MULTI_BLOCK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TM_MULTI_BLOCK_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TM_MULTI_BLOCK_A::MULTIPLE)
    }
}
#[doc = "Field `CMD_RSPNS_TYPE` reader - Type of expected response"]
pub type CMD_RSPNS_TYPE_R = crate::FieldReader<u8, RESPONSE_A>;
#[doc = "Type of expected response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESPONSE_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    _136BITS = 1,
    #[doc = "2: `10`"]
    _48BITS = 2,
    #[doc = "3: `11`"]
    _48BITS_USING_BUSY = 3,
}
impl From<RESPONSE_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPONSE_A) -> Self {
        variant as _
    }
}
impl CMD_RSPNS_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPONSE_A {
        match self.bits {
            0 => RESPONSE_A::NONE,
            1 => RESPONSE_A::_136BITS,
            2 => RESPONSE_A::_48BITS,
            3 => RESPONSE_A::_48BITS_USING_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESPONSE_A::NONE
    }
    #[doc = "Checks if the value of the field is `_136BITS`"]
    #[inline(always)]
    pub fn is_136bits(&self) -> bool {
        *self == RESPONSE_A::_136BITS
    }
    #[doc = "Checks if the value of the field is `_48BITS`"]
    #[inline(always)]
    pub fn is_48bits(&self) -> bool {
        *self == RESPONSE_A::_48BITS
    }
    #[doc = "Checks if the value of the field is `_48BITS_USING_BUSY`"]
    #[inline(always)]
    pub fn is_48bits_using_busy(&self) -> bool {
        *self == RESPONSE_A::_48BITS_USING_BUSY
    }
}
#[doc = "Field `CMD_RSPNS_TYPE` writer - Type of expected response"]
pub type CMD_RSPNS_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMDTM_SPEC, u8, RESPONSE_A, 2, O>;
impl<'a, const O: u8> CMD_RSPNS_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RESPONSE_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _136bits(self) -> &'a mut W {
        self.variant(RESPONSE_A::_136BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _48bits(self) -> &'a mut W {
        self.variant(RESPONSE_A::_48BITS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _48bits_using_busy(self) -> &'a mut W {
        self.variant(RESPONSE_A::_48BITS_USING_BUSY)
    }
}
#[doc = "Field `CMD_CRCCHK_EN` reader - Check the responses CRC"]
pub type CMD_CRCCHK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRCCHK_EN` writer - Check the responses CRC"]
pub type CMD_CRCCHK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTM_SPEC, bool, O>;
#[doc = "Field `CMD_IXCHK_EN` reader - Check that the response has the same command index"]
pub type CMD_IXCHK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_IXCHK_EN` writer - Check that the response has the same command index"]
pub type CMD_IXCHK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTM_SPEC, bool, O>;
#[doc = "Field `CMD_ISDATA` reader - Command involves data"]
pub type CMD_ISDATA_R = crate::BitReader<bool>;
#[doc = "Field `CMD_ISDATA` writer - Command involves data"]
pub type CMD_ISDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDTM_SPEC, bool, O>;
#[doc = "Field `CMD_TYPE` reader - Type of command to be issued"]
pub type CMD_TYPE_R = crate::FieldReader<u8, CMD_TYPE_A>;
#[doc = "Type of command to be issued\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_TYPE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SUSPEND = 1,
    #[doc = "2: `10`"]
    RESUME = 2,
    #[doc = "3: `11`"]
    ABORT = 3,
}
impl From<CMD_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_TYPE_A) -> Self {
        variant as _
    }
}
impl CMD_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_TYPE_A {
        match self.bits {
            0 => CMD_TYPE_A::NORMAL,
            1 => CMD_TYPE_A::SUSPEND,
            2 => CMD_TYPE_A::RESUME,
            3 => CMD_TYPE_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMD_TYPE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMD_TYPE_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMD_TYPE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMD_TYPE_A::ABORT
    }
}
#[doc = "Field `CMD_TYPE` writer - Type of command to be issued"]
pub type CMD_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMDTM_SPEC, u8, CMD_TYPE_A, 2, O>;
impl<'a, const O: u8> CMD_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::SUSPEND)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::RESUME)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMD_TYPE_A::ABORT)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command index to be issued"]
pub type CMD_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_INDEX` writer - Command index to be issued"]
pub type CMD_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDTM_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 1 - Enable block counter"]
    #[inline(always)]
    pub fn tm_blkcnt_en(&self) -> TM_BLKCNT_EN_R {
        TM_BLKCNT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Command after completion"]
    #[inline(always)]
    pub fn tm_auto_cmd_en(&self) -> TM_AUTO_CMD_EN_R {
        TM_AUTO_CMD_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction of data transfer"]
    #[inline(always)]
    pub fn tm_dat_dir(&self) -> TM_DAT_DIR_R {
        TM_DAT_DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Type of data transfer"]
    #[inline(always)]
    pub fn tm_multi_block(&self) -> TM_MULTI_BLOCK_R {
        TM_MULTI_BLOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Type of expected response"]
    #[inline(always)]
    pub fn cmd_rspns_type(&self) -> CMD_RSPNS_TYPE_R {
        CMD_RSPNS_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Check the responses CRC"]
    #[inline(always)]
    pub fn cmd_crcchk_en(&self) -> CMD_CRCCHK_EN_R {
        CMD_CRCCHK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Check that the response has the same command index"]
    #[inline(always)]
    pub fn cmd_ixchk_en(&self) -> CMD_IXCHK_EN_R {
        CMD_IXCHK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Command involves data"]
    #[inline(always)]
    pub fn cmd_isdata(&self) -> CMD_ISDATA_R {
        CMD_ISDATA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Type of command to be issued"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command index to be issued"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable block counter"]
    #[inline(always)]
    #[must_use]
    pub fn tm_blkcnt_en(&mut self) -> TM_BLKCNT_EN_W<1> {
        TM_BLKCNT_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Command after completion"]
    #[inline(always)]
    #[must_use]
    pub fn tm_auto_cmd_en(&mut self) -> TM_AUTO_CMD_EN_W<2> {
        TM_AUTO_CMD_EN_W::new(self)
    }
    #[doc = "Bit 4 - Direction of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_dat_dir(&mut self) -> TM_DAT_DIR_W<4> {
        TM_DAT_DIR_W::new(self)
    }
    #[doc = "Bit 5 - Type of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_multi_block(&mut self) -> TM_MULTI_BLOCK_W<5> {
        TM_MULTI_BLOCK_W::new(self)
    }
    #[doc = "Bits 16:17 - Type of expected response"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_rspns_type(&mut self) -> CMD_RSPNS_TYPE_W<16> {
        CMD_RSPNS_TYPE_W::new(self)
    }
    #[doc = "Bit 19 - Check the responses CRC"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crcchk_en(&mut self) -> CMD_CRCCHK_EN_W<19> {
        CMD_CRCCHK_EN_W::new(self)
    }
    #[doc = "Bit 20 - Check that the response has the same command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ixchk_en(&mut self) -> CMD_IXCHK_EN_W<20> {
        CMD_IXCHK_EN_W::new(self)
    }
    #[doc = "Bit 21 - Command involves data"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_isdata(&mut self) -> CMD_ISDATA_W<21> {
        CMD_ISDATA_W::new(self)
    }
    #[doc = "Bits 22:23 - Type of command to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<22> {
        CMD_TYPE_W::new(self)
    }
    #[doc = "Bits 24:29 - Command index to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<24> {
        CMD_INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Issue commands to the card\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdtm](index.html) module"]
pub struct CMDTM_SPEC;
impl crate::RegisterSpec for CMDTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdtm::R](R) reader structure"]
impl crate::Readable for CMDTM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdtm::W](W) writer structure"]
impl crate::Writable for CMDTM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDTM to value 0"]
impl crate::Resettable for CMDTM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CMDTM` reader"]
pub type R = crate::R<CMDTM_SPEC>;
#[doc = "Register `CMDTM` writer"]
pub type W = crate::W<CMDTM_SPEC>;
#[doc = "Field `TM_BLKCNT_EN` reader - Enable block counter"]
pub type TM_BLKCNT_EN_R = crate::BitReader;
#[doc = "Field `TM_BLKCNT_EN` writer - Enable block counter"]
pub type TM_BLKCNT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_AUTO_CMD_EN` reader - Command after completion"]
pub type TM_AUTO_CMD_EN_R = crate::FieldReader<TM_AUTO_CMD_EN_A>;
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
impl crate::FieldSpec for TM_AUTO_CMD_EN_A {
    type Ux = u8;
}
impl TM_AUTO_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TM_AUTO_CMD_EN_A> {
        match self.bits {
            0 => Some(TM_AUTO_CMD_EN_A::NONE),
            1 => Some(TM_AUTO_CMD_EN_A::CMD12),
            2 => Some(TM_AUTO_CMD_EN_A::CMD23),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::NONE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::CMD12
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == TM_AUTO_CMD_EN_A::CMD23
    }
}
#[doc = "Field `TM_AUTO_CMD_EN` writer - Command after completion"]
pub type TM_AUTO_CMD_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TM_AUTO_CMD_EN_A>;
impl<'a, REG> TM_AUTO_CMD_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TM_AUTO_CMD_EN_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut crate::W<REG> {
        self.variant(TM_AUTO_CMD_EN_A::CMD12)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TM_DAT_DIR_A {
        match self.bits {
            false => TM_DAT_DIR_A::HOST_TO_CARD,
            true => TM_DAT_DIR_A::CARD_TO_HOST,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_host_to_card(&self) -> bool {
        *self == TM_DAT_DIR_A::HOST_TO_CARD
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_card_to_host(&self) -> bool {
        *self == TM_DAT_DIR_A::CARD_TO_HOST
    }
}
#[doc = "Field `TM_DAT_DIR` writer - Direction of data transfer"]
pub type TM_DAT_DIR_W<'a, REG> = crate::BitWriter<'a, REG, TM_DAT_DIR_A>;
impl<'a, REG> TM_DAT_DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn host_to_card(self) -> &'a mut crate::W<REG> {
        self.variant(TM_DAT_DIR_A::HOST_TO_CARD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn card_to_host(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> TM_MULTI_BLOCK_A {
        match self.bits {
            false => TM_MULTI_BLOCK_A::SINGLE,
            true => TM_MULTI_BLOCK_A::MULTIPLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TM_MULTI_BLOCK_A::SINGLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == TM_MULTI_BLOCK_A::MULTIPLE
    }
}
#[doc = "Field `TM_MULTI_BLOCK` writer - Type of data transfer"]
pub type TM_MULTI_BLOCK_W<'a, REG> = crate::BitWriter<'a, REG, TM_MULTI_BLOCK_A>;
impl<'a, REG> TM_MULTI_BLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TM_MULTI_BLOCK_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(TM_MULTI_BLOCK_A::MULTIPLE)
    }
}
#[doc = "Field `CMD_RSPNS_TYPE` reader - Type of expected response"]
pub type CMD_RSPNS_TYPE_R = crate::FieldReader<RESPONSE_A>;
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
impl crate::FieldSpec for RESPONSE_A {
    type Ux = u8;
}
impl CMD_RSPNS_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESPONSE_A {
        match self.bits {
            0 => RESPONSE_A::NONE,
            1 => RESPONSE_A::_136BITS,
            2 => RESPONSE_A::_48BITS,
            3 => RESPONSE_A::_48BITS_USING_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESPONSE_A::NONE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_136bits(&self) -> bool {
        *self == RESPONSE_A::_136BITS
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_48bits(&self) -> bool {
        *self == RESPONSE_A::_48BITS
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_48bits_using_busy(&self) -> bool {
        *self == RESPONSE_A::_48BITS_USING_BUSY
    }
}
#[doc = "Field `CMD_RSPNS_TYPE` writer - Type of expected response"]
pub type CMD_RSPNS_TYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RESPONSE_A>;
impl<'a, REG> CMD_RSPNS_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RESPONSE_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _136bits(self) -> &'a mut crate::W<REG> {
        self.variant(RESPONSE_A::_136BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _48bits(self) -> &'a mut crate::W<REG> {
        self.variant(RESPONSE_A::_48BITS)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _48bits_using_busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESPONSE_A::_48BITS_USING_BUSY)
    }
}
#[doc = "Field `CMD_CRCCHK_EN` reader - Check the responses CRC"]
pub type CMD_CRCCHK_EN_R = crate::BitReader;
#[doc = "Field `CMD_CRCCHK_EN` writer - Check the responses CRC"]
pub type CMD_CRCCHK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IXCHK_EN` reader - Check that the response has the same command index"]
pub type CMD_IXCHK_EN_R = crate::BitReader;
#[doc = "Field `CMD_IXCHK_EN` writer - Check that the response has the same command index"]
pub type CMD_IXCHK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_ISDATA` reader - Command involves data"]
pub type CMD_ISDATA_R = crate::BitReader;
#[doc = "Field `CMD_ISDATA` writer - Command involves data"]
pub type CMD_ISDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TYPE` reader - Type of command to be issued"]
pub type CMD_TYPE_R = crate::FieldReader<CMD_TYPE_A>;
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
impl crate::FieldSpec for CMD_TYPE_A {
    type Ux = u8;
}
impl CMD_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_TYPE_A {
        match self.bits {
            0 => CMD_TYPE_A::NORMAL,
            1 => CMD_TYPE_A::SUSPEND,
            2 => CMD_TYPE_A::RESUME,
            3 => CMD_TYPE_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMD_TYPE_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMD_TYPE_A::SUSPEND
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMD_TYPE_A::RESUME
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMD_TYPE_A::ABORT
    }
}
#[doc = "Field `CMD_TYPE` writer - Type of command to be issued"]
pub type CMD_TYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CMD_TYPE_A>;
impl<'a, REG> CMD_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TYPE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TYPE_A::SUSPEND)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TYPE_A::RESUME)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_TYPE_A::ABORT)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command index to be issued"]
pub type CMD_INDEX_R = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command index to be issued"]
pub type CMD_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDTM")
            .field("cmd_index", &format_args!("{}", self.cmd_index().bits()))
            .field("cmd_type", &format_args!("{}", self.cmd_type().bits()))
            .field("cmd_isdata", &format_args!("{}", self.cmd_isdata().bit()))
            .field(
                "cmd_ixchk_en",
                &format_args!("{}", self.cmd_ixchk_en().bit()),
            )
            .field(
                "cmd_crcchk_en",
                &format_args!("{}", self.cmd_crcchk_en().bit()),
            )
            .field(
                "cmd_rspns_type",
                &format_args!("{}", self.cmd_rspns_type().bits()),
            )
            .field(
                "tm_multi_block",
                &format_args!("{}", self.tm_multi_block().bit()),
            )
            .field("tm_dat_dir", &format_args!("{}", self.tm_dat_dir().bit()))
            .field(
                "tm_auto_cmd_en",
                &format_args!("{}", self.tm_auto_cmd_en().bits()),
            )
            .field(
                "tm_blkcnt_en",
                &format_args!("{}", self.tm_blkcnt_en().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMDTM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Enable block counter"]
    #[inline(always)]
    #[must_use]
    pub fn tm_blkcnt_en(&mut self) -> TM_BLKCNT_EN_W<CMDTM_SPEC> {
        TM_BLKCNT_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Command after completion"]
    #[inline(always)]
    #[must_use]
    pub fn tm_auto_cmd_en(&mut self) -> TM_AUTO_CMD_EN_W<CMDTM_SPEC> {
        TM_AUTO_CMD_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Direction of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_dat_dir(&mut self) -> TM_DAT_DIR_W<CMDTM_SPEC> {
        TM_DAT_DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Type of data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tm_multi_block(&mut self) -> TM_MULTI_BLOCK_W<CMDTM_SPEC> {
        TM_MULTI_BLOCK_W::new(self, 5)
    }
    #[doc = "Bits 16:17 - Type of expected response"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_rspns_type(&mut self) -> CMD_RSPNS_TYPE_W<CMDTM_SPEC> {
        CMD_RSPNS_TYPE_W::new(self, 16)
    }
    #[doc = "Bit 19 - Check the responses CRC"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crcchk_en(&mut self) -> CMD_CRCCHK_EN_W<CMDTM_SPEC> {
        CMD_CRCCHK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Check that the response has the same command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ixchk_en(&mut self) -> CMD_IXCHK_EN_W<CMDTM_SPEC> {
        CMD_IXCHK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Command involves data"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_isdata(&mut self) -> CMD_ISDATA_W<CMDTM_SPEC> {
        CMD_ISDATA_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - Type of command to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<CMDTM_SPEC> {
        CMD_TYPE_W::new(self, 22)
    }
    #[doc = "Bits 24:29 - Command index to be issued"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<CMDTM_SPEC> {
        CMD_INDEX_W::new(self, 24)
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
#[doc = "Issue commands to the card\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdtm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdtm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDTM_SPEC;
impl crate::RegisterSpec for CMDTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdtm::R`](R) reader structure"]
impl crate::Readable for CMDTM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdtm::W`](W) writer structure"]
impl crate::Writable for CMDTM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDTM to value 0"]
impl crate::Resettable for CMDTM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

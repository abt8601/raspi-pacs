#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `CMD_INHIBIT` reader - Command line still in use"]
pub type CMD_INHIBIT_R = crate::BitReader;
#[doc = "Field `CMD_INHIBIT` writer - Command line still in use"]
pub type CMD_INHIBIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAT_INHIBIT` reader - Data lines still in use"]
pub type DAT_INHIBIT_R = crate::BitReader;
#[doc = "Field `DAT_INHIBIT` writer - Data lines still in use"]
pub type DAT_INHIBIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAT_ACTIVE` reader - At least one data line is active"]
pub type DAT_ACTIVE_R = crate::BitReader;
#[doc = "Field `DAT_ACTIVE` writer - At least one data line is active"]
pub type DAT_ACTIVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRITE_TRANSFER` reader - Write transfer is active"]
pub type WRITE_TRANSFER_R = crate::BitReader;
#[doc = "Field `WRITE_TRANSFER` writer - Write transfer is active"]
pub type WRITE_TRANSFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `READ_TRANSFER` reader - Read transfer is active"]
pub type READ_TRANSFER_R = crate::BitReader;
#[doc = "Field `READ_TRANSFER` writer - Read transfer is active"]
pub type READ_TRANSFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFFER_WRITE_ENABLE` reader - The buffer has space for new data"]
pub type BUFFER_WRITE_ENABLE_R = crate::BitReader;
#[doc = "Field `BUFFER_WRITE_ENABLE` writer - The buffer has space for new data"]
pub type BUFFER_WRITE_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFFER_READ_ENABLE` reader - New data is available to read"]
pub type BUFFER_READ_ENABLE_R = crate::BitReader;
#[doc = "Field `BUFFER_READ_ENABLE` writer - New data is available to read"]
pub type BUFFER_READ_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAT_LEVEL0` reader - Value of DAT\\[3:0\\]"]
pub type DAT_LEVEL0_R = crate::FieldReader;
#[doc = "Field `DAT_LEVEL0` writer - Value of DAT\\[3:0\\]"]
pub type DAT_LEVEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CMD_LEVEL` reader - Value of CMD"]
pub type CMD_LEVEL_R = crate::BitReader;
#[doc = "Field `CMD_LEVEL` writer - Value of CMD"]
pub type CMD_LEVEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAT_LEVEL1` reader - Value of DAT\\[7:4\\]"]
pub type DAT_LEVEL1_R = crate::FieldReader;
#[doc = "Field `DAT_LEVEL1` writer - Value of DAT\\[7:4\\]"]
pub type DAT_LEVEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    pub fn cmd_inhibit(&self) -> CMD_INHIBIT_R {
        CMD_INHIBIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    pub fn dat_inhibit(&self) -> DAT_INHIBIT_R {
        DAT_INHIBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    pub fn dat_active(&self) -> DAT_ACTIVE_R {
        DAT_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    pub fn write_transfer(&self) -> WRITE_TRANSFER_R {
        WRITE_TRANSFER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    pub fn read_transfer(&self) -> READ_TRANSFER_R {
        READ_TRANSFER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    pub fn buffer_write_enable(&self) -> BUFFER_WRITE_ENABLE_R {
        BUFFER_WRITE_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    pub fn buffer_read_enable(&self) -> BUFFER_READ_ENABLE_R {
        BUFFER_READ_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    pub fn dat_level0(&self) -> DAT_LEVEL0_R {
        DAT_LEVEL0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    pub fn cmd_level(&self) -> CMD_LEVEL_R {
        CMD_LEVEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    pub fn dat_level1(&self) -> DAT_LEVEL1_R {
        DAT_LEVEL1_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("dat_level1", &format_args!("{}", self.dat_level1().bits()))
            .field("cmd_level", &format_args!("{}", self.cmd_level().bit()))
            .field("dat_level0", &format_args!("{}", self.dat_level0().bits()))
            .field(
                "buffer_read_enable",
                &format_args!("{}", self.buffer_read_enable().bit()),
            )
            .field(
                "buffer_write_enable",
                &format_args!("{}", self.buffer_write_enable().bit()),
            )
            .field(
                "read_transfer",
                &format_args!("{}", self.read_transfer().bit()),
            )
            .field(
                "write_transfer",
                &format_args!("{}", self.write_transfer().bit()),
            )
            .field("dat_active", &format_args!("{}", self.dat_active().bit()))
            .field("dat_inhibit", &format_args!("{}", self.dat_inhibit().bit()))
            .field("cmd_inhibit", &format_args!("{}", self.cmd_inhibit().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Command line still in use"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_inhibit(&mut self) -> CMD_INHIBIT_W<STATUS_SPEC, 0> {
        CMD_INHIBIT_W::new(self)
    }
    #[doc = "Bit 1 - Data lines still in use"]
    #[inline(always)]
    #[must_use]
    pub fn dat_inhibit(&mut self) -> DAT_INHIBIT_W<STATUS_SPEC, 1> {
        DAT_INHIBIT_W::new(self)
    }
    #[doc = "Bit 2 - At least one data line is active"]
    #[inline(always)]
    #[must_use]
    pub fn dat_active(&mut self) -> DAT_ACTIVE_W<STATUS_SPEC, 2> {
        DAT_ACTIVE_W::new(self)
    }
    #[doc = "Bit 8 - Write transfer is active"]
    #[inline(always)]
    #[must_use]
    pub fn write_transfer(&mut self) -> WRITE_TRANSFER_W<STATUS_SPEC, 8> {
        WRITE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 9 - Read transfer is active"]
    #[inline(always)]
    #[must_use]
    pub fn read_transfer(&mut self) -> READ_TRANSFER_W<STATUS_SPEC, 9> {
        READ_TRANSFER_W::new(self)
    }
    #[doc = "Bit 10 - The buffer has space for new data"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_write_enable(&mut self) -> BUFFER_WRITE_ENABLE_W<STATUS_SPEC, 10> {
        BUFFER_WRITE_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - New data is available to read"]
    #[inline(always)]
    #[must_use]
    pub fn buffer_read_enable(&mut self) -> BUFFER_READ_ENABLE_W<STATUS_SPEC, 11> {
        BUFFER_READ_ENABLE_W::new(self)
    }
    #[doc = "Bits 20:23 - Value of DAT\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dat_level0(&mut self) -> DAT_LEVEL0_W<STATUS_SPEC, 20> {
        DAT_LEVEL0_W::new(self)
    }
    #[doc = "Bit 24 - Value of CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_level(&mut self) -> CMD_LEVEL_W<STATUS_SPEC, 24> {
        CMD_LEVEL_W::new(self)
    }
    #[doc = "Bits 25:28 - Value of DAT\\[7:4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dat_level1(&mut self) -> DAT_LEVEL1_W<STATUS_SPEC, 25> {
        DAT_LEVEL1_W::new(self)
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
#[doc = "Status info for debugging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

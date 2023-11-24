#[doc = "Register `CONTROL1` reader"]
pub type R = crate::R<CONTROL1_SPEC>;
#[doc = "Register `CONTROL1` writer"]
pub type W = crate::W<CONTROL1_SPEC>;
#[doc = "Field `CLK_INTLEN` reader - Enable internal clock"]
pub type CLK_INTLEN_R = crate::BitReader;
#[doc = "Field `CLK_INTLEN` writer - Enable internal clock"]
pub type CLK_INTLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_STABLE` reader - SD Clock stable"]
pub type CLK_STABLE_R = crate::BitReader;
#[doc = "Field `CLK_EN` reader - SD Clock enable"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - SD Clock enable"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GENSEL` reader - Mode of clock generation"]
pub type CLK_GENSEL_R = crate::BitReader<CLK_GENSEL_A>;
#[doc = "Mode of clock generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_GENSEL_A {
    #[doc = "0: `0`"]
    DIVIDED = 0,
    #[doc = "1: `1`"]
    PROGRAMMABLE = 1,
}
impl From<CLK_GENSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_GENSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_GENSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_GENSEL_A {
        match self.bits {
            false => CLK_GENSEL_A::DIVIDED,
            true => CLK_GENSEL_A::PROGRAMMABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_divided(&self) -> bool {
        *self == CLK_GENSEL_A::DIVIDED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_programmable(&self) -> bool {
        *self == CLK_GENSEL_A::PROGRAMMABLE
    }
}
#[doc = "Field `CLK_GENSEL` writer - Mode of clock generation"]
pub type CLK_GENSEL_W<'a, REG> = crate::BitWriter<'a, REG, CLK_GENSEL_A>;
impl<'a, REG> CLK_GENSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn divided(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_GENSEL_A::DIVIDED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn programmable(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_GENSEL_A::PROGRAMMABLE)
    }
}
#[doc = "Field `CLK_FREQ_MS2` reader - Clock base divider MSBs"]
pub type CLK_FREQ_MS2_R = crate::FieldReader;
#[doc = "Field `CLK_FREQ_MS2` writer - Clock base divider MSBs"]
pub type CLK_FREQ_MS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_FREQ8` reader - Clock base divider LSB"]
pub type CLK_FREQ8_R = crate::FieldReader;
#[doc = "Field `CLK_FREQ8` writer - Clock base divider LSB"]
pub type CLK_FREQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA_TOUNIT` reader - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
pub type DATA_TOUNIT_R = crate::FieldReader;
#[doc = "Field `DATA_TOUNIT` writer - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
pub type DATA_TOUNIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRST_HC` reader - Reset the complete host circuit"]
pub type SRST_HC_R = crate::BitReader;
#[doc = "Field `SRST_HC` writer - Reset the complete host circuit"]
pub type SRST_HC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST_CMD` reader - Reset the command handling circuit"]
pub type SRST_CMD_R = crate::BitReader;
#[doc = "Field `SRST_CMD` writer - Reset the command handling circuit"]
pub type SRST_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST_DATA` reader - Reset the data handling circuit"]
pub type SRST_DATA_R = crate::BitReader;
#[doc = "Field `SRST_DATA` writer - Reset the data handling circuit"]
pub type SRST_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable internal clock"]
    #[inline(always)]
    pub fn clk_intlen(&self) -> CLK_INTLEN_R {
        CLK_INTLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD Clock stable"]
    #[inline(always)]
    pub fn clk_stable(&self) -> CLK_STABLE_R {
        CLK_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode of clock generation"]
    #[inline(always)]
    pub fn clk_gensel(&self) -> CLK_GENSEL_R {
        CLK_GENSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Clock base divider MSBs"]
    #[inline(always)]
    pub fn clk_freq_ms2(&self) -> CLK_FREQ_MS2_R {
        CLK_FREQ_MS2_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Clock base divider LSB"]
    #[inline(always)]
    pub fn clk_freq8(&self) -> CLK_FREQ8_R {
        CLK_FREQ8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
    #[inline(always)]
    pub fn data_tounit(&self) -> DATA_TOUNIT_R {
        DATA_TOUNIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Reset the complete host circuit"]
    #[inline(always)]
    pub fn srst_hc(&self) -> SRST_HC_R {
        SRST_HC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset the command handling circuit"]
    #[inline(always)]
    pub fn srst_cmd(&self) -> SRST_CMD_R {
        SRST_CMD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset the data handling circuit"]
    #[inline(always)]
    pub fn srst_data(&self) -> SRST_DATA_R {
        SRST_DATA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL1")
            .field("srst_data", &format_args!("{}", self.srst_data().bit()))
            .field("srst_cmd", &format_args!("{}", self.srst_cmd().bit()))
            .field("srst_hc", &format_args!("{}", self.srst_hc().bit()))
            .field(
                "data_tounit",
                &format_args!("{}", self.data_tounit().bits()),
            )
            .field("clk_freq8", &format_args!("{}", self.clk_freq8().bits()))
            .field(
                "clk_freq_ms2",
                &format_args!("{}", self.clk_freq_ms2().bits()),
            )
            .field("clk_gensel", &format_args!("{}", self.clk_gensel().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("clk_stable", &format_args!("{}", self.clk_stable().bit()))
            .field("clk_intlen", &format_args!("{}", self.clk_intlen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CONTROL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable internal clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intlen(&mut self) -> CLK_INTLEN_W<CONTROL1_SPEC> {
        CLK_INTLEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONTROL1_SPEC> {
        CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Mode of clock generation"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gensel(&mut self) -> CLK_GENSEL_W<CONTROL1_SPEC> {
        CLK_GENSEL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Clock base divider MSBs"]
    #[inline(always)]
    #[must_use]
    pub fn clk_freq_ms2(&mut self) -> CLK_FREQ_MS2_W<CONTROL1_SPEC> {
        CLK_FREQ_MS2_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Clock base divider LSB"]
    #[inline(always)]
    #[must_use]
    pub fn clk_freq8(&mut self) -> CLK_FREQ8_W<CONTROL1_SPEC> {
        CLK_FREQ8_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tounit(&mut self) -> DATA_TOUNIT_W<CONTROL1_SPEC> {
        DATA_TOUNIT_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reset the complete host circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_hc(&mut self) -> SRST_HC_W<CONTROL1_SPEC> {
        SRST_HC_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reset the command handling circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_cmd(&mut self) -> SRST_CMD_W<CONTROL1_SPEC> {
        SRST_CMD_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset the data handling circuit"]
    #[inline(always)]
    #[must_use]
    pub fn srst_data(&mut self) -> SRST_DATA_W<CONTROL1_SPEC> {
        SRST_DATA_W::new(self, 26)
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
#[doc = "Configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL1_SPEC;
impl crate::RegisterSpec for CONTROL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control1::R`](R) reader structure"]
impl crate::Readable for CONTROL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control1::W`](W) writer structure"]
impl crate::Writable for CONTROL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL1 to value 0"]
impl crate::Resettable for CONTROL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

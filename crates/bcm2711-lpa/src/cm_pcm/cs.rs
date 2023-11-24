#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `SRC` reader - Clock source"]
pub type SRC_R = crate::FieldReader<SRC_A>;
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "1: `1`"]
    XOSC = 1,
    #[doc = "2: `10`"]
    TEST0 = 2,
    #[doc = "3: `11`"]
    TEST1 = 3,
    #[doc = "4: `100`"]
    PLLA = 4,
    #[doc = "5: `101`"]
    PLLB = 5,
    #[doc = "6: `110`"]
    PLLC = 6,
    #[doc = "7: `111`"]
    HDMI = 7,
    #[doc = "0: `0`"]
    GND = 0,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRC_A {
    type Ux = u8;
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRC_A {
        match self.bits {
            1 => SRC_A::XOSC,
            2 => SRC_A::TEST0,
            3 => SRC_A::TEST1,
            4 => SRC_A::PLLA,
            5 => SRC_A::PLLB,
            6 => SRC_A::PLLC,
            7 => SRC_A::HDMI,
            _ => SRC_A::GND,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRC_A::XOSC
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_test0(&self) -> bool {
        *self == SRC_A::TEST0
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_test1(&self) -> bool {
        *self == SRC_A::TEST1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_plla(&self) -> bool {
        *self == SRC_A::PLLA
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_pllb(&self) -> bool {
        *self == SRC_A::PLLB
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_pllc(&self) -> bool {
        *self == SRC_A::PLLC
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_hdmi(&self) -> bool {
        *self == SRC_A::HDMI
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        matches!(self.variant(), SRC_A::GND)
    }
}
#[doc = "Field `SRC` writer - Clock source"]
pub type SRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, SRC_A>;
impl<'a, REG> SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::XOSC)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn test0(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::TEST0)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn test1(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::TEST1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn plla(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::PLLA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pllb(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::PLLB)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pllc(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::PLLC)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn hdmi(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::HDMI)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::GND)
    }
}
#[doc = "Field `ENAB` reader - Enable the clock generator. (Switch SRC first.)"]
pub type ENAB_R = crate::BitReader;
#[doc = "Field `ENAB` writer - Enable the clock generator. (Switch SRC first.)"]
pub type ENAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILL` reader - Stop and reset the generator"]
pub type KILL_R = crate::BitReader;
#[doc = "Field `KILL` writer - Stop and reset the generator"]
pub type KILL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Indicates the clock generator is running"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `FLIP` reader - Generate an edge on output. (For testing)"]
pub type FLIP_R = crate::BitReader;
#[doc = "Field `FLIP` writer - Generate an edge on output. (For testing)"]
pub type FLIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASH` reader - MASH control, stage count"]
pub type MASH_R = crate::FieldReader;
#[doc = "Field `MASH` writer - MASH control, stage count"]
pub type MASH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Password. Always 0x5a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PASSWD_AW {
    #[doc = "90: `1011010`"]
    PASSWD = 90,
}
impl From<PASSWD_AW> for u8 {
    #[inline(always)]
    fn from(variant: PASSWD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PASSWD_AW {
    type Ux = u8;
}
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PASSWD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PASSWD_AW>;
impl<'a, REG> PASSWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(PASSWD_AW::PASSWD)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    pub fn enab(&self) -> ENAB_R {
        ENAB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop and reset the generator"]
    #[inline(always)]
    pub fn kill(&self) -> KILL_R {
        KILL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the clock generator is running"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate an edge on output. (For testing)"]
    #[inline(always)]
    pub fn flip(&self) -> FLIP_R {
        FLIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - MASH control, stage count"]
    #[inline(always)]
    pub fn mash(&self) -> MASH_R {
        MASH_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("mash", &format_args!("{}", self.mash().bits()))
            .field("flip", &format_args!("{}", self.flip().bit()))
            .field("busy", &format_args!("{}", self.busy().bit()))
            .field("kill", &format_args!("{}", self.kill().bit()))
            .field("enab", &format_args!("{}", self.enab().bit()))
            .field("src", &format_args!("{}", self.src().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<CS_SPEC> {
        SRC_W::new(self, 0)
    }
    #[doc = "Bit 4 - Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> ENAB_W<CS_SPEC> {
        ENAB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop and reset the generator"]
    #[inline(always)]
    #[must_use]
    pub fn kill(&mut self) -> KILL_W<CS_SPEC> {
        KILL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Generate an edge on output. (For testing)"]
    #[inline(always)]
    #[must_use]
    pub fn flip(&mut self) -> FLIP_W<CS_SPEC> {
        FLIP_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - MASH control, stage count"]
    #[inline(always)]
    #[must_use]
    pub fn mash(&mut self) -> MASH_W<CS_SPEC> {
        MASH_W::new(self, 9)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<CS_SPEC> {
        PASSWD_W::new(self, 24)
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
#[doc = "Control / Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

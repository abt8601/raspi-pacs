#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Clock source"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
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
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            1 => Some(SRC_A::XOSC),
            2 => Some(SRC_A::TEST0),
            3 => Some(SRC_A::TEST1),
            4 => Some(SRC_A::PLLA),
            5 => Some(SRC_A::PLLB),
            6 => Some(SRC_A::PLLC),
            7 => Some(SRC_A::HDMI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `TEST0`"]
    #[inline(always)]
    pub fn is_test0(&self) -> bool {
        *self == SRC_A::TEST0
    }
    #[doc = "Checks if the value of the field is `TEST1`"]
    #[inline(always)]
    pub fn is_test1(&self) -> bool {
        *self == SRC_A::TEST1
    }
    #[doc = "Checks if the value of the field is `PLLA`"]
    #[inline(always)]
    pub fn is_plla(&self) -> bool {
        *self == SRC_A::PLLA
    }
    #[doc = "Checks if the value of the field is `PLLB`"]
    #[inline(always)]
    pub fn is_pllb(&self) -> bool {
        *self == SRC_A::PLLB
    }
    #[doc = "Checks if the value of the field is `PLLC`"]
    #[inline(always)]
    pub fn is_pllc(&self) -> bool {
        *self == SRC_A::PLLC
    }
    #[doc = "Checks if the value of the field is `HDMI`"]
    #[inline(always)]
    pub fn is_hdmi(&self) -> bool {
        *self == SRC_A::HDMI
    }
}
#[doc = "Field `SRC` writer - Clock source"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, SRC_A, 4, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SRC_A::XOSC)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn test0(self) -> &'a mut W {
        self.variant(SRC_A::TEST0)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn test1(self) -> &'a mut W {
        self.variant(SRC_A::TEST1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn plla(self) -> &'a mut W {
        self.variant(SRC_A::PLLA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pllb(self) -> &'a mut W {
        self.variant(SRC_A::PLLB)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pllc(self) -> &'a mut W {
        self.variant(SRC_A::PLLC)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn hdmi(self) -> &'a mut W {
        self.variant(SRC_A::HDMI)
    }
}
#[doc = "Field `ENAB` reader - Enable the clock generator. (Switch SRC first.)"]
pub type ENAB_R = crate::BitReader<bool>;
#[doc = "Field `ENAB` writer - Enable the clock generator. (Switch SRC first.)"]
pub type ENAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `KILL` reader - Stop and reset the generator"]
pub type KILL_R = crate::BitReader<bool>;
#[doc = "Field `KILL` writer - Stop and reset the generator"]
pub type KILL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - Indicates the clock generator is running"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FLIP` reader - Generate an edge on output. (For testing)"]
pub type FLIP_R = crate::BitReader<bool>;
#[doc = "Field `FLIP` writer - Generate an edge on output. (For testing)"]
pub type FLIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `MASH` reader - MASH control, stage count"]
pub type MASH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASH` writer - MASH control, stage count"]
pub type MASH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, u8, 2, O>;
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
#[doc = "Field `PASSWD` writer - Password. Always 0x5a"]
pub type PASSWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, PASSWD_AW, 8, O>;
impl<'a, const O: u8> PASSWD_W<'a, O> {
    #[doc = "`1011010`"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
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
impl W {
    #[doc = "Bits 0:3 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 4 - Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> ENAB_W<4> {
        ENAB_W::new(self)
    }
    #[doc = "Bit 5 - Stop and reset the generator"]
    #[inline(always)]
    #[must_use]
    pub fn kill(&mut self) -> KILL_W<5> {
        KILL_W::new(self)
    }
    #[doc = "Bit 8 - Generate an edge on output. (For testing)"]
    #[inline(always)]
    #[must_use]
    pub fn flip(&mut self) -> FLIP_W<8> {
        FLIP_W::new(self)
    }
    #[doc = "Bits 9:10 - MASH control, stage count"]
    #[inline(always)]
    #[must_use]
    pub fn mash(&mut self) -> MASH_W<9> {
        MASH_W::new(self)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<24> {
        PASSWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control / Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `DIVF` reader - Fractional part of divisor"]
pub type DIVF_R = crate::FieldReader<u16>;
#[doc = "Field `DIVF` writer - Fractional part of divisor"]
pub type DIVF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DIVI` reader - Integer part of divisor"]
pub type DIVI_R = crate::FieldReader<u16>;
#[doc = "Field `DIVI` writer - Integer part of divisor"]
pub type DIVI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
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
pub type PASSWD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, PASSWD_AW>;
impl<'a, REG, const O: u8> PASSWD_W<'a, REG, O>
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
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    pub fn divf(&self) -> DIVF_R {
        DIVF_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    pub fn divi(&self) -> DIVI_R {
        DIVI_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV")
            .field("divi", &format_args!("{}", self.divi().bits()))
            .field("divf", &format_args!("{}", self.divf().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional part of divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divf(&mut self) -> DIVF_W<DIV_SPEC, 0> {
        DIVF_W::new(self)
    }
    #[doc = "Bits 12:23 - Integer part of divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divi(&mut self) -> DIVI_W<DIV_SPEC, 12> {
        DIVI_W::new(self)
    }
    #[doc = "Bits 24:31 - Password. Always 0x5a"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<DIV_SPEC, 24> {
        PASSWD_W::new(self)
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
#[doc = "Clock divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

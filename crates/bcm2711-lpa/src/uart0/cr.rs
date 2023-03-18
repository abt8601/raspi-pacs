#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UARTEN` reader - UARTEN"]
pub type UARTEN_R = crate::BitReader<bool>;
#[doc = "Field `UARTEN` writer - UARTEN"]
pub type UARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SIREN` reader - SIREN"]
pub type SIREN_R = crate::BitReader<bool>;
#[doc = "Field `SIREN` writer - SIREN"]
pub type SIREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SIRLP` reader - SIRLP"]
pub type SIRLP_R = crate::BitReader<bool>;
#[doc = "Field `SIRLP` writer - SIRLP"]
pub type SIRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXE` reader - TXE"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - TXE"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXE` reader - RXE"]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - RXE"]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DTR` reader - DTR"]
pub type DTR_R = crate::BitReader<bool>;
#[doc = "Field `DTR` writer - DTR"]
pub type DTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RTS` reader - RTS"]
pub type RTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` writer - RTS"]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RTSEN` reader - RTSEN"]
pub type RTSEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSEN` writer - RTSEN"]
pub type RTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CTSEN` reader - CTSEN"]
pub type CTSEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSEN` writer - CTSEN"]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UARTEN"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UARTEN_W<0> {
        UARTEN_W::new(self)
    }
    #[doc = "Bit 1 - SIREN"]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SIREN_W<1> {
        SIREN_W::new(self)
    }
    #[doc = "Bit 2 - SIRLP"]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SIRLP_W<2> {
        SIRLP_W::new(self)
    }
    #[doc = "Bit 8 - TXE"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<8> {
        TXE_W::new(self)
    }
    #[doc = "Bit 9 - RXE"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<9> {
        RXE_W::new(self)
    }
    #[doc = "Bit 10 - DTR"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<10> {
        DTR_W::new(self)
    }
    #[doc = "Bit 11 - RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<11> {
        RTS_W::new(self)
    }
    #[doc = "Bit 14 - RTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<14> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 15 - CTSEN"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<15> {
        CTSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

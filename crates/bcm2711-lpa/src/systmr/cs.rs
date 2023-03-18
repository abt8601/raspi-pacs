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
#[doc = "Field `M0` reader - System timer match 0"]
pub type M0_R = crate::BitReader<bool>;
#[doc = "Field `M0` writer - System timer match 0"]
pub type M0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `M1` reader - System timer match 1"]
pub type M1_R = crate::BitReader<bool>;
#[doc = "Field `M1` writer - System timer match 1"]
pub type M1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `M2` reader - System timer match 2"]
pub type M2_R = crate::BitReader<bool>;
#[doc = "Field `M2` writer - System timer match 2"]
pub type M2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `M3` reader - System timer match 3"]
pub type M3_R = crate::BitReader<bool>;
#[doc = "Field `M3` writer - System timer match 3"]
pub type M3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    pub fn m2(&self) -> M2_R {
        M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    pub fn m3(&self) -> M3_R {
        M3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<0> {
        M0_W::new(self)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<1> {
        M1_W::new(self)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2_W<2> {
        M2_W::new(self)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3_W<3> {
        M3_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

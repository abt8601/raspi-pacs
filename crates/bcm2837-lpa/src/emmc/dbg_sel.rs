#[doc = "Register `DBG_SEL` reader"]
pub struct R(crate::R<DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SEL` writer"]
pub struct W(crate::W<DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SEL_SPEC>;
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
impl From<crate::W<DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECT` reader - "]
pub type SELECT_R = crate::BitReader<SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELECT_A {
    #[doc = "0: `0`"]
    RECEIVER_FIFO = 0,
    #[doc = "1: `1`"]
    OTHERS = 1,
}
impl From<SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECT_A {
        match self.bits {
            false => SELECT_A::RECEIVER_FIFO,
            true => SELECT_A::OTHERS,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVER_FIFO`"]
    #[inline(always)]
    pub fn is_receiver_fifo(&self) -> bool {
        *self == SELECT_A::RECEIVER_FIFO
    }
    #[doc = "Checks if the value of the field is `OTHERS`"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == SELECT_A::OTHERS
    }
}
#[doc = "Field `SELECT` writer - "]
pub type SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_SEL_SPEC, SELECT_A, O>;
impl<'a, const O: u8> SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receiver_fifo(self) -> &'a mut W {
        self.variant(SELECT_A::RECEIVER_FIFO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn others(self) -> &'a mut W {
        self.variant(SELECT_A::OTHERS)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "What submodules are accessed by the debug bus\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel](index.html) module"]
pub struct DBG_SEL_SPEC;
impl crate::RegisterSpec for DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sel::R](R) reader structure"]
impl crate::Readable for DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sel::W](W) writer structure"]
impl crate::Writable for DBG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_SEL to value 0"]
impl crate::Resettable for DBG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DBG_SEL` reader"]
pub type R = crate::R<DBG_SEL_SPEC>;
#[doc = "Register `DBG_SEL` writer"]
pub type W = crate::W<DBG_SEL_SPEC>;
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
    pub const fn variant(&self) -> SELECT_A {
        match self.bits {
            false => SELECT_A::RECEIVER_FIFO,
            true => SELECT_A::OTHERS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_receiver_fifo(&self) -> bool {
        *self == SELECT_A::RECEIVER_FIFO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == SELECT_A::OTHERS
    }
}
#[doc = "Field `SELECT` writer - "]
pub type SELECT_W<'a, REG> = crate::BitWriter<'a, REG, SELECT_A>;
impl<'a, REG> SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receiver_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(SELECT_A::RECEIVER_FIFO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_SEL")
            .field("select", &format_args!("{}", self.select().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DBG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<DBG_SEL_SPEC> {
        SELECT_W::new(self, 0)
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
#[doc = "What submodules are accessed by the debug bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_SEL_SPEC;
impl crate::RegisterSpec for DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sel::R`](R) reader structure"]
impl crate::Readable for DBG_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_sel::W`](W) writer structure"]
impl crate::Writable for DBG_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_SEL to value 0"]
impl crate::Resettable for DBG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

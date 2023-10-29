#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IBRD_SPEC>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IBRD_SPEC>;
#[doc = "Field `BAUDDIVINT` reader - BAUDDIVINT"]
pub type BAUDDIVINT_R = crate::FieldReader<u16>;
#[doc = "Field `BAUDDIVINT` writer - BAUDDIVINT"]
pub type BAUDDIVINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BAUDDIVINT"]
    #[inline(always)]
    pub fn bauddivint(&self) -> BAUDDIVINT_R {
        BAUDDIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBRD")
            .field("bauddivint", &format_args!("{}", self.bauddivint().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IBRD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - BAUDDIVINT"]
    #[inline(always)]
    #[must_use]
    pub fn bauddivint(&mut self) -> BAUDDIVINT_W<IBRD_SPEC, 0> {
        BAUDDIVINT_W::new(self)
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
#[doc = "Integer Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBRD_SPEC;
impl crate::RegisterSpec for IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

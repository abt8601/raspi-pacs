#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IFLS_SPEC>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IFLS_SPEC>;
#[doc = "Field `TXIFLSEL` reader - TXIFLSEL"]
pub type TXIFLSEL_R = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - TXIFLSEL"]
pub type TXIFLSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXIFLSEL` reader - RXIFLSEL"]
pub type RXIFLSEL_R = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - RXIFLSEL"]
pub type RXIFLSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFLS")
            .field("txiflsel", &format_args!("{}", self.txiflsel().bits()))
            .field("rxiflsel", &format_args!("{}", self.rxiflsel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IFLS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W<IFLS_SPEC> {
        TXIFLSEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W<IFLS_SPEC> {
        RXIFLSEL_W::new(self, 3)
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
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IFLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

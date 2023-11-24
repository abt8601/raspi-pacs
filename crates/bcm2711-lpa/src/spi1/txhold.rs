#[doc = "Register `TXHOLD%s` reader"]
pub type R = crate::R<TXHOLD_SPEC>;
#[doc = "Register `TXHOLD%s` writer"]
pub type W = crate::W<TXHOLD_SPEC>;
#[doc = "Field `DATA` reader - FIFO data access"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - FIFO data access"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXHOLD")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXHOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TXHOLD_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Writing to the FIFO will maintain CS at the end of the access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXHOLD_SPEC;
impl crate::RegisterSpec for TXHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txhold::R`](R) reader structure"]
impl crate::Readable for TXHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txhold::W`](W) writer structure"]
impl crate::Writable for TXHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXHOLD%s to value 0"]
impl crate::Resettable for TXHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

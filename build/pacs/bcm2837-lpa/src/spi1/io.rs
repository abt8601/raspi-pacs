#[doc = "Register `IO%s` reader"]
pub type R = crate::R<IO_SPEC>;
#[doc = "Register `IO%s` writer"]
pub type W = crate::W<IO_SPEC>;
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
        f.debug_struct("IO")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<IO_SPEC> {
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
#[doc = "Writing to the FIFO will deassert CS at the end of the access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_SPEC;
impl crate::RegisterSpec for IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io::R`](R) reader structure"]
impl crate::Readable for IO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io::W`](W) writer structure"]
impl crate::Writable for IO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO%s to value 0"]
impl crate::Resettable for IO_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `A` reader"]
pub type R = crate::R<A_SPEC>;
#[doc = "Register `A` writer"]
pub type W = crate::W<A_SPEC>;
#[doc = "Field `ADDR` reader - Slave address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Slave address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A")
            .field("addr", &format_args!("{}", self.addr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<A_SPEC, 0> {
        ADDR_W::new(self)
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
#[doc = "Slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_SPEC;
impl crate::RegisterSpec for A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a::R`](R) reader structure"]
impl crate::Readable for A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a::W`](W) writer structure"]
impl crate::Writable for A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A to value 0"]
impl crate::Resettable for A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

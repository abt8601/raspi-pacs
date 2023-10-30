#[doc = "Register `SPI_INT_SPT` reader"]
pub type R = crate::R<SPI_INT_SPT_SPEC>;
#[doc = "Register `SPI_INT_SPT` writer"]
pub type W = crate::W<SPI_INT_SPT_SPEC>;
#[doc = "Field `SELECT` reader - "]
pub type SELECT_R = crate::FieldReader;
#[doc = "Field `SELECT` writer - "]
pub type SELECT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INT_SPT")
            .field("select", &format_args!("{}", self.select().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPI_INT_SPT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<SPI_INT_SPT_SPEC, 0> {
        SELECT_W::new(self)
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
#[doc = "Interrupts in SPI mode depend on CS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_int_spt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_int_spt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_INT_SPT_SPEC;
impl crate::RegisterSpec for SPI_INT_SPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_int_spt::R`](R) reader structure"]
impl crate::Readable for SPI_INT_SPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_int_spt::W`](W) writer structure"]
impl crate::Writable for SPI_INT_SPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_INT_SPT to value 0"]
impl crate::Resettable for SPI_INT_SPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

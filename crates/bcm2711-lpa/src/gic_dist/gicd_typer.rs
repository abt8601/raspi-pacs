#[doc = "Register `GICD_TYPER` reader"]
pub struct R(crate::R<GICD_TYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_TYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_TYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_TYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IT_LINES_NUMBER` reader - Interrupt line number"]
pub type IT_LINES_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_NUMBER` reader - CPU number"]
pub type CPU_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY_EXTENSION` reader - Security extension implemented"]
pub type SECURITY_EXTENSION_R = crate::BitReader<bool>;
#[doc = "Field `LSPI` reader - Lockable SPI count"]
pub type LSPI_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Interrupt line number"]
    #[inline(always)]
    pub fn it_lines_number(&self) -> IT_LINES_NUMBER_R {
        IT_LINES_NUMBER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - CPU number"]
    #[inline(always)]
    pub fn cpu_number(&self) -> CPU_NUMBER_R {
        CPU_NUMBER_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - Security extension implemented"]
    #[inline(always)]
    pub fn security_extension(&self) -> SECURITY_EXTENSION_R {
        SECURITY_EXTENSION_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Lockable SPI count"]
    #[inline(always)]
    pub fn lspi(&self) -> LSPI_R {
        LSPI_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[doc = "Interrupt Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_typer](index.html) module"]
pub struct GICD_TYPER_SPEC;
impl crate::RegisterSpec for GICD_TYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_typer::R](R) reader structure"]
impl crate::Readable for GICD_TYPER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_TYPER to value 0"]
impl crate::Resettable for GICD_TYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

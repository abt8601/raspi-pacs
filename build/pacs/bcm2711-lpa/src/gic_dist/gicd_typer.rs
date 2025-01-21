#[doc = "Register `GICD_TYPER` reader"]
pub type R = crate::R<GICD_TYPER_SPEC>;
#[doc = "Field `IT_LINES_NUMBER` reader - Interrupt line number"]
pub type IT_LINES_NUMBER_R = crate::FieldReader;
#[doc = "Field `CPU_NUMBER` reader - CPU number"]
pub type CPU_NUMBER_R = crate::FieldReader;
#[doc = "Field `SECURITY_EXTENSION` reader - Security extension implemented"]
pub type SECURITY_EXTENSION_R = crate::BitReader;
#[doc = "Field `LSPI` reader - Lockable SPI count"]
pub type LSPI_R = crate::FieldReader;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_TYPER")
            .field(
                "it_lines_number",
                &format_args!("{}", self.it_lines_number().bits()),
            )
            .field("cpu_number", &format_args!("{}", self.cpu_number().bits()))
            .field(
                "security_extension",
                &format_args!("{}", self.security_extension().bit()),
            )
            .field("lspi", &format_args!("{}", self.lspi().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_TYPER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt Controller Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_typer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_TYPER_SPEC;
impl crate::RegisterSpec for GICD_TYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_typer::R`](R) reader structure"]
impl crate::Readable for GICD_TYPER_SPEC {}
#[doc = "`reset()` method sets GICD_TYPER to value 0"]
impl crate::Resettable for GICD_TYPER_SPEC {
    const RESET_VALUE: u32 = 0;
}

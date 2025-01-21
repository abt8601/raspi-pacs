#[doc = "Register `CONFIG0` reader"]
pub type R = crate::R<CONFIG0_SPEC>;
#[doc = "Register `CONFIG0` writer"]
pub type W = crate::W<CONFIG0_SPEC>;
#[doc = "Field `IRQEN` reader - Enable the interrupt when data is available"]
pub type IRQEN_R = crate::BitReader;
#[doc = "Field `IRQEN` writer - Enable the interrupt when data is available"]
pub type IRQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    pub fn irqen(&self) -> IRQEN_R {
        IRQEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG0")
            .field("irqen", &format_args!("{}", self.irqen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CONFIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IRQEN_W<CONFIG0_SPEC> {
        IRQEN_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG0_SPEC;
impl crate::RegisterSpec for CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config0::R`](R) reader structure"]
impl crate::Readable for CONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config0::W`](W) writer structure"]
impl crate::Writable for CONFIG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}

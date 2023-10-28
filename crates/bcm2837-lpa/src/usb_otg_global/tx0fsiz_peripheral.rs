#[doc = "Register `TX0FSIZ_Peripheral` reader"]
pub type R = crate::R<TX0FSIZ_PERIPHERAL_SPEC>;
#[doc = "Register `TX0FSIZ_Peripheral` writer"]
pub type W = crate::W<TX0FSIZ_PERIPHERAL_SPEC>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX0FSIZ_Peripheral")
            .field("tx0fsa", &format_args!("{}", self.tx0fsa().bits()))
            .field("tx0fd", &format_args!("{}", self.tx0fd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX0FSIZ_PERIPHERAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<TX0FSIZ_PERIPHERAL_SPEC, 0> {
        TX0FSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<TX0FSIZ_PERIPHERAL_SPEC, 16> {
        TX0FD_W::new(self)
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
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx0fsiz_peripheral::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx0fsiz_peripheral::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX0FSIZ_PERIPHERAL_SPEC;
impl crate::RegisterSpec for TX0FSIZ_PERIPHERAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx0fsiz_peripheral::R`](R) reader structure"]
impl crate::Readable for TX0FSIZ_PERIPHERAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx0fsiz_peripheral::W`](W) writer structure"]
impl crate::Writable for TX0FSIZ_PERIPHERAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX0FSIZ_Peripheral to value 0x0200"]
impl crate::Resettable for TX0FSIZ_PERIPHERAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}

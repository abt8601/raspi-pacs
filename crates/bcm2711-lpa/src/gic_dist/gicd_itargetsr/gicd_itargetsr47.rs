#[doc = "Register `GICD_ITARGETSR47` reader"]
pub type R = crate::R<GICD_ITARGETSR47_SPEC>;
#[doc = "Register `GICD_ITARGETSR47` writer"]
pub type W = crate::W<GICD_ITARGETSR47_SPEC>;
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type INT188_R = crate::FieldReader;
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type INT188_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type INT189_R = crate::FieldReader;
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type INT189_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type INT190_R = crate::FieldReader;
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type INT190_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type INT191_R = crate::FieldReader;
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type INT191_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> INT188_R {
        INT188_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> INT189_R {
        INT189_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> INT190_R {
        INT190_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> INT191_R {
        INT191_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR47")
            .field("int188", &format_args!("{}", self.int188().bits()))
            .field("int189", &format_args!("{}", self.int189().bits()))
            .field("int190", &format_args!("{}", self.int190().bits()))
            .field("int191", &format_args!("{}", self.int191().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR47_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> INT188_W<GICD_ITARGETSR47_SPEC, 0> {
        INT188_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> INT189_W<GICD_ITARGETSR47_SPEC, 8> {
        INT189_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> INT190_W<GICD_ITARGETSR47_SPEC, 16> {
        INT190_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> INT191_W<GICD_ITARGETSR47_SPEC, 24> {
        INT191_W::new(self)
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
#[doc = "Interrupt Processor Target 188 - 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR47_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr47::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR47_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr47::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR47_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR47 to value 0"]
impl crate::Resettable for GICD_ITARGETSR47_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

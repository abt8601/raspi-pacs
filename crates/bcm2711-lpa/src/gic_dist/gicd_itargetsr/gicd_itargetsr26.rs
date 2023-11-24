#[doc = "Register `GICD_ITARGETSR26` reader"]
pub type R = crate::R<GICD_ITARGETSR26_SPEC>;
#[doc = "Register `GICD_ITARGETSR26` writer"]
pub type W = crate::W<GICD_ITARGETSR26_SPEC>;
#[doc = "Field `ISP` reader - ISP"]
pub type ISP_R = crate::FieldReader;
#[doc = "Field `ISP` writer - ISP"]
pub type ISP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::FieldReader;
#[doc = "Field `USB` writer - USB"]
pub type USB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::FieldReader;
#[doc = "Field `V3D` writer - V3D"]
pub type V3D_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TRANSPOSER_R = crate::FieldReader;
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TRANSPOSER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> ISP_R {
        ISP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TRANSPOSER_R {
        TRANSPOSER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR26")
            .field("isp", &format_args!("{}", self.isp().bits()))
            .field("usb", &format_args!("{}", self.usb().bits()))
            .field("v3d", &format_args!("{}", self.v3d().bits()))
            .field("transposer", &format_args!("{}", self.transposer().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR26_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> ISP_W<GICD_ITARGETSR26_SPEC> {
        ISP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<GICD_ITARGETSR26_SPEC> {
        USB_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3D_W<GICD_ITARGETSR26_SPEC> {
        V3D_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TRANSPOSER_W<GICD_ITARGETSR26_SPEC> {
        TRANSPOSER_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 104 - 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR26_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr26::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR26_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr26::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR26_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR26 to value 0"]
impl crate::Resettable for GICD_ITARGETSR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

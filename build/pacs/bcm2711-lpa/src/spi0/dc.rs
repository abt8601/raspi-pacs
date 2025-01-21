#[doc = "Register `DC` reader"]
pub type R = crate::R<DC_SPEC>;
#[doc = "Register `DC` writer"]
pub type W = crate::W<DC_SPEC>;
#[doc = "Field `TDREQ` reader - DMA Write request threshold"]
pub type TDREQ_R = crate::FieldReader;
#[doc = "Field `TDREQ` writer - DMA Write request threshold"]
pub type TDREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPANIC` reader - DMA write panic threshold"]
pub type TPANIC_R = crate::FieldReader;
#[doc = "Field `TPANIC` writer - DMA write panic threshold"]
pub type TPANIC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDREQ` reader - DMA read request threshold"]
pub type RDREQ_R = crate::FieldReader;
#[doc = "Field `RDREQ` writer - DMA read request threshold"]
pub type RDREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPANIC` reader - DMA read panic threshold"]
pub type RPANIC_R = crate::FieldReader;
#[doc = "Field `RPANIC` writer - DMA read panic threshold"]
pub type RPANIC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    pub fn tdreq(&self) -> TDREQ_R {
        TDREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    pub fn tpanic(&self) -> TPANIC_R {
        TPANIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    pub fn rdreq(&self) -> RDREQ_R {
        RDREQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    pub fn rpanic(&self) -> RPANIC_R {
        RPANIC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DC")
            .field("rpanic", &format_args!("{}", self.rpanic().bits()))
            .field("rdreq", &format_args!("{}", self.rdreq().bits()))
            .field("tpanic", &format_args!("{}", self.tpanic().bits()))
            .field("tdreq", &format_args!("{}", self.tdreq().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tdreq(&mut self) -> TDREQ_W<DC_SPEC> {
        TDREQ_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tpanic(&mut self) -> TPANIC_W<DC_SPEC> {
        TPANIC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rdreq(&mut self) -> RDREQ_W<DC_SPEC> {
        RDREQ_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rpanic(&mut self) -> RPANIC_W<DC_SPEC> {
        RPANIC_W::new(self, 24)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_SPEC;
impl crate::RegisterSpec for DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc::R`](R) reader structure"]
impl crate::Readable for DC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc::W`](W) writer structure"]
impl crate::Writable for DC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC to value 0x3020_1050"]
impl crate::Resettable for DC_SPEC {
    const RESET_VALUE: u32 = 0x3020_1050;
}

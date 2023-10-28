#[doc = "Register `GICC_BPR` reader"]
pub type R = crate::R<GICC_BPR_SPEC>;
#[doc = "Register `GICC_BPR` writer"]
pub type W = crate::W<GICC_BPR_SPEC>;
#[doc = "Field `BINARY_POINT` reader - Split point between group priority and subpriority"]
pub type BINARY_POINT_R = crate::FieldReader;
#[doc = "Field `BINARY_POINT` writer - Split point between group priority and subpriority"]
pub type BINARY_POINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_BPR")
            .field(
                "binary_point",
                &format_args!("{}", self.binary_point().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_BPR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<GICC_BPR_SPEC, 0> {
        BINARY_POINT_W::new(self)
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
#[doc = "Binary Point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_bpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_bpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_BPR_SPEC;
impl crate::RegisterSpec for GICC_BPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_bpr::R`](R) reader structure"]
impl crate::Readable for GICC_BPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicc_bpr::W`](W) writer structure"]
impl crate::Writable for GICC_BPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_BPR to value 0"]
impl crate::Resettable for GICC_BPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FULL1` reader - FIFO full"]
pub type FULL1_R = crate::BitReader<bool>;
#[doc = "Field `FULL1` writer - FIFO full"]
pub type FULL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `EMPT1` reader - FIFO empty"]
pub type EMPT1_R = crate::BitReader<bool>;
#[doc = "Field `EMPT1` writer - FIFO empty"]
pub type EMPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `WERR1` reader - FIFO write error"]
pub type WERR1_R = crate::BitReader<bool>;
#[doc = "Field `WERR1` writer - FIFO write error"]
pub type WERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `RERR1` reader - FIFO read error"]
pub type RERR1_R = crate::BitReader<bool>;
#[doc = "Field `RERR1` writer - FIFO read error"]
pub type RERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `GAPO1` reader - Channel 1 gap occurred"]
pub type GAPO1_R = crate::BitReader<bool>;
#[doc = "Field `GAPO1` writer - Channel 1 gap occurred"]
pub type GAPO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `GAPO2` reader - Channel 2 gap occurred"]
pub type GAPO2_R = crate::BitReader<bool>;
#[doc = "Field `GAPO2` writer - Channel 2 gap occurred"]
pub type GAPO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `GAPO3` reader - Channel 3 gap occurred"]
pub type GAPO3_R = crate::BitReader<bool>;
#[doc = "Field `GAPO3` writer - Channel 3 gap occurred"]
pub type GAPO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `GAPO4` reader - Channel 4 gap occurred"]
pub type GAPO4_R = crate::BitReader<bool>;
#[doc = "Field `GAPO4` writer - Channel 4 gap occurred"]
pub type GAPO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `STA1` reader - Channel 1 state"]
pub type STA1_R = crate::BitReader<bool>;
#[doc = "Field `STA1` writer - Channel 1 state"]
pub type STA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `STA2` reader - Channel 2 state"]
pub type STA2_R = crate::BitReader<bool>;
#[doc = "Field `STA2` writer - Channel 2 state"]
pub type STA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `STA3` reader - Channel 3 state"]
pub type STA3_R = crate::BitReader<bool>;
#[doc = "Field `STA3` writer - Channel 3 state"]
pub type STA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `STA4` reader - Channel 4 state"]
pub type STA4_R = crate::BitReader<bool>;
#[doc = "Field `STA4` writer - Channel 4 state"]
pub type STA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    pub fn empt1(&self) -> EMPT1_R {
        EMPT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    pub fn werr1(&self) -> WERR1_R {
        WERR1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    pub fn rerr1(&self) -> RERR1_R {
        RERR1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    pub fn gapo1(&self) -> GAPO1_R {
        GAPO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    pub fn gapo2(&self) -> GAPO2_R {
        GAPO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    pub fn gapo3(&self) -> GAPO3_R {
        GAPO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    pub fn gapo4(&self) -> GAPO4_R {
        GAPO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    pub fn sta1(&self) -> STA1_R {
        STA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    pub fn sta2(&self) -> STA2_R {
        STA2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    pub fn sta3(&self) -> STA3_R {
        STA3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    pub fn sta4(&self) -> STA4_R {
        STA4_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> FULL1_W<0> {
        FULL1_W::new(self)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn empt1(&mut self) -> EMPT1_W<1> {
        EMPT1_W::new(self)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    #[must_use]
    pub fn werr1(&mut self) -> WERR1_W<2> {
        WERR1_W::new(self)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    #[must_use]
    pub fn rerr1(&mut self) -> RERR1_W<3> {
        RERR1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo1(&mut self) -> GAPO1_W<4> {
        GAPO1_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo2(&mut self) -> GAPO2_W<5> {
        GAPO2_W::new(self)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo3(&mut self) -> GAPO3_W<6> {
        GAPO3_W::new(self)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo4(&mut self) -> GAPO4_W<7> {
        GAPO4_W::new(self)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta1(&mut self) -> STA1_W<9> {
        STA1_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta2(&mut self) -> STA2_W<10> {
        STA2_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta3(&mut self) -> STA3_W<11> {
        STA3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta4(&mut self) -> STA4_W<12> {
        STA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

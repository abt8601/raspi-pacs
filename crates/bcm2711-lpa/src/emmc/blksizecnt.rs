#[doc = "Register `BLKSIZECNT` reader"]
pub struct R(crate::R<BLKSIZECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKSIZECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKSIZECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKSIZECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKSIZECNT` writer"]
pub struct W(crate::W<BLKSIZECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKSIZECNT_SPEC>;
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
impl From<crate::W<BLKSIZECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKSIZECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLKSIZE` reader - Block size in bytes"]
pub type BLKSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLKSIZE` writer - Block size in bytes"]
pub type BLKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLKSIZECNT_SPEC, u16, u16, 10, O>;
#[doc = "Field `BLKCNT` reader - Number of blocks to be transferred"]
pub type BLKCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLKCNT` writer - Number of blocks to be transferred"]
pub type BLKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLKSIZECNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:9 - Block size in bytes"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Number of blocks to be transferred"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Block size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<0> {
        BLKSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Number of blocks to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<16> {
        BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Numer and size in bytes for data block to be transferred\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blksizecnt](index.html) module"]
pub struct BLKSIZECNT_SPEC;
impl crate::RegisterSpec for BLKSIZECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blksizecnt::R](R) reader structure"]
impl crate::Readable for BLKSIZECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blksizecnt::W](W) writer structure"]
impl crate::Writable for BLKSIZECNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLKSIZECNT to value 0"]
impl crate::Resettable for BLKSIZECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

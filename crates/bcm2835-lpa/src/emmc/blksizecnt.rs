#[doc = "Register `BLKSIZECNT` reader"]
pub type R = crate::R<BLKSIZECNT_SPEC>;
#[doc = "Register `BLKSIZECNT` writer"]
pub type W = crate::W<BLKSIZECNT_SPEC>;
#[doc = "Field `BLKSIZE` reader - Block size in bytes"]
pub type BLKSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `BLKSIZE` writer - Block size in bytes"]
pub type BLKSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `BLKCNT` reader - Number of blocks to be transferred"]
pub type BLKCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLKCNT` writer - Number of blocks to be transferred"]
pub type BLKCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLKSIZECNT")
            .field("blkcnt", &format_args!("{}", self.blkcnt().bits()))
            .field("blksize", &format_args!("{}", self.blksize().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BLKSIZECNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Block size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<BLKSIZECNT_SPEC, 0> {
        BLKSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Number of blocks to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<BLKSIZECNT_SPEC, 16> {
        BLKCNT_W::new(self)
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
#[doc = "Numer and size in bytes for data block to be transferred\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksizecnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksizecnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLKSIZECNT_SPEC;
impl crate::RegisterSpec for BLKSIZECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksizecnt::R`](R) reader structure"]
impl crate::Readable for BLKSIZECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blksizecnt::W`](W) writer structure"]
impl crate::Writable for BLKSIZECNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLKSIZECNT to value 0"]
impl crate::Resettable for BLKSIZECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

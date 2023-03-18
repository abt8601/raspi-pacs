#[doc = "Register `BOOT_TIMEOUT` reader"]
pub struct R(crate::R<BOOT_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_TIMEOUT` writer"]
pub struct W(crate::W<BOOT_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_TIMEOUT_SPEC>;
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
impl From<crate::W<BOOT_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of SD clock cycles to wait for boot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_timeout](index.html) module"]
pub struct BOOT_TIMEOUT_SPEC;
impl crate::RegisterSpec for BOOT_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_timeout::R](R) reader structure"]
impl crate::Readable for BOOT_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_timeout::W](W) writer structure"]
impl crate::Writable for BOOT_TIMEOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOT_TIMEOUT to value 0"]
impl crate::Resettable for BOOT_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

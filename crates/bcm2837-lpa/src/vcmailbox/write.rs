#[doc = "Register `WRITE` writer"]
pub struct W(crate::W<WRITE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITE_SPEC>;
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
impl From<crate::W<WRITE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITE_SPEC>) -> Self {
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
#[doc = "Write messages to the VideoCore\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [write](index.html) module"]
pub struct WRITE_SPEC;
impl crate::RegisterSpec for WRITE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [write::W](W) writer structure"]
impl crate::Writable for WRITE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}

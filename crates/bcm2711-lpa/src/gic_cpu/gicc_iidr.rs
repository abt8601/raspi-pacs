#[doc = "Register `GICC_IIDR` reader"]
pub type R = crate::R<GICC_IIDR_SPEC>;
#[doc = "Register `GICC_IIDR` writer"]
pub type W = crate::W<GICC_IIDR_SPEC>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<ID_A>;
#[doc = "ID\n\nValue on reset: 33690683"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ID_A {
    #[doc = "33690683: ID is valid"]
    VALID = 33690683,
}
impl From<ID_A> for u32 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ID_A {
    type Ux = u32;
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ID_A> {
        match self.bits {
            33690683 => Some(ID_A::VALID),
            _ => None,
        }
    }
    #[doc = "ID is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == ID_A::VALID
    }
}
#[doc = "Field `ID` writer - ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, ID_A>;
impl<'a, REG, const O: u8> ID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "ID is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(ID_A::VALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_IIDR")
            .field("id", &format_args!("{}", self.id().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_IIDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<GICC_IIDR_SPEC, 0> {
        ID_W::new(self)
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
#[doc = "CPU Interface Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_iidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_iidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_IIDR_SPEC;
impl crate::RegisterSpec for GICC_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_iidr::R`](R) reader structure"]
impl crate::Readable for GICC_IIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicc_iidr::W`](W) writer structure"]
impl crate::Writable for GICC_IIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_IIDR to value 0x0202_143b"]
impl crate::Resettable for GICC_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_143b;
}

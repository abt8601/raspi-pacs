#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TOCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TOCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PHYIF` reader - PHY Interface width"]
pub type PHYIF_R = crate::BitReader<PHYIF_A>;
#[doc = "PHY Interface width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYIF_A {
    #[doc = "0: `0`"]
    _8BIT = 0,
    #[doc = "1: `1`"]
    _16BIT = 1,
}
impl From<PHYIF_A> for bool {
    #[inline(always)]
    fn from(variant: PHYIF_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYIF_A {
        match self.bits {
            false => PHYIF_A::_8BIT,
            true => PHYIF_A::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == PHYIF_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == PHYIF_A::_16BIT
    }
}
#[doc = "Field `PHYIF` writer - PHY Interface width"]
pub type PHYIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYIF_A, O>;
impl<'a, const O: u8> PHYIF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PHYIF_A::_8BIT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PHYIF_A::_16BIT)
    }
}
#[doc = "Field `PHYTYPE` reader - PHY Type"]
pub type PHYTYPE_R = crate::BitReader<PHYTYPE_A>;
#[doc = "PHY Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYTYPE_A {
    #[doc = "0: `0`"]
    UTMI = 0,
    #[doc = "1: `1`"]
    ULPI = 1,
}
impl From<PHYTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: PHYTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYTYPE_A {
        match self.bits {
            false => PHYTYPE_A::UTMI,
            true => PHYTYPE_A::ULPI,
        }
    }
    #[doc = "Checks if the value of the field is `UTMI`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == PHYTYPE_A::UTMI
    }
    #[doc = "Checks if the value of the field is `ULPI`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == PHYTYPE_A::ULPI
    }
}
#[doc = "Field `PHYTYPE` writer - PHY Type"]
pub type PHYTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYTYPE_A, O>;
impl<'a, const O: u8> PHYTYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn utmi(self) -> &'a mut W {
        self.variant(PHYTYPE_A::UTMI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ulpi(self) -> &'a mut W {
        self.variant(PHYTYPE_A::ULPI)
    }
}
#[doc = "Field `FSIF` reader - Full speed interface"]
pub type FSIF_R = crate::BitReader<FSIF_A>;
#[doc = "Full speed interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSIF_A {
    #[doc = "0: `0`"]
    _6PIN = 0,
    #[doc = "1: `1`"]
    _3PIN = 1,
}
impl From<FSIF_A> for bool {
    #[inline(always)]
    fn from(variant: FSIF_A) -> Self {
        variant as u8 != 0
    }
}
impl FSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSIF_A {
        match self.bits {
            false => FSIF_A::_6PIN,
            true => FSIF_A::_3PIN,
        }
    }
    #[doc = "Checks if the value of the field is `_6PIN`"]
    #[inline(always)]
    pub fn is_6pin(&self) -> bool {
        *self == FSIF_A::_6PIN
    }
    #[doc = "Checks if the value of the field is `_3PIN`"]
    #[inline(always)]
    pub fn is_3pin(&self) -> bool {
        *self == FSIF_A::_3PIN
    }
}
#[doc = "Field `FSIF` writer - Full speed interface"]
pub type FSIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, FSIF_A, O>;
impl<'a, const O: u8> FSIF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _6pin(self) -> &'a mut W {
        self.variant(FSIF_A::_6PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3pin(self) -> &'a mut W {
        self.variant(FSIF_A::_3PIN)
    }
}
#[doc = "Field `PHYSEL` reader - Transceiver select"]
pub type PHYSEL_R = crate::BitReader<PHYSEL_A>;
#[doc = "Transceiver select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYSEL_A {
    #[doc = "0: `0`"]
    USB20 = 0,
    #[doc = "1: `1`"]
    USB11 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYSEL_A {
        match self.bits {
            false => PHYSEL_A::USB20,
            true => PHYSEL_A::USB11,
        }
    }
    #[doc = "Checks if the value of the field is `USB20`"]
    #[inline(always)]
    pub fn is_usb20(&self) -> bool {
        *self == PHYSEL_A::USB20
    }
    #[doc = "Checks if the value of the field is `USB11`"]
    #[inline(always)]
    pub fn is_usb11(&self) -> bool {
        *self == PHYSEL_A::USB11
    }
}
#[doc = "Field `PHYSEL` writer - Transceiver select"]
pub type PHYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, PHYSEL_A, O>;
impl<'a, const O: u8> PHYSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn usb20(self) -> &'a mut W {
        self.variant(PHYSEL_A::USB20)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn usb11(self) -> &'a mut W {
        self.variant(PHYSEL_A::USB11)
    }
}
#[doc = "Field `DDRSEL` reader - ULPI data rate"]
pub type DDRSEL_R = crate::BitReader<DDRSEL_A>;
#[doc = "ULPI data rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRSEL_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<DDRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DDRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DDRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRSEL_A {
        match self.bits {
            false => DDRSEL_A::SINGLE,
            true => DDRSEL_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDRSEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == DDRSEL_A::DOUBLE
    }
}
#[doc = "Field `DDRSEL` writer - ULPI data rate"]
pub type DDRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, DDRSEL_A, O>;
impl<'a, const O: u8> DDRSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDRSEL_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(DDRSEL_A::DOUBLE)
    }
}
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TRDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TRDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PHYLPCS` reader - PHY Low-power clock select"]
pub type PHYLPCS_R = crate::BitReader<bool>;
#[doc = "Field `PHYLPCS` writer - PHY Low-power clock select"]
pub type PHYLPCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPIFSLS` reader - ULPI FS/LS select"]
pub type ULPIFSLS_R = crate::BitReader<bool>;
#[doc = "Field `ULPIFSLS` writer - ULPI FS/LS select"]
pub type ULPIFSLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPIAR` reader - ULPI Auto-resume"]
pub type ULPIAR_R = crate::BitReader<bool>;
#[doc = "Field `ULPIAR` writer - ULPI Auto-resume"]
pub type ULPIAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPICSM` reader - ULPI Clock SuspendM"]
pub type ULPICSM_R = crate::BitReader<bool>;
#[doc = "Field `ULPICSM` writer - ULPI Clock SuspendM"]
pub type ULPICSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPIEVBUSD` reader - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_R = crate::BitReader<bool>;
#[doc = "Field `ULPIEVBUSD` writer - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPIEVBUSI` reader - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_R = crate::BitReader<bool>;
#[doc = "Field `ULPIEVBUSI` writer - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TSDPS` reader - TermSel DLine pulsing selection"]
pub type TSDPS_R = crate::BitReader<bool>;
#[doc = "Field `TSDPS` writer - TermSel DLine pulsing selection"]
pub type TSDPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `PCCI` reader - Indicator complement"]
pub type PCCI_R = crate::BitReader<bool>;
#[doc = "Field `PCCI` writer - Indicator complement"]
pub type PCCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `PTCI` reader - Indicator pass through"]
pub type PTCI_R = crate::BitReader<bool>;
#[doc = "Field `PTCI` writer - Indicator pass through"]
pub type PTCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `ULPIIPD` reader - ULPI interface protect disable"]
pub type ULPIIPD_R = crate::BitReader<bool>;
#[doc = "Field `ULPIIPD` writer - ULPI interface protect disable"]
pub type ULPIIPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FHMOD` reader - Forced host mode"]
pub type FHMOD_R = crate::BitReader<bool>;
#[doc = "Field `FHMOD` writer - Forced host mode"]
pub type FHMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FDMOD` reader - Forced peripheral mode"]
pub type FDMOD_R = crate::BitReader<bool>;
#[doc = "Field `FDMOD` writer - Forced peripheral mode"]
pub type FDMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `CTXPKT` reader - Corrupt Tx packet"]
pub type CTXPKT_R = crate::BitReader<bool>;
#[doc = "Field `CTXPKT` writer - Corrupt Tx packet"]
pub type CTXPKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    pub fn phytype(&self) -> PHYTYPE_R {
        PHYTYPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    pub fn fsif(&self) -> FSIF_R {
        FSIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    pub fn ddrsel(&self) -> DDRSEL_R {
        DDRSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(&self) -> PHYLPCS_R {
        PHYLPCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(&self) -> ULPIFSLS_R {
        ULPIFSLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(&self) -> ULPIAR_R {
        ULPIAR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(&self) -> ULPICSM_R {
        ULPICSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(&self) -> ULPIEVBUSD_R {
        ULPIEVBUSD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    pub fn ulpievbusi(&self) -> ULPIEVBUSI_R {
        ULPIEVBUSI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    pub fn pcci(&self) -> PCCI_R {
        PCCI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    pub fn ptci(&self) -> PTCI_R {
        PTCI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    pub fn ulpiipd(&self) -> ULPIIPD_R {
        ULPIIPD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tocal(&mut self) -> TOCAL_W<0> {
        TOCAL_W::new(self)
    }
    #[doc = "Bit 3 - PHY Interface width"]
    #[inline(always)]
    #[must_use]
    pub fn phyif(&mut self) -> PHYIF_W<3> {
        PHYIF_W::new(self)
    }
    #[doc = "Bit 4 - PHY Type"]
    #[inline(always)]
    #[must_use]
    pub fn phytype(&mut self) -> PHYTYPE_W<4> {
        PHYTYPE_W::new(self)
    }
    #[doc = "Bit 5 - Full speed interface"]
    #[inline(always)]
    #[must_use]
    pub fn fsif(&mut self) -> FSIF_W<5> {
        FSIF_W::new(self)
    }
    #[doc = "Bit 6 - Transceiver select"]
    #[inline(always)]
    #[must_use]
    pub fn physel(&mut self) -> PHYSEL_W<6> {
        PHYSEL_W::new(self)
    }
    #[doc = "Bit 7 - ULPI data rate"]
    #[inline(always)]
    #[must_use]
    pub fn ddrsel(&mut self) -> DDRSEL_W<7> {
        DDRSEL_W::new(self)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn trdt(&mut self) -> TRDT_W<10> {
        TRDT_W::new(self)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    #[must_use]
    pub fn phylpcs(&mut self) -> PHYLPCS_W<15> {
        PHYLPCS_W::new(self)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    #[must_use]
    pub fn ulpifsls(&mut self) -> ULPIFSLS_W<17> {
        ULPIFSLS_W::new(self)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiar(&mut self) -> ULPIAR_W<18> {
        ULPIAR_W::new(self)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    #[must_use]
    pub fn ulpicsm(&mut self) -> ULPICSM_W<19> {
        ULPICSM_W::new(self)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusd(&mut self) -> ULPIEVBUSD_W<20> {
        ULPIEVBUSD_W::new(self)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusi(&mut self) -> ULPIEVBUSI_W<21> {
        ULPIEVBUSI_W::new(self)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsdps(&mut self) -> TSDPS_W<22> {
        TSDPS_W::new(self)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    #[must_use]
    pub fn pcci(&mut self) -> PCCI_W<23> {
        PCCI_W::new(self)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    #[must_use]
    pub fn ptci(&mut self) -> PTCI_W<24> {
        PTCI_W::new(self)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiipd(&mut self) -> ULPIIPD_W<25> {
        ULPIIPD_W::new(self)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhmod(&mut self) -> FHMOD_W<29> {
        FHMOD_W::new(self)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdmod(&mut self) -> FDMOD_W<30> {
        FDMOD_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctxpkt(&mut self) -> CTXPKT_W<31> {
        CTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS USB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00;
}

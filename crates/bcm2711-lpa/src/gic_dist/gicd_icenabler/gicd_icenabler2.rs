#[doc = "Register `GICD_ICENABLER2` reader"]
pub struct R(crate::R<GICD_ICENABLER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICENABLER2` writer"]
pub struct W(crate::W<GICD_ICENABLER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER2_SPEC>;
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
impl From<crate::W<GICD_ICENABLER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader<bool>;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader<bool>;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader<bool>;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::BitReader<bool>;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::BitReader<bool>;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::BitReader<bool>;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::BitReader<bool>;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::BitReader<bool>;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::BitReader<bool>;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::BitReader<bool>;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::BitReader<bool>;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type INT80_R = crate::BitReader<bool>;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type INT80_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type INT81_R = crate::BitReader<bool>;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type INT81_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type INT82_R = crate::BitReader<bool>;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type INT82_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type INT83_R = crate::BitReader<bool>;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type INT83_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type INT84_R = crate::BitReader<bool>;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type INT84_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type INT85_R = crate::BitReader<bool>;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type INT85_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type INT86_R = crate::BitReader<bool>;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type INT86_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type INT87_R = crate::BitReader<bool>;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type INT87_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type INT88_R = crate::BitReader<bool>;
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type INT88_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type INT89_R = crate::BitReader<bool>;
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type INT89_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type INT90_R = crate::BitReader<bool>;
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type INT90_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type INT91_R = crate::BitReader<bool>;
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type INT91_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type INT92_R = crate::BitReader<bool>;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type INT92_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type INT93_R = crate::BitReader<bool>;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type INT93_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type INT94_R = crate::BitReader<bool>;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type INT94_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type INT95_R = crate::BitReader<bool>;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type INT95_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> INT80_R {
        INT80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> INT81_R {
        INT81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> INT82_R {
        INT82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> INT83_R {
        INT83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> INT84_R {
        INT84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> INT85_R {
        INT85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> INT86_R {
        INT86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> INT87_R {
        INT87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> INT88_R {
        INT88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> INT89_R {
        INT89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> INT90_R {
        INT90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> INT91_R {
        INT91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> INT92_R {
        INT92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> INT93_R {
        INT93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> INT94_R {
        INT94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> INT95_R {
        INT95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<0> {
        TIMER_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<1> {
        MAILBOX_W::new(self)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<2> {
        DOORBELL0_W::new(self)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<3> {
        DOORBELL1_W::new(self)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<4> {
        VPU0_HALTED_W::new(self)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<5> {
        VPU1_HALTED_W::new(self)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<6> {
        ARM_ADDRESS_ERROR_W::new(self)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<7> {
        ARM_AXI_ERROR_W::new(self)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<8> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<9> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<10> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<11> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<12> {
        SWI4_W::new(self)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<13> {
        SWI5_W::new(self)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<14> {
        SWI6_W::new(self)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<15> {
        SWI7_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> INT80_W<16> {
        INT80_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> INT81_W<17> {
        INT81_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> INT82_W<18> {
        INT82_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> INT83_W<19> {
        INT83_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> INT84_W<20> {
        INT84_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> INT85_W<21> {
        INT85_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> INT86_W<22> {
        INT86_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> INT87_W<23> {
        INT87_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> INT88_W<24> {
        INT88_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> INT89_W<25> {
        INT89_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> INT90_W<26> {
        INT90_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> INT91_W<27> {
        INT91_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> INT92_W<28> {
        INT92_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> INT93_W<29> {
        INT93_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> INT94_W<30> {
        INT94_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> INT95_W<31> {
        INT95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear-Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler2](index.html) module"]
pub struct GICD_ICENABLER2_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icenabler2::R](R) reader structure"]
impl crate::Readable for GICD_ICENABLER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler2::W](W) writer structure"]
impl crate::Writable for GICD_ICENABLER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ICENABLER2 to value 0"]
impl crate::Resettable for GICD_ICENABLER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `BASIC_PENDING` reader"]
pub type R = crate::R<BASIC_PENDING_SPEC>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader;
#[doc = "Field `PENDING_1` reader - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
pub type PENDING_1_R = crate::BitReader;
#[doc = "Field `PENDING_2` reader - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
pub type PENDING_2_R = crate::BitReader;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type INT7_R = crate::BitReader;
#[doc = "Field `INT9` reader - Interrupt 9"]
pub type INT9_R = crate::BitReader;
#[doc = "Field `INT10` reader - Interrupt 10"]
pub type INT10_R = crate::BitReader;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type INT18_R = crate::BitReader;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type INT19_R = crate::BitReader;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type INT53_R = crate::BitReader;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type INT54_R = crate::BitReader;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type INT55_R = crate::BitReader;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type INT56_R = crate::BitReader;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type INT57_R = crate::BitReader;
#[doc = "Field `INT62` reader - Interrupt 62"]
pub type INT62_R = crate::BitReader;
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
    #[doc = "Bit 8 - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
    #[inline(always)]
    pub fn pending_1(&self) -> PENDING_1_R {
        PENDING_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
    #[inline(always)]
    pub fn pending_2(&self) -> PENDING_2_R {
        PENDING_2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> INT53_R {
        INT53_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> INT54_R {
        INT54_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> INT55_R {
        INT55_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> INT56_R {
        INT56_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> INT57_R {
        INT57_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 62"]
    #[inline(always)]
    pub fn int62(&self) -> INT62_R {
        INT62_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASIC_PENDING")
            .field("timer", &format_args!("{}", self.timer().bit()))
            .field("mailbox", &format_args!("{}", self.mailbox().bit()))
            .field("doorbell0", &format_args!("{}", self.doorbell0().bit()))
            .field("doorbell1", &format_args!("{}", self.doorbell1().bit()))
            .field("vpu0_halted", &format_args!("{}", self.vpu0_halted().bit()))
            .field("vpu1_halted", &format_args!("{}", self.vpu1_halted().bit()))
            .field(
                "arm_address_error",
                &format_args!("{}", self.arm_address_error().bit()),
            )
            .field(
                "arm_axi_error",
                &format_args!("{}", self.arm_axi_error().bit()),
            )
            .field("pending_1", &format_args!("{}", self.pending_1().bit()))
            .field("pending_2", &format_args!("{}", self.pending_2().bit()))
            .field("int7", &format_args!("{}", self.int7().bit()))
            .field("int9", &format_args!("{}", self.int9().bit()))
            .field("int10", &format_args!("{}", self.int10().bit()))
            .field("int18", &format_args!("{}", self.int18().bit()))
            .field("int19", &format_args!("{}", self.int19().bit()))
            .field("int53", &format_args!("{}", self.int53().bit()))
            .field("int54", &format_args!("{}", self.int54().bit()))
            .field("int55", &format_args!("{}", self.int55().bit()))
            .field("int56", &format_args!("{}", self.int56().bit()))
            .field("int57", &format_args!("{}", self.int57().bit()))
            .field("int62", &format_args!("{}", self.int62().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BASIC_PENDING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Basic pending info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_pending::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASIC_PENDING_SPEC;
impl crate::RegisterSpec for BASIC_PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`basic_pending::R`](R) reader structure"]
impl crate::Readable for BASIC_PENDING_SPEC {}
#[doc = "`reset()` method sets BASIC_PENDING to value 0"]
impl crate::Resettable for BASIC_PENDING_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_IPRIORITYR {
    #[doc = "0x00 - Interrupt Priority 0 - 3 (Lower is first)"]
    pub gicd_ipriorityr0: GICD_IPRIORITYR0,
    #[doc = "0x04 - Interrupt Priority 4 - 7 (Lower is first)"]
    pub gicd_ipriorityr1: GICD_IPRIORITYR1,
    #[doc = "0x08 - Interrupt Priority 8 - 11 (Lower is first)"]
    pub gicd_ipriorityr2: GICD_IPRIORITYR2,
    #[doc = "0x0c - Interrupt Priority 12 - 15 (Lower is first)"]
    pub gicd_ipriorityr3: GICD_IPRIORITYR3,
    #[doc = "0x10 - Interrupt Priority 16 - 19 (Lower is first)"]
    pub gicd_ipriorityr4: GICD_IPRIORITYR4,
    #[doc = "0x14 - Interrupt Priority 20 - 23 (Lower is first)"]
    pub gicd_ipriorityr5: GICD_IPRIORITYR5,
    #[doc = "0x18 - Interrupt Priority 24 - 27 (Lower is first)"]
    pub gicd_ipriorityr6: GICD_IPRIORITYR6,
    #[doc = "0x1c - Interrupt Priority 28 - 31 (Lower is first)"]
    pub gicd_ipriorityr7: GICD_IPRIORITYR7,
    #[doc = "0x20 - Interrupt Priority 32 - 35 (Lower is first)"]
    pub gicd_ipriorityr8: GICD_IPRIORITYR8,
    #[doc = "0x24 - Interrupt Priority 36 - 39 (Lower is first)"]
    pub gicd_ipriorityr9: GICD_IPRIORITYR9,
    #[doc = "0x28 - Interrupt Priority 40 - 43 (Lower is first)"]
    pub gicd_ipriorityr10: GICD_IPRIORITYR10,
    #[doc = "0x2c - Interrupt Priority 44 - 47 (Lower is first)"]
    pub gicd_ipriorityr11: GICD_IPRIORITYR11,
    #[doc = "0x30 - Interrupt Priority 48 - 51 (Lower is first)"]
    pub gicd_ipriorityr12: GICD_IPRIORITYR12,
    #[doc = "0x34 - Interrupt Priority 52 - 55 (Lower is first)"]
    pub gicd_ipriorityr13: GICD_IPRIORITYR13,
    #[doc = "0x38 - Interrupt Priority 56 - 59 (Lower is first)"]
    pub gicd_ipriorityr14: GICD_IPRIORITYR14,
    #[doc = "0x3c - Interrupt Priority 60 - 63 (Lower is first)"]
    pub gicd_ipriorityr15: GICD_IPRIORITYR15,
    #[doc = "0x40 - Interrupt Priority 64 - 67 (Lower is first)"]
    pub gicd_ipriorityr16: GICD_IPRIORITYR16,
    #[doc = "0x44 - Interrupt Priority 68 - 71 (Lower is first)"]
    pub gicd_ipriorityr17: GICD_IPRIORITYR17,
    #[doc = "0x48 - Interrupt Priority 72 - 75 (Lower is first)"]
    pub gicd_ipriorityr18: GICD_IPRIORITYR18,
    #[doc = "0x4c - Interrupt Priority 76 - 79 (Lower is first)"]
    pub gicd_ipriorityr19: GICD_IPRIORITYR19,
    #[doc = "0x50 - Interrupt Priority 80 - 83 (Lower is first)"]
    pub gicd_ipriorityr20: GICD_IPRIORITYR20,
    #[doc = "0x54 - Interrupt Priority 84 - 87 (Lower is first)"]
    pub gicd_ipriorityr21: GICD_IPRIORITYR21,
    #[doc = "0x58 - Interrupt Priority 88 - 91 (Lower is first)"]
    pub gicd_ipriorityr22: GICD_IPRIORITYR22,
    #[doc = "0x5c - Interrupt Priority 92 - 95 (Lower is first)"]
    pub gicd_ipriorityr23: GICD_IPRIORITYR23,
    #[doc = "0x60 - Interrupt Priority 96 - 99 (Lower is first)"]
    pub gicd_ipriorityr24: GICD_IPRIORITYR24,
    #[doc = "0x64 - Interrupt Priority 100 - 103 (Lower is first)"]
    pub gicd_ipriorityr25: GICD_IPRIORITYR25,
    #[doc = "0x68 - Interrupt Priority 104 - 107 (Lower is first)"]
    pub gicd_ipriorityr26: GICD_IPRIORITYR26,
    #[doc = "0x6c - Interrupt Priority 108 - 111 (Lower is first)"]
    pub gicd_ipriorityr27: GICD_IPRIORITYR27,
    #[doc = "0x70 - Interrupt Priority 112 - 115 (Lower is first)"]
    pub gicd_ipriorityr28: GICD_IPRIORITYR28,
    #[doc = "0x74 - Interrupt Priority 116 - 119 (Lower is first)"]
    pub gicd_ipriorityr29: GICD_IPRIORITYR29,
    #[doc = "0x78 - Interrupt Priority 120 - 123 (Lower is first)"]
    pub gicd_ipriorityr30: GICD_IPRIORITYR30,
    #[doc = "0x7c - Interrupt Priority 124 - 127 (Lower is first)"]
    pub gicd_ipriorityr31: GICD_IPRIORITYR31,
    #[doc = "0x80 - Interrupt Priority 128 - 131 (Lower is first)"]
    pub gicd_ipriorityr32: GICD_IPRIORITYR32,
    #[doc = "0x84 - Interrupt Priority 132 - 135 (Lower is first)"]
    pub gicd_ipriorityr33: GICD_IPRIORITYR33,
    #[doc = "0x88 - Interrupt Priority 136 - 139 (Lower is first)"]
    pub gicd_ipriorityr34: GICD_IPRIORITYR34,
    #[doc = "0x8c - Interrupt Priority 140 - 143 (Lower is first)"]
    pub gicd_ipriorityr35: GICD_IPRIORITYR35,
    #[doc = "0x90 - Interrupt Priority 144 - 147 (Lower is first)"]
    pub gicd_ipriorityr36: GICD_IPRIORITYR36,
    #[doc = "0x94 - Interrupt Priority 148 - 151 (Lower is first)"]
    pub gicd_ipriorityr37: GICD_IPRIORITYR37,
    #[doc = "0x98 - Interrupt Priority 152 - 155 (Lower is first)"]
    pub gicd_ipriorityr38: GICD_IPRIORITYR38,
    #[doc = "0x9c - Interrupt Priority 156 - 159 (Lower is first)"]
    pub gicd_ipriorityr39: GICD_IPRIORITYR39,
    #[doc = "0xa0 - Interrupt Priority 160 - 163 (Lower is first)"]
    pub gicd_ipriorityr40: GICD_IPRIORITYR40,
    #[doc = "0xa4 - Interrupt Priority 164 - 167 (Lower is first)"]
    pub gicd_ipriorityr41: GICD_IPRIORITYR41,
    #[doc = "0xa8 - Interrupt Priority 168 - 171 (Lower is first)"]
    pub gicd_ipriorityr42: GICD_IPRIORITYR42,
    #[doc = "0xac - Interrupt Priority 172 - 175 (Lower is first)"]
    pub gicd_ipriorityr43: GICD_IPRIORITYR43,
    #[doc = "0xb0 - Interrupt Priority 176 - 179 (Lower is first)"]
    pub gicd_ipriorityr44: GICD_IPRIORITYR44,
    #[doc = "0xb4 - Interrupt Priority 180 - 183 (Lower is first)"]
    pub gicd_ipriorityr45: GICD_IPRIORITYR45,
    #[doc = "0xb8 - Interrupt Priority 184 - 187 (Lower is first)"]
    pub gicd_ipriorityr46: GICD_IPRIORITYR46,
    #[doc = "0xbc - Interrupt Priority 188 - 191 (Lower is first)"]
    pub gicd_ipriorityr47: GICD_IPRIORITYR47,
    #[doc = "0xc0 - Interrupt Priority 192 - 195 (Lower is first)"]
    pub gicd_ipriorityr48: GICD_IPRIORITYR48,
    #[doc = "0xc4 - Interrupt Priority 196 - 199 (Lower is first)"]
    pub gicd_ipriorityr49: GICD_IPRIORITYR49,
    #[doc = "0xc8 - Interrupt Priority 200 - 203 (Lower is first)"]
    pub gicd_ipriorityr50: GICD_IPRIORITYR50,
    #[doc = "0xcc - Interrupt Priority 204 - 207 (Lower is first)"]
    pub gicd_ipriorityr51: GICD_IPRIORITYR51,
    #[doc = "0xd0 - Interrupt Priority 208 - 211 (Lower is first)"]
    pub gicd_ipriorityr52: GICD_IPRIORITYR52,
    #[doc = "0xd4 - Interrupt Priority 212 - 215 (Lower is first)"]
    pub gicd_ipriorityr53: GICD_IPRIORITYR53,
    #[doc = "0xd8 - Interrupt Priority 216 - 219 (Lower is first)"]
    pub gicd_ipriorityr54: GICD_IPRIORITYR54,
    #[doc = "0xdc - Interrupt Priority 220 - 223 (Lower is first)"]
    pub gicd_ipriorityr55: GICD_IPRIORITYR55,
}
#[doc = "GICD_IPRIORITYR0 (rw) register accessor: Interrupt Priority 0 - 3 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr0`]
module"]
pub type GICD_IPRIORITYR0 = crate::Reg<gicd_ipriorityr0::GICD_IPRIORITYR0_SPEC>;
#[doc = "Interrupt Priority 0 - 3 (Lower is first)"]
pub mod gicd_ipriorityr0;
#[doc = "GICD_IPRIORITYR1 (rw) register accessor: Interrupt Priority 4 - 7 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr1`]
module"]
pub type GICD_IPRIORITYR1 = crate::Reg<gicd_ipriorityr1::GICD_IPRIORITYR1_SPEC>;
#[doc = "Interrupt Priority 4 - 7 (Lower is first)"]
pub mod gicd_ipriorityr1;
#[doc = "GICD_IPRIORITYR2 (rw) register accessor: Interrupt Priority 8 - 11 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr2`]
module"]
pub type GICD_IPRIORITYR2 = crate::Reg<gicd_ipriorityr2::GICD_IPRIORITYR2_SPEC>;
#[doc = "Interrupt Priority 8 - 11 (Lower is first)"]
pub mod gicd_ipriorityr2;
#[doc = "GICD_IPRIORITYR3 (rw) register accessor: Interrupt Priority 12 - 15 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr3`]
module"]
pub type GICD_IPRIORITYR3 = crate::Reg<gicd_ipriorityr3::GICD_IPRIORITYR3_SPEC>;
#[doc = "Interrupt Priority 12 - 15 (Lower is first)"]
pub mod gicd_ipriorityr3;
#[doc = "GICD_IPRIORITYR4 (rw) register accessor: Interrupt Priority 16 - 19 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr4`]
module"]
pub type GICD_IPRIORITYR4 = crate::Reg<gicd_ipriorityr4::GICD_IPRIORITYR4_SPEC>;
#[doc = "Interrupt Priority 16 - 19 (Lower is first)"]
pub mod gicd_ipriorityr4;
#[doc = "GICD_IPRIORITYR5 (rw) register accessor: Interrupt Priority 20 - 23 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr5`]
module"]
pub type GICD_IPRIORITYR5 = crate::Reg<gicd_ipriorityr5::GICD_IPRIORITYR5_SPEC>;
#[doc = "Interrupt Priority 20 - 23 (Lower is first)"]
pub mod gicd_ipriorityr5;
#[doc = "GICD_IPRIORITYR6 (rw) register accessor: Interrupt Priority 24 - 27 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr6`]
module"]
pub type GICD_IPRIORITYR6 = crate::Reg<gicd_ipriorityr6::GICD_IPRIORITYR6_SPEC>;
#[doc = "Interrupt Priority 24 - 27 (Lower is first)"]
pub mod gicd_ipriorityr6;
#[doc = "GICD_IPRIORITYR7 (rw) register accessor: Interrupt Priority 28 - 31 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr7`]
module"]
pub type GICD_IPRIORITYR7 = crate::Reg<gicd_ipriorityr7::GICD_IPRIORITYR7_SPEC>;
#[doc = "Interrupt Priority 28 - 31 (Lower is first)"]
pub mod gicd_ipriorityr7;
#[doc = "GICD_IPRIORITYR8 (rw) register accessor: Interrupt Priority 32 - 35 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr8`]
module"]
pub type GICD_IPRIORITYR8 = crate::Reg<gicd_ipriorityr8::GICD_IPRIORITYR8_SPEC>;
#[doc = "Interrupt Priority 32 - 35 (Lower is first)"]
pub mod gicd_ipriorityr8;
#[doc = "GICD_IPRIORITYR9 (rw) register accessor: Interrupt Priority 36 - 39 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr9`]
module"]
pub type GICD_IPRIORITYR9 = crate::Reg<gicd_ipriorityr9::GICD_IPRIORITYR9_SPEC>;
#[doc = "Interrupt Priority 36 - 39 (Lower is first)"]
pub mod gicd_ipriorityr9;
#[doc = "GICD_IPRIORITYR10 (rw) register accessor: Interrupt Priority 40 - 43 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr10`]
module"]
pub type GICD_IPRIORITYR10 = crate::Reg<gicd_ipriorityr10::GICD_IPRIORITYR10_SPEC>;
#[doc = "Interrupt Priority 40 - 43 (Lower is first)"]
pub mod gicd_ipriorityr10;
#[doc = "GICD_IPRIORITYR11 (rw) register accessor: Interrupt Priority 44 - 47 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr11`]
module"]
pub type GICD_IPRIORITYR11 = crate::Reg<gicd_ipriorityr11::GICD_IPRIORITYR11_SPEC>;
#[doc = "Interrupt Priority 44 - 47 (Lower is first)"]
pub mod gicd_ipriorityr11;
#[doc = "GICD_IPRIORITYR12 (rw) register accessor: Interrupt Priority 48 - 51 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr12`]
module"]
pub type GICD_IPRIORITYR12 = crate::Reg<gicd_ipriorityr12::GICD_IPRIORITYR12_SPEC>;
#[doc = "Interrupt Priority 48 - 51 (Lower is first)"]
pub mod gicd_ipriorityr12;
#[doc = "GICD_IPRIORITYR13 (rw) register accessor: Interrupt Priority 52 - 55 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr13`]
module"]
pub type GICD_IPRIORITYR13 = crate::Reg<gicd_ipriorityr13::GICD_IPRIORITYR13_SPEC>;
#[doc = "Interrupt Priority 52 - 55 (Lower is first)"]
pub mod gicd_ipriorityr13;
#[doc = "GICD_IPRIORITYR14 (rw) register accessor: Interrupt Priority 56 - 59 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr14`]
module"]
pub type GICD_IPRIORITYR14 = crate::Reg<gicd_ipriorityr14::GICD_IPRIORITYR14_SPEC>;
#[doc = "Interrupt Priority 56 - 59 (Lower is first)"]
pub mod gicd_ipriorityr14;
#[doc = "GICD_IPRIORITYR15 (rw) register accessor: Interrupt Priority 60 - 63 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr15`]
module"]
pub type GICD_IPRIORITYR15 = crate::Reg<gicd_ipriorityr15::GICD_IPRIORITYR15_SPEC>;
#[doc = "Interrupt Priority 60 - 63 (Lower is first)"]
pub mod gicd_ipriorityr15;
#[doc = "GICD_IPRIORITYR16 (rw) register accessor: Interrupt Priority 64 - 67 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr16`]
module"]
pub type GICD_IPRIORITYR16 = crate::Reg<gicd_ipriorityr16::GICD_IPRIORITYR16_SPEC>;
#[doc = "Interrupt Priority 64 - 67 (Lower is first)"]
pub mod gicd_ipriorityr16;
#[doc = "GICD_IPRIORITYR17 (rw) register accessor: Interrupt Priority 68 - 71 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr17`]
module"]
pub type GICD_IPRIORITYR17 = crate::Reg<gicd_ipriorityr17::GICD_IPRIORITYR17_SPEC>;
#[doc = "Interrupt Priority 68 - 71 (Lower is first)"]
pub mod gicd_ipriorityr17;
#[doc = "GICD_IPRIORITYR18 (rw) register accessor: Interrupt Priority 72 - 75 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr18`]
module"]
pub type GICD_IPRIORITYR18 = crate::Reg<gicd_ipriorityr18::GICD_IPRIORITYR18_SPEC>;
#[doc = "Interrupt Priority 72 - 75 (Lower is first)"]
pub mod gicd_ipriorityr18;
#[doc = "GICD_IPRIORITYR19 (rw) register accessor: Interrupt Priority 76 - 79 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr19`]
module"]
pub type GICD_IPRIORITYR19 = crate::Reg<gicd_ipriorityr19::GICD_IPRIORITYR19_SPEC>;
#[doc = "Interrupt Priority 76 - 79 (Lower is first)"]
pub mod gicd_ipriorityr19;
#[doc = "GICD_IPRIORITYR20 (rw) register accessor: Interrupt Priority 80 - 83 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr20`]
module"]
pub type GICD_IPRIORITYR20 = crate::Reg<gicd_ipriorityr20::GICD_IPRIORITYR20_SPEC>;
#[doc = "Interrupt Priority 80 - 83 (Lower is first)"]
pub mod gicd_ipriorityr20;
#[doc = "GICD_IPRIORITYR21 (rw) register accessor: Interrupt Priority 84 - 87 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr21`]
module"]
pub type GICD_IPRIORITYR21 = crate::Reg<gicd_ipriorityr21::GICD_IPRIORITYR21_SPEC>;
#[doc = "Interrupt Priority 84 - 87 (Lower is first)"]
pub mod gicd_ipriorityr21;
#[doc = "GICD_IPRIORITYR22 (rw) register accessor: Interrupt Priority 88 - 91 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr22`]
module"]
pub type GICD_IPRIORITYR22 = crate::Reg<gicd_ipriorityr22::GICD_IPRIORITYR22_SPEC>;
#[doc = "Interrupt Priority 88 - 91 (Lower is first)"]
pub mod gicd_ipriorityr22;
#[doc = "GICD_IPRIORITYR23 (rw) register accessor: Interrupt Priority 92 - 95 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr23`]
module"]
pub type GICD_IPRIORITYR23 = crate::Reg<gicd_ipriorityr23::GICD_IPRIORITYR23_SPEC>;
#[doc = "Interrupt Priority 92 - 95 (Lower is first)"]
pub mod gicd_ipriorityr23;
#[doc = "GICD_IPRIORITYR24 (rw) register accessor: Interrupt Priority 96 - 99 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr24`]
module"]
pub type GICD_IPRIORITYR24 = crate::Reg<gicd_ipriorityr24::GICD_IPRIORITYR24_SPEC>;
#[doc = "Interrupt Priority 96 - 99 (Lower is first)"]
pub mod gicd_ipriorityr24;
#[doc = "GICD_IPRIORITYR25 (rw) register accessor: Interrupt Priority 100 - 103 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr25`]
module"]
pub type GICD_IPRIORITYR25 = crate::Reg<gicd_ipriorityr25::GICD_IPRIORITYR25_SPEC>;
#[doc = "Interrupt Priority 100 - 103 (Lower is first)"]
pub mod gicd_ipriorityr25;
#[doc = "GICD_IPRIORITYR26 (rw) register accessor: Interrupt Priority 104 - 107 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr26`]
module"]
pub type GICD_IPRIORITYR26 = crate::Reg<gicd_ipriorityr26::GICD_IPRIORITYR26_SPEC>;
#[doc = "Interrupt Priority 104 - 107 (Lower is first)"]
pub mod gicd_ipriorityr26;
#[doc = "GICD_IPRIORITYR27 (rw) register accessor: Interrupt Priority 108 - 111 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr27`]
module"]
pub type GICD_IPRIORITYR27 = crate::Reg<gicd_ipriorityr27::GICD_IPRIORITYR27_SPEC>;
#[doc = "Interrupt Priority 108 - 111 (Lower is first)"]
pub mod gicd_ipriorityr27;
#[doc = "GICD_IPRIORITYR28 (rw) register accessor: Interrupt Priority 112 - 115 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr28`]
module"]
pub type GICD_IPRIORITYR28 = crate::Reg<gicd_ipriorityr28::GICD_IPRIORITYR28_SPEC>;
#[doc = "Interrupt Priority 112 - 115 (Lower is first)"]
pub mod gicd_ipriorityr28;
#[doc = "GICD_IPRIORITYR29 (rw) register accessor: Interrupt Priority 116 - 119 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr29`]
module"]
pub type GICD_IPRIORITYR29 = crate::Reg<gicd_ipriorityr29::GICD_IPRIORITYR29_SPEC>;
#[doc = "Interrupt Priority 116 - 119 (Lower is first)"]
pub mod gicd_ipriorityr29;
#[doc = "GICD_IPRIORITYR30 (rw) register accessor: Interrupt Priority 120 - 123 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr30`]
module"]
pub type GICD_IPRIORITYR30 = crate::Reg<gicd_ipriorityr30::GICD_IPRIORITYR30_SPEC>;
#[doc = "Interrupt Priority 120 - 123 (Lower is first)"]
pub mod gicd_ipriorityr30;
#[doc = "GICD_IPRIORITYR31 (rw) register accessor: Interrupt Priority 124 - 127 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr31`]
module"]
pub type GICD_IPRIORITYR31 = crate::Reg<gicd_ipriorityr31::GICD_IPRIORITYR31_SPEC>;
#[doc = "Interrupt Priority 124 - 127 (Lower is first)"]
pub mod gicd_ipriorityr31;
#[doc = "GICD_IPRIORITYR32 (rw) register accessor: Interrupt Priority 128 - 131 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr32`]
module"]
pub type GICD_IPRIORITYR32 = crate::Reg<gicd_ipriorityr32::GICD_IPRIORITYR32_SPEC>;
#[doc = "Interrupt Priority 128 - 131 (Lower is first)"]
pub mod gicd_ipriorityr32;
#[doc = "GICD_IPRIORITYR33 (rw) register accessor: Interrupt Priority 132 - 135 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr33`]
module"]
pub type GICD_IPRIORITYR33 = crate::Reg<gicd_ipriorityr33::GICD_IPRIORITYR33_SPEC>;
#[doc = "Interrupt Priority 132 - 135 (Lower is first)"]
pub mod gicd_ipriorityr33;
#[doc = "GICD_IPRIORITYR34 (rw) register accessor: Interrupt Priority 136 - 139 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr34`]
module"]
pub type GICD_IPRIORITYR34 = crate::Reg<gicd_ipriorityr34::GICD_IPRIORITYR34_SPEC>;
#[doc = "Interrupt Priority 136 - 139 (Lower is first)"]
pub mod gicd_ipriorityr34;
#[doc = "GICD_IPRIORITYR35 (rw) register accessor: Interrupt Priority 140 - 143 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr35`]
module"]
pub type GICD_IPRIORITYR35 = crate::Reg<gicd_ipriorityr35::GICD_IPRIORITYR35_SPEC>;
#[doc = "Interrupt Priority 140 - 143 (Lower is first)"]
pub mod gicd_ipriorityr35;
#[doc = "GICD_IPRIORITYR36 (rw) register accessor: Interrupt Priority 144 - 147 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr36`]
module"]
pub type GICD_IPRIORITYR36 = crate::Reg<gicd_ipriorityr36::GICD_IPRIORITYR36_SPEC>;
#[doc = "Interrupt Priority 144 - 147 (Lower is first)"]
pub mod gicd_ipriorityr36;
#[doc = "GICD_IPRIORITYR37 (rw) register accessor: Interrupt Priority 148 - 151 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr37`]
module"]
pub type GICD_IPRIORITYR37 = crate::Reg<gicd_ipriorityr37::GICD_IPRIORITYR37_SPEC>;
#[doc = "Interrupt Priority 148 - 151 (Lower is first)"]
pub mod gicd_ipriorityr37;
#[doc = "GICD_IPRIORITYR38 (rw) register accessor: Interrupt Priority 152 - 155 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr38`]
module"]
pub type GICD_IPRIORITYR38 = crate::Reg<gicd_ipriorityr38::GICD_IPRIORITYR38_SPEC>;
#[doc = "Interrupt Priority 152 - 155 (Lower is first)"]
pub mod gicd_ipriorityr38;
#[doc = "GICD_IPRIORITYR39 (rw) register accessor: Interrupt Priority 156 - 159 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr39`]
module"]
pub type GICD_IPRIORITYR39 = crate::Reg<gicd_ipriorityr39::GICD_IPRIORITYR39_SPEC>;
#[doc = "Interrupt Priority 156 - 159 (Lower is first)"]
pub mod gicd_ipriorityr39;
#[doc = "GICD_IPRIORITYR40 (rw) register accessor: Interrupt Priority 160 - 163 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr40`]
module"]
pub type GICD_IPRIORITYR40 = crate::Reg<gicd_ipriorityr40::GICD_IPRIORITYR40_SPEC>;
#[doc = "Interrupt Priority 160 - 163 (Lower is first)"]
pub mod gicd_ipriorityr40;
#[doc = "GICD_IPRIORITYR41 (rw) register accessor: Interrupt Priority 164 - 167 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr41`]
module"]
pub type GICD_IPRIORITYR41 = crate::Reg<gicd_ipriorityr41::GICD_IPRIORITYR41_SPEC>;
#[doc = "Interrupt Priority 164 - 167 (Lower is first)"]
pub mod gicd_ipriorityr41;
#[doc = "GICD_IPRIORITYR42 (rw) register accessor: Interrupt Priority 168 - 171 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr42`]
module"]
pub type GICD_IPRIORITYR42 = crate::Reg<gicd_ipriorityr42::GICD_IPRIORITYR42_SPEC>;
#[doc = "Interrupt Priority 168 - 171 (Lower is first)"]
pub mod gicd_ipriorityr42;
#[doc = "GICD_IPRIORITYR43 (rw) register accessor: Interrupt Priority 172 - 175 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr43`]
module"]
pub type GICD_IPRIORITYR43 = crate::Reg<gicd_ipriorityr43::GICD_IPRIORITYR43_SPEC>;
#[doc = "Interrupt Priority 172 - 175 (Lower is first)"]
pub mod gicd_ipriorityr43;
#[doc = "GICD_IPRIORITYR44 (rw) register accessor: Interrupt Priority 176 - 179 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr44`]
module"]
pub type GICD_IPRIORITYR44 = crate::Reg<gicd_ipriorityr44::GICD_IPRIORITYR44_SPEC>;
#[doc = "Interrupt Priority 176 - 179 (Lower is first)"]
pub mod gicd_ipriorityr44;
#[doc = "GICD_IPRIORITYR45 (rw) register accessor: Interrupt Priority 180 - 183 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr45`]
module"]
pub type GICD_IPRIORITYR45 = crate::Reg<gicd_ipriorityr45::GICD_IPRIORITYR45_SPEC>;
#[doc = "Interrupt Priority 180 - 183 (Lower is first)"]
pub mod gicd_ipriorityr45;
#[doc = "GICD_IPRIORITYR46 (rw) register accessor: Interrupt Priority 184 - 187 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr46`]
module"]
pub type GICD_IPRIORITYR46 = crate::Reg<gicd_ipriorityr46::GICD_IPRIORITYR46_SPEC>;
#[doc = "Interrupt Priority 184 - 187 (Lower is first)"]
pub mod gicd_ipriorityr46;
#[doc = "GICD_IPRIORITYR47 (rw) register accessor: Interrupt Priority 188 - 191 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr47`]
module"]
pub type GICD_IPRIORITYR47 = crate::Reg<gicd_ipriorityr47::GICD_IPRIORITYR47_SPEC>;
#[doc = "Interrupt Priority 188 - 191 (Lower is first)"]
pub mod gicd_ipriorityr47;
#[doc = "GICD_IPRIORITYR48 (rw) register accessor: Interrupt Priority 192 - 195 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr48`]
module"]
pub type GICD_IPRIORITYR48 = crate::Reg<gicd_ipriorityr48::GICD_IPRIORITYR48_SPEC>;
#[doc = "Interrupt Priority 192 - 195 (Lower is first)"]
pub mod gicd_ipriorityr48;
#[doc = "GICD_IPRIORITYR49 (rw) register accessor: Interrupt Priority 196 - 199 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr49`]
module"]
pub type GICD_IPRIORITYR49 = crate::Reg<gicd_ipriorityr49::GICD_IPRIORITYR49_SPEC>;
#[doc = "Interrupt Priority 196 - 199 (Lower is first)"]
pub mod gicd_ipriorityr49;
#[doc = "GICD_IPRIORITYR50 (rw) register accessor: Interrupt Priority 200 - 203 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr50`]
module"]
pub type GICD_IPRIORITYR50 = crate::Reg<gicd_ipriorityr50::GICD_IPRIORITYR50_SPEC>;
#[doc = "Interrupt Priority 200 - 203 (Lower is first)"]
pub mod gicd_ipriorityr50;
#[doc = "GICD_IPRIORITYR51 (rw) register accessor: Interrupt Priority 204 - 207 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr51`]
module"]
pub type GICD_IPRIORITYR51 = crate::Reg<gicd_ipriorityr51::GICD_IPRIORITYR51_SPEC>;
#[doc = "Interrupt Priority 204 - 207 (Lower is first)"]
pub mod gicd_ipriorityr51;
#[doc = "GICD_IPRIORITYR52 (rw) register accessor: Interrupt Priority 208 - 211 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr52`]
module"]
pub type GICD_IPRIORITYR52 = crate::Reg<gicd_ipriorityr52::GICD_IPRIORITYR52_SPEC>;
#[doc = "Interrupt Priority 208 - 211 (Lower is first)"]
pub mod gicd_ipriorityr52;
#[doc = "GICD_IPRIORITYR53 (rw) register accessor: Interrupt Priority 212 - 215 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr53`]
module"]
pub type GICD_IPRIORITYR53 = crate::Reg<gicd_ipriorityr53::GICD_IPRIORITYR53_SPEC>;
#[doc = "Interrupt Priority 212 - 215 (Lower is first)"]
pub mod gicd_ipriorityr53;
#[doc = "GICD_IPRIORITYR54 (rw) register accessor: Interrupt Priority 216 - 219 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr54`]
module"]
pub type GICD_IPRIORITYR54 = crate::Reg<gicd_ipriorityr54::GICD_IPRIORITYR54_SPEC>;
#[doc = "Interrupt Priority 216 - 219 (Lower is first)"]
pub mod gicd_ipriorityr54;
#[doc = "GICD_IPRIORITYR55 (rw) register accessor: Interrupt Priority 220 - 223 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr55`]
module"]
pub type GICD_IPRIORITYR55 = crate::Reg<gicd_ipriorityr55::GICD_IPRIORITYR55_SPEC>;
#[doc = "Interrupt Priority 220 - 223 (Lower is first)"]
pub mod gicd_ipriorityr55;

# bcm2711-lpa

[![crates.io](https://img.shields.io/crates/v/bcm2711-lpa.svg)](https://crates.io/crates/bcm2711-lpa)
[![docs.rs](https://img.shields.io/docsrs/bcm2711-lpa)](https://docs.rs/bcm2711-lpa)

Peripheral access crate for BCM2711 found in the Raspberry Pi 4.

This crate is generated by [`svd2rust`](https://crates.io/crates/svd2rust)
from the
[SVD file](https://github.com/abt8601/broadcom-peripherals/blob/6bc44a4fd5c956249b9d8815f66a9df41b5791b1/svd/gen/bcm2711_lpa.svd)
in
[`abt8601/broadcom-peripherals@6bc44a4`](https://github.com/abt8601/broadcom-peripherals/tree/6bc44a4fd5c956249b9d8815f66a9df41b5791b1),
which is based on that in
[`adafruit/broadcom-peripherals@d3a6b50`](https://github.com/adafruit/broadcom-peripherals/tree/d3a6b50a21e7dd49ba4bfa0374da3407594caa50).
(The SVD files in these two repositories are identical,
save that that in the former has the missing tags required by `svd2rust`.)
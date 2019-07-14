//! Board resources

use e310x_hal::device::{DevicePeripherals, DeviceResources, DeviceGpioPins};
use e310x_hal::core::CorePeripherals;

/// Board resources
pub struct BoardResources {
    /// Core peripherals
    pub core_peripherals: CorePeripherals,

    /// Device peripherals
    pub peripherals: DevicePeripherals,

    /// Board pins
    pub pins: DeviceGpioPins,
}

impl From<DeviceResources> for BoardResources {
    fn from(r: DeviceResources) -> Self {
        BoardResources {
            core_peripherals: r.core_peripherals,
            peripherals: r.peripherals,
            pins: r.pins.into()
        }
    }
}

impl BoardResources {
    /// Returns all the board resources *once*
    #[inline]
    pub fn take() -> Option<Self> {
        DeviceResources::take().map(BoardResources::from)
    }

    /// Unchecked version of `BoardResources::take`
    pub unsafe fn steal() -> Self {
        DeviceResources::steal().into()
    }
}

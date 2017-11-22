use riscv::csr;
use riscv::interrupt::Nr;
use e310x::PLIC;
pub use e310x::Interrupt;

/// Priority of a plic::Interrupt.
#[derive(Clone, Copy)]
pub enum Priority {
    P0, P1, P2, P3, P4, P5, P6, P7,
}

impl Priority {
    /// Takes a read interrupt priority or plic threshold
    /// register value and returns a plic::Priority enum.
    fn from(prio: u32) -> Priority {
        match prio {
            0 => Priority::P0,
            1 => Priority::P1,
            2 => Priority::P2,
            3 => Priority::P3,
            4 => Priority::P4,
            5 => Priority::P5,
            6 => Priority::P6,
            7 => Priority::P7,
            _ => unreachable!(),
        }
    }
}

impl Into<u32> for Priority {
    /// Returns the numeric priority for wirting to a
    /// interrupt priority or the plic threshold register.
    fn into(self) -> u32 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
            Priority::P5 => 5,
            Priority::P6 => 6,
            Priority::P7 => 7,
        }
    }
}

/// Plic interface
pub struct Plic<'a>(pub &'a PLIC);

impl<'a> Clone for Plic<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Plic<'a> {
}

/// Represents a register offset and mask for
/// accessing an individual bit in a register
/// file.
struct Loc {
    offset: usize,
    mask: u32,
}

impl Loc {
    /// Computes the location of an interrupt.
    //#[inline]
    pub fn from(intr: Interrupt) -> Self {
        // offset = nr / 32
        // bit = nr % 32
        // 32 = 2 ^ 5
        let nr = intr.nr();
        let bit = nr & 31;
        Self {
            offset: (nr >> 5) as usize,
            mask: 1 << bit
        }
    }

    /// Checks if bit is set.
    //#[inline]
    pub fn is_set(&self, bits: u32) -> bool {
        bits & self.mask == self.mask
    }
}

impl<'a> Plic<'a> {
    /// Initializes PLIC controller by resetting all
    /// enable bits to 0 and enables MachineExternal
    /// interrupts.
    pub fn init(&self) {
        for reg in self.0.enable.iter() {
            unsafe {
                reg.write(|w| w.bits(0));
            }
        }
        self.set_threshold(Priority::P0);
        self.enable_mext();
    }

    /// Enable MachineExternal interrupts.
    #[inline]
    pub fn enable_mext(&self) {
        csr::mie.set(|w| w.mext());
    }

    /// Disable MachineExternal interrupts.
    #[inline]
    pub fn disable_mext(&self) {
        csr::mie.clear(|w| w.mext());
    }

    /// Returns true when plic::Interrupt is pending.
    pub fn is_pending(&self, intr: Interrupt) -> bool {
        let loc = Loc::from(intr);
        let pending = self.0.pending[loc.offset].read();
        loc.is_set(pending.bits())
    }

    /// Returns true when plic::Interrupt is enabled.
    pub fn is_enabled(&self, intr: Interrupt) -> bool {
        let loc = Loc::from(intr);
        let enable = self.0.enable[loc.offset].read();
        loc.is_set(enable.bits())
    }

    /// Enables plic::Interrupt.
    pub fn enable(&self, intr: Interrupt) {
        let loc = Loc::from(intr);
        unsafe {
            self.0.enable[loc.offset]
                .modify(|r, w| w.bits(r.bits() | loc.mask));
        }
    }

    /// Disables plic::Interrupt.
    pub fn disable(&self, intr: Interrupt) {
        let loc = Loc::from(intr);
        unsafe {
            self.0.enable[loc.offset]
                .modify(|r, w| w.bits(r.bits() & !loc.mask));
        }
    }

    /// Claims the plic::Interrupt with the highest priority.
    pub fn claim(&self) -> Interrupt {
        Interrupt::from(self.0.claim.read().bits() as u8)
    }

    /// Notifies the PLIC that the claimed plic::Interrupt is
    /// complete.
    pub fn complete(&self, intr: Interrupt) {
        unsafe {
            self.0.claim.write(|w| w.bits(intr.nr() as u32));
        }
    }

    /// Returns the plic::Priority of a plic::Interrupt.
    pub fn get_priority(&self, intr: Interrupt) -> Priority {
        Priority::from(self.0.priority[intr.nr() as usize].read().bits())
    }

    /// Sets the plic::Priority of a plic::Interrupt.
    pub fn set_priority(&self, intr: Interrupt, prio: Priority) {
        unsafe {
            self.0.priority[intr.nr() as usize]
                .write(|w| w.bits(prio.into()));
        }
    }

    /// Returns the PLIC threshold priority.
    pub fn get_threshold(&self) -> Priority {
        Priority::from(self.0.threshold.read().bits())
    }

    /// Sets the PLIC threshold priority. This disables all
    /// interrupts with a lower plic::Priority.
    pub fn set_threshold(&self, prio: Priority) {
        unsafe {
            self.0.threshold.write(|w| w.bits(prio.into()));
        }
    }
}

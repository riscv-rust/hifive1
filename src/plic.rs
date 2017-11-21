use riscv::csr;
use riscv::interrupt::Nr;
use e310x::PLIC;
pub use e310x::Interrupt;

pub enum Priority {
    Never, P1, P2, P3, P4, P5, P6, P7,
}

impl Priority {
    pub fn from(prio: u32) -> Priority {
        match prio {
            0 => Priority::Never,
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
    fn into(self) -> u32 {
        match self {
            Priority::Never => 0,
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

pub struct Plic<'a>(pub &'a PLIC);

impl<'a> Clone for Plic<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Plic<'a> {
}

impl<'a> Plic<'a> {
    /*pub fn init(&self) {
        for reg in self.0.enable.iter() {
            unsafe {
                reg.write(|w| w.bits(0));
            }
        }

        //interrupt!(MachineExternal, plic::plic_handler());
    }*/

    pub fn enable_mexternal(&self) {
        csr::mie.set(|w| w.mext());
    }

    pub fn clear_mexternal(&self) {
        csr::mie.clear(|w| w.mext());
    }

    pub fn is_pending(&self, intr: Interrupt) -> bool {
        let mask = 1 << (intr.nr() % 32);
        let pending = self.0.pending[(intr.nr() / 32) as usize].read();
        pending.bits() & mask == mask
    }

    pub fn is_enabled(&self, intr: Interrupt) -> bool {
        let mask = 1 << (intr.nr() % 32);
        let enable = self.0.enable[(intr.nr() / 32) as usize].read();
        enable.bits() & mask == mask
    }

    pub fn enable(&self, intr: Interrupt) {
        let mask = 1 << (intr.nr() % 32);
        unsafe {
            self.0.enable[(intr.nr() / 32) as usize]
                .modify(|r, w| w.bits(r.bits() | mask));
        }
    }

    pub fn disable(&self, intr: Interrupt) {
        let mask = 1 << (intr.nr() % 32);
        unsafe {
            self.0.enable[(intr.nr() / 32) as usize]
                .modify(|r, w| w.bits(r.bits() & !mask));
        }
    }

    pub fn claim(&self) -> Interrupt {
        Interrupt::from(self.0.claim.read().bits() as u8)
    }

    pub fn complete(&self, intr: Interrupt) {
        unsafe {
            self.0.claim.write(|w| w.bits(intr.nr() as u32));
        }
    }

    pub fn get_priority(&self, intr: Interrupt) -> Priority {
        Priority::from(self.0.priority[(intr.nr() - 1) as usize].read().bits())
    }

    pub fn set_priority(&self, intr: Interrupt, prio: Priority) {
        unsafe {
            self.0.priority[(intr.nr() - 1) as usize]
                .write(|w| w.bits(prio.into()));
        }
    }

    pub fn get_threshold(&self) -> Priority {
        Priority::from(self.0.threshold.read().bits())
    }

    pub fn set_threshold(&self, prio: Priority) {
        unsafe {
            self.0.threshold.write(|w| w.bits(prio.into()));
        }
    }
}

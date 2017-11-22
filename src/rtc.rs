//! RTC
use e310x::{PLIC, RTC};
use plic::{Plic, Interrupt, Priority};

/// Rtc configuration
pub struct RtcConf {
    enalways: bool,
    scale: u8,
    counter: u64,
    cmp: u32,
    priority: Priority,
}

impl RtcConf {
    pub fn new() -> Self {
        Self {
            enalways: true,
            scale: 0,
            counter: 0,
            cmp: 0,
            priority: Priority::P1,
        }
    }

    pub fn set_enalways(&mut self, en: bool) -> &mut Self {
        self.enalways = en;
        self
    }

    pub fn set_scale(&mut self, scale: u8) -> &mut Self {
        assert!(scale < 16);
        self.scale = scale;
        self
    }

    pub fn set_counter(&mut self, counter: u64) -> &mut Self {
        assert!(counter < (1 << 49) - 1);
        self.counter = counter;
        self
    }
    pub fn set_cmp(&mut self, cmp: u32) -> &mut Self {
        self.cmp = cmp;
        self
    }

    pub fn set_priority(&mut self, prio: Priority) -> &mut Self {
        self.priority = prio;
        self
    }

    pub fn end(&self, rtc: &RTC) {
        //let plic = Plic(plic);
        let rtc = Rtc(rtc);

        //plic.disable(Interrupt::RTC);

        unsafe {
            rtc.0.rtccfg.modify(|_, w| {
                w.enalways().bit(self.enalways)
                    .scale().bits(self.scale)
            });

            rtc.0.rtchi.write(|w| w.bits((self.counter >> 32) as u32));
            rtc.0.rtclo.write(|w| w.bits(self.counter as u32));
            rtc.0.rtccmp.write(|w| w.bits(self.cmp));
        }

        //plic.set_priority(Interrupt::RTC, self.priority);
        //plic.enable(Interrupt::RTC);
    }
}


/// Rtc interface
pub struct Rtc<'a>(pub &'a RTC);

impl<'a> Clone for Rtc<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Rtc<'a> {}

impl<'a> ::hal::Timer for Rtc<'a> {
    type Time = ::aonclk::Ticks<u32>;

    fn get_timeout(&self) -> ::aonclk::Ticks<u32> {
        ::aonclk::Ticks(self.0.rtccmp.read().bits())
    }

    fn pause(&self) {
        self.0.rtccfg.modify(|_, w| w.enalways().bit(false));
    }

    fn restart(&self) {
        unsafe {
            self.0.rtchi.write(|w| w.bits(0));
            self.0.rtclo.write(|w| w.bits(0));
        }
        self.0.rtccfg.modify(|_, w| w.enalways().bit(true));
    }

    fn resume(&self) {
        self.0.rtccfg.modify(|_, w| w.enalways().bit(true));
    }

    fn set_timeout<T>(&self, timeout: T)
        where
        T: Into<::aonclk::Ticks<u32>>,
    {
        self.pause();
        unsafe {
            self.0.rtccmp.write(|w| w.bits(1));
        }
        self.restart();
    }

    fn wait(&self) -> ::nb::Result<(), !> {
        if self.0.rtccfg.read().cmpip().bit() {
            Ok(())
        } else {
            Err(::nb::Error::WouldBlock)
        }
    }
}

//! Units of time

macro_rules! map {
    ($Self:ident) => {
        impl<T> $Self<T> {
            /// Applies the function `f` to inner value
            pub fn map<F>(self, f: F) -> $Self<T>
            where
                F: FnOnce(T) -> T
            {
                $Self(f(self.0))
            }
        }
    }
}

/// `Hz^-1`
#[derive(Clone, Copy, Debug)]
pub struct IHertz<T>(pub T);

impl<T> IHertz<T> {
    /// Invert this quantity
    pub fn invert(self) -> Hertz<T> {
        Hertz(self.0)
    }
}

map!(IHertz);

/// `Hz`
#[derive(Clone, Copy, Debug)]
pub struct Hertz<T>(pub T);

impl<T> Hertz<T> {
    /// Invert this quantity
    pub fn invert(self) -> IHertz<T> {
        IHertz(self.0)
    }
}

map!(Hertz);

/// `us`
#[derive(Clone, Copy, Debug)]
pub struct Microseconds<T>(pub T);

map!(Microseconds);

/// `ms`
#[derive(Clone, Copy, Debug)]
pub struct Milliseconds<T>(pub T);

map!(Milliseconds);

/// `s`
#[derive(Clone, Copy, Debug)]
pub struct Seconds<T>(pub T);

map!(Seconds);

/// `u32` and `u64` extension trait
pub trait UExt<T> {
    /// Wrap in `Hz`
    fn hz(self) -> Hertz<T>;

    /// Wrap in `Milliseconds`
    fn ms(self) -> Milliseconds<T>;

    /// Wrap in `Seconds`
    fn s(self) -> Seconds<T>;

    /// Wrap in `Microseconds`
    fn us(self) -> Microseconds<T>;
}

impl UExt<u32> for u32 {
    fn hz(self) -> Hertz<u32> {
        Hertz(self)
    }

    fn ms(self) -> Milliseconds<u32> {
        Milliseconds(self)
    }

    fn s(self) -> Seconds<u32> {
        Seconds(self)
    }

    fn us(self) -> Microseconds<u32> {
        Microseconds(self)
    }
}

impl UExt<u64> for u64 {
    fn hz(self) -> Hertz<u64> {
        Hertz(self)
    }

    fn ms(self) -> Milliseconds<u64> {
        Milliseconds(self)
    }

    fn s(self) -> Seconds<u64> {
        Seconds(self)
    }

    fn us(self) -> Microseconds<u64> {
        Microseconds(self)
    }
}

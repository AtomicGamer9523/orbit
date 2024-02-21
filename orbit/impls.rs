use crate::*;

macro_rules! i {
    ($($v:literal $type:ty)*) => ($(
        impl OrbitEvent for $type {
            #[inline(always)]
            fn name(&self) -> &'static str { $v }
        }
    )*);
}

i! {
    "unit" ()
    "u8" u8
    "u16" u16
    "u32" u32
    "u64" u64
    "u128" u128
    "usize" usize
    "i8" i8
    "i16" i16
    "i32" i32
    "i64" i64
    "i128" i128
    "isize" isize
    "f32" f32
    "f64" f64
    "char" char
    "bool" bool
    "str" &'static str
    "string" String
}

impl<T> OrbitEvent for Option<T>
where
    T: OrbitEvent,
{
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Some(v) => v.name(),
            None => "None",
        }
    }
}

impl<F, T> OrbitEventHandler<T> for F
where
    T: OrbitEvent,
    F: Fn(&T),
{
    #[inline(always)]
    fn handle(&self, event: &T) {
        self(event);
    }
}

use rand::Rng;
use rand::distr::uniform::{SampleRange, SampleUniform};

pub fn any_i8() -> i8 { any_within(i8::MIN..=i8::MAX) }

fn any_within<T: SampleUniform, R: SampleRange<T>>(bounds: R) -> T {
    let mut rng = rand::rng();
    rng.random_range(bounds)
}

pub fn any_u8() -> u8 { any_within(0..=u8::MAX) }

pub fn any_u8_except(except: u8) -> u8 { any_except(|x| x == except, any_u8) }

fn any_except<F: Fn(T) -> bool, T: Copy + PartialEq + SampleUniform>(except: F, any_value: fn() -> T) -> T {
    let value = any_value();
    if !except(value) {
        value
    } else {
        any_except(except, any_value)
    }
}

pub fn any_i16() -> i16 { any_within(i16::MIN..=i16::MAX) }

pub fn any_u16() -> u16 { any_within(0..=u16::MAX) }

pub fn any_i32() -> i32 { any_within(i32::MIN..=i32::MAX) }

pub fn any_u32() -> u32 { any_within(0..=u32::MAX) }

pub fn any_u32_within<R: SampleRange<u32>>(bounds: R) -> u32 { any_within(bounds) }

pub fn any_u32_except(except: u32) -> u32 { any_except(|x| x == except, any_u32) }

pub fn any_i64() -> i64 { any_within(i64::MIN..=i64::MAX) }

pub fn any_u64() -> u64 { any_within(0..=u64::MAX) }

pub fn any_isize() -> isize {
    match isize::BITS {
        i16::BITS => any_i16() as isize,
        i32::BITS => any_i32() as isize,
        i64::BITS => any_i64() as isize,
        _ => panic!("Unhandled number of bits for an isize")
    }
}

pub fn any_usize() -> usize { any_within(0..=usize::MAX) }

pub fn any_usize_within<R: SampleRange<usize>>(bounds: R) -> usize { any_within(bounds) }

pub fn any_usize_except(except: usize) -> usize { any_except(|x| x == except, any_usize) }

pub fn any_usize_except_in(except: &[usize]) -> usize {
    any_except(|x| except.contains(&x), any_usize)
}

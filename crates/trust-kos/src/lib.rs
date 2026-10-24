//                    ☦
//         ╔════════════════════╗
//         ║  The wind blows    ║
//         ║  where it wishes.  ║
//         ╚════════════════════╝
//
//   John 3:8. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Raw `extern "C"` ABI. No wasm-bindgen. No SharedArrayBuffer.
//! The main thread compiles the module and posts it to an AudioWorklet.
//! `trust_process` does not allocate. 128-frame blocks are the contract.
//! Web latency is best-effort. The native latency note does not apply here.

#![cfg_attr(target_arch = "wasm32", no_std)]

use core::cell::UnsafeCell;

use trust_core::{
    Event, BLOCK_FRAMES, PARAM_BITS, PARAM_CEILING_DB_MILLI, PARAM_GAIN_MILLI, PARAM_MIX_MILLI,
    PARAM_RATE_HZ,
};
use trust_dsp::Krush;

struct Slot<T>(UnsafeCell<T>);
unsafe impl<T> Sync for Slot<T> {}

static KRUSH: Slot<Krush> = Slot(UnsafeCell::new(Krush::new(48_000)));
static IN_L: Slot<[f32; BLOCK_FRAMES]> = Slot(UnsafeCell::new([0.0; BLOCK_FRAMES]));
static IN_R: Slot<[f32; BLOCK_FRAMES]> = Slot(UnsafeCell::new([0.0; BLOCK_FRAMES]));
static OUT_L: Slot<[f32; BLOCK_FRAMES]> = Slot(UnsafeCell::new([0.0; BLOCK_FRAMES]));
static OUT_R: Slot<[f32; BLOCK_FRAMES]> = Slot(UnsafeCell::new([0.0; BLOCK_FRAMES]));

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// `trust-core` links `alloc`. The worklet path does not allocate.
// A request to allocate aborts instead of growing the module.
#[cfg(target_arch = "wasm32")]
struct AbortAlloc;

#[cfg(target_arch = "wasm32")]
unsafe impl core::alloc::GlobalAlloc for AbortAlloc {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        core::arch::wasm32::unreachable()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static TRUST_WASM_ALLOC: AbortAlloc = AbortAlloc;

#[no_mangle]
pub extern "C" fn trust_block_frames() -> u32 {
    BLOCK_FRAMES as u32
}

#[no_mangle]
pub extern "C" fn trust_in_l() -> *mut f32 {
    IN_L.0.get().cast()
}

#[no_mangle]
pub extern "C" fn trust_in_r() -> *mut f32 {
    IN_R.0.get().cast()
}

#[no_mangle]
pub extern "C" fn trust_out_l() -> *mut f32 {
    OUT_L.0.get().cast()
}

#[no_mangle]
pub extern "C" fn trust_out_r() -> *mut f32 {
    OUT_R.0.get().cast()
}

/// # Safety
/// Single-threaded. The worklet does not share this state.
#[no_mangle]
pub unsafe extern "C" fn trust_init(sample_rate: u32) {
    let krush = unsafe { &mut *KRUSH.0.get() };
    krush.set_sample_rate(sample_rate.max(1));
    krush.set_bits(8);
    krush.set_rate_hz(8_000);
    krush.snap_gain(1000);
    krush.snap_mix(1000);
    krush.set_ceiling_db_milli(-1000);
}

/// # Safety
/// Single-threaded, same as [`trust_init`].
#[no_mangle]
pub unsafe extern "C" fn trust_set_param(id: u32, value: i32) {
    let krush = unsafe { &mut *KRUSH.0.get() };
    match id {
        PARAM_BITS => krush.set_bits(value),
        PARAM_RATE_HZ => krush.set_rate_hz(value),
        PARAM_GAIN_MILLI => krush.set_gain_milli(value),
        PARAM_MIX_MILLI => krush.set_mix_milli(value),
        PARAM_CEILING_DB_MILLI => krush.set_ceiling_db_milli(value),
        _ => {}
    }
}

/// # Safety
/// `in_l`, `in_r`, `out_l`, `out_r` each point at `frames` f32 samples.
/// `frames` may be 128. The function does not allocate.
#[no_mangle]
pub unsafe extern "C" fn trust_process(
    in_l: *const f32,
    in_r: *const f32,
    out_l: *mut f32,
    out_r: *mut f32,
    frames: u32,
) {
    if in_l.is_null() || in_r.is_null() || out_l.is_null() || out_r.is_null() || frames == 0 {
        return;
    }
    let n = frames as usize;
    let input_l = unsafe { core::slice::from_raw_parts(in_l, n) };
    let input_r = unsafe { core::slice::from_raw_parts(in_r, n) };
    let output_l = unsafe { core::slice::from_raw_parts_mut(out_l, n) };
    let output_r = unsafe { core::slice::from_raw_parts_mut(out_r, n) };
    let krush = unsafe { &mut *KRUSH.0.get() };
    let events: &[Event] = &[];
    krush.process(input_l, input_r, output_l, output_r, events);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_abi_crushes_128_frames_without_allocating_a_buffer() {
        unsafe {
            trust_init(48_000);
            trust_set_param(PARAM_BITS, 4);
            trust_set_param(PARAM_RATE_HZ, 48_000);
            trust_set_param(PARAM_MIX_MILLI, 1000);
            let input = [0.4f32; BLOCK_FRAMES];
            let mut out_l = [0.0f32; BLOCK_FRAMES];
            let mut out_r = [0.0f32; BLOCK_FRAMES];
            trust_process(
                input.as_ptr(),
                input.as_ptr(),
                out_l.as_mut_ptr(),
                out_r.as_mut_ptr(),
                BLOCK_FRAMES as u32,
            );
            let crushed = trust_dsp::crush_sample(0.4, 4);
            assert!((out_l[0] - crushed).abs() < 1e-5);
            assert!((out_r[BLOCK_FRAMES - 1] - crushed).abs() < 1e-5);
            assert_eq!(trust_block_frames(), 128);
        }
    }
}

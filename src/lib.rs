//! Set the panicking behavior to enable the d13 red led on the back of the
//! pygamer. It steals the underlying d13 pin, sets it to function_a and then
//! into_push_pull before enabling, so you COULD still use it for during normal
//! program operation.
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! use pygamer_panic_led as _;
//!
//! fn main() {
//!     panic!("argument is ignored");
//! }
//! ```
//!
//! # Breakable symbols
//!
//! With the panic handler being `#[inline(never)]` the symbol
//! `rust_begin_unwind` will be available to place a breakpoint on to halt when
//! a panic is happening.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use pygamer as hal;

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        //pin d13 a23
        let pin_no = 23;
        let variant = 0;
        let pinmux = &(*hal::pac::PORT::ptr()).group0.pmux;
        let pincfg = &(*hal::pac::PORT::ptr()).group0.pincfg;
        let dirset = &(*hal::pac::PORT::ptr()).group0.dirset;
        let outset = &(*hal::pac::PORT::ptr()).group0.outset;

        //into_function_a
        pinmux[pin_no >> 1].modify(|_, w| {
            if pin_no & 1 == 1 {
                // Odd-numbered pin
                w.pmuxo().bits(variant)
            } else {
                // Even-numbered pin
                w.pmuxe().bits(variant)
            }
        });

        pincfg[pin_no].modify(|_, bits| bits.pmuxen().set_bit());

        //into_push_pull
        dirset.write(|bits| {
            bits.bits(1 << pin_no);
            bits
        });

        pincfg[pin_no].write(|bits| {
            bits.pmuxen().clear_bit();
            bits.inen().set_bit();
            bits.pullen().clear_bit();
            bits.drvstr().clear_bit();
            bits
        });

        //set_high
        outset.write(|bits| {
            bits.bits(1 << pin_no);
            bits
        });
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

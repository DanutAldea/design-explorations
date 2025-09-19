#![no_main]
#![no_std]

use allow_pin::{command, trait_based::AllowRo, trait_based::*};
use core::pin::{Pin, pin};

static WELCOME: [u8; 2] = *b"hi";

fn console_write<A: AllowRo<[u8]> + ?Sized>(to_write: Pin<&mut A>) -> Result<(), u32> {
    to_write.allow_ro(0x1, 0x1)?;
    command(0x1, 0x1, 14, 0)?;
    // Wait for an upcall here.
    Ok(())
}

#[unsafe(no_mangle)]
fn _start() -> Result<(), u32> {
    let mut buffer: Pin<&mut StaticBuffer<[u8]>> =
        pin!(StaticBuffer::<[u8]>::from(&WELCOME as &[u8]));
    console_write(buffer.as_mut())?;

    let mut buffer: Pin<&mut Buffer<[u8]>> = pin!(Buffer::<[u8; _]>::from(*b"my"));
    console_write(buffer.as_mut())?;
    todo!()
}

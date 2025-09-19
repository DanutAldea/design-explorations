#![no_main]
#![no_std]

use allow_pin::{command, full_dynamic::*};
use core::pin::{Pin, pin};

static WELCOME: [u8; 2] = *b"hi";

fn console_write_stack(to_write: Pin<&mut Buffer<[u8]>>) -> Result<(), u32> {
    to_write.allow(DynamicType::Ro, 0x1, 0x1)?;
    command(0x1, 0x1, 14, 0)?;
    // Wait for an upcall here.
    Ok(())
}

fn console_write_static(to_write: Pin<&mut StaticBuffer<[u8]>>) -> Result<(), u32> {
    to_write.allow(0x1, 0x1)?;
    command(0x1, 0x1, 14, 0)?;
    // Wait for an upcall here.
    Ok(())
}

#[unsafe(no_mangle)]
fn _start() -> Result<(), u32> {
    let mut buffer = pin!(StaticBuffer::<[u8]>::from(&WELCOME as &[u8]));
    console_write_static(buffer.as_mut())?;

    let mut buffer = pin!(Buffer::<[u8; _]>::from(*b"my"));
    console_write_stack(buffer.as_mut())
}

#![crate_type = "lib"]
#![crate_name = "rust_pigpio"]
#![allow(dead_code)]
#![allow(non_snake_case)]

//! #Rust PiGPIO
//!
//! The Rust wrapper of the C library functions
pub mod constants;
pub mod pwm;

use std::string::String;

use constants::*;

const OK: i32 = 0;
const INIT_FAILED: i32 = -1;
const BAD_USER_GPIO: i32 = -2;
const BAD_GPIO: i32 = -3;
const BAD_MODE: i32 = -4;
const BAD_LEVEL: i32 = -5;
const BAD_PUD: i32 = -6;
const DEFAULT_ERROR: &str = "Unknown error.";

pub const INPUT: GpioMode = GpioMode::INPUT;
pub const OUTPUT: GpioMode = GpioMode::OUTPUT;

pub const ON: Level = Level::ON;
pub const OFF: Level = Level::OFF;

pub type GpioResult = Result<(), String>;
pub type GpioResponse = Result<u32, String>;

#[link(name = "pigpio", kind = "dylib")]
extern "C" {
    fn gpioInitialise() -> i32;
    fn gpioTerminate();
    fn gpioSetMode(gpio: u32, mode: u32) -> i32;
    fn gpioGetMode(gpio: u32) -> i32;
    fn gpioSetPullUpDown(gpio: u32, pud: u32) -> i32;
    fn gpioRead(gpio: u32) -> i32;
    fn gpioWrite(gpio: u32, level: u32) -> i32;
    fn gpioDelay(micros: u32) -> u32;
    fn gpioTick() -> u32;
    fn gpioSetAlertFunc(user_gpio: u32, alert_func: extern "C" fn(u32, u32, u32)) -> i32;
    fn gpioTrigger(user_gpio: u32, pulseLen: u32, level: u32) -> i32;
    fn gpioSetWatchdog(user_gpio: u32, timeout: u32) -> i32;
}

/// Initializes the library.
///
/// Initialize must be called before using the other library functions with some exceptions not yet wrapped.
pub fn initialize() -> GpioResponse {
    let result = unsafe { gpioInitialise() };
    match result {
        INIT_FAILED => Err("Initialize failed".to_string()),
        _ => Ok(result as u32),
    }
}

/// Terminates the library.
///
/// Call before program exit.
/// This function resets the used DMA channels, releases memory, and terminates any running threads.
pub fn terminate() {
    unsafe { gpioTerminate() };
}

/// Sets the GPIO mode, typically input or output.
pub fn set_mode(gpio: u32, mode: GpioMode) -> GpioResult {
    match unsafe { gpioSetMode(gpio, mode as u32) } {
        OK => Ok(()),
        BAD_GPIO => Err("Bad gpio".to_string()),
        BAD_MODE => Err("Bad mode".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Gets the GPIO mode.
pub fn get_mode(gpio: u32) -> GpioResponse {
    let response = unsafe { gpioGetMode(gpio) };
    match response {
        BAD_GPIO => Err("Bad gpio".to_string()),
        _ => Ok(response as u32),
    }
}

/// Sets or clears resistor pull ups or downs on the GPIO.
pub fn set_pull_up_down(gpio: u32, pud: Pud) -> GpioResult {
    match unsafe { gpioSetPullUpDown(gpio, pud as u32) } {
        OK => Ok(()),
        BAD_GPIO => Err("Bad gpio".to_string()),
        BAD_PUD => Err("Bad pud".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Reads the GPIO level, on or off.
pub fn read(gpio: u32) -> GpioResponse {
    match unsafe { gpioRead(gpio) } {
        1 => Ok(1),
        0 => Ok(0),
        BAD_GPIO => Err("Bad gpio".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Sets the GPIO level, on or off.
/// If PWM or servo pulses are active on the GPIO they are switched off.
pub fn write(gpio: u32, level: Level) -> GpioResult {
    match unsafe { gpioWrite(gpio, level as u32) } {
        OK => Ok(()),
        BAD_GPIO => Err("Bad gpio".to_string()),
        BAD_LEVEL => Err("Bad level".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

/// Delays for at least the number of microseconds specified by microseconds.
pub fn delay(microseconds: u32) -> u32 {
    unsafe { gpioDelay(microseconds) }
}

/// Returns the current system tick.
///
/// Tick is the number of microseconds since system boot.
///
/// As tick is an unsigned 32 bit quantity it wraps around after 2^32 microseconds, which
/// is approximately 1 hour 12 minutes.
pub fn system_tick() -> u32 {
    unsafe { gpioTick() }
}

/// Registers a function to be called (a callback) when the specified GPIO changes state
// http://abyz.me.uk/rpi/pigpio/cif.html#gpioSetAlertFunc
pub fn set_alert_func(gpio: u32, alert_func: extern "C" fn(u32, u32, u32)) -> GpioResult {
    match unsafe { gpioSetAlertFunc(gpio, alert_func) } {
        OK => Ok(()),
        BAD_USER_GPIO => Err("Bad user gpio".to_string()),
        _ => Err(DEFAULT_ERROR.to_string()),
    }
}

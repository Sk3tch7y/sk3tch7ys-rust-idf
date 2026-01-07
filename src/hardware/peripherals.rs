use crate::hardware::peripherals::gpio::AnyOutputPin;
use crate::hardware::peripherals::gpio::PinDriver;
use crate::hardware::pins;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::Pin;
use esp_idf_hal::peripherals::Peripherals;
use once_cell::sync::Lazy;
use std::boxed::Box;
use std::sync::Mutex;

pub static PERIPHERALS: Lazy<Mutex<Option<Box<Peripherals>>>> = Lazy::new(|| Mutex::new(None));

pub fn peripherals_init() {
    let mut guard = PERIPHERALS.lock().unwrap();
    if guard.is_none() {
        match Peripherals::take() {
            Ok(peripherals) => {
                *guard = Some(Box::new(peripherals));
                assert!(guard.is_some());
            }
            Err(_e) => {
                // if this happens, fault the program
                assert!(false);
            }
        }
    }
}

pub fn get_esp_output_pin(pin: pins::Pin) -> Option<PinDriver<AnyOutputPin, Output>> {
    //take peripherals lock

    let guard = PERIPHERALS.lock().unwrap();
    let peripherals = guard.as_ref().unwrap();

    match pin {
        pins::Pin::GPIO0 => Some(PinDriver::output(pins.gpio0.take().unwrap()).ok()?),
        pins::Pin::GPIO1 => Some(PinDriver::output(pins.gpio1.take().unwrap()).ok()?),
        // Add more pins as needed
    }
}

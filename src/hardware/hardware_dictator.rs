use crate::hardware::peripherals::PERIPHERALS;
use crate::logger;
use crate::message::message;
use crate::message::msg;
use esp_idf_hal::gpio::PinDriver;
use once_cell::sync::Lazy;

pub unsafe extern "C" fn hardware_dictator_task_wrapper(_arg: *mut core::ffi::c_void) {
    hardware_dictator_task();
}

static HARDWARE_DICTATOR_LOGGER: Lazy<logger::log::Logger> =
    Lazy::new(|| logger::log::Logger::new("HARDWARE_DICTATOR"));

static mut MSG_QUEUE: msg::MsgQueueHandle = 0x7FFFFF;

pub fn hardware_dictator_init() {
    unsafe {
        MSG_QUEUE = msg::create_msg_queue();
        msg::subscribe(MSG_QUEUE, message::MessageType::MidHardwareRequest);
        msg::subscribe(MSG_QUEUE, message::MessageType::MidHardwareRelease);
    }
}

pub fn hardware_dictator_task() {
    //intialization
    hardware_dictator_init();
    //core task loop
    loop {
        if let Some(msg) = unsafe { msg::get(MSG_QUEUE, 1000) } {
            let msg: bool = msg.payload.downcast_ref::<bool>();
            if led_mode {
                turn_led_on(&mut led);
            } else {
                turn_led_off(&mut led);
            }
        }
    }
}

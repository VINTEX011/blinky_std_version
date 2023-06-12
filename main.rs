use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use esp_idf_sys as _;
use esp_println::println;

fn main() {
    esp_idf_sys::link_patches();

    println!("Starting 0-input");
    println!("This application is a basic blinky program that turns an LED on and off every 1 second.\n");

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();

    loop {
        println!("LED ON");
        FreeRtos::delay_ms(1000);

        led_pin.set_high().unwrap();
        println!("LED OFF");
        FreeRtos::delay_ms(1000);
    }
}

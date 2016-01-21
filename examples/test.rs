extern crate ws2811;

use ws2811::*;
use std::mem;

fn main() {
  println!("Starting");
  unsafe {
      let mut ledstring = ws2811_t {
        device: mem::uninitialized(),
        rpi_hw: mem::uninitialized(),
        freq: 800000,
        dmanum: 5,
        channel: [ws2811_channel_t {
                    gpionum: 18,
                    invert: 0,
                    count: 256,
                    brightness: 32,
                    strip_type: mem::uninitialized(),
                    leds: mem::uninitialized(),
                }, 
                  ws2811_channel_t {
                    gpionum: 0,
                    invert: 0,
                    count: 0,
                    brightness: 0,
                    strip_type: mem::uninitialized(),
                    leds: mem::uninitialized(),
                }]
      };
        ws2811_init(&mut ledstring);
   }
}

unsafe fn set_led(foo: &mut ws2811_t, index: isize, value: u32) {
    *foo.channel[0].leds.offset(index) = value
}
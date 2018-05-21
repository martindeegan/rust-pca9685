#![cfg(target_os = "linux")]

extern crate pca9685;
extern crate linux_embedded_hal;

use linux_embedded_hal::{I2cdev, Delay};

use pca9685::*;
use std::time::{Instant,Duration};
use std::thread::sleep_ms;

fn main() {
	let device = I2cdev::new("/dev/i2c-1", 0x40).unwrap();
	let mut pca9685 = PCA9685::new(device, Delay::new(), 50).unwrap();
	pca9685.set_frequency(100);
	pca9685.set_all_duty_cycle(0);

	let servo_max = 2000.0;
	let servo_min = 1000.0;
	let mut i = servo_min;
	pca9685.set_all_duty_cycle(0);
	sleep_ms(3000);

	// Arm
	// pca9685.set_all_pulse_length(1100.0);
	// sleep_ms(1000);
	pca9685.set_pulse_length(0, servo_min);
	sleep_ms(3000);

	pca9685.set_pulse_length(0, 1250.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1200.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1150.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1100.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1050.0);
	sleep_ms(1000);
	pca9685.set_pulse_length(0, 1250.0);
	sleep_ms(3000);

	pca9685.set_all_duty_cycle(0);

	// loop {
	// 	if i < servo_max {
	// 		i += 1;
	// 		println!("{}", i);
	// 		pca9685.set_all_pulse_length(0, (i as f32) / 4096.0 * 10000.0);
	// 		std::thread::sleep_ms(10);
	// 	}
	// 	else {
	// 		loop {
	// 			if i > servo_min {
	// 				i -= 1;
	// 				println!("{}", i);
	// 				pca9685.set_all_pulse_length(0, (i as f32) / 4096.0 * 10000.0);
	// 				std::thread::sleep_ms(10);
	// 			}
	// 			else {
	// 				break;
	// 			}
	// 		}
	// 	}
	// }
}
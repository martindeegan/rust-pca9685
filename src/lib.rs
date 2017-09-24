extern crate i2cdev;
extern crate byteorder;

use i2cdev::core::I2CDevice;
pub use i2cdev::linux::{LinuxI2CDevice,LinuxI2CError};
use std::thread::sleep;
use std::time::Duration;

const MODE_1_REG: u8 = 0x00;
const MODE_2_REG: u8 = 0x01;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_ON_L: u8 = 0xFA;
const ALL_ON_H: u8 = 0xFB;
const ALL_OFF_L: u8 = 0xFC;
const ALL_OFF_H: u8 = 0xFD;
const PRE_SCALE_REG: u8 = 0xFE;

const AUTO_INCREMENT: u8 = 0b1 << 5;

pub struct PCA9685 {
	pub device: LinuxI2CDevice,
	mode: u8,
	frequency: f32,
	period: f32,
	time_per_tick: f32
}

impl PCA9685 {
	pub fn new(device: LinuxI2CDevice, frequency: u16) -> Result<PCA9685, LinuxI2CError> {
		let mut mode1 = 0x01;
		let mut pca9685 = PCA9685{ device: device, mode: 0x01, frequency: 0.0, period: 0.0, time_per_tick: 0.0 };
		pca9685.set_all_duty_cycle(0);
		pca9685.device.smbus_write_byte_data(MODE_2_REG, 0x04);
		pca9685.device.smbus_write_byte_data(MODE_1_REG, mode1);
		sleep(Duration::from_millis(6));
		mode1 &= !0x01;
		pca9685.device.smbus_write_byte_data(MODE_1_REG, mode1);
		pca9685.mode = mode1;
		sleep(Duration::from_millis(6));
		Ok(pca9685)
	}

	/// 'frequency' must be between 40 and 1000
	pub fn set_frequency(&mut self, frequency: u16) -> Result<(), LinuxI2CError> {
		assert!(frequency >= 40 && frequency <= 1000);
		self.frequency = frequency as f32;
		self.period = 1000000.0 / (frequency as f32);
		self.time_per_tick = self.period / 4096.0;

		let mut prescalelevel = 25000000.0;
		prescalelevel /= 4096.0;
		prescalelevel /= frequency as f32;
		prescalelevel -= 1.0;
		try!(self.device.smbus_write_byte_data(MODE_1_REG, (self.mode & 0x7F) | 0x10));
		try!(self.device.smbus_write_byte_data(PRE_SCALE_REG, prescalelevel as u8));
		try!(self.device.smbus_write_byte_data(MODE_1_REG, self.mode));
		sleep(Duration::from_millis(6));
		try!(self.device.smbus_write_byte_data(MODE_1_REG, self.mode | 0x80));
		Ok(())
	}

	/// 'duty_cycle' must be between 0 and 4095.
	pub fn set_duty_cycle(&mut self, channel: u8, duty_cycle: u16) -> Result<(), LinuxI2CError> {
		assert!(duty_cycle < 4096);
		// let off = 4095 - duty_cycle;
		try!(self.device.smbus_write_byte_data(LED0_ON_L+4*channel, 0));
		try!(self.device.smbus_write_byte_data(LED0_ON_H+4*channel, 0));
		try!(self.device.smbus_write_byte_data(LED0_OFF_L+4*channel, (duty_cycle & 0xFF) as u8));
		try!(self.device.smbus_write_byte_data(LED0_OFF_H+4*channel, (duty_cycle >> 8) as u8));
		Ok(())
	}

	/// 'duty_cycle' must be between 0 and 4095.
	pub fn set_all_duty_cycle(&mut self, duty_cycle: u16) -> Result<(), LinuxI2CError> {
		assert!(duty_cycle < 4096);
		// let off = 4095 - duty_cycle;
		try!(self.device.smbus_write_byte_data(ALL_ON_L, 0));
		try!(self.device.smbus_write_byte_data(ALL_ON_H, 0));
		try!(self.device.smbus_write_byte_data(ALL_OFF_L, (duty_cycle & 0xFF) as u8));
		try!(self.device.smbus_write_byte_data(ALL_OFF_H, (duty_cycle >> 8) as u8));
		Ok(())
	}

	/// 'us' must be less than 1 / frequency.
	pub fn set_pulse_length(&mut self, channel: u8, us: f32) -> Result<(), LinuxI2CError> {
		assert!(us < self.period);
		let duty_cycle = (us / self.time_per_tick) as u16;
		try!(self.set_duty_cycle(channel, duty_cycle));
		Ok(())
	}

	/// 'us' must be less than 1 / frequency.
	pub fn set_all_pulse_length(&mut self, us: f32) -> Result<(), LinuxI2CError> {
		assert!(us < self.period);
		let duty_cycle = (us / self.time_per_tick) as u16;
		try!(self.set_all_duty_cycle(duty_cycle));
		Ok(())
	}
}






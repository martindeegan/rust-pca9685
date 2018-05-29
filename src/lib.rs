#![no_std]

#![feature(never_type)]
#![feature(unproven)]
extern crate embedded_hal as hal;
extern crate futures;

extern crate nb;
extern crate byteorder;

use hal::blocking::{i2c, delay};

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

/// PCA9685 Object
pub struct PCA9685<I2C, DELAY> {
	pub device: I2C,
	delay: DELAY,
	mode: u8,
	frequency: f32,
	period: f32,
	time_per_tick: f32
}

/// PCA9685 Error types (underlying I2C errors are re-exported)
pub enum PCA9685Error<I2CError> {
	I2C(I2CError),
	InvalidFrequency(u16),
	InvalidDutyCycle(u16),
	InvalidPulseLength(f32, f32),
}

impl <I2CError>From<I2CError> for PCA9685Error<I2CError> {
	fn from(e: I2CError) -> PCA9685Error<I2CError> {
		PCA9685Error::I2C(e)
	}
}

impl<E, I2C, DELAY> PCA9685<I2C, DELAY>
where
    I2C: i2c::Write<Error = E> + i2c::Read <Error = E> + i2c::WriteRead<Error = E>,
	DELAY: delay::DelayMs<usize>,
{
	pub fn new(device: I2C, delay: DELAY, frequency: u16) -> Result<Self, PCA9685Error<E>> {
		let mut mode1 = 0x01;
		let mut pca9685 = PCA9685{ device: device, delay: delay, mode: 0x01, frequency: 0.0, period: 0.0, time_per_tick: 0.0 };

		pca9685.set_all_duty_cycle(0)?;
		pca9685.device.write(MODE_2_REG, &[0x04])?;
		pca9685.device.write(MODE_1_REG, &[mode1])?;
		pca9685.delay.delay_ms(6);

		mode1 &= !0x01;
		pca9685.device.write(MODE_1_REG, &[mode1])?;
		pca9685.mode = mode1;
		pca9685.delay.delay_ms(6);

		pca9685.set_frequency(frequency)?;

		Ok(pca9685)
	}

	/// 'frequency' must be between 40 and 1000
	pub fn set_frequency(&mut self, frequency: u16) -> Result<(), PCA9685Error<E>> {
		//Check frequency bounds
		if frequency >= 40 && frequency <= 1000 {
			return Err(PCA9685Error::InvalidFrequency(frequency))
		}

		// Calculate freq/period/prescaler
		self.frequency = frequency as f32;
		self.period = 1000000.0 / (frequency as f32);
		self.time_per_tick = self.period / 4096.0;

		let mut prescale_level = 25000000.0;
		prescale_level /= 4096.0;
		prescale_level /= frequency as f32;
		prescale_level -= 1.0;

		// Write configuration
		self.device.write(MODE_1_REG, 		&[(self.mode & 0x7F) | 0x10])?;
		self.device.write(PRE_SCALE_REG, 	&[prescale_level as u8])?;
		self.device.write(MODE_1_REG, 		&[self.mode])?;
		self.delay.delay_ms(6);

		self.device.write(MODE_1_REG, &[self.mode | 0x80])?;
		Ok(())
	}

	/// 'duty_cycle' must be between 0 and 4095.
	pub fn set_duty_cycle(&mut self, channel: u8, duty_cycle: u16) -> Result<(), PCA9685Error<E>> {
		if duty_cycle < 4096 {
			return Err(PCA9685Error::InvalidDutyCycle(duty_cycle))
		}

		// let off = 4095 - duty_cycle;
		self.device.write(LED0_ON_L+4*channel, &[0])?;
		self.device.write(LED0_ON_H+4*channel, &[0])?;
		self.device.write(LED0_OFF_L+4*channel, &[(duty_cycle & 0xFF) as u8])?;
		self.device.write(LED0_OFF_H+4*channel, &[(duty_cycle >> 8) as u8])?;
		Ok(())
	}

	/// 'duty_cycle' must be between 0 and 4095.
	pub fn set_all_duty_cycle(&mut self, duty_cycle: u16) -> Result<(), PCA9685Error<E>> {
		if duty_cycle < 4096 {
			return Err(PCA9685Error::InvalidDutyCycle(duty_cycle))
		}

		// let off = 4095 - duty_cycle;
		self.device.write(ALL_ON_L, &[0])?;
		self.device.write(ALL_ON_H, &[0])?;
		self.device.write(ALL_OFF_L, &[(duty_cycle & 0xFF) as u8])?;
		self.device.write(ALL_OFF_H, &[(duty_cycle >> 8) as u8])?;
		Ok(())
	}

	/// 'us' must be less than 1 / frequency.
	pub fn set_pulse_length(&mut self, channel: u8, us: f32) -> Result<(), PCA9685Error<E>> {
		if us < self.period{
			return Err(PCA9685Error::InvalidPulseLength(us, self.period))
		}

		let duty_cycle = (us / self.time_per_tick) as u16;
		try!(self.set_duty_cycle(channel, duty_cycle));
		Ok(())
	}

	/// 'us' must be less than 1 / frequency.
	pub fn set_all_pulse_length(&mut self, us: f32) -> Result<(), PCA9685Error<E>> {
		if us < self.period{
			return Err(PCA9685Error::InvalidPulseLength(us, self.period))
		}

		let duty_cycle = (us / self.time_per_tick) as u16;
		try!(self.set_all_duty_cycle(duty_cycle));
		Ok(())
	}
}






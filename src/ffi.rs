// FFI bindings to rtl-sdr.h (librtlsdr)
// Copyright Adam Greig <adam@adamgreig.com> 2014
// Licensed under MIT license

use libc::{c_void, c_int, c_char, c_uchar};

// rtlsdr_tuner enum
pub const RTLSDR_TUNER_UNKNOWN: c_int = 0;
pub const RTLSDR_TUNER_E4000: c_int = 1;
pub const RTLSDR_TUNER_FC0012: c_int = 2;
pub const RTLSDR_TUNER_FC0013: c_int = 3;
pub const RTLSDR_TUNER_FC2580: c_int = 4;
pub const RTLSDR_TUNER_R820T: c_int = 5;
pub const RTLSDR_TUNER_R828D: c_int = 6;

#[allow(non_camel_case_types)]
pub enum rtlsdr_dev {}

#[link(name="rtlsdr")]
unsafe extern "C" {
    pub fn rtlsdr_get_device_count() -> u32;
    pub fn rtlsdr_get_device_name(index: u32) -> *const c_char;

    // String buffers must be 256 bytes
    // Returns 0 on success.
    pub fn rtlsdr_get_device_usb_strings(
        index: u32, manufact: *mut c_char,
        product: *mut c_char, serial: *mut c_char) -> c_int;

    // Returns the index of the first matching device on success, -1 if name is
    // NULL, -2 if no devices found, -3 if no matching devices found
    pub fn rtlsdr_get_index_by_serial(serial: *const c_char) -> c_int;

    pub fn rtlsdr_open(dev: *mut *mut rtlsdr_dev, index: u32) -> c_int;
    pub fn rtlsdr_close(dev: *mut rtlsdr_dev) -> c_int;

    // rtl_freq and tuner_freq in Hz
    // Returns 0 on success.
    pub fn rtlsdr_set_xtal_freq(dev: *mut rtlsdr_dev, rtl_freq: u32,
                                tuner_freq: u32) -> c_int;

    // rtl_freq and tuner_freq in Hz
    // Returns 0 on success.
    pub fn rtlsdr_get_xtal_freq(dev: *mut rtlsdr_dev, rtl_freq: *mut u32,
                                tuner_freq: *mut u32) -> c_int;

    // String buffers must be 256 bytes
    // Returns 0 on success.
    pub fn rtlsdr_get_usb_strings(dev: *mut rtlsdr_dev, manufact: *mut c_char,
                                  product: *mut c_char, serial: *mut c_char)
                                  -> c_int;

    // Returns 0 on success, -1 if device handle invalid, -2 if EEPROM size is
    // exceeded, -3 if no EEPROM was found
    pub fn rtlsdr_write_eeprom(dev: *mut rtlsdr_dev, data: *mut u8,
                               offset: u8, len: u16) -> c_int;

    // Returns 0 on success, -1 if device handle invalid, -2 if EEPROM size is
    // exceeded, -3 if no EEPROM was found
    pub fn rtlsdr_read_eeprom(dev: *mut rtlsdr_dev, data: *mut u8,
                              offset: u8, len: u16) -> c_int;

    pub fn rtlsdr_set_center_freq(dev: *mut rtlsdr_dev, freq: u32)
                                  -> c_int;

    // Returns frequency in Hz (or 0 on error)
    pub fn rtlsdr_get_center_freq(dev: *mut rtlsdr_dev) -> u32;

    // Returns 0 on success
    pub fn rtlsdr_set_freq_correction(dev: *mut rtlsdr_dev, ppm: c_int)
                                      -> c_int;

    // Return correction value in ppm
    pub fn rtlsdr_get_freq_correction(dev: *mut rtlsdr_dev) -> c_int;

    // Returns an rtlsdr_tuner enum
    pub fn rtlsdr_get_tuner_type(dev: *mut rtlsdr_dev) -> c_int;

    // Call with `gains=NULL` to return length of `gains`, then allocate and
    // call again to have `gains` populated.
    // Returns number of available gains (<=0 on error)
    pub fn rtlsdr_get_tuner_gains(dev: *mut rtlsdr_dev, gains: *mut c_int)
                                  -> c_int;

    // Returns 0 on success
    pub fn rtlsdr_set_tuner_gain(dev: *mut rtlsdr_dev, gain: c_int) -> c_int;

    // Returns gain (in tengths of dB) (0 on error)
    pub fn rtlsdr_get_tuner_gain(dev: *mut rtlsdr_dev) -> c_int;

    // Returns 0 on sucess
    pub fn rtlsdr_set_tuner_if_gain(dev: *mut rtlsdr_dev, stage: c_int,
                                    gain: c_int) -> c_int;

    // Returns 0 on success
    pub fn rtlsdr_set_tuner_gain_mode(dev: *mut rtlsdr_dev, manual: c_int)
                                      -> c_int;

    // bw=0 means automatic bandwidth selection
    // return 0 on success
    pub fn rtlsdr_set_tuner_bandwidth(dev: *mut rtlsdr_dev, bw: u32) -> c_int;

    pub fn rtlsdr_set_sample_rate(dev: *mut rtlsdr_dev, rate: u32)
                                  -> c_int;

    // Returns sample rate in Hz (0 on error)
    pub fn rtlsdr_get_sample_rate(dev: *mut rtlsdr_dev) -> u32;

    // 1 = enable test
    // 0 = disable test
    // Returns 0 on success
    pub fn rtlsdr_set_testmode(dev: *mut rtlsdr_dev, on: c_int) -> c_int;

    // 1 = enable AGC
    // 0 = disable AGC
    // Returns 0 on success
    pub fn rtlsdr_set_agc_mode(dev: *mut rtlsdr_dev, on: c_int) -> c_int;

    // 0 = disable direct sampling
    // 1 = I-ADC input enabled
    // 2 = Q-ADC input enabled
    // Returns 0 on success
    pub fn rtlsdr_set_direct_sampling(dev: *mut rtlsdr_dev, on: c_int)
                                      -> c_int;
    
    // Returns -1 for error, 0 for disabled, 1 for I-ADC, 2 for Q-ADC
    pub fn rtlsdr_get_direct_sampling(dev: *mut rtlsdr_dev) -> c_int;

    // 1 = enable offset tuning
    // 0 = disable offset tuning
    // Returns 0 for success
    pub fn rtlsdr_set_offset_tuning(dev: *mut rtlsdr_dev, on: c_int) -> c_int;

    // Returns -1 for error, 0 for disabled, 1 for enabled
    pub fn rtlsdr_get_offset_tuning(dev: *mut rtlsdr_dev) -> c_int;

    pub fn rtlsdr_reset_buffer(dev: *mut rtlsdr_dev) -> c_int;
    pub fn rtlsdr_read_sync(dev: *mut rtlsdr_dev, buf: *mut c_void, len: c_int,
                            n_read: *mut c_int) -> c_int;

    // Set `buf_num` to 0 for default of 32
    // Set `buf_len` to 0 for default of 16 * 32 * 512
    // Returns 0 on success
    pub fn rtlsdr_read_async(dev: *mut rtlsdr_dev,
                             cb: extern fn(*mut c_uchar,
                                           u32, *mut c_void),
                             buf_num: u32, buf_len: u32) -> c_int;
    
    // Returns 0 on success
    pub fn rtlsdr_cancel_async(dev: *mut rtlsdr_dev) -> c_int;

}

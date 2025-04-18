// Rust interface to librtlsdr FFI bindings
// Copyright Adam Greig <adam@adamgreig.com> 2014
// Licensed under MIT license

#![allow(dead_code)]

extern crate libc;
mod ffi;

#[derive(Debug)]
pub struct RTLSDRError {
    errno: i32,
    errstr: String
}

impl std::fmt::Display for RTLSDRError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RTL-SDR error: {} ({})", self.errstr, self.errno)
    }
}

impl std::error::Error for RTLSDRError {}

fn rtlsdr_error(errno: libc::c_int, errstr: &str) -> RTLSDRError {
    RTLSDRError { errno: errno as i32, errstr: errstr.to_string() }
}

#[derive(Clone, Copy,Debug)]
pub enum DirectSampling {
    Disabled, I, Q
}

pub struct RTLSDRDevice {
    ptr: *mut ffi::rtlsdr_dev
}

impl Drop for RTLSDRDevice {
    #[inline(never)]
    fn drop(&mut self) {
        unsafe { ffi::rtlsdr_close(self.ptr); }
    }
}

/// Get the number of detected RTL-SDR devices.
pub fn get_device_count() -> i32 {
    let count = unsafe { ffi::rtlsdr_get_device_count() };
    count as i32
}

/// Get the name for a specific RTL-SDR device index.
pub fn get_device_name(index: i32) -> String {
    let s = unsafe { ffi::rtlsdr_get_device_name(index as u32) };
    let slice = unsafe { std::ffi::CStr::from_ptr(s).to_bytes() };
    std::str::from_utf8(slice).unwrap().to_string()
}

/// A set of USB strings for an RTL-SDR device.
pub struct USBStrings {
    pub manufacturer: String, pub product: String, pub serial: String
}

/// Get the USB strings for a specific RTL-SDR device index.
pub fn get_device_usb_strings(index: i32)
                                     -> Result<USBStrings, RTLSDRError> {
    let mut mn: [libc::c_char; 256] = [0; 256];
    let mut pd: [libc::c_char; 256] = [0; 256];
    let mut sr: [libc::c_char; 256] = [0; 256];
    match unsafe { ffi::rtlsdr_get_device_usb_strings(index as u32,
                                                      mn.as_mut_ptr(),
                                                      pd.as_mut_ptr(),
                                                      sr.as_mut_ptr()) } {
        0 => unsafe { Ok(USBStrings {
            manufacturer: std::str::from_utf8(
                std::ffi::CStr::from_ptr(mn.as_ptr()).to_bytes())
                .unwrap().to_string(),
            product: std::str::from_utf8(
                std::ffi::CStr::from_ptr(pd.as_ptr()).to_bytes())
                .unwrap().to_string(),
            serial: std::str::from_utf8(
                std::ffi::CStr::from_ptr(sr.as_ptr()).to_bytes())
                .unwrap().to_string()
        })},
        err => Err(rtlsdr_error(err, "Unknown"))
    }
}

/// Get the index of a specific RTL-SDR by serial number.
pub fn get_index_by_serial(serial: String) -> Result<i32, RTLSDRError> {
    let s = std::ffi::CString::new(serial).unwrap();
    match unsafe { ffi::rtlsdr_get_index_by_serial(s.as_ptr()) } {
        -1 => Err(rtlsdr_error(-1, "No name provided")),
        -2 => Err(rtlsdr_error(-2, "No devices found")),
        -3 => Err(rtlsdr_error(-3, "No devices with that name found")),
        index => Ok(index as i32)
    }
}

/// Open an RTL-SDR device by index.
///
/// Returns a Result on an RTLSDRDevice object which exposes further
/// methods.
pub fn open(index: i32) -> Result<RTLSDRDevice, RTLSDRError> {
    let mut device: RTLSDRDevice = RTLSDRDevice { ptr: std::ptr::null_mut() };
    let idx = index as u32;
    match unsafe { ffi::rtlsdr_open(&mut device.ptr, idx) } {
        0 => Ok(device),
        err => Err(rtlsdr_error(err, "Unknown"))
    }
}

impl RTLSDRDevice {
    /// Close a previously opened RTL-SDR device.
    pub fn close(&mut self) -> Result<(), RTLSDRError> {
        match unsafe { ffi::rtlsdr_close(self.ptr) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set crystal frequency (in Hz) for an opened device.
    ///
    /// NOTE: Only do this if you know what you're doing, for instance if you
    /// are injecting your own clock signal or correcting for the frequency of
    /// the onboard xtal. In general the RTL and the tuner will be fed from the
    /// same clock.
    pub fn set_xtal_freq(&mut self, rtl_freq: u32, tuner_freq: u32)
                         -> Result<(), RTLSDRError> {
        let rfreq = rtl_freq as u32;
        let tfreq = tuner_freq as u32;
        match unsafe { ffi::rtlsdr_set_xtal_freq(self.ptr, rfreq, tfreq) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get crystal frequency (in Hz) for an opened device.
    ///
    /// Returns a tuple of (RTL freq, tuner freq).
    pub fn get_xtal_freq(&mut self)
                         -> Result<(u32, u32), RTLSDRError> {
        let mut rtl_freq: u32 = 0;
        let mut tuner_freq: u32 = 0;
        match unsafe { ffi::rtlsdr_get_xtal_freq(self.ptr, &mut rtl_freq,
                                                 &mut tuner_freq) } {
            0 => Ok((rtl_freq as u32, tuner_freq as u32)),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get USB strings for an opened device.
    pub fn get_usb_strings(&mut self)
                           -> Result<USBStrings, RTLSDRError> {
        let mut mn: [libc::c_char; 256] = [0; 256];
        let mut pd: [libc::c_char; 256] = [0; 256];
        let mut sr: [libc::c_char; 256] = [0; 256];
        match unsafe { ffi::rtlsdr_get_usb_strings(self.ptr,
                                                   mn.as_mut_ptr(),
                                                   pd.as_mut_ptr(),
                                                   sr.as_mut_ptr()) } {
            0 => unsafe { Ok(USBStrings {
                manufacturer: std::str::from_utf8(
                    std::ffi::CStr::from_ptr(mn.as_ptr()).to_bytes())
                    .unwrap().to_string(),
                product: std::str::from_utf8(
                    std::ffi::CStr::from_ptr(pd.as_ptr()).to_bytes())
                    .unwrap().to_string(),
                serial: std::str::from_utf8(
                    std::ffi::CStr::from_ptr(sr.as_ptr()).to_bytes())
                    .unwrap().to_string()
            })},
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set the RTL-SDR's centre frequency (in Hz).
    pub fn set_center_freq(&mut self, frequency: u32)
                           -> Result<(), RTLSDRError> {
        let freq = frequency as u32;
        match unsafe { ffi::rtlsdr_set_center_freq(self.ptr, freq)} {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get the RTL-SDR's center frequency (in Hz).
    pub fn get_center_freq(&mut self) -> Result<u32, RTLSDRError> {
        match unsafe { ffi::rtlsdr_get_center_freq(self.ptr) } {
            0 => Err(rtlsdr_error(0, "Unknown")),
            freq => Ok(freq as u32)
        }
    }

    /// Set the RTL-SDR's frequency correction (in ppm).
    pub fn set_freq_correction(&mut self, ppm: i32) -> Result<(), RTLSDRError> {
        let cppm = ppm as libc::c_int;
        match unsafe { ffi::rtlsdr_set_freq_correction(self.ptr, cppm) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get the RTL-SDR's current frequency correction (in ppm).
    pub fn get_freq_correction(&mut self) -> i32 {
        let ppm = unsafe { ffi::rtlsdr_get_freq_correction(self.ptr) };
        ppm as i32
    }

    /// Get the RTL-SDR's tuner type.
    ///
    /// Returns a tuple (id: i32, name: String).
    pub fn get_tuner_type(&mut self) -> (i32, String) {
        match unsafe { ffi::rtlsdr_get_tuner_type(self.ptr) } {
            ffi::RTLSDR_TUNER_E4000 =>
                (ffi::RTLSDR_TUNER_E4000 as i32, "E4000".to_string()),
            ffi::RTLSDR_TUNER_FC0012 =>
                (ffi::RTLSDR_TUNER_FC0012 as i32, "FC0012".to_string()),
            ffi::RTLSDR_TUNER_FC0013 =>
                (ffi::RTLSDR_TUNER_FC0013 as i32, "FC0013".to_string()),
            ffi::RTLSDR_TUNER_FC2580 =>
                (ffi::RTLSDR_TUNER_FC2580 as i32, "FC2580".to_string()),
            ffi::RTLSDR_TUNER_R820T =>
                (ffi::RTLSDR_TUNER_R820T as i32, "R820T".to_string()),
            ffi::RTLSDR_TUNER_R828D =>
                (ffi::RTLSDR_TUNER_R828D as i32, "R828D".to_string()),
            _ => (ffi::RTLSDR_TUNER_UNKNOWN as i32, "Unknown".to_string())
        }
    }

    /// Get a Vec of allowable tuner gains.
    ///
    /// Gains are specified in tenths-of-a-dB. The number of allowable gains
    /// depends on the attached hardware.
    pub fn get_tuner_gains(&mut self)
                           -> Result<std::vec::Vec<i32>, RTLSDRError> {
        use std::vec::Vec;
        let null = std::ptr::null_mut();
        let len = unsafe { ffi::rtlsdr_get_tuner_gains(self.ptr, null) };
        if len > 0 {
            let mut out: Vec<libc::c_int> = Vec::with_capacity(len as usize);
            unsafe { out.set_len(len as usize) };
            match unsafe { ffi::rtlsdr_get_tuner_gains(self.ptr,
                                                       out.as_mut_ptr()) } {
                l if l == len => Ok(out.iter().map(|&g| g as i32).collect()),
                err => Err(rtlsdr_error(err, "Could not get list of gains"))
            }
        } else {
            Err(rtlsdr_error(len, "Could not get number of gains"))
        }
    }

    /// Set tuner gain (from list of allowable gains).
    pub fn set_tuner_gain(&mut self, gain: i32) -> Result<(), RTLSDRError> {
        let g = gain as libc::c_int;
        match unsafe { ffi::rtlsdr_set_tuner_gain(self.ptr, g) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get current tuner gain (in tenths of dB)
    pub fn get_tuner_gain(&mut self) -> i32 {
        let gain = unsafe { ffi::rtlsdr_get_tuner_gain(self.ptr) };
        gain as i32
    }

    /// Set tuner IF gain (in tenths of dB).
    ///
    /// `stage` specifies which intermediate gain stage to set (1-6 for E4000).
    pub fn set_tuner_if_gain(&mut self, stage: i32, gain: i32)
                             -> Result<(), RTLSDRError> {
        match unsafe { ffi::rtlsdr_set_tuner_if_gain(self.ptr,
                                                     stage as libc::c_int,
                                                     gain as libc::c_int) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set automatic or manual gain.
    ///
    /// Manual gain must be enabled for set_gain to work.
    pub fn set_tuner_gain_mode(&mut self, manual: bool)
                               -> Result<(), RTLSDRError> {
        let m: libc::c_int = match manual { true => 1, false => 0 };
        match unsafe { ffi::rtlsdr_set_tuner_gain_mode(self.ptr, m) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set sample rate (in Hz).
    pub fn set_sample_rate(&mut self, rate: u32) -> Result<(), RTLSDRError> {
        let r = rate as libc::c_uint;
        match unsafe { ffi::rtlsdr_set_sample_rate(self.ptr, r) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get current sample rate (in Hz).
    pub fn get_sample_rate(&mut self) -> Result<u32, RTLSDRError> {
        match unsafe { ffi::rtlsdr_get_sample_rate(self.ptr) } {
            0 => Err(rtlsdr_error(0, "Unknown")),
            rate => Ok(rate as u32)
        }
    }

    /// Set bandwidth (in Hz).
    pub fn set_tuner_bandwidth(&mut self, bw: u32) -> Result<(), RTLSDRError> {
        let bwc = bw as libc::c_uint;
        match unsafe { ffi::rtlsdr_set_tuner_bandwidth(self.ptr, bwc) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set test mode on or off.
    ///
    /// Test mode turns on an 8 bit counter rather than sampling the radio
    /// input. The counter is generated inside the RTL2832.
    pub fn set_test_mode(&mut self, enabled: bool) -> Result<(), RTLSDRError> {
        let t: libc::c_int = match enabled { true => 1, false => 0 };
        match unsafe { ffi::rtlsdr_set_testmode(self.ptr, t) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set AGC on or off.
    ///
    /// Enables or disables the internal digital AGC of the RTL2832.
    pub fn set_agc_mode(&mut self, enabled: bool) -> Result<(), RTLSDRError> {
        let a: libc::c_int = match enabled { true => 1, false => 0 };
        match unsafe { ffi::rtlsdr_set_agc_mode(self.ptr, a) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Set direct sampling.
    ///
    /// When enabled, the IF mode of the RTL2832 is activated, and
    /// set_center_freq will control the IF frequency, allowing tuning from 0
    /// to 28.8MHz.
    pub fn set_direct_sampling(&mut self, mode: DirectSampling)
                               -> Result<(), RTLSDRError> {
        let m: libc::c_int = match mode {
            DirectSampling::Disabled => 0,
            DirectSampling::I => 1,
            DirectSampling::Q => 2
        };

        match unsafe { ffi::rtlsdr_set_direct_sampling(self.ptr, m) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get current direct sampling mode.
    pub fn get_direct_sampling(&mut self)
                               -> Result<DirectSampling, RTLSDRError> {
        match unsafe { ffi::rtlsdr_get_direct_sampling(self.ptr) } {
            0 => Ok(DirectSampling::Disabled),
            1 => Ok(DirectSampling::I),
            2 => Ok(DirectSampling::Q),
            err => Err(rtlsdr_error(err, "Unknown")),
        }
    }

    /// Set offset tuning mode on/off. Only use on zero IF tuners.
    ///
    /// Useful to avoid DC offsets and 1/f noise.
    pub fn set_offset_tuning(&mut self, enabled: bool)
                             -> Result<(), RTLSDRError> {
        let e: libc::c_int = match enabled { true => 1, false => 0 };
        match unsafe { ffi::rtlsdr_set_offset_tuning(self.ptr, e) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Get current offset tuning status.
    pub fn get_offset_tuning(&mut self) -> Result<bool, RTLSDRError> {
        match unsafe { ffi::rtlsdr_get_offset_tuning(self.ptr) } {
            0 => Ok(false),
            1 => Ok(true),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Reset streaming buffer.
    pub fn reset_buffer(&mut self) -> Result<(), RTLSDRError> {
        match unsafe { ffi::rtlsdr_reset_buffer(self.ptr) } {
            0 => Ok(()),
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }

    /// Read a buffer synchronously.
    pub fn read_sync(&mut self, len: usize)
                     -> Result<std::vec::Vec<u8>, RTLSDRError> {
        use std::vec::Vec;
        let mut v: Vec<u8> = Vec::with_capacity(len);
        let mut n: libc::c_int = 0;
        let ptr: *mut libc::c_void = v.as_mut_ptr() as *mut libc::c_void;
        match unsafe { ffi::rtlsdr_read_sync(self.ptr, ptr, len as libc::c_int,
                                             &mut n) } {
            0 => {
                unsafe { v.set_len(n as usize) };
                Ok(v)
            },
            err => Err(rtlsdr_error(err, "Unknown"))
        }
    }
}

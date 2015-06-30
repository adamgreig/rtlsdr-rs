// Demonstration of the RTL-SDR crate
// Copyright Adam Greig <adam@adamgreig.com> 2014
// Licensed under MIT license

extern crate rtlsdr;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let count = rtlsdr::get_device_count();
    println!("Found {} device(s)", count);

    for index in 0..count {
        println!("Index {}:", index);

        let name = rtlsdr::get_device_name(index);
        println!("  Name: {}", name);

        let strs = rtlsdr::get_device_usb_strings(index).unwrap();
        println!("  Manufacturer: {}", strs.manufacturer);
        println!("  Product:      {}", strs.product);
        println!("  Serial:       {}", strs.serial);
        println!("");

        let idx2 = rtlsdr::get_index_by_serial(strs.serial).unwrap();
        println!("  Index looked up by serial: {}", idx2);

        println!("  Opening device...");
        let mut dev = rtlsdr::open(index).unwrap();
        println!("");

        println!("  Getting USB strings from opened device...");
        let strs2 = dev.get_usb_strings().unwrap();
        println!("  Manufacturer: {}", strs2.manufacturer);
        println!("  Product:      {}", strs2.product);
        println!("  Serial:       {}", strs2.serial);
        println!("");

        let (rtl_freq, tuner_freq) = dev.get_xtal_freq().unwrap();
        println!("  RTL clock freq: {}Hz", rtl_freq);
        println!("  Tuner clock freq: {}Hz", tuner_freq);

        println!("  Setting RTL and tuner clock frequencies to same...");
        dev.set_xtal_freq(rtl_freq, tuner_freq).unwrap();

        println!("  Setting centre frequency to 434MHz...");
        dev.set_center_freq(434_000_000).unwrap();

        let cfreq = dev.get_center_freq().unwrap();
        println!("  Read current centre frequency: {}Hz", cfreq);

        let ppm = dev.get_freq_correction();
        println!("  Current freq correction: {}ppm", ppm);

        println!("  Setting freq correction to 1ppm...");
        dev.set_freq_correction(1).unwrap();

        let (t_id, t_name) = dev.get_tuner_type();
        println!("  Tuner is a {} (id {})", &t_name, &t_id);

        //println!("  Setting gain to manual...");
        //dev.set_tuner_gain_mode(true).unwrap();

        let gains = dev.get_tuner_gains().unwrap();
        println!("  Available gains: {:?}", &gains);

        //println!("  Setting gain to second option {}dB",
                 //(gains[1] as f64)/10.0f64);
        //dev.set_tuner_gain(gains[1]).unwrap();

        //let gain = dev.get_tuner_gain();
        //println!("  Current gain: {}dB", (gain as f64)/10.0f64);

        println!("  Setting sample rate to 24kHz...");
        dev.set_sample_rate(24000).unwrap();

        let rate = dev.get_sample_rate().unwrap();
        println!("  Current sample rate: {}Hz", rate);

        //println!("  Setting test mode off...");
        //dev.set_test_mode(false).unwrap();

        //println!("  Setting AGC off...");
        //dev.set_agc_mode(false).unwrap();

        //println!("  Disabling direct sampling");
        //dev.set_direct_sampling(rtlsdr::DirectSampling::Disabled).unwrap();

        let m = dev.get_direct_sampling().unwrap();
        println!("  Direct sampling mode: {:?}", m);

        dev.reset_buffer().unwrap();

        let data = dev.read_sync(131072).unwrap();

        let mut file = File::create(&Path::new("data.bin")).unwrap();
        file.write(&data);

        println!("  Closing device...");
        dev.close().unwrap();
    }
}

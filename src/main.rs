use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use image::io::Reader;
use image::save_buffer_with_format;



use freenect_sys::{freenect_device, freenect_led_options_LED_BLINK_GREEN, freenect_set_video_callback, freenect_set_depth_callback};

use kinect_control::FreenectContext;

extern "C" fn video_cb(dev: *mut freenect_device,
                       data: *mut ::std::os::raw::c_void,
                       timestamp: u32) {
    println!("Video callback: {}", timestamp);

    // todo: calculate size based on the various dimensions. 

    let bytes = 921600;
    let width = 640;
    let height = 480;

    unsafe { 
        let rgb_data: &[u8] = std::slice::from_raw_parts(data as *mut u8, bytes);
        println!("Some data: ({}, {}, {})", rgb_data[0], rgb_data[1], rgb_data[2]);
        save_buffer_with_format("myimg.jpg", rgb_data, width, height, image::ColorType::Rgb8, image::ImageFormat::Jpeg).unwrap();
    }

    // The following three lines simply load a test image and convert it into buffer
    // let (width, height) = (img.width(), img.height());
    // let img_byte_vec = img.into_raw();
    // // The next line is what you want

}

extern "C" fn depth_cb(dev: *mut freenect_device,
                       data: *mut ::std::os::raw::c_void,
                       timestamp: u32) {
    println!("Video callback: {}", timestamp);

    // todo: calculate size based on the various dimensions. 
    let bytes = 921600;
    let width = 640;
    let height = 480;

    unsafe { 
        let rgb_data: &[u8] = std::slice::from_raw_parts(data as *mut u8, 100);
        println!("Depth: {:?}", rgb_data);
    }

    // The following three lines simply load a test image and convert it into buffer
    // let (width, height) = (img.width(), img.height());
    // let img_byte_vec = img.into_raw();
    // // The next line is what you want

}

fn main() {
    println!("hello world");

    let f = FreenectContext::new();
    f.set_led(freenect_led_options_LED_BLINK_GREEN);

    let mut angle = 0.1;

    let r = f.set_video_mode();
    println!("Set video mode result: {}", r);

    unsafe {
        freenect_set_video_callback(f.dev, Some(video_cb))
    }

    unsafe {
        freenect_set_depth_callback(f.dev, Some(depth_cb))
    }

    println!("Start video: {}", f.start_video());
    println!("Start video: {}", f.start_depth());

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        // println!("Setting angle to {}", angle);
        // let r = f.set_tilt_degs(angle);
        // println!("Result: {}", r);
        // angle += 1.0;
        f.process_events();
        // std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

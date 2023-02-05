
use freenect_sys::*;

pub struct FreenectContext {
    ctx: *mut freenect_context,
    // temporary public.
    pub dev: *mut freenect_device,
}

impl FreenectContext{
    pub fn new() -> Self {
        let ctx = unsafe {
            let mut ctx_a: *mut freenect_context = 0 as *mut _;
            freenect_init(&mut ctx_a as *mut _, 0 as *mut freenect_usb_context);
            ctx_a
        };
        
        let dev = unsafe {
            let mut device_a: *mut freenect_device = 0 as *mut _;
            freenect_open_device(ctx, &mut device_a as *mut _, 0);
            device_a
        };

        Self {
            ctx,
            dev
        }
    }

    pub fn set_led(&self, led: freenect_led_options) {
        unsafe {
            freenect_set_led(self.dev, led);
        }
    }

    pub fn set_tilt_degs(&self, angle: f64) -> i32 {
        unsafe {
            freenect_set_tilt_degs(self.dev, angle)
        }
    }

    pub fn set_video_mode(&self) -> i32 {
        unsafe {
            freenect_set_video_mode(self.dev,
                                    freenect_find_video_mode(freenect_resolution_FREENECT_RESOLUTION_MEDIUM,
                                                             freenect_video_format_FREENECT_VIDEO_RGB))
        }
    }

    pub fn set_depth_mode(&self) -> i32 {
        unsafe {
            freenect_set_depth_mode(self.dev,
                                    freenect_find_depth_mode(freenect_resolution_FREENECT_RESOLUTION_MEDIUM,
                                                             FREENECT_DEPTH_MM_MAX_VALUE))
        }
    }

    pub fn set_video_callback(&self, callback: freenect_video_cb) {
    }

    pub fn start_video(&self) -> i32 {
        unsafe {
            freenect_start_video(self.dev)
        }
    }

    pub fn start_depth(&self) -> i32 {
        unsafe {
            freenect_start_depth(self.dev)
        }
    }

    pub fn process_events(&self) -> i32 {
        unsafe {
            freenect_process_events(self.ctx)
        }
    }

}


impl Drop for FreenectContext {
    fn drop(&mut self) {
        println!("Dropping Context");
        unsafe {
            freenect_stop_video(self.dev);
            freenect_close_device(self.dev);
            freenect_shutdown(self.ctx);
        }
    }
}



#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!("freenect_bindings.rs");


pub struct FreenectContext {
    ctx: *mut freenect_context,
    dev: *mut freenect_device,
}

impl FreenectContext{
    pub fn new() -> Self {
        let ctx = unsafe {
            // don't really need this entry, more rather just need a *mut that points to nothing.. 
            let mut ctx = freenect_context { _unused: [] };
            let mut ctx_a: *mut freenect_context = &mut ctx as *mut _;
            freenect_init(&mut ctx_a as *mut _, 0 as *mut freenect_usb_context);

            ctx_a
        };
        
        let dev = unsafe {
            let mut device = freenect_device { _unused: [] };
            let mut device_a: *mut freenect_device = &mut device as *mut _;
            freenect_open_device(ctx, &mut device_a as *mut _, 0);
            device_a
        };

        Self {
            ctx,
            dev
        }
    }


    pub fn sync_set_led(&self, led: freenect_led_options) {
        unsafe {
            
        }
    }
}



impl Drop for FreenectContext {
    fn drop(&mut self) {
        // todo!()
    }
}



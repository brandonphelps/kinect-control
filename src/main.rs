use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::FourCC;

use jpeg_decoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new capture device with a few extra parameters
    let mut dev = Device::with_path("/dev/video0").expect("Failed to open device");

    // Let's say we want to explicitly request another format
    let mut fmt = dev.format().expect("Failed to read format");
    println!("Read fmt\n{}", fmt);
    // fmt.width = 1280;
    // fmt.height = 720;
    fmt.fourcc = FourCC::new(b"UYVY");
    fmt = dev.set_format(&fmt).expect("Failed to write format");

    // The actual format chosen by the device driver may differ from what we
    // requested! Print it out to get an idea of what is actually used now.
    println!("Format in use:\n{}", fmt);

    // Now we'd like to capture some frames!
    // First, we need to create a stream to read buffers from. We choose a
    // mapped buffer stream, which uses mmap to directly access the device
    // frame buffer. No buffers are copied nor allocated, so this is actually
    // a zero-copy operation.

    // To achieve the best possible performance, you may want to use a
    // UserBufferStream instance, but this is not supported on all devices,
    // so we stick to the mapped case for this example.
    // Please refer to the rustdoc docs for a more detailed explanation about
    // buffer transfers.

    if let Ok(controls) = dev.query_controls() {
        for control in controls  {
            println!("{}", control);
        }
    }

    // Create the stream, which will internally 'allocate' (as in map) the
    // number of requested buffers for us.
    let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");

    // At this point, the stream is ready and all buffers are setup.
    // We can now read frames (represented as buffers) by iterating through
    // the stream. Once an error condition occurs, the iterator will return
    // None.
    loop {
        let (buf, meta) = stream.next().unwrap();
        println!(
            "Buffer size: {}, seq: {}, timestamp: {}",
            buf.len(),
            meta.sequence,
            meta.timestamp
        );

        // To process the captured data, you can pass it somewhere else.
        // If you want to modify the data or extend its lifetime, you have to
        // copy it. This is a best-effort tradeoff solution that allows for
        // zero-copy readers while enforcing a full clone of the data for
        // writers.

        //println!("{:?}", &buf[..32]);
        // presume Y410

        let u: u16 = buf[0] as u16 + (buf[1] as u16 & 0x3) << 8;
        let y: u16 = (buf[1] as u16 & (!0x2)) + 0xF & buf[2] as u16;
        let v: u16 = (buf[2] as u16 & 0xF0) + (buf[3] as u16 & 0x3f);
        let a: u8 = buf[3] & 0xC;
        
        //let pixel = u16::from_ne_bytes([buf[0], buf[1]]);
        //println!("pixel: {:b}", pixel);
        println!("A: {} V: {} Y: {} U: {}", a, v, y, u);
        

            
        // let data = match &fmt.fourcc.repr {
        //     b"RGB3" => buf.to_vec(),
        //     b"MJPG" => {
        //         let mut decoder = jpeg_decoder::Decoder::new(buf);
        //         decoder.decode().expect("Failed to decode jpeg")
        //     },
        //     x => {
        //         panic!("Unhandled format: {:?}", x);
        //     }
        // };

        // println!("{:?}", &data[..512]);
    }

    Ok(())
}



fn yuv_to_rgb(y: u16, u: u16, v: u16) -> (u16, u16, u16) {
    todo!()
}

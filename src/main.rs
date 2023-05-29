use std::{env, io::Write, path::Path};
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os().skip(1);
    
    let device_path_str = args
        .next()
        .ok_or_else(|| anyhow!("usage: video-capture <device> <file>"))?;

    let file_path = args
        .next()
        .ok_or_else(|| anyhow!("usage: video-capture <device> <file>"))?;

    let max_fps = 30;

    let device_path = Path::new(&device_path_str);
    let mut device = h264_webcam_stream::get_device(&device_path)?;
    let mut stream = h264_webcam_stream::stream(&mut device, max_fps)?;

    let mut file = std::fs::File::create(file_path)?;

    for _ in 0..60 {
        let (h264_bytes, _) = stream.next(false)?;
        // Record the h264 video to a file
        file.write_all(&h264_bytes[..])?;
    }

    Ok(())

}

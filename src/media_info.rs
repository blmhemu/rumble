use std::error::Error;
use std::process::Command;
use std::str;

//TODO: Take ffmpeg and ffprobe paths rather than hardcoding.

//TODO: Put in another file ??
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

pub fn get_duration(video_file_name: &str) -> Result<f32> {
    let ffprobe_args = &[
        // Input file
        "-i",
        video_file_name,
        // Do not print unwanted stuff
        "-v",
        "quiet",
        // We only want duration
        "-show_entries",
        "format=duration",
        // Formatting options : no wrappers and key
        "-of",
        "default=noprint_wrappers=1:nokey=1",
    ];

    let output = Command::new("ffprobe").args(ffprobe_args).output()?;

    let duration: f32 = str::from_utf8(&output.stdout)?.trim().parse()?;

    Ok(duration)
}

pub fn get_resolution(video_file_name: &str) -> Result<Resolution> {
    let ffprobe_args = &[
        // Input file
        "-i",
        video_file_name,
        // Do not print unwanted stuff
        "-v",
        "quiet",
        // We only want duration
        "-show_entries",
        "stream=width,height",
        // Formatting options : no wrappers and key
        "-of",
        "default=noprint_wrappers=1:nokey=1",
    ];

    let output = Command::new("ffprobe").args(ffprobe_args).output()?;

    let mut lines = str::from_utf8(&output.stdout)?.trim().lines();

    // Currently NoneError is experimental in rust.
    // Once supported we can do the following.
    // let width: usize = lines.next()?.parse()?;
    // let height: usize = lines.next()?.parse()?;

    let width = lines.next();

    if width.is_none() {
        Err("None value for width in media_info::get_resolution")?
    }

    let height = lines.next();

    if height.is_none() {
        Err("None value for height in media_info::get_resolution")?
    }

    let width = width.unwrap().parse()?;
    let height = height.unwrap().parse()?;

    Ok(Resolution { width, height })
}

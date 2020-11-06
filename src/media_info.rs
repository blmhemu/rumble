use std::error::Error;
use std::process::Command;
use std::str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//TODO: Take ffmpeg and ffprobe paths rather than assuming they are in PATH.

//TODO: Put in another file ??
type BoxedResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamArray {
    pub streams: Vec<Stream>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub index: usize,
    pub tags: HashMap<String, String>
}

pub enum StreamType {
    Video,
    Audio,
    Subtitle
}

pub fn get_stream_info(media_file: &str, stream_type: StreamType) -> BoxedResult<StreamArray> {

    let stream_type = match stream_type {
        StreamType::Video => "v",
        StreamType::Audio => "a",
        StreamType::Subtitle => "s",
    };

    let ffprobe_args = &[
        "-i",
        media_file,
        "-v",
        "quiet",
        "-select_streams",
        stream_type,
        "-show_entries",
        // TODO: Get only required data : "stream=index:stream_tags=language",
        "stream",
        "-of",
        "json",
    ];

    let output = Command::new("ffprobe").args(ffprobe_args).output()?;

    let json = str::from_utf8(&output.stdout)?;

    let streams: StreamArray = serde_json::from_str(json)?;

    Ok(streams)

}

pub fn get_duration(media_file: &str) -> BoxedResult<f32> {
    let ffprobe_args = &[
        // Input file
        "-i",
        media_file,
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

pub fn get_resolution(media_file: &str) -> BoxedResult<Resolution> {
    let ffprobe_args = &[
        // Input file
        "-i",
        media_file,
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

    let mut lines = std::str::from_utf8(&output.stdout)?.trim().lines();

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

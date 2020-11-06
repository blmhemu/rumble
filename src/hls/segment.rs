use regex::Regex;
use std::process::Command;
use warp::http::header;
use warp::http::Response;

const HLS_SEGMENT_DURATION: f32 = 6.0;
const MPEGTS_HEADER_VALUE: &str = "video/MP2T";

pub async fn audio_segment_handler(
    media_file: String,
    stream_index: u8,
    segment_str: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Do a sanity check on media_file if the file is available

    let valid_seg_re = Regex::new(r#"^\d{4}.aac$"#).unwrap();
    if !valid_seg_re.is_match(&segment_str) {
        println!("Mismatch regexp hls::audio_segment_handler");
        return Err(warp::reject::not_found());
    }

    // Unwraps here should be safe as we are matching only after above validation.
    let caps = Regex::new(r#"^\d{4}"#)
        .unwrap()
        .captures(&segment_str)
        .unwrap();
    let segment_number: usize = caps.get(0).map(|m| m.as_str()).unwrap().parse().unwrap();

    get_audio_segment(&media_file, segment_number, stream_index)
}

fn get_audio_segment(
    media_file: &str,
    segment_number: usize,
    stream_index: u8,
) -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = HLS_SEGMENT_DURATION * segment_number as f32;

    //TODO: Subtitles Support
    //TODO: Audio Selection
    //TODO: Video Bitrate based on resolution
    let ffmpeg_args = &[
        // Exit if taking longer than 45 seconds
        "-timelimit",
        "45",
        // Seek till given start time
        "-ss",
        &format!("{:.4}", start_time),
        // Input
        "-i",
        media_file,
        // Segment time
        "-t",
        &format!("{:.4}", HLS_SEGMENT_DURATION),
        // Select stream
        "-map",
        &format!("0:a:{}", stream_index),
        // Audio
        "-c:a",
        "aac",
        "-b:a",
        "128k",
        "-ac",
        "2",
        // Force key_frames for exact split
        "-force_key_frames",
        &format!("expr:gte(t,n_forced*{:.4})", HLS_SEGMENT_DURATION),
        "-f",
        "ssegment",
        "-segment_time",
        &format!("{:.4}", HLS_SEGMENT_DURATION),
        "-initial_offset",
        &format!("{:.4}", start_time),
        "pipe:%04d.aac",
    ];

    let output = Command::new("ffmpeg").args(ffmpeg_args).output();

    // TODO: Check for .mp4 support
    match output {
        Ok(out) => Ok(Response::builder()
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static(MPEGTS_HEADER_VALUE),
            )
            .body(out.stdout)),
        Err(e) => {
            println!("Error in hls::get_audio_segment: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

pub async fn video_segment_handler(
    media_file: String,
    resolution: u16,
    segment_str: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Do a sanity check on media_file if the file is available

    let valid_seg_re = Regex::new(r#"^\d{4}.ts$"#).unwrap();
    if !valid_seg_re.is_match(&segment_str) {
        println!("Mismatch regexp hls::segment_handler");
        return Err(warp::reject::not_found());
    }

    // Unwraps here should be safe as we are matching only after above validation.
    let caps = Regex::new(r#"^\d{4}"#)
        .unwrap()
        .captures(&segment_str)
        .unwrap();
    let segment_number: usize = caps.get(0).map(|m| m.as_str()).unwrap().parse().unwrap();

    get_video_segment(&media_file, resolution, segment_number)
}

fn get_video_segment(
    media_file: &str,
    resolution: u16,
    segment_number: usize,
) -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = HLS_SEGMENT_DURATION * segment_number as f32;

    //TODO: Subtitles Support
    //TODO: Audio Selection
    //TODO: Video Bitrate based on resolution
    let ffmpeg_args = &[
        // Exit if taking longer than 45 seconds
        "-timelimit",
        "45",
        // Seek till given start time
        "-ss",
        &format!("{:.4}", start_time),
        // Input
        "-i",
        media_file,
        // Segment time
        "-t",
        &format!("{:.4}", HLS_SEGMENT_DURATION),
        // Video
        "-vf",
        &format!("scale=-2:{}", resolution),
        "-vcodec",
        "libx264",
        "-preset",
        "veryfast",
        // Audio
        "-c:a",
        "aac",
        "-b:a",
        "128k",
        "-ac",
        "2",
        "-pix_fmt",
        "yuv420p",
        // Force key_frames for exact split
        "-force_key_frames",
        &format!("expr:gte(t,n_forced*{:.4})", HLS_SEGMENT_DURATION),
        "-f",
        "ssegment",
        "-segment_time",
        &format!("{:.4}", HLS_SEGMENT_DURATION),
        "-initial_offset",
        &format!("{:.4}", start_time),
        "pipe:%04d.ts",
    ];

    let output = Command::new("ffmpeg").args(ffmpeg_args).output();

    // TODO: Check for .mp4 support
    match output {
        Ok(out) => Ok(Response::builder()
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static(MPEGTS_HEADER_VALUE),
            )
            .body(out.stdout)),
        Err(e) => {
            println!("Error in hls::get_video_segment: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

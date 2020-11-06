use regex::Regex;
use std::process::Command;
use warp::http::header;
use warp::http::Response;

const HLS_SEGMENT_DURATION: usize = 6;
const MPEGTS_HEADER_VALUE: &str = "video/MP2T";

pub async fn segment_handler(
    video_file: String,
    resolution: u16,
    segment_str: String,
) -> Result<impl warp::Reply, warp::Rejection> {
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

    get_segment(&video_file, resolution, segment_number)
}

fn get_segment(
    video_file: &str,
    resolution: u16,
    segment_number: usize,
) -> Result<impl warp::Reply, warp::Rejection> {
    let start_time = segment_number * HLS_SEGMENT_DURATION;

    //TODO: Subtitles Support
    //TODO: Audio Selection
    //TODO: Video Bitrate based on resolution
    let ffmpeg_args = &[
        // Exit if taking longer than 45 seconds
        "-timelimit",
        "45",
        // Seek till given start time
        "-ss",
        &format!("{:.4}", start_time as f32),
        // Input
        "-i",
        video_file,
        // Segment time
        "-t",
        &format!("{:.4}", HLS_SEGMENT_DURATION as f32),
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
        "expr:gte(t,n_forced*5.000)",
        "-f",
        "ssegment",
        "-segment_time",
        &format!("{:.2}", HLS_SEGMENT_DURATION as f32),
        "-initial_offset",
        &format!("{:.2}", start_time as f32),
        "pipe:%03d.ts",
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
            println!("Error in hls::get_segment: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

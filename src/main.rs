mod hls;
mod indexer;
mod media_info;

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use warp::Filter;

#[tokio::main]
async fn main() {
    // We will update this when there is a change.
    // https://github.com/notify-rs/notify
    // Hence the use of Arc and RwLock.
    // TODO: Look into https://github.com/vorner/arc-swap
    let videos = Arc::new(RwLock::new(HashMap::new()));
    // let mut videos: Arc::new(HashMap<String, String> = HashMap::new());
    indexer::get_supported_video_files(videos.clone(), "/Users/hbollamreddi/Movies");

    let debug = false;
    if debug {
        println!("{:?}", videos);
        
        let location = "ldp.mkv";

        let duration = media_info::get_duration(location);
        match duration {
            Ok(x) => println!("Duration is: {} seconds", x),
            Err(e) => println!("Error from media_info::get_duration : {}", e),
        }

        let resolution = media_info::get_resolution(location);
        match resolution {
            Ok(x) => println!("Resolution is: {:?}", x),
            Err(e) => println!("Error from media_info::get_resolution : {}", e),
        }

        let streams = media_info::get_stream_info(location, media_info::StreamType::Audio);
        match streams {
            Ok(x) => println!("Streams : {:?}", x),
            Err(e) => println!("Error from media_info::get_subtitle_streams : {}", e),
        }

        let streams = media_info::get_stream_info(location, media_info::StreamType::Subtitle);
        match streams {
            Ok(x) => println!("Streams : {:?}", x),
            Err(e) => println!("Error from media_info::get_subtitle_streams : {}", e),
        }
    }

    let index = warp::path::end().and(warp::fs::file("./ui/index.html"));
    let js = warp::path("js").and(warp::fs::dir("./ui3/js"));
    // Get this better
    let list_videos = warp::path!("list")
        .and(warp::path::end())
        .map(move || videos.clone())
        .and_then(indexer::list_videos_handler);
    let main_playlist = warp::path!("playlist" / String)
        .and(warp::path::end())
        .and_then(hls::playlist::master_playlist_handler);
    let video_playlist = warp::path!("video" / String / usize)
        .and(warp::path::end())
        .and_then(hls::playlist::res_playlist_handler);
    let audio_playlist = warp::path!("audio" / String / u8)
        .and(warp::path::end())
        .and_then(hls::playlist::audio_playlist_handler);
    let subs_playlist = warp::path!("subs" / String / u8)
        .and(warp::path::end())
        .and_then(hls::playlist::subs_playlist_handler);
    let video = warp::path!("video" / String / u16 / String)
        .and(warp::path::end())
        .and_then(hls::segment::video_segment_handler);
    let audio = warp::path!("audio" / String / u8 / String)
        .and(warp::path::end())
        .and_then(hls::segment::audio_segment_handler);
    let subs = warp::path!("subs" / String / u8 / String)
        .and(warp::path::end())
        .and_then(hls::segment::subtitle_segment_handler);

    warp::serve(
        list_videos
            .or(js)
            .or(index)
            .or(main_playlist)
            .or(audio_playlist)
            .or(subs_playlist)
            .or(video_playlist)
            .or(video)
            .or(audio)
            .or(subs),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}

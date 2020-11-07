mod hls;
mod media_info;

use warp::Filter;

#[tokio::main]
async fn main() {
    let debug = false;
    if debug {
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
        js.or(index)
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

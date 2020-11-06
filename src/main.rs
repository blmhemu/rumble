mod hls;
mod media_info;

use warp::Filter;

#[tokio::main]
async fn main() {
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

    let index = warp::path::end().and(warp::fs::file("ui/index.html"));
    let segment = warp::path!("play" / String / u16 / String)
        .and(warp::path::end())
        .and_then(hls::segment::segment_handler);
    let get_res_playlist = warp::path!("playlist" / String)
        .and(warp::path::end())
        .and_then(hls::playlist::res_playlist_handler);

    // let get_play = warp::get()
    //     .and(warp::path("playlist"))
    //     .and(warp::path::param())
    //     .and(warp::path::end())
    //     .and_then(get_play_fn);

    warp::serve(index.or(segment).or(get_res_playlist))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

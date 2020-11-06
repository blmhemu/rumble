mod media_info;
mod hls;

use warp::Filter;

#[tokio::main]
async fn main() {
    let location = "bb.mkv";
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

    let streams = media_info::get_stream_info(location, media_info::StreamType::Subtitle);
    match streams {
        Ok(x) => println!("Streams : {:?}", x),
        Err(e) => println!("Error from media_info::get_subtitle_streams : {}", e),
    }

    let index = warp::path::end()
        .and(warp::fs::file("ui/index.html"));
    let segment = warp::path!("play" / String / u16 / String)
        .and_then(hls::segment::segment_handler);
    let get_res_playlist = warp::get()
        .and(warp::path("playlist"))
        .and(warp::path::end())
        .and_then(getResPlaylist);

    // let get_play = warp::get()
    //     .and(warp::path("playlist"))
    //     .and(warp::path::param())
    //     .and(warp::path::end())
    //     .and_then(get_play_fn);

    warp::serve(index.or(segment).or(get_res_playlist))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

pub async fn getResPlaylist() -> Result<impl warp::Reply, warp::Rejection> {
    let duration = 3623.00;
    let hlsSegmentLength = 6.00;

    let mut playlist = String::new();
    playlist.push_str("#EXTM3U\n");
    playlist.push_str("#EXT-X-VERSION:3\n");
    playlist.push_str("#EXT-X-TARGETDURATION:6\n");
    playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");
    playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
    playlist.push_str("#EXT-X-ALLOW-CACHE:YES\n");

    let mut leftover = duration;
    let mut segmentIndex = 0;

    while leftover > 0 as f32 {
        if leftover > hlsSegmentLength {
            playlist.push_str(&format!("#EXTINF:{:.6},\n", hlsSegmentLength as f32));
        } else {
            playlist.push_str(&format!("#EXTINF:{:.6},\n", leftover as f32));
        }
        playlist.push_str(&format!("http://localhost:3030/play/as.mkv/720/{:04}.ts\n", segmentIndex));
        segmentIndex += 1;
        leftover = leftover - hlsSegmentLength;
    }

    playlist.push_str("#EXT-X-ENDLIST\n");

    // let resp_builder = Response::builder()
    // .header(http::header::CONTENT_TYPE, http::HeaderValue::from_static("application/mpeg-xurl"))
    // .
    // resp_builder.

    // Response::builder()
    // .header(:"cool", "lol")
    Ok(warp::reply::Response::new(playlist.into()))
    // Response::from(playlist)
}

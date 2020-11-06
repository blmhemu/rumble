use crate::media_info;
use warp::http;
use warp::http::Response;

// TODO: Take from segment.rs
const HLS_SEGMENT_DURATION: f32 = 6.0;
const M3U8_HEADER_VALUE: &str = "application/x-mpegURL";
const PRESET_RESOLUTIONS: &'static [usize] = &[1080, 720, 480];

// pub async fn master_playlist_handler() -> Result<impl warp::Reply, warp::Rejection> {}

pub fn get_master_playlist(media_file: &str) -> Result<impl warp::Reply, warp::Rejection> {
    let media_duration = media_info::get_duration(media_file);

    if media_duration.is_err() {
        return Err(warp::reject::not_found());
    }

    let media_resolution = media_info::get_resolution(media_file);

    if media_resolution.is_err() {
        return Err(warp::reject::not_found());
    }

    // Safety check done above
    let media_duration = media_duration.unwrap();
    let height = media_resolution.unwrap().height;

    let mut playlist = String::new();
    playlist.push_str("#EXTM3U\n");
    playlist.push_str("\n");

    // populate_audio tracks
    let a = media_info::get_stream_info(media_file, media_info::StreamType::Audio);
    if a.is_err() {
        return Err(warp::reject::not_found());
    }

    let a = a.unwrap();
    for (pos, s) in a.streams.iter().enumerate() {
        match s.tags.get("language") {
            Some(lang) => playlist.push_str(&format!(
                r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="bipbop_audio",LANGUAGE="{}","#,
                lang
            )),
            None => playlist.push_str(r#"#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID="bipbop_audio","#),
        }
        match s.tags.get("title") {
            Some(title) => playlist.push_str(&format!(r#"NAME="{}","#, title)),
            None => playlist.push_str(&format!(r#"NAME="Track-{}","#, pos + 1)),
        }

        match s.disposition.get("default") {
            Some(default) => {
                if *default == 1 as isize {
                    playlist.push_str(&format!("AUTOSELECT=YES,DEFAULT=YES\n"));
                } else {
                    let uri = &format!("audio_{:02}/index.m3u8", s.index);
                    playlist.push_str(&format!(r#"AUTOSELECT=NO,DEFAULT=NO,URI="{}""#, uri));
                    playlist.push_str("\n");
                }
            }
            None => {
                let uri = &format!("audio_{:02}/index.m3u8", s.index);
                playlist.push_str(&format!(r#"AUTOSELECT=NO,DEFAULT=NO,URI="{}""#, uri));
                playlist.push_str("\n");
            }
        }
    }

    println!("{}", playlist);
    Ok(warp::reply())
}

pub async fn res_playlist_handler(media_file: String) -> Result<impl warp::Reply, warp::Rejection> {
    get_res_playlist(&media_file, 720)
}

fn get_res_playlist(
    media_file: &str,
    resolution: usize,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut playlist = String::new();
    playlist.push_str("#EXTM3U\n");
    playlist.push_str("#EXT-X-VERSION:3\n");
    playlist.push_str(&format!("#EXT-X-TARGETDURATION:{}\n", HLS_SEGMENT_DURATION));
    playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");
    playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
    playlist.push_str("#EXT-X-ALLOW-CACHE:YES\n");

    let leftover = media_info::get_duration(media_file);
    if leftover.is_err() {
        return Err(warp::reject::not_found());
    }

    let mut leftover = leftover.unwrap();

    let mut segmentIndex = 0;

    while leftover > 0 as f32 {
        if leftover > HLS_SEGMENT_DURATION {
            playlist.push_str(&format!("#EXTINF:{:.6},\n", HLS_SEGMENT_DURATION));
        } else {
            playlist.push_str(&format!("#EXTINF:{:.6},\n", leftover));
        }
        playlist.push_str(&format!(
            "/play/{}/{}/{:04}.ts\n",
            media_file.trim(),
            resolution,
            segmentIndex
        ));
        segmentIndex += 1;
        leftover = leftover - HLS_SEGMENT_DURATION;
    }

    playlist.push_str("#EXT-X-ENDLIST\n");

    Ok(Response::builder()
        .header(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static(M3U8_HEADER_VALUE),
        )
        .body(playlist))
}

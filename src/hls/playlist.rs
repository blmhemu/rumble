use crate::media_info;

// TODO: Take from segment.rs
const HLS_SEGMENT_DURATION: usize = 6;
const M3U8_HEADER_VALUE: &str = "application/x-mpegURL";
const PRESET_RESOLUTIONS: &'static [i32] = &[1080, 720, 480];


// pub async fn master_playlist_handler() -> Result<impl warp::Reply, warp::Rejection> {}

fn get_master_playlist(media_file: &str) -> Result<impl warp::Reply, warp::Rejection> {

    let media_duration = media_info::get_duration(media_file);

    if media_duration.is_err() {
        return Err(warp::reject::not_found())
    }

    let media_resolution = media_info::get_resolution(media_file);

    if media_resolution.is_err() {
        return Err(warp::reject::not_found())
    }

    // Safety check done above
    let media_duration = media_duration.unwrap();
    let height = media_resolution.unwrap().height;

    let mut playlist = String::new();
    playlist.push_str("#EXTM3U\n");
    playlist.push_str("\n");
    playlist.push_str("#EXT-X-STREAM-INF:\n");

    Ok(warp::reply())

}

// pub async fn res_playlist_handler() -> Result<impl warp::Reply, warp::Rejection> {}

fn get_res_playlist() {
    let mut playlist = String::new();
    playlist.push_str("#EXTM3U\n");
    playlist.push_str("#EXT-X-VERSION:3\n");
    playlist.push_str(&format!("#EXT-X-TARGETDURATION:{}\n", HLS_SEGMENT_DURATION));
    playlist.push_str("#EXT-X-MEDIA-SEQUENCE:0\n");
    playlist.push_str("#EXT-X-PLAYLIST-TYPE:VOD\n");
    playlist.push_str("#EXT-X-ALLOW-CACHE:YES\n");
}

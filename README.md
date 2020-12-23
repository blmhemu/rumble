# Rumble
Note: This is prerelease software and a few variables may be hardcoded.
To be fast and powerful media server written in rust.

Inspired by [jellyfin](https://jellyfin.org/), [olaris](https://gitlab.com/olaris/olaris-server) and [gohls](https://github.com/shimberger/gohls)
Jellyfin is a bit resource intensive and seemed slow to run on low power devices. Rust being inherently fast and correct, would run on most low power devices.

# Design
FFmpeg is heavily used to segment the video stream at required times and is served to the requesting media player.
HLS supported : Player can request for streams at various pre defined resolutions adpatively based on the bandwidth available.
Supports multi subs and multi audio.

# API Design
Api design is currently unstable and might change in future.

/playlist -> Gives the m3u8 file (HLS playlist file)
/video -> Gives the .ts segment at requested time / segment number
/audio -> Same as video but for audio
/subs -> Subs are not segmented and are provided as a single file due to low size

# ToDo
Tracked at https://github.com/blmhemu/rumble/projects/1

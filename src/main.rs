use std::{fs, thread::sleep, time::Duration};

use libtorrent_sys::ffi::{self, DownloadStatus};

// public domain CD
const MAGNET_LINK: &'static str = "magnet:?xt=urn:btih:a88fda5954e89178c372716a6a78b8180ed4dad3&dn=The+WIRED+CD+-+Rip.+Sample.+Mash.+Share&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fwired-cd.torrent";

pub fn main() {
    let _ = fs::remove_dir_all("./output/");
    let mut session = ffi::create_session_with_alerts();
    let torrent = ffi::add_torrent(
        session.pin_mut(),
        &*ffi::parse_magnet_link(MAGNET_LINK, "./output/"),
    );
    let mut open_torrents = 1u16;
    'outer: loop {
        let alerts = ffi::handle_alerts(session.pin_mut(), &mut open_torrents, "./output/resume/");
        for alert in alerts {
            if alert.status == DownloadStatus::Finished {
                println!("Torrent finished");
                break 'outer;
            } else if alert.status == DownloadStatus::Error {
                println!("Torrent failed");
                break 'outer;
            }
        }
        println!(
            "Torrent progress: {}%",
            ffi::status_get_progress(&ffi::get_torrent_status(&torrent)) * 100.
        );
        sleep(Duration::from_millis(500))
    }
    println!("All torrents finished");
}

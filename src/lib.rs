// Copyright (c) 2022 Nicolas Chevalier
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[cxx::bridge(namespace = "libtorrent")]
pub mod ffi {
    enum TorrentStatus {
        Running,
        Finished,
        Error,
    }

    struct GetStatusResult<'a> {
        status: TorrentStatus,
        torrent: &'a TorrentHandle,
    }

    unsafe extern "C++" {
        include!("src/lt.h");

        #[rust_name = "Session"]
        type session;

        #[rust_name = "TorrentHandle"]
        type torrent_handle;

        pub fn create_session_with_alerts() -> UniquePtr<Session>;

        pub fn get_status_updates(ses: Pin<&mut Session>) -> Vec<GetStatusResult>;
    }
}

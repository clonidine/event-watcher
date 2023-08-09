use std::env;

use dotenv::dotenv;
use inotify::{Inotify, WatchMask};

pub mod event;

fn main() {
    let path_buf = dotenv().ok();

    match path_buf {
        Some(_) => {
            let path = env::var("WATCHER_PATH");

            match path {
                Ok(path) => {
                    let inotify = Inotify::init();

                    match inotify {
                        Ok(mut inotify) => {
                            let mut buffer = [0u8; 4096];

                            let descriptor_watcher =
                                inotify.watches().add(path, WatchMask::ALL_EVENTS);

                            match descriptor_watcher {
                                Ok(_) => event::collect_events_info(&mut inotify, &mut buffer),
                                Err(e) => panic!("{}", e),
                            }
                        }

                        Err(err) => panic!("Cannot init Inotify: {}", err),
                    }
                }

                Err(e) => panic!("Cannot load env key 'PATH': {}", e),
            }
        }

        None => panic!("Cannot load environment variables."),
    }
}

use std::ffi::OsStr;

use inotify::{Event, EventMask, Inotify};

pub fn collect_events_info(inotify: &mut Inotify, buffer: &mut [u8; 4096]) {
    loop {
        let events = match inotify.read_events_blocking(buffer) {
            Ok(e) => e,
            Err(err) => panic!("Error while trying to read events blocking: {}", err),
        };

        for event in events {
            let mask = event.mask;
            get_event_info(&event, mask);
        }
    }
}

pub fn get_event_type<'a>(mask: &EventMask) -> &'a str {
    match mask {
        _ if mask.contains(EventMask::CREATE) => "Created",
        _ if mask.contains(EventMask::DELETE) => "Deleted",
        _ if mask.contains(EventMask::ACCESS) => "Accessed",
        _ if mask.contains(EventMask::MOVED_FROM | EventMask::MOVED_TO) => "Moved",
        _ => "UNKNOWN",
    }
}

pub fn is_dir<'a>(mask: &EventMask) -> &'a str {
    let is_dir = EventMask::from(EventMask::ISDIR);

    match mask {
        _ if mask.contains(is_dir) => "Directory",
        _ => "File",
    }
}

pub fn get_event_info(event: &Event<&OsStr>, mask: EventMask) {
    let event_type = get_event_type(&mask);

    if event_type != "UNKNOWN" {
        let event_name = event.name;

        if let Some(event_name) = event_name {
            let event_mask_type = is_dir(&mask);

            println!("{} {}: {:?}", event_mask_type, event_type, event_name);
        }
    }
}

use core::time::Duration;
use notify_debouncer_mini::{new_debouncer, notify::*, DebounceEventResult};
use std::path::Path;

type Callback = fn();

pub struct Notify {
    debouncer: notify_debouncer_mini::Debouncer<RecommendedWatcher>,
}

impl Notify {
    pub fn new(callback: Callback) -> Self {
        // Select recommended watcher for debouncer.
        // Using a callback here, could also be a channel.
        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    events
                        .iter()
                        .for_each(|e| println!("Event {:?} for {:?}", e.kind, e.path));
                    (callback)();
                }
                Err(e) => println!("Error {:?}", e),
            },
        )
        .unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        debouncer
            .watcher()
            .watch(
                Path::new("C:\\Users\\shk\\.weeks.config"),
                RecursiveMode::Recursive,
            )
            .unwrap();

        Notify { debouncer }
    }
    // note that dropping the debouncer (as will happen here) also ends the debouncer
    // thus this demo would need an endless loop to keep running
}

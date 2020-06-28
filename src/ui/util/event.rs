use crate::log::Log;
use chrono::prelude::*;
use tokio::sync::mpsc;
use yaml_rust::Yaml;

#[derive(Debug)]
pub(crate) enum Event {
    DescribeResponse {
        start: DateTime<Local>,
        yaml: Vec<Yaml>,
        next_token: Option<String>,
    },
    Log(Log),
}

pub(crate) struct Events {
    pub(crate) tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
}

pub(crate) fn new() -> Events {
    let (tx, rx) = mpsc::channel(100);

    Events { tx, rx }
}

pub(crate) fn next(event: &mut Events) -> Option<Event> {
    match event.rx.try_recv() {
        Ok(e) => Some(e),
        Err(_) => None,
    }
}

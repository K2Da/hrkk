use crate::log::Log;
use crate::service::prelude::Yaml;
use chrono::prelude::*;
use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) enum Event {
    ListResponse {
        start: DateTime<Local>,
        yaml: crate::service::ResourceList,
        next_token: Option<String>,
    },
    GetResponse {
        start: DateTime<Local>,
        yaml: Yaml,
        resource_index: usize,
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

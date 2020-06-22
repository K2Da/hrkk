use crate::error::Error;
use std::io;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use yaml_rust::Yaml;

use termion::event::Key;
use termion::input::TermRead;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Debug)]
pub enum Event {
    Input(Key),
    DescribeResponse {
        yaml: Vec<Yaml>,
        next_token: Option<String>,
    },
    Tick,
    Err(Error),
}

pub struct Events {
    pub tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,

    pub exit_enter: Arc<AtomicBool>,
    pub exit_esc: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

pub fn new() -> Events {
    let config = Config::default();
    with_config(config.clone())
}

pub fn with_config(config: Config) -> Events {
    let (tx, rx) = mpsc::channel(100);
    let exit_enter = Arc::new(AtomicBool::new(false));
    let exit_esc = Arc::new(AtomicBool::new(false));

    {
        let mut tx = tx.clone();
        let exit_enter = exit_enter.clone();
        let exit_esc = exit_esc.clone();
        tokio::spawn(async move {
            let std_in = io::stdin();
            for evt in std_in.keys() {
                if let Ok(key) = evt {
                    if let Err(_) = tx.send(Event::Input(key)).await {
                        return;
                    }

                    if key == Key::Esc && exit_esc.load(Ordering::Relaxed)
                        || key == Key::Char('\n') && exit_enter.load(Ordering::Relaxed)
                        || key == Key::Ctrl('c')
                    {
                        return;
                    }
                }
            }
        });
    }

    {
        let mut tx = tx.clone();
        tokio::spawn(async move {
            loop {
                if let Err(_) = tx.send(Event::Tick).await {
                    return;
                }
                thread::sleep(config.tick_rate);
            }
        });
    }

    Events {
        tx,
        rx,
        exit_enter,
        exit_esc,
    }
}

pub async fn next(event: &mut Events) -> Option<Event> {
    event.rx.recv().await
}

impl Events {
    pub fn set_exit_key(&mut self, enter: bool, esc: bool) {
        self.exit_enter.store(enter, Ordering::Relaxed);
        self.exit_esc.store(esc, Ordering::Relaxed);
    }
}

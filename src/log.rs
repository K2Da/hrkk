use crate::color;
use crate::show::{Texts, Txt};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum LogLevel {
    ERROR,
    INFO,
}

#[derive(Debug, Clone)]
pub(crate) struct Log {
    time: String,
    level: LogLevel,
    msg: String,
}

impl Log {
    fn new(level: LogLevel, msg: &str) -> Self {
        Log {
            time: Local::now().format("%X ").to_string(),
            level,
            msg: format!("{}\n", msg),
        }
    }

    pub(crate) fn error(msg: &str) -> Self {
        Self::new(LogLevel::ERROR, msg)
    }

    pub(crate) fn info(msg: &str) -> Self {
        Self::new(LogLevel::INFO, msg)
    }

    pub(crate) fn to_txt(&self) -> Vec<Txt> {
        vec![
            Txt::raw(&self.time),
            Txt::colored(
                &self.msg,
                match self.level {
                    LogLevel::ERROR => color::ERROR,
                    LogLevel::INFO => color::INFO,
                },
            ),
        ]
    }
}

#[derive(Clone)]
pub struct Logs(Vec<Log>);

impl Logs {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn push(&mut self, log: Log) {
        self.0.push(log);
    }

    pub(crate) fn error(&mut self, msg: &str) {
        self.0.push(Log::error(msg));
    }

    pub(crate) fn info(&mut self, msg: &str) {
        self.0.push(Log::info(msg));
    }

    pub(crate) fn to_text(&self, count: usize) -> Texts {
        Texts(
            if count != 0 && self.0.len() > count {
                &self.0[self.0.len() - count..]
            } else {
                &self.0
            }
            .iter()
            .map(|t| t.to_txt())
            .into_iter()
            .flatten()
            .collect(),
        )
    }
}

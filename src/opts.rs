use crate::color::ColoredString;
use crate::error::Error::*;
use crate::error::Result;
use ansi_term::Color;
use rusoto_core::Region;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hrkk")]
pub struct Opts {
    /// Max results of aws api request. Default is max size for each api type.
    #[structopt(short = "l", long = "limit")]
    pub limit: Option<i64>,

    /// Max aws request count.
    #[structopt(short = "r", long = "request-count")]
    pub request_count: Option<u8>,

    /// Use cached api response. With this flag, "limit" and "request-count" parameters are ignored.
    #[structopt(short = "c", long = "cache")]
    pub cache: bool,

    /// Store aws api response as "response_body.txt" in the cache directory.
    #[structopt(short = "b", long = "debug")]
    pub debug: bool,

    /// Export selected items as yaml in the current directory.
    #[structopt(short = "e", long = "export")]
    pub export: bool,

    /// Profile name for the aws api request.
    #[structopt(short = "p", long = "profile")]
    pub profile: Option<String>,

    /// Aws region for the aws api request.
    #[structopt(short = "g", long = "region")]
    pub region: Option<String>,

    /// Delimiter for the output text. default is ","
    #[structopt(short = "d", long = "delimiter")]
    pub delimiter: Option<String>,

    /// Sub command.
    #[structopt(subcommand)]
    pub sub_command: Option<SubCommand>,
}

impl Opts {
    pub fn validate(&self) -> Result<()> {
        if let Some(limit) = self.limit {
            if limit < 5 || 1000 < limit {
                return Err(ArgumentError(
                    "limit must be between 1 and 1000".to_string(),
                ));
            }
        }

        if let Some(count) = self.request_count {
            if count == 0 || 10 < count {
                return Err(ArgumentError(
                    "request-count must be between 1 and 10".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn request_count(&self) -> usize {
        match self.request_count {
            Some(count) => count as usize,
            None => 1 as usize,
        }
    }

    pub fn delimiter(&self) -> String {
        match &self.delimiter {
            Some(delimiter) => delimiter.to_string(),
            None => ",".to_string(),
        }
    }

    pub fn set_profile(&self) {
        if let Some(profile) = &self.profile {
            std::env::set_var("AWS_PROFILE", profile)
        }
    }

    pub fn region(&self) -> Result<Region> {
        Ok(match &self.region {
            Some(region) => Region::from_str(region)?,
            None => Region::default(),
        })
    }

    pub fn colored_string(&self) -> ColoredString {
        let mut sb = ColoredString::empty();

        sb.push_str(if self.cache {
            ColoredString::new(" Cache ", None, Some(Color::Yellow))
        } else {
            ColoredString::no_style("")
        });

        sb.push_str(if self.export {
            ColoredString::new(" Export ", None, Some(Color::Green))
        } else {
            ColoredString::no_style("")
        });

        sb
    }
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum SubCommand {
    /// CloudWatch.
    #[structopt(name = "cloudwatch")]
    Cloudwatch {
        #[structopt(subcommand)]
        command: CloudwatchCommand,
    },

    /// Ec2
    #[structopt(name = "ec2")]
    Ec2 {
        #[structopt(subcommand)]
        command: Ec2Command,
    },

    /// Cloudwatch Logs.
    #[structopt(name = "logs")]
    Logs {
        #[structopt(subcommand)]
        command: LogsCommand,
    },

    /// RDS.
    #[structopt(name = "ssm")]
    Rds {
        #[structopt(subcommand)]
        command: RdsCommand,
    },

    /// Systems Manager.
    #[structopt(name = "ssm")]
    Ssm {
        #[structopt(subcommand)]
        command: SsmCommand,
    },

    /// List or delete cached api responses.
    #[structopt(name = "cache")]
    Cache {
        #[structopt(subcommand)]
        command: CacheCommand,
    },
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Ec2Command {
    #[structopt(name = "instance")]
    Instance,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum LogsCommand {
    #[structopt(name = "log-group")]
    LogGroup,
    #[structopt(name = "log-stream")]
    LogStream {
        /// log group name
        log_group_name: Option<String>,
    },
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum RdsCommand {
    #[structopt(name = "db-instance")]
    DbInstance,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum SsmCommand {
    #[structopt(name = "automation-execution")]
    AutomationExecution,
    #[structopt(name = "document")]
    Document,
    #[structopt(name = "session")]
    Session {
        /// "Active" or "History"
        state: Option<String>,
    },
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum CloudwatchCommand {
    #[structopt(name = "alarm")]
    Alarm,
    #[structopt(name = "alarm-history")]
    AlarmHistory,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum CacheCommand {
    /// Creates session or assume role based on provided profile type.
    #[structopt(name = "list")]
    List,
    /// Creates session or assume role based on provided profile type.
    #[structopt(name = "clear")]
    Clear,
}

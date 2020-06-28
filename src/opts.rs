use crate::error::Error::*;
use crate::error::Result;
use rusoto_core::Region;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "hrkk")]
pub(crate) struct Opts {
    /// Initial aws request count. Hit "M" to fetch more.
    #[structopt(short = "r", long = "request-count")]
    pub(crate) request_count: Option<u8>,

    /// Store aws api response as "response_body.txt" in the cache directory.
    #[structopt(short = "b", long = "debug")]
    pub(crate) debug: bool,

    /// Viewer window shows resources in YAML format.
    #[structopt(short = "y", long = "yaml")]
    pub(crate) yaml: bool,

    /// Profile name for the aws api request.
    #[structopt(short = "p", long = "profile")]
    pub(crate) profile: Option<String>,

    /// Aws region for the aws api request.
    #[structopt(short = "g", long = "region")]
    pub(crate) region: Option<String>,

    /// Delimiter for the output text. default is ","
    #[structopt(short = "d", long = "delimiter")]
    pub(crate) delimiter: Option<String>,

    /// Sub command.
    #[structopt(subcommand)]
    pub(crate) sub_command: Option<SubCommand>,
}

impl Opts {
    pub(crate) fn validate(&self) -> Result<()> {
        if let Some(count) = self.request_count {
            if count == 0 || 10 < count {
                return Err(ArgumentError(
                    "request-count must be between 1 and 10".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub(crate) fn request_count(&self) -> usize {
        match self.request_count {
            Some(count) => count as usize,
            None => 1 as usize,
        }
    }

    pub(crate) fn delimiter(&self) -> String {
        match &self.delimiter {
            Some(delimiter) => delimiter.to_string(),
            None => ",".to_string(),
        }
    }

    pub(crate) fn set_profile(&self) {
        if let Some(profile) = &self.profile {
            std::env::set_var("AWS_PROFILE", profile)
        }
    }

    pub(crate) fn region(&self) -> Result<Region> {
        Ok(match &self.region {
            Some(region) => Region::from_str(region)?,
            None => Region::default(),
        })
    }
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum SubCommand {
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
    #[structopt(name = "rds")]
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
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum Ec2Command {
    #[structopt(name = "instance")]
    Instance,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum LogsCommand {
    #[structopt(name = "log-group")]
    LogGroup,
    #[structopt(name = "log-stream")]
    LogStream {
        /// log group name
        log_group_name: Option<String>,
    },
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum RdsCommand {
    #[structopt(name = "db-instance")]
    DbInstance,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum SsmCommand {
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

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub(crate) enum CloudwatchCommand {
    #[structopt(name = "alarm")]
    Alarm,
    #[structopt(name = "alarm-history")]
    AlarmHistory,
}

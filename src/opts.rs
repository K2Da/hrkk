use crate::error::Error::*;
use crate::error::Result;
use rusoto_core::Region;
use serde::Serialize;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Serialize)]
#[structopt(name = "hrkk")]
pub(crate) struct Opts {
    /// Initial aws request count for list- or describe- api. Hit "M" to fetch more.
    #[structopt(short = "l", long = "list-request")]
    pub(crate) list_request_count: Option<u8>,

    /// Initial aws request count for get- api. Hit "G" to get more.
    #[structopt(short = "g", long = "get-request")]
    pub(crate) get_request_count: Option<u8>,

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
    #[structopt(short = "r", long = "region")]
    pub(crate) region: Option<String>,

    /// Delimiter for the output text. default is ","
    #[structopt(short = "d", long = "delimiter")]
    pub(crate) delimiter: Option<String>,

    /// Output aws console url for the selected resources
    #[structopt(short = "u", long = "console-url")]
    pub(crate) console_url: bool,

    /// Sub command.
    #[structopt(subcommand)]
    pub(crate) sub_command: Option<SubCommand>,
}

impl Opts {
    pub(crate) fn validate(&self) -> Result<()> {
        if let Some(count) = self.list_request_count {
            if count == 0 || 10 < count {
                return Err(ArgumentError(
                    "request-count must be between 1 and 10".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub(crate) fn list_request_count(&self) -> usize {
        match self.list_request_count {
            Some(count) => count as usize,
            None => 1 as usize,
        }
    }

    pub(crate) fn get_request_count(&self) -> usize {
        match self.get_request_count {
            Some(count) => count as usize,
            None => 10 as usize,
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

    pub(crate) fn region_name(&self) -> String {
        if let Ok(region) = self.region() {
            region.name().to_string()
        } else {
            "-".to_string()
        }
    }

    pub(crate) fn output_type(&self) -> OutputType {
        if self.console_url {
            OutputType::ConsoleURL
        } else {
            OutputType::ResourceIdentifier
        }
    }
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum SubCommand {
    /// ACM
    #[structopt(name = "acm")]
    Acm {
        #[structopt(subcommand)]
        command: Acm,
    },

    /// Athena
    #[structopt(name = "athena")]
    Athena {
        #[structopt(subcommand)]
        command: Athena,
    },

    /// AutoScaling
    #[structopt(name = "autoscaling")]
    Autoscaling {
        #[structopt(subcommand)]
        command: Autoscaling,
    },

    /// Cloudformation
    #[structopt(name = "cloudformation")]
    Cloudformation {
        #[structopt(subcommand)]
        command: Cloudformation,
    },

    /// Cloudfront
    #[structopt(name = "cloudfront")]
    Cloudfront {
        #[structopt(subcommand)]
        command: Cloudfront,
    },

    /// CloudWatch
    #[structopt(name = "cloudwatch")]
    Cloudwatch {
        #[structopt(subcommand)]
        command: Cloudwatch,
    },

    /// Ec2
    #[structopt(name = "ec2")]
    Ec2 {
        #[structopt(subcommand)]
        command: Ec2,
    },

    /// ElastiCache
    #[structopt(name = "elasticache")]
    Elasticache {
        #[structopt(subcommand)]
        command: Elasticache,
    },

    /// Elastictranscoder
    #[structopt(name = "elastictranscoder")]
    Elastictranscoder {
        #[structopt(subcommand)]
        command: Elastictranscoder,
    },

    /// Elastic Load Balancing
    #[structopt(name = "elb")]
    Elb {
        #[structopt(subcommand)]
        command: Elb,
    },

    /// Elasticsearch Service
    #[structopt(name = "es")]
    Es {
        #[structopt(subcommand)]
        command: Es,
    },

    /// Firehose Service
    #[structopt(name = "es")]
    Firehose {
        #[structopt(subcommand)]
        command: Firehose,
    },

    /// IAM
    #[structopt(name = "iam")]
    Iam {
        #[structopt(subcommand)]
        command: Iam,
    },

    /// Kinesis
    #[structopt(name = "kinesis")]
    Kinesis {
        #[structopt(subcommand)]
        command: Kinesis,
    },

    /// Lambda
    #[structopt(name = "lambda")]
    Lambda {
        #[structopt(subcommand)]
        command: Lambda,
    },

    /// Cloudwatch Logs.
    #[structopt(name = "logs")]
    Logs {
        #[structopt(subcommand)]
        command: Logs,
    },

    /// RDS.
    #[structopt(name = "rds")]
    Rds {
        #[structopt(subcommand)]
        command: RdsCommand,
    },

    /// Route53
    #[structopt(name = "route53")]
    Route53 {
        #[structopt(subcommand)]
        command: Route53,
    },

    /// S3
    #[structopt(name = "s3")]
    S3 {
        #[structopt(subcommand)]
        command: S3Command,
    },

    /// Systems Manager
    #[structopt(name = "ssm")]
    Ssm {
        #[structopt(subcommand)]
        command: Ssm,
    },
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Ec2 {
    #[structopt(name = "instance")]
    Instance,
    #[structopt(name = "launch-template")]
    LaunchTemplate,
    #[structopt(name = "image")]
    Image,
    #[structopt(name = "security-group")]
    SecurityGroup,
    #[structopt(name = "subnet")]
    Subnet,
    #[structopt(name = "vpc")]
    Vpc,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Elasticache {
    #[structopt(name = "cache-cluster")]
    CacheCluster,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Elastictranscoder {
    #[structopt(name = "pipeline")]
    Pipeline,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Elb {
    #[structopt(name = "load-balancer")]
    LoadBalancer,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Es {
    #[structopt(name = "domain")]
    Domain,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Firehose {
    #[structopt(name = "delivery-stream")]
    DeliveryStream,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Iam {
    #[structopt(name = "user")]
    User,
    #[structopt(name = "group")]
    Group,
    #[structopt(name = "role")]
    Role,
    #[structopt(name = "policy")]
    Policy,
    #[structopt(name = "mfa-device")]
    MfaDevice,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Kinesis {
    #[structopt(name = "stream")]
    Stream,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Logs {
    #[structopt(name = "log-group")]
    LogGroup,
    #[structopt(name = "log-stream")]
    LogStream {
        /// log group name
        log_group_name: Option<String>,
    },
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Lambda {
    #[structopt(name = "function")]
    Function,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum RdsCommand {
    #[structopt(name = "db-instance")]
    DbInstance,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum S3Command {
    #[structopt(name = "bucket")]
    Bucket,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Ssm {
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

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Cloudformation {
    #[structopt(name = "stack")]
    Stack,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Cloudfront {
    #[structopt(name = "distribution")]
    Distribution,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Cloudwatch {
    #[structopt(name = "alarm")]
    Alarm,
    #[structopt(name = "alarm-history")]
    AlarmHistory,
    #[structopt(name = "metric")]
    Metric,
    #[structopt(name = "dashboard")]
    Dashboard,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Acm {
    #[structopt(name = "certificate")]
    Certificate,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Athena {
    #[structopt(name = "query-execution")]
    QueryExecution,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Autoscaling {
    #[structopt(name = "auto-scaling-group")]
    AutoScalingGroup,
}

#[derive(StructOpt, Debug, PartialEq, Clone, Serialize)]
pub(crate) enum Route53 {
    #[structopt(name = "hosted-zone")]
    HostedZone,
    #[structopt(name = "resource-record-set")]
    ResourceRecordSet {
        /// hosted zone id
        zone_id: Option<String>,
    },
}

pub(crate) enum OutputType {
    ResourceIdentifier,
    ConsoleURL,
}

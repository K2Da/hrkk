pub(crate) use crate::api::list::make_vec;
pub(crate) use crate::error::Error::*;
pub(crate) use crate::error::Result;
pub(crate) use crate::opts::Opts;
pub(crate) use crate::opts::*;
pub(crate) use crate::service::{
    merge_yamls, next_token, resource_by_name, tag_value, AwsResource, ExecuteTarget, GetApi, Info,
    JsonGetApi, JsonListApi, JsonListMethod, Limit, ListApi, ResourceList, ResourceUrl, XmlGetApi,
    XmlListApi, XmlListMethod,
};
pub(crate) use crate::show;
pub(crate) use crate::show::Section;
pub(crate) use inflector::Inflector;
pub(crate) use rusoto_core::Region;
pub(crate) use rusoto_signature::SignedRequest;
pub(crate) use serde::Serialize;
pub(crate) use serde_json::json;
pub(crate) use yaml_rust::Yaml;

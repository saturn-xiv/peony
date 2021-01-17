#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

extern crate actix_files;
extern crate amq_protocol_uri;
extern crate askama;
extern crate base64;
extern crate bytes;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate csv;
extern crate elasticsearch;
extern crate encoding_rs;
extern crate eui48;
extern crate futures;
extern crate grpcio;
extern crate hex;
extern crate humantime;
extern crate ini;
extern crate ipnetwork;
extern crate jsonwebtoken;
extern crate lettre;
extern crate md5;
extern crate mime;
extern crate nix;
extern crate percent_encoding;
extern crate protobuf;
extern crate r2d2;
extern crate rand;
extern crate redis as redis_;
extern crate regex;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate rusoto_sqs;
extern crate serde;
extern crate serde_xml_rs;
extern crate sodiumoxide;
extern crate tempfile;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate validator;
extern crate xml;
extern crate yaml_rust;

#[macro_use]
pub mod macros;

#[allow(deprecated)]
pub mod protos;

pub mod app;
pub mod aws;
pub mod cache;
pub mod crypto;
pub mod dict;
pub mod env;
pub mod errors;
pub mod hal;
pub mod i18n;
pub mod jwt;
pub mod mailer;
pub mod oauth;
pub mod orm;
pub mod parser;
pub mod plugins;
pub mod queue;
pub mod request;
pub mod settings;
pub mod sys;
pub mod twilio;

pub const XML_HEADER: &str = r###"<?xml version="1.0" encoding="utf-8" ?>"###;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Plain,
    Html,
    Markdown,
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Plain
    }
}

use std::fmt::{self};
use std::result::Result as StdResult;

use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    StdNetAddrParse(std::net::AddrParseError),
    StdNumParseInt(std::num::ParseIntError),
    StdNumParseFloat(std::num::ParseFloatError),
    StdStrUtf8(std::str::Utf8Error),
    StdStringFromUtf8(std::string::FromUtf8Error),
    StdSystemTime(std::time::SystemTimeError),

    Askama(askama::Error),
    ActixMailbox(actix::MailboxError),
    ActixMultipart(actix_multipart::MultipartError),
    ActixWebBlockingSerdeJson(actix_web::error::BlockingError<serde_json::Error>),
    ActixWebCanceled(actix_web::error::Canceled),
    ActixWebClientSendRequest(actix_web::client::SendRequestError),
    ActixWebClientJsonPayload(actix_web::client::JsonPayloadError),
    ActixWebClientPayload(actix_web::client::PayloadError),
    ActixWebHttpInvalidHeaderName(actix_web::http::header::InvalidHeaderName),
    Base64Decode(base64::DecodeError),
    DieselResult(diesel::result::Error),
    ChronoParse(chrono::ParseError),
    Eui48Parse(eui48::ParseError),
    FlexBuffersDeserialization(flexbuffers::DeserializationError),
    FlexBuffersSerialization(flexbuffers::SerializationError),
    IniParse(ini::ParseError),
    Ipnetwork(ipnetwork::IpNetworkError),
    JsonWebToken(jsonwebtoken::errors::Error),
    Lapin(lapin::Error),
    Lettre(lettre::error::Error),
    LettreAddress(lettre::address::AddressError),
    LettreTransportSmtp(lettre::transport::smtp::Error),
    MimeFromStr(mime::FromStrError),
    Nix(nix::Error),
    HandlebarsRender(handlebars::RenderError),
    HandlebarsTemplate(handlebars::TemplateError),
    HandlebarsTemplateRender(handlebars::TemplateRenderError),
    OpensslStack(openssl::error::ErrorStack),
    OpensslAesKey(openssl::aes::KeyError),
    Oping(oping::PingError),
    PahoMqtt(paho_mqtt::Error),
    R2d2(r2d2::Error),
    Redis(redis::RedisError),
    Regex(regex::Error),
    RusotoCoreRegionParse(rusoto_core::region::ParseRegionError),
    RusotoCoreRequestTls(rusoto_core::request::TlsError),
    RusotoCoreS3DeleteObject(rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>),
    RusotoCoreS3GetBucketLocation(rusoto_core::RusotoError<rusoto_s3::GetBucketLocationError>),
    RusotoCoreS3PutObject(rusoto_core::RusotoError<rusoto_s3::PutObjectError>),
    RusotoCoreS3CreateBucket(rusoto_core::RusotoError<rusoto_s3::CreateBucketError>),
    RusotoCoreS3ListBuckets(rusoto_core::RusotoError<rusoto_s3::ListBucketsError>),
    RusotoCoreS3ListObjectsV2(rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>),
    RusotoCoreS3DeleteBucket(rusoto_core::RusotoError<rusoto_s3::DeleteBucketError>),
    RusotoCoreSqsGetQueueUrl(rusoto_core::RusotoError<rusoto_sqs::GetQueueUrlError>),
    RusotoCoreSqsSendMessage(rusoto_core::RusotoError<rusoto_sqs::SendMessageError>),
    RusotoCoreSqsReceiveMessage(rusoto_core::RusotoError<rusoto_sqs::ReceiveMessageError>),
    RusotoCoreSqsCreateQueue(rusoto_core::RusotoError<rusoto_sqs::CreateQueueError>),
    SerdeJson(serde_json::Error),
    SerdeXml(serde_xml_rs::Error),
    Serialport(serialport::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    UrlParse(url::ParseError),
    Validator(validator::ValidationErrors),
    XmlWriter(xml::writer::Error),
    YamlEmit(yaml_rust::EmitError),
    YamlScan(yaml_rust::ScanError),
    Zmq(zmq::Error),

    Http(StatusCode, Option<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StdIo(v) => v.fmt(f),
            Self::StdNetAddrParse(v) => v.fmt(f),
            Self::StdNumParseInt(v) => v.fmt(f),
            Self::StdNumParseFloat(v) => v.fmt(f),
            Self::StdStrUtf8(v) => v.fmt(f),
            Self::StdStringFromUtf8(v) => v.fmt(f),
            Self::StdSystemTime(v) => v.fmt(f),

            Self::Askama(v) => v.fmt(f),
            Self::ActixMailbox(v) => v.fmt(f),
            Self::ActixMultipart(v) => v.fmt(f),
            Self::ActixWebBlockingSerdeJson(v) => v.fmt(f),
            Self::ActixWebCanceled(v) => v.fmt(f),
            Self::ActixWebClientSendRequest(v) => v.fmt(f),
            Self::ActixWebClientJsonPayload(v) => v.fmt(f),
            Self::ActixWebClientPayload(v) => v.fmt(f),
            Self::ActixWebHttpInvalidHeaderName(v) => v.fmt(f),
            Self::Base64Decode(v) => v.fmt(f),
            Self::ChronoParse(v) => v.fmt(f),
            Self::DieselResult(v) => v.fmt(f),
            Self::Eui48Parse(v) => v.fmt(f),
            Self::FlexBuffersDeserialization(v) => v.fmt(f),
            Self::FlexBuffersSerialization(v) => v.fmt(f),
            Self::IniParse(v) => v.fmt(f),
            Self::Ipnetwork(v) => v.fmt(f),
            Self::JsonWebToken(v) => v.fmt(f),
            Self::Lapin(v) => v.fmt(f),
            Self::Lettre(v) => v.fmt(f),
            Self::LettreAddress(v) => v.fmt(f),
            Self::LettreTransportSmtp(v) => v.fmt(f),
            Self::MimeFromStr(v) => v.fmt(f),
            Self::Nix(v) => v.fmt(f),
            Self::HandlebarsRender(v) => v.fmt(f),
            Self::HandlebarsTemplate(v) => v.fmt(f),
            Self::HandlebarsTemplateRender(v) => v.fmt(f),
            Self::OpensslStack(v) => v.fmt(f),
            Self::OpensslAesKey(_) => write!(f, "aes key is not 128, 192, or 256 bits"),
            Self::Oping(v) => v.fmt(f),
            Self::PahoMqtt(v) => v.fmt(f),
            Self::R2d2(v) => v.fmt(f),
            Self::Redis(v) => v.fmt(f),
            Self::Regex(v) => v.fmt(f),
            Self::RusotoCoreRegionParse(v) => v.fmt(f),
            Self::RusotoCoreRequestTls(v) => v.fmt(f),
            Self::RusotoCoreS3DeleteObject(v) => v.fmt(f),
            Self::RusotoCoreS3GetBucketLocation(v) => v.fmt(f),
            Self::RusotoCoreS3PutObject(v) => v.fmt(f),
            Self::RusotoCoreS3CreateBucket(v) => v.fmt(f),
            Self::RusotoCoreS3ListBuckets(v) => v.fmt(f),
            Self::RusotoCoreS3ListObjectsV2(v) => v.fmt(f),
            Self::RusotoCoreS3DeleteBucket(v) => v.fmt(f),
            Self::RusotoCoreSqsGetQueueUrl(v) => v.fmt(f),
            Self::RusotoCoreSqsSendMessage(v) => v.fmt(f),
            Self::RusotoCoreSqsReceiveMessage(v) => v.fmt(f),
            Self::RusotoCoreSqsCreateQueue(v) => v.fmt(f),
            Self::SerdeJson(v) => v.fmt(f),
            Self::SerdeXml(v) => v.fmt(f),
            Self::Serialport(v) => v.fmt(f),
            Self::TomlDe(v) => v.fmt(f),
            Self::TomlSer(v) => v.fmt(f),
            Self::UrlParse(v) => v.fmt(f),
            Self::Validator(v) => v.fmt(f),
            Self::XmlWriter(v) => v.fmt(f),
            Self::YamlEmit(v) => v.fmt(f),
            Self::YamlScan(v) => v.fmt(f),
            Self::Zmq(v) => v.fmt(f),

            Self::Http(v, r) => match r {
                Some(r) => r.fmt(f),
                None => v.fmt(f),
            },
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::StdStringFromUtf8(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::StdNumParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::StdNumParseFloat(err)
    }
}

impl From<xml::writer::Error> for Error {
    fn from(err: xml::writer::Error) -> Self {
        Self::XmlWriter(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::StdStrUtf8(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::StdNetAddrParse(err)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::StdSystemTime(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Self::R2d2(err)
    }
}

impl From<lapin::Error> for Error {
    fn from(err: lapin::Error) -> Self {
        Self::Lapin(err)
    }
}

impl From<paho_mqtt::Error> for Error {
    fn from(err: paho_mqtt::Error) -> Self {
        Self::PahoMqtt(err)
    }
}

impl From<flexbuffers::DeserializationError> for Error {
    fn from(err: flexbuffers::DeserializationError) -> Self {
        Self::FlexBuffersDeserialization(err)
    }
}

impl From<flexbuffers::SerializationError> for Error {
    fn from(err: flexbuffers::SerializationError) -> Self {
        Self::FlexBuffersSerialization(err)
    }
}

impl From<rusoto_core::region::ParseRegionError> for Error {
    fn from(err: rusoto_core::region::ParseRegionError) -> Self {
        Self::RusotoCoreRegionParse(err)
    }
}

impl From<rusoto_core::request::TlsError> for Error {
    fn from(err: rusoto_core::request::TlsError) -> Self {
        Self::RusotoCoreRequestTls(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::DeleteObjectError>) -> Self {
        Self::RusotoCoreS3DeleteObject(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::GetBucketLocationError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::GetBucketLocationError>) -> Self {
        Self::RusotoCoreS3GetBucketLocation(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::PutObjectError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::PutObjectError>) -> Self {
        Self::RusotoCoreS3PutObject(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::CreateBucketError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::CreateBucketError>) -> Self {
        Self::RusotoCoreS3CreateBucket(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::ListBucketsError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::ListBucketsError>) -> Self {
        Self::RusotoCoreS3ListBuckets(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>) -> Self {
        Self::RusotoCoreS3ListObjectsV2(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::DeleteBucketError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_s3::DeleteBucketError>) -> Self {
        Self::RusotoCoreS3DeleteBucket(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_sqs::GetQueueUrlError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_sqs::GetQueueUrlError>) -> Self {
        Self::RusotoCoreSqsGetQueueUrl(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_sqs::SendMessageError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_sqs::SendMessageError>) -> Self {
        Self::RusotoCoreSqsSendMessage(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_sqs::ReceiveMessageError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_sqs::ReceiveMessageError>) -> Self {
        Self::RusotoCoreSqsReceiveMessage(err)
    }
}

impl From<rusoto_core::RusotoError<rusoto_sqs::CreateQueueError>> for Error {
    fn from(err: rusoto_core::RusotoError<rusoto_sqs::CreateQueueError>) -> Self {
        Self::RusotoCoreSqsCreateQueue(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::DieselResult(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Self::Askama(err)
    }
}

impl From<ini::ParseError> for Error {
    fn from(err: ini::ParseError) -> Self {
        Self::IniParse(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Self::Redis(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Self::Base64Decode(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Self::ChronoParse(err)
    }
}

impl From<eui48::ParseError> for Error {
    fn from(err: eui48::ParseError) -> Self {
        Self::Eui48Parse(err)
    }
}

impl From<ipnetwork::IpNetworkError> for Error {
    fn from(err: ipnetwork::IpNetworkError) -> Self {
        Self::Ipnetwork(err)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::JsonWebToken(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParse(err)
    }
}

impl From<lettre::error::Error> for Error {
    fn from(err: lettre::error::Error) -> Self {
        Self::Lettre(err)
    }
}

impl From<lettre::address::AddressError> for Error {
    fn from(err: lettre::address::AddressError) -> Self {
        Self::LettreAddress(err)
    }
}
impl From<lettre::transport::smtp::Error> for Error {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        Self::LettreTransportSmtp(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDe(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::TomlSer(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Self::SerdeXml(err)
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Self::Nix(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::Validator(err)
    }
}

impl From<mime::FromStrError> for Error {
    fn from(err: mime::FromStrError) -> Self {
        Self::MimeFromStr(err)
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Self::OpensslStack(err)
    }
}

impl From<openssl::aes::KeyError> for Error {
    fn from(err: openssl::aes::KeyError) -> Self {
        Self::OpensslAesKey(err)
    }
}

impl From<actix_multipart::MultipartError> for Error {
    fn from(err: actix_multipart::MultipartError) -> Self {
        Self::ActixMultipart(err)
    }
}

impl From<actix_web::error::BlockingError<serde_json::Error>> for Error {
    fn from(err: actix_web::error::BlockingError<serde_json::Error>) -> Self {
        Self::ActixWebBlockingSerdeJson(err)
    }
}

impl From<actix::MailboxError> for Error {
    fn from(err: actix::MailboxError) -> Self {
        Self::ActixMailbox(err)
    }
}
impl From<actix_web::http::header::InvalidHeaderName> for Error {
    fn from(err: actix_web::http::header::InvalidHeaderName) -> Self {
        Self::ActixWebHttpInvalidHeaderName(err)
    }
}

impl From<actix_web::client::SendRequestError> for Error {
    fn from(err: actix_web::client::SendRequestError) -> Self {
        Self::ActixWebClientSendRequest(err)
    }
}

impl From<actix_web::client::JsonPayloadError> for Error {
    fn from(err: actix_web::client::JsonPayloadError) -> Self {
        Self::ActixWebClientJsonPayload(err)
    }
}

impl From<actix_web::client::PayloadError> for Error {
    fn from(err: actix_web::client::PayloadError) -> Self {
        Self::ActixWebClientPayload(err)
    }
}

impl From<actix_web::error::Canceled> for Error {
    fn from(err: actix_web::error::Canceled) -> Self {
        Self::ActixWebCanceled(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Self::HandlebarsRender(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Self::HandlebarsTemplate(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        Self::HandlebarsTemplateRender(err)
    }
}

impl From<yaml_rust::EmitError> for Error {
    fn from(err: yaml_rust::EmitError) -> Self {
        Self::YamlEmit(err)
    }
}

impl From<yaml_rust::ScanError> for Error {
    fn from(err: yaml_rust::ScanError) -> Self {
        Self::YamlScan(err)
    }
}

impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Self {
        Self::Serialport(err)
    }
}

impl From<oping::PingError> for Error {
    fn from(err: oping::PingError) -> Self {
        Self::Oping(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Self::Regex(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Self {
        Self::Zmq(err)
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Interrupted, err.to_string())
    }
}
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .content_type(mime::TEXT_PLAIN_UTF_8.to_string())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Http(it, _) => it,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

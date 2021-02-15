use std::ops::Deref;

use actix_web::{post, web, HttpResponse, Responder};

use super::super::super::super::{
    errors::Result,
    orm::postgresql::Pool as Db,
    parser::to_xml_response,
    twilio::{
        sms::{CallbackForm, InboundResponse, ReplyForm},
        voice::VoiceForm,
    },
};

#[post("/callback")]
async fn callback(db: web::Data<Db>, form: web::Form<CallbackForm>) -> Result<impl Responder> {
    let db = db.get()?;
    let _db = db.deref();
    let form = form.into_inner();
    info!("receive {:?}", form);
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

#[post("/reply")]
async fn reply(db: web::Data<Db>, form: web::Form<ReplyForm>) -> Result<impl Responder> {
    let db = db.get()?;
    let _db = db.deref();
    let form = form.into_inner();
    info!("receive {:?}", form);

    // TODO
    to_xml_response(&InboundResponse { message: None })
}

#[post("/voice")]
async fn voice(db: web::Data<Db>, form: web::Form<VoiceForm>) -> Result<impl Responder> {
    let db = db.get()?;
    let _db = db.deref();
    let form = form.into_inner();
    info!("receive {:?}", form);
    // TODO

    to_xml_response(&InboundResponse { message: None })
}

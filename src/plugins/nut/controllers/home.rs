use actix_web::{get, web, HttpResponse, Responder};

use super::super::super::super::{errors::Result, orm::postgresql::Pool as Db};

#[get("/")]
async fn index(_db: web::Data<Db>) -> Result<impl Responder> {
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

#[get("/rss/{lang}.xml")]
async fn rss(_db: web::Data<Db>, path: web::Path<(String,)>) -> Result<impl Responder> {
    debug!("generate {}", path.into_inner().0);
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

// https://developers.google.com/search/docs/advanced/sitemaps/build-sitemap
// https://www.sitemaps.org/protocol.html
#[get("/sitemap.xml")]
async fn sitemap(_db: web::Data<Db>) -> Result<impl Responder> {
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

#[get("/sitemap/{lang}.xml")]
async fn sitemap_by_lang(
    _db: web::Data<Db>,
    _path: web::Path<(String,)>,
) -> Result<impl Responder> {
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

// https://developers.google.com/search/reference/robots_txt
#[get("/robots.txt")]
async fn robots_txt(_db: web::Data<Db>) -> Result<impl Responder> {
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

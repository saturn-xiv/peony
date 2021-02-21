pub mod home;
pub mod twilio;
pub mod users;

use actix_web::web;

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/twilio")
                    .service(twilio::callback)
                    .service(twilio::reply)
                    .service(twilio::voice),
            )
            .service(
                web::scope("/users")
                    .service(users::sign_in)
                    .service(users::sign_up),
            ),
    )
    .service(home::rss)
    .service(home::sitemap)
    .service(home::sitemap_by_lang)
    .service(home::robots_txt)
    .service(home::index);
}

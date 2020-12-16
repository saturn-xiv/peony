pub mod cache;
pub mod database;
pub mod generate;
pub mod server;

use std::ops::Deref;

use actix_web::http::StatusCode;
use clap::{App, Arg};

use super::{
    env,
    errors::{Error, Result},
    orm::migration::Dao as MigrationDao,
    parser,
    plugins::{forum, nut, Plugin},
};

pub fn launch() -> Result<()> {
    let matches = App::new(env::NAME)
        .version(env::VERSION)
        .author(env::AUTHORS)
        .about(env::DESCRIPTION)
        .before_help(env::BANNER)
        .after_help(env::HOMEPAGE)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .about("Config file(.toml)")
                .default_value("config.toml")
                .takes_value(true),
        )
        .subcommand(
            App::new("db")
                .about("PostgreSQL operations")
                .subcommand(App::new("migrate").about("Migrate database to latest migration"))
                .subcommand(App::new("rollback").about("Rollback database the last migration"))
                .subcommand(App::new("status").about("Show database schema status")),
        )
        .subcommand(
            App::new("cache")
                .about("Redis operations")
                .subcommand(App::new("list").about("List all cache keys"))
                .subcommand(App::new("clear").about("Remove all keys")),
        )
        .subcommand(
            App::new("crawler").about("Run crawler").arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .about("Crawler name")
                    .takes_value(true),
            ),
        )
        .subcommand(
            App::new("generate")
                .about("Generate files")
                .subcommand(
                    App::new("config")
                        .arg(
                            Arg::new("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .about("Name"),
                        )
                        .about("Generate config.toml"),
                )
                .subcommand(
                    App::new("nginx")
                        .arg(
                            Arg::new("domain")
                                .short('d')
                                .long("domain")
                                .required(true)
                                .takes_value(true)
                                .about("Domain name"),
                        )
                        .arg(
                            Arg::new("port")
                                .short('p')
                                .long("port")
                                .required(true)
                                .default_value("8080")
                                .takes_value(true)
                                .about("Listen port"),
                        )
                        .about("Generate nginx.conf"),
                )
                .subcommand(
                    App::new("systemd")
                        .arg(
                            Arg::new("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .about("Name"),
                        )
                        .about("Generate systemd service.conf"),
                ),
        )
        .subcommand(
            App::new("deploy")
                .about("Run deploy tasks by ssh & rsync")
                .arg(
                    Arg::new("job")
                        .short('j')
                        .long("job")
                        .required(true)
                        .takes_value(true)
                        .about("Job"),
                )
                .arg(
                    Arg::new("inventory")
                        .short('i')
                        .long("inventory")
                        .required(true)
                        .takes_value(true)
                        .about("Inventory"),
                ),
        )
        .subcommand(
            App::new("http")
                .about("HTTP api")
                .subcommand(App::new("server").about("Start http listener")),
        )
        .get_matches();
    if sodiumoxide::init().is_err() {
        return Err(Error::Http(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("init sodium".to_string()),
        ));
    }

    debug!("run on debug mode");
    if let Some(matches) = matches.subcommand_matches("generate") {
        if let Some(matches) = matches.subcommand_matches("config") {
            let name = matches.value_of("name").unwrap();
            return generate::config::run::<&str, env::Config>(name);
        }
        if let Some(matches) = matches.subcommand_matches("nginx") {
            let domain = matches.value_of("domain").unwrap();
            let port = matches.value_of("port").unwrap().parse()?;
            return generate::nginx::run(domain, port);
        }
        if let Some(matches) = matches.subcommand_matches("systemd") {
            let name = matches.value_of("name").unwrap();
            return generate::systemd::run(name);
        }
    }

    if let Some(matches) = matches.subcommand_matches("deploy") {
        let job = matches.value_of("job").unwrap();
        let inventory = matches.value_of("inventory").unwrap();
        // TODO
        // return deploy::run(&inventory, &job);
    }

    let config = matches.value_of("config").unwrap();
    info!("load configuration from {}", config);
    let config: env::Config = parser::from_toml(config)?;
    if let Some(matches) = matches.subcommand_matches("db") {
        let root = matches.value_of("folder").unwrap();
        let db = config.postgresql.open()?;
        let db = db.get()?;
        info!("using folder {}", root);
        let mut items = Vec::new();
        {
            items.extend(nut::Plugin::migrations());
            items.extend(forum::Plugin::migrations());
        }
        db.check()?;
        db.load(&items)?;
        if matches.subcommand_matches("migrate").is_some() {
            return db.migrate();
        }
        if matches.subcommand_matches("rollback").is_some() {
            return db.rollback();
        }
        if matches.subcommand_matches("status").is_some() {
            println!("VERSION\tNAME\t\tRUN AT");
            for it in db.status()? {
                println!("{}", it);
            }
            return Ok(());
        }
    }
    // let cfg = "config.toml";
    // let matches = clap::App::new(env::NAME)
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author(env::AUTHORS)
    //     .about(env::DESCRIPTION)
    //     .before_help(env::BANNER)
    //     .after_help(env::HOMEPAGE)
    //     .subcommand(generate::nginx::command())
    //     .subcommand(
    //         SubCommand::with_name(generate::config::NAME).about(&*generate::config::help(cfg)),
    //     )
    //     .subcommand(generate::systemd::command())
    //     .subcommand(database::migrate::command())
    //     .subcommand(database::rollback::command())
    //     .subcommand(database::status::command())
    //     .subcommand(i18n::sync::command())
    //     .subcommand(SubCommand::with_name(http::routes::NAME).about(http::routes::ABOUT))
    //     .get_matches();

    // if matches.subcommand_matches(http::routes::NAME).is_some() {
    //     return http::routes::run();
    // }

    // if matches.subcommand_matches(generate::config::NAME).is_some() {
    //     return generate::config::run::<&'static str, env::Config>(cfg);
    // }
    // if matches
    //     .subcommand_matches(generate::systemd::COMMAND_NAME)
    //     .is_some()
    // {
    //     return generate::systemd::run();
    // }

    // if let Some(matches) = matches.subcommand_matches(generate::nginx::COMMAND_NAME) {
    //     let name = matches.value_of(generate::nginx::ARG_SERVER_NAME).unwrap();
    //     return generate::nginx::run(name.to_string(), cfg.http.port);
    // }

    // if matches
    //     .subcommand_matches(i18n::sync::COMMAND_NAME)
    //     .is_some()
    // {
    //     return i18n::sync::run(cfg);
    // }

    // if matches
    //     .subcommand_matches(database::migrate::COMMAND_NAME)
    //     .is_some()
    // {
    //     let db = cfg.database.open()?;
    //     let db = db.get()?;
    //     return database::migrate::run(&db);
    // }
    // if matches
    //     .subcommand_matches(database::rollback::COMMAND_NAME)
    //     .is_some()
    // {
    //     let db = cfg.database.open()?;
    //     let db = db.get()?;
    //     return database::rollback::run(&db);
    // }
    // if matches
    //     .subcommand_matches(database::status::COMMAND_NAME)
    //     .is_some()
    // {
    //     let db = cfg.database.open()?;
    //     let db = db.get()?;
    //     return database::status::run(&db);
    // }

    // http::server::launch(cfg)
    Ok(())
}

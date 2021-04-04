pub mod db;
pub mod generate;
pub mod http;

use std::ops::Deref;

use clap::{App, Arg};

use super::{
    cache::Provider as CacheProvider,
    env,
    errors::Result,
    parser,
    plugins::{forum, nut, ops, Plugin},
};

pub async fn launch() -> Result<()> {
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
            App::new("db")
                .about("PostgreSql operations")
                .subcommand(App::new("migrate").about("Runs all pending migrations"))
                .subcommand(App::new("rollback").about("Reverts the latest run migration"))
                .subcommand(
                    App::new("status").about(
                        "Lists all available migrations, marking those that have been applied",
                    ),
                )
                .subcommand(App::new("reset").about("Reverts and re-runs the latest migration")),
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
                    Arg::new("recipe")
                        .short('r')
                        .long("recipe")
                        .required(true)
                        .takes_value(true)
                        .about("Recipe"),
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
        .subcommand(App::new("http").about("Start http listener"))
        .get_matches();

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
        let recipe = matches.value_of("recipe").unwrap();
        let inventory = matches.value_of("inventory").unwrap();
        return ops::deploy::run(&inventory, &recipe).await;
    }

    let config = matches.value_of("config").unwrap();
    info!("load configuration from {}", config);
    let config: env::Config = parser::from_toml(config)?;
    if let Some(matches) = matches.subcommand_matches("db") {
        let db = config.postgresql.open()?;
        let db = db.get()?;
        let db = db.deref();
        if matches.subcommand_matches("migrate").is_some() {
            return self::db::migrate(db);
        }
        if matches.subcommand_matches("rollback").is_some() {
            return self::db::rollback(db);
        }
        if matches.subcommand_matches("status").is_some() {
            return self::db::status(db);
        }
        if matches.subcommand_matches("reset").is_some() {
            return self::db::reset(db);
        }
    }

    if let Some(matches) = matches.subcommand_matches("cache") {
        let db = config.redis.open()?;
        if matches.subcommand_matches("list").is_some() {
            println!("{:<12} KEY", "TTL");
            for it in db.keys()? {
                println!("{:<12} {}", it.1, it.0);
            }
            return Ok(());
        }
        if matches.subcommand_matches("clear").is_some() {
            return db.clear();
        }
    }

    http::launch(config).await
}

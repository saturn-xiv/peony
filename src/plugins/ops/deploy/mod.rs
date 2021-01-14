pub mod models;

use std::io::Error as StdIoError;
use std::result::Result as StdResult;

use actix::{Actor, Context, Handler, Message};

use super::super::super::errors::Result;

#[derive(Message)]
#[rtype(result = "StdResult<(), StdIoError>")]
struct SshMessage {
    inventory: String,
    host: String,
    vars: models::Vars,
    tasks: Vec<models::Command>,
}

struct SshActor;

impl Actor for SshActor {
    type Context = Context<Self>;
}

impl Handler<SshMessage> for SshActor {
    type Result = StdResult<(), StdIoError>;

    fn handle(&mut self, msg: SshMessage, _ctx: &mut Context<Self>) -> Self::Result {
        for it in msg.tasks {
            info!("run {} on {}", it, msg.host);
            it.run(&msg.inventory, &msg.host, &msg.vars)?;
        }
        Ok(())
    }
}

pub async fn run(inventory: &str, recipe: &str) -> Result<()> {
    let excutors = models::Recipe::load(recipe, inventory)?;
    for (hosts, tasks) in excutors {
        let addr = SshActor.start();
        for (host, vars) in hosts {
            let _ = addr
                .send(SshMessage {
                    inventory: inventory.to_string(),
                    vars: vars.clone(),
                    tasks: tasks.clone(),
                    host: host.clone(),
                })
                .await?;
        }
    }

    info!("Done.");
    Ok(())
}

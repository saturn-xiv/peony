use grpcio::{RpcContext, UnarySink};

use super::super::{i18n, orm::migration::New as Migration, settings};

pub struct Plugin {}

// impl nut_grpc::NutService for Plugin {
//     fn sign_in(
//         &mut self,
//         ctx: RpcContext,
//         req: nut::SignInRequest,
//         sink: UnarySink<nut::SignInResponse>,
//     ) {
//     }
//     fn sign_up(&mut self, ctx: RpcContext, req: nut::SignUpRequest, sink: UnarySink<nut::Ok>) {}
// }

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20201214174607",
            name: "create-auth",
            up: include_str!("create-auth-up.sql"),
            down: include_str!("create-auth-down.sql"),
        });
        items.push(Migration {
            version: "20201214174759",
            name: "create-site",
            up: include_str!("create-site-up.sql"),
            down: include_str!("create-site-down.sql"),
        });
        items.push(Migration {
            version: "20201214175056",
            name: "create-settings",
            up: settings::UP,
            down: settings::DOWN,
        });
        items.push(Migration {
            version: "20201214175102",
            name: "create-locales",
            up: i18n::UP,
            down: i18n::DOWN,
        });
        items
    }
}

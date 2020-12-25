use grpcio::{RpcContext, UnarySink};

use super::super::orm::migration::New as Migration;

pub struct Plugin {}

// impl forum_grpc::ForumService for Plugin {
//     fn create_post(
//         &mut self,
//         ctx: RpcContext,
//         req: forum::CreatePostRequest,
//         sink: UnarySink<nut::Ok>,
//     ) {
//     }
// }

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20201214175333",
            name: "create-forum",
            up: include_str!("create-up.sql"),
            down: include_str!("create-down.sql"),
        });
        items
    }
}

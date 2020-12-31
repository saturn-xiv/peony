use grpcio::{RpcContext, UnarySink};

use super::super::super::protos::{
    empty::Empty, forum::CreatePostRequest, forum_grpc::ForumService,
};

impl ForumService for super::Plugin {
    fn create_post(&mut self, _ctx: RpcContext, _req: CreatePostRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
}

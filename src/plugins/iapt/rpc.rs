use grpcio::{RpcContext, UnarySink};

use super::super::super::protos::{empty::Empty, iapt::ImportRequest, iapt_grpc::IaptService};

impl IaptService for super::Plugin {
    fn import(&mut self, _ctx: RpcContext, _req: ImportRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
}

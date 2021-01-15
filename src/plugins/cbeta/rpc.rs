use grpcio::{RpcContext, UnarySink};

use super::super::super::protos::{cbeta::ImportRequest, cbeta_grpc::CbetaService, empty::Empty};

impl CbetaService for super::Plugin {
    fn import(&mut self, _ctx: RpcContext, _req: ImportRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
}

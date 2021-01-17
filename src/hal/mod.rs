pub mod audio;
pub mod gpio;
pub mod tty;

use grpcio::{RpcContext, UnarySink};

use super::protos::{
    empty::Empty,
    hal::{
        AudioPlayRequest, ButtonResponse, LedGetRequest, LedGetResponse, LedSetRequest, TtyRequest,
        TtyResponse,
    },
    hal_grpc::{AudioService, GpioService, TtyService},
};

pub struct Hardware {}

impl AudioService for Hardware {
    fn play(&mut self, _ctx: RpcContext, _req: AudioPlayRequest, _sink: UnarySink<Empty>) {}
    fn stop(&mut self, _ctx: RpcContext, _req: Empty, _sink: UnarySink<Empty>) {}
}

impl TtyService for Hardware {
    fn write(&mut self, _ctx: RpcContext, _req: TtyRequest, _sink: UnarySink<Empty>) {}
    fn read(&mut self, _ctx: RpcContext, _req: Empty, _sink: UnarySink<TtyResponse>) {}
}

impl GpioService for Hardware {
    fn led_get(&mut self, _ctx: RpcContext, _req: LedGetRequest, _sink: UnarySink<LedGetResponse>) {
    }
    fn led_set(&mut self, _ctx: RpcContext, _req: LedSetRequest, _sink: UnarySink<Empty>) {}
    fn button_report(&mut self, _ctx: RpcContext, _req: Empty, _sink: UnarySink<ButtonResponse>) {}
}

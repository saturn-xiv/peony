// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_IAPT_SERVICE_IMPORT: ::grpcio::Method<
    super::iapt::ImportRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.iapt.IaptService/Import",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

#[derive(Clone)]
pub struct IaptServiceClient {
    client: ::grpcio::Client,
}

impl IaptServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        IaptServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn import_opt(
        &self,
        req: &super::iapt::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_IAPT_SERVICE_IMPORT, req, opt)
    }

    pub fn import(
        &self,
        req: &super::iapt::ImportRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_async_opt(
        &self,
        req: &super::iapt::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_IAPT_SERVICE_IMPORT, req, opt)
    }

    pub fn import_async(
        &self,
        req: &super::iapt::ImportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.import_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait IaptService {
    fn import(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::iapt::ImportRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
}

pub fn create_iapt_service<S: IaptService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_IAPT_SERVICE_IMPORT, move |ctx, req, resp| {
        instance.import(ctx, req, resp)
    });
    builder.build()
}

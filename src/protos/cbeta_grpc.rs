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

const METHOD_CBETA_SERVICE_IMPORT: ::grpcio::Method<
    super::cbeta::ImportRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.cbeta.CbetaService/Import",
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
pub struct CbetaServiceClient {
    client: ::grpcio::Client,
}

impl CbetaServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CbetaServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn import_opt(
        &self,
        req: &super::cbeta::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_CBETA_SERVICE_IMPORT, req, opt)
    }

    pub fn import(
        &self,
        req: &super::cbeta::ImportRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_async_opt(
        &self,
        req: &super::cbeta::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_CBETA_SERVICE_IMPORT, req, opt)
    }

    pub fn import_async(
        &self,
        req: &super::cbeta::ImportRequest,
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

pub trait CbetaService {
    fn import(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::cbeta::ImportRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
}

pub fn create_cbeta_service<S: CbetaService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_CBETA_SERVICE_IMPORT, move |ctx, req, resp| {
        instance.import(ctx, req, resp)
    });
    builder.build()
}

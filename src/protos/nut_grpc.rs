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

const METHOD_NUT_SERVICE_HEARTBEAT: ::grpcio::Method<
    super::empty::Empty,
    super::nut::HeartbeatResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.nut.NutService/Heartbeat",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_NUT_SERVICE_SET_LOCALE: ::grpcio::Method<
    super::nut::SetLocaleRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.nut.NutService/SetLocale",
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
pub struct NutServiceClient {
    client: ::grpcio::Client,
}

impl NutServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NutServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn heartbeat_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::nut::HeartbeatResponse> {
        self.client
            .unary_call(&METHOD_NUT_SERVICE_HEARTBEAT, req, opt)
    }

    pub fn heartbeat(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<super::nut::HeartbeatResponse> {
        self.heartbeat_opt(req, ::grpcio::CallOption::default())
    }

    pub fn heartbeat_async_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::HeartbeatResponse>> {
        self.client
            .unary_call_async(&METHOD_NUT_SERVICE_HEARTBEAT, req, opt)
    }

    pub fn heartbeat_async(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::HeartbeatResponse>> {
        self.heartbeat_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_locale_opt(
        &self,
        req: &super::nut::SetLocaleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_NUT_SERVICE_SET_LOCALE, req, opt)
    }

    pub fn set_locale(
        &self,
        req: &super::nut::SetLocaleRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.set_locale_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_locale_async_opt(
        &self,
        req: &super::nut::SetLocaleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_NUT_SERVICE_SET_LOCALE, req, opt)
    }

    pub fn set_locale_async(
        &self,
        req: &super::nut::SetLocaleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.set_locale_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait NutService {
    fn heartbeat(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::empty::Empty,
        sink: ::grpcio::UnarySink<super::nut::HeartbeatResponse>,
    );
    fn set_locale(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::nut::SetLocaleRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
}

pub fn create_nut_service<S: NutService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NUT_SERVICE_HEARTBEAT, move |ctx, req, resp| {
        instance.heartbeat(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NUT_SERVICE_SET_LOCALE, move |ctx, req, resp| {
        instance.set_locale(ctx, req, resp)
    });
    builder.build()
}

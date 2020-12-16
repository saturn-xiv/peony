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

const METHOD_NUT_SERVICE_SIGN_IN: ::grpcio::Method<super::nut::SignInRequest, super::nut::SignInResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NutService/SignIn",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NUT_SERVICE_SIGN_UP: ::grpcio::Method<super::nut::SignUpRequest, super::nut::Ok> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NutService/SignUp",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
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

    pub fn sign_in_opt(&self, req: &super::nut::SignInRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::nut::SignInResponse> {
        self.client.unary_call(&METHOD_NUT_SERVICE_SIGN_IN, req, opt)
    }

    pub fn sign_in(&self, req: &super::nut::SignInRequest) -> ::grpcio::Result<super::nut::SignInResponse> {
        self.sign_in_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_in_async_opt(&self, req: &super::nut::SignInRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::SignInResponse>> {
        self.client.unary_call_async(&METHOD_NUT_SERVICE_SIGN_IN, req, opt)
    }

    pub fn sign_in_async(&self, req: &super::nut::SignInRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::SignInResponse>> {
        self.sign_in_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_up_opt(&self, req: &super::nut::SignUpRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::nut::Ok> {
        self.client.unary_call(&METHOD_NUT_SERVICE_SIGN_UP, req, opt)
    }

    pub fn sign_up(&self, req: &super::nut::SignUpRequest) -> ::grpcio::Result<super::nut::Ok> {
        self.sign_up_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_up_async_opt(&self, req: &super::nut::SignUpRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::Ok>> {
        self.client.unary_call_async(&METHOD_NUT_SERVICE_SIGN_UP, req, opt)
    }

    pub fn sign_up_async(&self, req: &super::nut::SignUpRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::nut::Ok>> {
        self.sign_up_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NutService {
    fn sign_in(&mut self, ctx: ::grpcio::RpcContext, req: super::nut::SignInRequest, sink: ::grpcio::UnarySink<super::nut::SignInResponse>);
    fn sign_up(&mut self, ctx: ::grpcio::RpcContext, req: super::nut::SignUpRequest, sink: ::grpcio::UnarySink<super::nut::Ok>);
}

pub fn create_nut_service<S: NutService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NUT_SERVICE_SIGN_IN, move |ctx, req, resp| {
        instance.sign_in(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NUT_SERVICE_SIGN_UP, move |ctx, req, resp| {
        instance.sign_up(ctx, req, resp)
    });
    builder.build()
}

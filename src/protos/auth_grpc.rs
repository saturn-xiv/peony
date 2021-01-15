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

const METHOD_USER_SERVICE_IMPORT: ::grpcio::Method<
    super::auth::ImportRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/Import",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_USER_SERVICE_SIGN_IN: ::grpcio::Method<
    super::auth::SignInRequest,
    super::auth::SignInResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/SignIn",
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
pub struct UserServiceClient {
    client: ::grpcio::Client,
}

impl UserServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UserServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn import_opt(
        &self,
        req: &super::auth::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_IMPORT, req, opt)
    }

    pub fn import(
        &self,
        req: &super::auth::ImportRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_async_opt(
        &self,
        req: &super::auth::ImportRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_IMPORT, req, opt)
    }

    pub fn import_async(
        &self,
        req: &super::auth::ImportRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.import_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_in_opt(
        &self,
        req: &super::auth::SignInRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::auth::SignInResponse> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_SIGN_IN, req, opt)
    }

    pub fn sign_in(
        &self,
        req: &super::auth::SignInRequest,
    ) -> ::grpcio::Result<super::auth::SignInResponse> {
        self.sign_in_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_in_async_opt(
        &self,
        req: &super::auth::SignInRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SignInResponse>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_SIGN_IN, req, opt)
    }

    pub fn sign_in_async(
        &self,
        req: &super::auth::SignInRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SignInResponse>> {
        self.sign_in_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait UserService {
    fn import(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::ImportRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn sign_in(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::SignInRequest,
        sink: ::grpcio::UnarySink<super::auth::SignInResponse>,
    );
}

pub fn create_user_service<S: UserService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_IMPORT, move |ctx, req, resp| {
        instance.import(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_SIGN_IN, move |ctx, req, resp| {
        instance.sign_in(ctx, req, resp)
    });
    builder.build()
}

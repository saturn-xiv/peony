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

const METHOD_USER_SERVICE_SIGN_UP: ::grpcio::Method<
    super::auth::SignUpRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/SignUp",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_USER_SERVICE_CONFIRM: ::grpcio::Method<
    super::auth::EmailRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/Confirm",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_USER_SERVICE_UNLOCK: ::grpcio::Method<super::auth::EmailRequest, super::empty::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/peony.auth.UserService/Unlock",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_USER_SERVICE_FORGOT_PASSWORD: ::grpcio::Method<
    super::auth::EmailRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/ForgotPassword",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_USER_SERVICE_RESET_PASSWORD: ::grpcio::Method<
    super::auth::ResetPasswordRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.auth.UserService/ResetPassword",
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

    pub fn sign_up_opt(
        &self,
        req: &super::auth::SignUpRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_SIGN_UP, req, opt)
    }

    pub fn sign_up(
        &self,
        req: &super::auth::SignUpRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.sign_up_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_up_async_opt(
        &self,
        req: &super::auth::SignUpRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_SIGN_UP, req, opt)
    }

    pub fn sign_up_async(
        &self,
        req: &super::auth::SignUpRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.sign_up_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn confirm_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_CONFIRM, req, opt)
    }

    pub fn confirm(
        &self,
        req: &super::auth::EmailRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.confirm_opt(req, ::grpcio::CallOption::default())
    }

    pub fn confirm_async_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_CONFIRM, req, opt)
    }

    pub fn confirm_async(
        &self,
        req: &super::auth::EmailRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.confirm_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_UNLOCK, req, opt)
    }

    pub fn unlock(&self, req: &super::auth::EmailRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.unlock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_async_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_UNLOCK, req, opt)
    }

    pub fn unlock_async(
        &self,
        req: &super::auth::EmailRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.unlock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forgot_password_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_FORGOT_PASSWORD, req, opt)
    }

    pub fn forgot_password(
        &self,
        req: &super::auth::EmailRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.forgot_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forgot_password_async_opt(
        &self,
        req: &super::auth::EmailRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_FORGOT_PASSWORD, req, opt)
    }

    pub fn forgot_password_async(
        &self,
        req: &super::auth::EmailRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.forgot_password_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_password_opt(
        &self,
        req: &super::auth::ResetPasswordRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_USER_SERVICE_RESET_PASSWORD, req, opt)
    }

    pub fn reset_password(
        &self,
        req: &super::auth::ResetPasswordRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.reset_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_password_async_opt(
        &self,
        req: &super::auth::ResetPasswordRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_USER_SERVICE_RESET_PASSWORD, req, opt)
    }

    pub fn reset_password_async(
        &self,
        req: &super::auth::ResetPasswordRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.reset_password_async_opt(req, ::grpcio::CallOption::default())
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
    fn sign_up(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::SignUpRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn confirm(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::EmailRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn unlock(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::EmailRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn forgot_password(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::EmailRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn reset_password(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::auth::ResetPasswordRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
}

pub fn create_user_service<S: UserService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_IMPORT, move |ctx, req, resp| {
        instance.import(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_SIGN_IN, move |ctx, req, resp| {
        instance.sign_in(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_SIGN_UP, move |ctx, req, resp| {
        instance.sign_up(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_CONFIRM, move |ctx, req, resp| {
        instance.confirm(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_SERVICE_UNLOCK, move |ctx, req, resp| {
        instance.unlock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_USER_SERVICE_FORGOT_PASSWORD,
        move |ctx, req, resp| instance.forgot_password(ctx, req, resp),
    );
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_USER_SERVICE_RESET_PASSWORD,
        move |ctx, req, resp| instance.reset_password(ctx, req, resp),
    );
    builder.build()
}

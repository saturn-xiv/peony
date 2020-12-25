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

const METHOD_FORUM_SERVICE_CREATE_POST: ::grpcio::Method<super::forum::CreatePostRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.forum.models.ForumService/CreatePost",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ForumServiceClient {
    client: ::grpcio::Client,
}

impl ForumServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ForumServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_post_opt(&self, req: &super::forum::CreatePostRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_FORUM_SERVICE_CREATE_POST, req, opt)
    }

    pub fn create_post(&self, req: &super::forum::CreatePostRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.create_post_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_post_async_opt(&self, req: &super::forum::CreatePostRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_FORUM_SERVICE_CREATE_POST, req, opt)
    }

    pub fn create_post_async(&self, req: &super::forum::CreatePostRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.create_post_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ForumService {
    fn create_post(&mut self, ctx: ::grpcio::RpcContext, req: super::forum::CreatePostRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
}

pub fn create_forum_service<S: ForumService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_FORUM_SERVICE_CREATE_POST, move |ctx, req, resp| {
        instance.create_post(ctx, req, resp)
    });
    builder.build()
}

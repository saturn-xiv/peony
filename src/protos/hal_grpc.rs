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

const METHOD_TTY_SERVICE_WRITE: ::grpcio::Method<super::hal::TtyRequest, super::empty::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/peony.hal.TtyService/Write",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pb_ser,
            de: ::grpcio::pb_de,
        },
    };

const METHOD_TTY_SERVICE_READ: ::grpcio::Method<super::empty::Empty, super::hal::TtyResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/peony.hal.TtyService/Read",
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
pub struct TtyServiceClient {
    client: ::grpcio::Client,
}

impl TtyServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TtyServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(
        &self,
        req: &super::hal::TtyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_TTY_SERVICE_WRITE, req, opt)
    }

    pub fn write(&self, req: &super::hal::TtyRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.write_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_async_opt(
        &self,
        req: &super::hal::TtyRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_TTY_SERVICE_WRITE, req, opt)
    }

    pub fn write_async(
        &self,
        req: &super::hal::TtyRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.write_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::hal::TtyResponse> {
        self.client.unary_call(&METHOD_TTY_SERVICE_READ, req, opt)
    }

    pub fn read(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::hal::TtyResponse> {
        self.read_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_async_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::TtyResponse>> {
        self.client
            .unary_call_async(&METHOD_TTY_SERVICE_READ, req, opt)
    }

    pub fn read_async(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::TtyResponse>> {
        self.read_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait TtyService {
    fn write(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::hal::TtyRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn read(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::empty::Empty,
        sink: ::grpcio::UnarySink<super::hal::TtyResponse>,
    );
}

pub fn create_tty_service<S: TtyService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TTY_SERVICE_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TTY_SERVICE_READ, move |ctx, req, resp| {
        instance.read(ctx, req, resp)
    });
    builder.build()
}

const METHOD_AUDIO_SERVICE_PLAY: ::grpcio::Method<
    super::hal::AudioPlayRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.hal.AudioService/Play",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_AUDIO_SERVICE_STOP: ::grpcio::Method<super::empty::Empty, super::empty::Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/peony.hal.AudioService/Stop",
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
pub struct AudioServiceClient {
    client: ::grpcio::Client,
}

impl AudioServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AudioServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn play_opt(
        &self,
        req: &super::hal::AudioPlayRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_AUDIO_SERVICE_PLAY, req, opt)
    }

    pub fn play(
        &self,
        req: &super::hal::AudioPlayRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.play_opt(req, ::grpcio::CallOption::default())
    }

    pub fn play_async_opt(
        &self,
        req: &super::hal::AudioPlayRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_AUDIO_SERVICE_PLAY, req, opt)
    }

    pub fn play_async(
        &self,
        req: &super::hal::AudioPlayRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.play_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_AUDIO_SERVICE_STOP, req, opt)
    }

    pub fn stop(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.stop_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_async_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_AUDIO_SERVICE_STOP, req, opt)
    }

    pub fn stop_async(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.stop_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait AudioService {
    fn play(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::hal::AudioPlayRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn stop(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::empty::Empty,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
}

pub fn create_audio_service<S: AudioService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUDIO_SERVICE_PLAY, move |ctx, req, resp| {
        instance.play(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_AUDIO_SERVICE_STOP, move |ctx, req, resp| {
        instance.stop(ctx, req, resp)
    });
    builder.build()
}

const METHOD_GPIO_SERVICE_LED_SET: ::grpcio::Method<
    super::hal::LedSetRequest,
    super::empty::Empty,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.hal.GpioService/LedSet",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_GPIO_SERVICE_LED_GET: ::grpcio::Method<
    super::hal::LedGetRequest,
    super::hal::LedGetResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.hal.GpioService/LedGet",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_GPIO_SERVICE_BUTTON_REPORT: ::grpcio::Method<
    super::empty::Empty,
    super::hal::ButtonResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/peony.hal.GpioService/ButtonReport",
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
pub struct GpioServiceClient {
    client: ::grpcio::Client,
}

impl GpioServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GpioServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn led_set_opt(
        &self,
        req: &super::hal::LedSetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.client
            .unary_call(&METHOD_GPIO_SERVICE_LED_SET, req, opt)
    }

    pub fn led_set(
        &self,
        req: &super::hal::LedSetRequest,
    ) -> ::grpcio::Result<super::empty::Empty> {
        self.led_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn led_set_async_opt(
        &self,
        req: &super::hal::LedSetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client
            .unary_call_async(&METHOD_GPIO_SERVICE_LED_SET, req, opt)
    }

    pub fn led_set_async(
        &self,
        req: &super::hal::LedSetRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.led_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn led_get_opt(
        &self,
        req: &super::hal::LedGetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::hal::LedGetResponse> {
        self.client
            .unary_call(&METHOD_GPIO_SERVICE_LED_GET, req, opt)
    }

    pub fn led_get(
        &self,
        req: &super::hal::LedGetRequest,
    ) -> ::grpcio::Result<super::hal::LedGetResponse> {
        self.led_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn led_get_async_opt(
        &self,
        req: &super::hal::LedGetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::LedGetResponse>> {
        self.client
            .unary_call_async(&METHOD_GPIO_SERVICE_LED_GET, req, opt)
    }

    pub fn led_get_async(
        &self,
        req: &super::hal::LedGetRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::LedGetResponse>> {
        self.led_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn button_report_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::hal::ButtonResponse> {
        self.client
            .unary_call(&METHOD_GPIO_SERVICE_BUTTON_REPORT, req, opt)
    }

    pub fn button_report(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<super::hal::ButtonResponse> {
        self.button_report_opt(req, ::grpcio::CallOption::default())
    }

    pub fn button_report_async_opt(
        &self,
        req: &super::empty::Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::ButtonResponse>> {
        self.client
            .unary_call_async(&METHOD_GPIO_SERVICE_BUTTON_REPORT, req, opt)
    }

    pub fn button_report_async(
        &self,
        req: &super::empty::Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hal::ButtonResponse>> {
        self.button_report_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait GpioService {
    fn led_set(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::hal::LedSetRequest,
        sink: ::grpcio::UnarySink<super::empty::Empty>,
    );
    fn led_get(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::hal::LedGetRequest,
        sink: ::grpcio::UnarySink<super::hal::LedGetResponse>,
    );
    fn button_report(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::empty::Empty,
        sink: ::grpcio::UnarySink<super::hal::ButtonResponse>,
    );
}

pub fn create_gpio_service<S: GpioService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GPIO_SERVICE_LED_SET, move |ctx, req, resp| {
        instance.led_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GPIO_SERVICE_LED_GET, move |ctx, req, resp| {
        instance.led_get(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder
        .add_unary_handler(&METHOD_GPIO_SERVICE_BUTTON_REPORT, move |ctx, req, resp| {
            instance.button_report(ctx, req, resp)
        });
    builder.build()
}

// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: nut.proto
#ifndef GRPC_nut_2eproto__INCLUDED
#define GRPC_nut_2eproto__INCLUDED

#include "nut.pb.h"

#include <functional>
#include <grpc/impl/codegen/port_platform.h>
#include <grpcpp/impl/codegen/async_generic_service.h>
#include <grpcpp/impl/codegen/async_stream.h>
#include <grpcpp/impl/codegen/async_unary_call.h>
#include <grpcpp/impl/codegen/client_callback.h>
#include <grpcpp/impl/codegen/client_context.h>
#include <grpcpp/impl/codegen/completion_queue.h>
#include <grpcpp/impl/codegen/message_allocator.h>
#include <grpcpp/impl/codegen/method_handler.h>
#include <grpcpp/impl/codegen/proto_utils.h>
#include <grpcpp/impl/codegen/rpc_method.h>
#include <grpcpp/impl/codegen/server_callback.h>
#include <grpcpp/impl/codegen/server_callback_handlers.h>
#include <grpcpp/impl/codegen/server_context.h>
#include <grpcpp/impl/codegen/service_type.h>
#include <grpcpp/impl/codegen/status.h>
#include <grpcpp/impl/codegen/stub_options.h>
#include <grpcpp/impl/codegen/sync_stream.h>

// import "google/protobuf/empty.proto";
// import "google/protobuf/timestamp.proto";
// import "google/protobuf/duration.proto";
//
class NutService final {
 public:
  static constexpr char const* service_full_name() {
    return "NutService";
  }
  class StubInterface {
   public:
    virtual ~StubInterface() {}
    virtual ::grpc::Status SignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::SignInResponse* response) = 0;
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>> AsyncSignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>>(AsyncSignInRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>> PrepareAsyncSignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>>(PrepareAsyncSignInRaw(context, request, cq));
    }
    virtual ::grpc::Status SignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::Ok* response) = 0;
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::Ok>> AsyncSignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::Ok>>(AsyncSignUpRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::Ok>> PrepareAsyncSignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::Ok>>(PrepareAsyncSignUpRaw(context, request, cq));
    }
    class experimental_async_interface {
     public:
      virtual ~experimental_async_interface() {}
      virtual void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, std::function<void(::grpc::Status)>) = 0;
      #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      virtual void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, ::grpc::ClientUnaryReactor* reactor) = 0;
      #else
      virtual void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, ::grpc::experimental::ClientUnaryReactor* reactor) = 0;
      #endif
      virtual void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, std::function<void(::grpc::Status)>) = 0;
      #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      virtual void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, ::grpc::ClientUnaryReactor* reactor) = 0;
      #else
      virtual void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, ::grpc::experimental::ClientUnaryReactor* reactor) = 0;
      #endif
    };
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    typedef class experimental_async_interface async_interface;
    #endif
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    async_interface* async() { return experimental_async(); }
    #endif
    virtual class experimental_async_interface* experimental_async() { return nullptr; }
  private:
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>* AsyncSignInRaw(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::SignInResponse>* PrepareAsyncSignInRaw(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::Ok>* AsyncSignUpRaw(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::Ok>* PrepareAsyncSignUpRaw(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) = 0;
  };
  class Stub final : public StubInterface {
   public:
    Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel);
    ::grpc::Status SignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::SignInResponse* response) override;
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::SignInResponse>> AsyncSignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::SignInResponse>>(AsyncSignInRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::SignInResponse>> PrepareAsyncSignIn(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::SignInResponse>>(PrepareAsyncSignInRaw(context, request, cq));
    }
    ::grpc::Status SignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::Ok* response) override;
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::Ok>> AsyncSignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::Ok>>(AsyncSignUpRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::Ok>> PrepareAsyncSignUp(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::Ok>>(PrepareAsyncSignUpRaw(context, request, cq));
    }
    class experimental_async final :
      public StubInterface::experimental_async_interface {
     public:
      void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, std::function<void(::grpc::Status)>) override;
      #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, ::grpc::ClientUnaryReactor* reactor) override;
      #else
      void SignIn(::grpc::ClientContext* context, const ::SignInRequest* request, ::SignInResponse* response, ::grpc::experimental::ClientUnaryReactor* reactor) override;
      #endif
      void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, std::function<void(::grpc::Status)>) override;
      #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, ::grpc::ClientUnaryReactor* reactor) override;
      #else
      void SignUp(::grpc::ClientContext* context, const ::SignUpRequest* request, ::Ok* response, ::grpc::experimental::ClientUnaryReactor* reactor) override;
      #endif
     private:
      friend class Stub;
      explicit experimental_async(Stub* stub): stub_(stub) { }
      Stub* stub() { return stub_; }
      Stub* stub_;
    };
    class experimental_async_interface* experimental_async() override { return &async_stub_; }

   private:
    std::shared_ptr< ::grpc::ChannelInterface> channel_;
    class experimental_async async_stub_{this};
    ::grpc::ClientAsyncResponseReader< ::SignInResponse>* AsyncSignInRaw(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::SignInResponse>* PrepareAsyncSignInRaw(::grpc::ClientContext* context, const ::SignInRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::Ok>* AsyncSignUpRaw(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::Ok>* PrepareAsyncSignUpRaw(::grpc::ClientContext* context, const ::SignUpRequest& request, ::grpc::CompletionQueue* cq) override;
    const ::grpc::internal::RpcMethod rpcmethod_SignIn_;
    const ::grpc::internal::RpcMethod rpcmethod_SignUp_;
  };
  static std::unique_ptr<Stub> NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options = ::grpc::StubOptions());

  class Service : public ::grpc::Service {
   public:
    Service();
    virtual ~Service();
    virtual ::grpc::Status SignIn(::grpc::ServerContext* context, const ::SignInRequest* request, ::SignInResponse* response);
    virtual ::grpc::Status SignUp(::grpc::ServerContext* context, const ::SignUpRequest* request, ::Ok* response);
  };
  template <class BaseClass>
  class WithAsyncMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithAsyncMethod_SignIn() {
      ::grpc::Service::MarkMethodAsync(0);
    }
    ~WithAsyncMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignIn(::grpc::ServerContext* context, ::SignInRequest* request, ::grpc::ServerAsyncResponseWriter< ::SignInResponse>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithAsyncMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithAsyncMethod_SignUp() {
      ::grpc::Service::MarkMethodAsync(1);
    }
    ~WithAsyncMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignUp(::grpc::ServerContext* context, ::SignUpRequest* request, ::grpc::ServerAsyncResponseWriter< ::Ok>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(1, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  typedef WithAsyncMethod_SignIn<WithAsyncMethod_SignUp<Service > > AsyncService;
  template <class BaseClass>
  class ExperimentalWithCallbackMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    ExperimentalWithCallbackMethod_SignIn() {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::Service::
    #else
      ::grpc::Service::experimental().
    #endif
        MarkMethodCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::SignInRequest, ::SignInResponse>(
            [this](
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
                   ::grpc::CallbackServerContext*
    #else
                   ::grpc::experimental::CallbackServerContext*
    #endif
                     context, const ::SignInRequest* request, ::SignInResponse* response) { return this->SignIn(context, request, response); }));}
    void SetMessageAllocatorFor_SignIn(
        ::grpc::experimental::MessageAllocator< ::SignInRequest, ::SignInResponse>* allocator) {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::GetHandler(0);
    #else
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::experimental().GetHandler(0);
    #endif
      static_cast<::grpc::internal::CallbackUnaryHandler< ::SignInRequest, ::SignInResponse>*>(handler)
              ->SetMessageAllocator(allocator);
    }
    ~ExperimentalWithCallbackMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    virtual ::grpc::ServerUnaryReactor* SignIn(
      ::grpc::CallbackServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/)
    #else
    virtual ::grpc::experimental::ServerUnaryReactor* SignIn(
      ::grpc::experimental::CallbackServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/)
    #endif
      { return nullptr; }
  };
  template <class BaseClass>
  class ExperimentalWithCallbackMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    ExperimentalWithCallbackMethod_SignUp() {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::Service::
    #else
      ::grpc::Service::experimental().
    #endif
        MarkMethodCallback(1,
          new ::grpc::internal::CallbackUnaryHandler< ::SignUpRequest, ::Ok>(
            [this](
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
                   ::grpc::CallbackServerContext*
    #else
                   ::grpc::experimental::CallbackServerContext*
    #endif
                     context, const ::SignUpRequest* request, ::Ok* response) { return this->SignUp(context, request, response); }));}
    void SetMessageAllocatorFor_SignUp(
        ::grpc::experimental::MessageAllocator< ::SignUpRequest, ::Ok>* allocator) {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::GetHandler(1);
    #else
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::experimental().GetHandler(1);
    #endif
      static_cast<::grpc::internal::CallbackUnaryHandler< ::SignUpRequest, ::Ok>*>(handler)
              ->SetMessageAllocator(allocator);
    }
    ~ExperimentalWithCallbackMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    virtual ::grpc::ServerUnaryReactor* SignUp(
      ::grpc::CallbackServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/)
    #else
    virtual ::grpc::experimental::ServerUnaryReactor* SignUp(
      ::grpc::experimental::CallbackServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/)
    #endif
      { return nullptr; }
  };
  #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
  typedef ExperimentalWithCallbackMethod_SignIn<ExperimentalWithCallbackMethod_SignUp<Service > > CallbackService;
  #endif

  typedef ExperimentalWithCallbackMethod_SignIn<ExperimentalWithCallbackMethod_SignUp<Service > > ExperimentalCallbackService;
  template <class BaseClass>
  class WithGenericMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithGenericMethod_SignIn() {
      ::grpc::Service::MarkMethodGeneric(0);
    }
    ~WithGenericMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
  };
  template <class BaseClass>
  class WithGenericMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithGenericMethod_SignUp() {
      ::grpc::Service::MarkMethodGeneric(1);
    }
    ~WithGenericMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
  };
  template <class BaseClass>
  class WithRawMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawMethod_SignIn() {
      ::grpc::Service::MarkMethodRaw(0);
    }
    ~WithRawMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignIn(::grpc::ServerContext* context, ::grpc::ByteBuffer* request, ::grpc::ServerAsyncResponseWriter< ::grpc::ByteBuffer>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithRawMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawMethod_SignUp() {
      ::grpc::Service::MarkMethodRaw(1);
    }
    ~WithRawMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignUp(::grpc::ServerContext* context, ::grpc::ByteBuffer* request, ::grpc::ServerAsyncResponseWriter< ::grpc::ByteBuffer>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(1, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class ExperimentalWithRawCallbackMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    ExperimentalWithRawCallbackMethod_SignIn() {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::Service::
    #else
      ::grpc::Service::experimental().
    #endif
        MarkMethodRawCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::grpc::ByteBuffer, ::grpc::ByteBuffer>(
            [this](
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
                   ::grpc::CallbackServerContext*
    #else
                   ::grpc::experimental::CallbackServerContext*
    #endif
                     context, const ::grpc::ByteBuffer* request, ::grpc::ByteBuffer* response) { return this->SignIn(context, request, response); }));
    }
    ~ExperimentalWithRawCallbackMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    virtual ::grpc::ServerUnaryReactor* SignIn(
      ::grpc::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)
    #else
    virtual ::grpc::experimental::ServerUnaryReactor* SignIn(
      ::grpc::experimental::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)
    #endif
      { return nullptr; }
  };
  template <class BaseClass>
  class ExperimentalWithRawCallbackMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    ExperimentalWithRawCallbackMethod_SignUp() {
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
      ::grpc::Service::
    #else
      ::grpc::Service::experimental().
    #endif
        MarkMethodRawCallback(1,
          new ::grpc::internal::CallbackUnaryHandler< ::grpc::ByteBuffer, ::grpc::ByteBuffer>(
            [this](
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
                   ::grpc::CallbackServerContext*
    #else
                   ::grpc::experimental::CallbackServerContext*
    #endif
                     context, const ::grpc::ByteBuffer* request, ::grpc::ByteBuffer* response) { return this->SignUp(context, request, response); }));
    }
    ~ExperimentalWithRawCallbackMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    #ifdef GRPC_CALLBACK_API_NONEXPERIMENTAL
    virtual ::grpc::ServerUnaryReactor* SignUp(
      ::grpc::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)
    #else
    virtual ::grpc::experimental::ServerUnaryReactor* SignUp(
      ::grpc::experimental::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)
    #endif
      { return nullptr; }
  };
  template <class BaseClass>
  class WithStreamedUnaryMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithStreamedUnaryMethod_SignIn() {
      ::grpc::Service::MarkMethodStreamed(0,
        new ::grpc::internal::StreamedUnaryHandler<
          ::SignInRequest, ::SignInResponse>(
            [this](::grpc::ServerContext* context,
                   ::grpc::ServerUnaryStreamer<
                     ::SignInRequest, ::SignInResponse>* streamer) {
                       return this->StreamedSignIn(context,
                         streamer);
                  }));
    }
    ~WithStreamedUnaryMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable regular version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::SignInRequest* /*request*/, ::SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    // replace default version of method with streamed unary
    virtual ::grpc::Status StreamedSignIn(::grpc::ServerContext* context, ::grpc::ServerUnaryStreamer< ::SignInRequest,::SignInResponse>* server_unary_streamer) = 0;
  };
  template <class BaseClass>
  class WithStreamedUnaryMethod_SignUp : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithStreamedUnaryMethod_SignUp() {
      ::grpc::Service::MarkMethodStreamed(1,
        new ::grpc::internal::StreamedUnaryHandler<
          ::SignUpRequest, ::Ok>(
            [this](::grpc::ServerContext* context,
                   ::grpc::ServerUnaryStreamer<
                     ::SignUpRequest, ::Ok>* streamer) {
                       return this->StreamedSignUp(context,
                         streamer);
                  }));
    }
    ~WithStreamedUnaryMethod_SignUp() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable regular version of this method
    ::grpc::Status SignUp(::grpc::ServerContext* /*context*/, const ::SignUpRequest* /*request*/, ::Ok* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    // replace default version of method with streamed unary
    virtual ::grpc::Status StreamedSignUp(::grpc::ServerContext* context, ::grpc::ServerUnaryStreamer< ::SignUpRequest,::Ok>* server_unary_streamer) = 0;
  };
  typedef WithStreamedUnaryMethod_SignIn<WithStreamedUnaryMethod_SignUp<Service > > StreamedUnaryService;
  typedef Service SplitStreamedService;
  typedef WithStreamedUnaryMethod_SignIn<WithStreamedUnaryMethod_SignUp<Service > > StreamedService;
};


#endif  // GRPC_nut_2eproto__INCLUDED
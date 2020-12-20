#include "queue.h"

peony::rabbitmq::Config::Config(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned int>().value_or(5672);
  this->user = root["user"].value<std::string>().value();
  this->password = root["password"].value<std::string>().value();
  this->virtual_host = root["virtual-host"].value<std::string>().value();
}

peony::rabbitmq::Config::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  root.insert("user", this->user);
  root.insert("password", this->password);
  root.insert("virtual-host", this->virtual_host);
  return root;
};

peony::rabbitmq::Connection::Connection(std::shared_ptr<Config> cfg,
                                        const std::string &queue)
    : channel_id(1),
      exchange(amqp_cstring_bytes("")),
      routing_key(amqp_cstring_bytes(queue.c_str())),
      queue(amqp_cstring_bytes(queue.c_str())) {
  connection = amqp_new_connection();
  {
    auto socket = amqp_tcp_socket_new(connection);
    if (!socket) {
      throw new std::invalid_argument("creating TCP socket");
    }
    auto status = amqp_socket_open(socket, cfg->host.c_str(), cfg->port);
    if (status) {
      throw new std::invalid_argument("opening TCP socket");
    }
  }

  die_on_amqp_error(amqp_login(connection, cfg->virtual_host.c_str(), 0,
                               1 << 17, 0, AMQP_SASL_METHOD_PLAIN,
                               cfg->user.c_str(), cfg->password.c_str()));
  amqp_channel_open(connection, channel_id);
  die_on_amqp_error(amqp_get_rpc_reply(connection));

  this->ensure_queue();
}

peony::rabbitmq::Connection::~Connection() {
  die_on_amqp_error(
      amqp_channel_close(connection, channel_id, AMQP_REPLY_SUCCESS));
  die_on_amqp_error(amqp_connection_close(connection, AMQP_REPLY_SUCCESS));
  die_on_error(amqp_destroy_connection(connection));
}

void peony::rabbitmq::Connection::ensure_queue() {
  amqp_queue_declare_ok_t *it = amqp_queue_declare(
      connection, channel_id, queue, 0, 1, 0, 0, amqp_empty_table);
  die_on_amqp_error(amqp_get_rpc_reply(connection));
  auto name = amqp_bytes_malloc_dup(it->queue);
  if (name.bytes == NULL) {
    throw std::runtime_error("out of memory while copying queue name");
  }
  if (exchange.len > 0) {
    amqp_queue_bind(connection, channel_id, queue, exchange, routing_key,
                    amqp_empty_table);
    die_on_amqp_error(amqp_get_rpc_reply(connection));
  }
}

void peony::rabbitmq::Connection::publish(const nlohmann::json &payload) {
  const auto id = peony::crypt::uuid();
  BOOST_LOG_TRIVIAL(info) << "publish message " << id;

  amqp_basic_properties_t props;
  {
    props._flags = AMQP_BASIC_CONTENT_TYPE_FLAG |
                   AMQP_BASIC_DELIVERY_MODE_FLAG | AMQP_BASIC_MESSAGE_ID_FLAG;
    props.content_type = amqp_cstring_bytes(PEONY_APPLICATION_JSON_UTF8);
    // persistent delivery mode
    props.delivery_mode = 2;
    props.message_id = amqp_cstring_bytes(id.c_str());
  }

  die_on_error(amqp_basic_publish(connection, channel_id, exchange, queue, 0, 0,
                                  &props,
                                  amqp_cstring_bytes(payload.dump().c_str())));
}

void peony::rabbitmq::Connection::consume(
    std::shared_ptr<peony::queue::Consumer> consumer) {
  amqp_basic_consume(connection, channel_id, queue, amqp_empty_bytes, 0, 1, 0,
                     amqp_empty_table);
  die_on_amqp_error(amqp_get_rpc_reply(connection));

  amqp_frame_t frame;
  for (;;) {
    amqp_rpc_reply_t reply;
    amqp_envelope_t envelope;

    amqp_maybe_release_buffers(connection);
    reply = amqp_consume_message(connection, &envelope, NULL, 0);

    if (AMQP_RESPONSE_NORMAL == reply.reply_type) {
      BOOST_LOG_TRIVIAL(info)
          << "delivery tag: " << (unsigned)envelope.delivery_tag
          << ", exchange: " << this->bytes2str(envelope.exchange)
          << ", routing key: " << this->bytes2str(envelope.routing_key);

      if (envelope.message.properties._flags & AMQP_BASIC_CONTENT_TYPE_FLAG) {
        BOOST_LOG_TRIVIAL(info)
            << "content-type: "
            << this->bytes2str(envelope.message.properties.content_type);
      }
      if (envelope.message.properties._flags & AMQP_BASIC_MESSAGE_ID_FLAG) {
        BOOST_LOG_TRIVIAL(info)
            << "message-id: "
            << this->bytes2str(envelope.message.properties.message_id);
      }
      const auto payload =
          nlohmann::json::parse(this->bytes2str(envelope.message.body));
      consumer->handle(payload);
      amqp_destroy_envelope(&envelope);
      BOOST_LOG_TRIVIAL(info) << "done.";
    } else {
      if (AMQP_RESPONSE_LIBRARY_EXCEPTION == reply.reply_type &&
          AMQP_STATUS_UNEXPECTED_STATE == reply.library_error) {
        if (AMQP_STATUS_OK != amqp_simple_wait_frame(connection, &frame)) {
          return;
        }

        if (AMQP_FRAME_METHOD == frame.frame_type) {
          switch (frame.payload.method.id) {
            case AMQP_BASIC_ACK_METHOD:
              BOOST_LOG_TRIVIAL(debug) << "basic ack";
              break;
            case AMQP_BASIC_RETURN_METHOD: {
              BOOST_LOG_TRIVIAL(warning) << "basic return";
              amqp_message_t message;
              reply = amqp_read_message(connection, frame.channel, &message, 0);
              if (AMQP_RESPONSE_NORMAL != reply.reply_type) {
                return;
              }
              amqp_destroy_message(&message);
            } break;
            case AMQP_CHANNEL_CLOSE_METHOD:
              BOOST_LOG_TRIVIAL(warning) << "channel close";
              return;
            case AMQP_CONNECTION_CLOSE_METHOD:
              BOOST_LOG_TRIVIAL(warning) << "connection close";
              return;
            default: {
              std::stringstream ss;
              ss << "an unexpected method was received "
                 << frame.payload.method.id;
              throw new std::invalid_argument(ss.str());
            }
              return;
          }
        }
      }
    }
  }
}

void peony::rabbitmq::Connection::die_on_error(int code) {
  if (AMQP_STATUS_OK != code) {
    throw std::invalid_argument(amqp_error_string2(code));
  }
}

void peony::rabbitmq::Connection::die_on_amqp_error(amqp_rpc_reply_t reply) {
  switch (reply.reply_type) {
    case AMQP_RESPONSE_NORMAL:
      return;
    case AMQP_RESPONSE_NONE:
      throw std::invalid_argument("missing RPC reply type!");
      break;
    case AMQP_RESPONSE_LIBRARY_EXCEPTION:
      throw std::invalid_argument(amqp_error_string2(reply.library_error));
      break;

    case AMQP_RESPONSE_SERVER_EXCEPTION:
      switch (reply.reply.id) {
        case AMQP_CONNECTION_CLOSE_METHOD: {
          amqp_connection_close_t *it =
              (amqp_connection_close_t *)reply.reply.decoded;

          std::stringstream ss;
          ss << "server connection error: " << it->reply_code
             << ", message: " << this->bytes2str(it->reply_text);
          throw std::invalid_argument(ss.str());
        } break;
        case AMQP_CHANNEL_CLOSE_METHOD: {
          amqp_channel_close_t *it =
              (amqp_channel_close_t *)reply.reply.decoded;
          std::stringstream ss;
          ss << "server channel error: " << it->reply_code
             << ", message: " << this->bytes2str(it->reply_text);
          throw std::invalid_argument(ss.str());
        } break;
        default: {
          std::stringstream ss;
          ss << "unknown server error, method id " << std::hex
             << reply.reply.id;
          throw std::invalid_argument(ss.str());
        } break;
      }
      break;
  }
}
std::string_view peony::rabbitmq::Connection::bytes2str(amqp_bytes_t buf) {
  std::string_view it((char *)buf.bytes);
  return it.substr(0, (int)buf.len);
}
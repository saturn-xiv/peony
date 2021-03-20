#include "peony.h"
#include "config.h"

#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>

peony::serial_port::Tty::~Tty() {
  std::lock_guard<std::mutex> lock(this->lock);

  port->cancel();
  port->close();
  port.reset();

  io_service.stop();
  io_service.reset();
}

std::vector<std::string> peony::serial_port::Tty::get_port_names() const {
  std::vector<std::string> items;
  // TODO
  return items;
}

void peony::serial_port::Tty::read() {
  boost::asio::async_read(
      *port, boost::asio::buffer(line),
      boost::bind(&peony::serial_port::Tty::on_receive, this, line, _1, _2));
}

void peony::serial_port::Tty::listen() { io_service.run(); }

std::optional<std::pair<size_t, size_t>> peony::serial_port::Tty::match(
    const std::string& payload, const std::string& begin,
    const std::string& end) {
  const auto b = payload.find(begin);
  if (b == std::string::npos) {
    return std::nullopt;
  }
  const auto e = payload.find(end);
  if (e == std::string::npos) {
    return std::nullopt;
  }
  return std::make_pair(b, e + end.length());
}

void peony::serial_port::Tty::on_receive(const char* buffer,
                                         const boost::system::error_code& ec,
                                         const size_t bytes_transferred) {
  std::lock_guard<std::mutex> lock(this->lock);

  if (ec) {
    BOOST_LOG_TRIVIAL(warning) << name << " " << ec.message();
  } else {
    payload += std::string(buffer, bytes_transferred);
    BOOST_LOG_TRIVIAL(debug) << name << " received " << bytes_transferred
                             << " bytes buffer: " << payload;
    const auto size = this->match(payload);
    if (size) {
      const auto it = size.value();
      const auto msg = payload.substr(it.first, it.second - it.first);
      BOOST_LOG_TRIVIAL(info) << "match " << name << ": " << msg;
      try {
        this->execute(msg);
      } catch (const std::exception& e) {
        BOOST_LOG_TRIVIAL(error) << "handle " << name << ": " << e.what();
      }
      BOOST_LOG_TRIVIAL(debug)
          << "split " << name << " :" << payload.substr(0, it.second);
      payload = payload.substr(it.second);
      if (payload.length() > 0) {
        BOOST_LOG_TRIVIAL(debug)
            << "remain " << name << " :" << payload.substr(0, it.second);
      }
    }

    if (payload.length() >= PEONY_SERIAL_PORT_READ_BUFFER_MAX_SIZE) {
      BOOST_LOG_TRIVIAL(warning) << "clear buffer " << name;
      payload.clear();
    }
  }

  read();
}

void peony::serial_port::Tty::write(const std::string& buf) {
  if (!port || !port->is_open()) {
    BOOST_LOG_TRIVIAL(warning) << name << " isn't open";
    return;
  }
  const auto size = buf.size();
  if (size == 0) {
    return;
  }
  BOOST_LOG_TRIVIAL(info) << "write " << size << " bytes to " << name << " "
                          << buf;
  std::lock_guard<std::mutex> lock(this->lock);
  boost::asio::write(*port, boost::asio::buffer(buf.c_str(), buf.size()));
}

void peony::serial_port::Tty::open(
    const std::string& name, const unsigned int baud_rate,
    const unsigned int character_size,
    const boost::asio::serial_port_base::stop_bits::type stop_bits,
    const boost::asio::serial_port_base::parity::type parity,
    const boost::asio::serial_port_base::flow_control::type flow_control) {
  BOOST_LOG_TRIVIAL(info) << "open serial port " << name
                          << " in exclusive mode";
  std::lock_guard<std::mutex> lock(this->lock);

  port = std::shared_ptr<boost::asio::serial_port>(
      new boost::asio::serial_port(io_service));
  port->open("/dev/" + name);

  port->set_option(boost::asio::serial_port_base::baud_rate(baud_rate));
  port->set_option(
      boost::asio::serial_port_base::character_size(character_size));
  port->set_option(boost::asio::serial_port_base::stop_bits(stop_bits));
  port->set_option(boost::asio::serial_port_base::parity(parity));
  port->set_option(boost::asio::serial_port_base::flow_control(flow_control));
}

void peony::Application::launch(int argc, char** argv) {
  std::string config;
  boost::program_options::options_description desc("Allowed options");
  desc.add_options()("help,h", "Prints help information")(
      "version,v", "Print version info and exit")("debug,d",
                                                  "Run on debug mode")(
      "config,c",
      boost::program_options::value<std::string>(&config)->default_value(
          "config.ini"),
      "Config file");

  boost::program_options::variables_map vm;
  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, desc), vm);
  boost::program_options::notify(vm);

  if (vm.count("help")) {
    std::cout << desc << std::endl;
    return;
  }
  if (vm.count("version")) {
    std::cout << PEONY_GIT_VERSION << std::endl;
    return;
  }

  boost::log::core::get()->set_filter(boost::log::trivial::severity >=
                                      (vm.count("debug")
                                           ? boost::log::trivial::debug
                                           : boost::log::trivial::info));

  BOOST_LOG_TRIVIAL(debug) << "run on debug level";
  BOOST_LOG_TRIVIAL(info) << "load config from " << config;
  boost::property_tree::ptree tree;
  boost::property_tree::read_ini(config, tree);
  this->launch(tree);
  BOOST_LOG_TRIVIAL(warning) << "exit...";
}

std::string peony::utils::mac(const std::string& name) {
  std::ifstream fd("/sys/class/net/" + name + "/address");
  std::string it((std::istreambuf_iterator<char>(fd)),
                 std::istreambuf_iterator<char>());
  boost::trim(it);
  return it;
}

#define PEONY_PING_WATCHDOG 5
void ping_watchdog(int sig) { alarm(PEONY_PING_WATCHDOG); }
void peony::utils::watchdog() {
  BOOST_LOG_TRIVIAL(info) << "start watch dog";
  signal(SIGALRM, ping_watchdog);
  alarm(PEONY_PING_WATCHDOG);
}

bool peony::utils::load_texture_from_file(const char* filename,
                                          GLuint* out_texture, int* out_width,
                                          int* out_height) {
  // Load from file
  int image_width = 0;
  int image_height = 0;
  unsigned char* image_data =
      stbi_load(filename, &image_width, &image_height, NULL, 4);
  if (image_data == NULL) return false;

  // Create a OpenGL texture identifier
  GLuint image_texture;
  glGenTextures(1, &image_texture);
  glBindTexture(GL_TEXTURE_2D, image_texture);

  // Setup filtering parameters for display
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                  GL_CLAMP_TO_EDGE);  // This is required on WebGL for non
                                      // power-of-two textures
  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);  // Same

  // Upload pixels into texture
#if defined(GL_UNPACK_ROW_LENGTH) && !defined(__EMSCRIPTEN__)
  glPixelStorei(GL_UNPACK_ROW_LENGTH, 0);
#endif
  glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, image_width, image_height, 0, GL_RGBA,
               GL_UNSIGNED_BYTE, image_data);
  stbi_image_free(image_data);

  *out_texture = image_texture;
  *out_width = image_width;
  *out_height = image_height;

  return true;
}
#ifndef PEONY_EMAIL_H_
#define PEONY_EMAIL_H_

#include <mailio/imap.hpp>
#include <mailio/message.hpp>
#include <mailio/mime.hpp>
#include <mailio/pop3.hpp>
#include <mailio/smtp.hpp>

#include "plugin.h"

namespace peony {
namespace dovecot {}
namespace postfix {}
namespace smtp {}
namespace imap {}
}  // namespace peony

#endif

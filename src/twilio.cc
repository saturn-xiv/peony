#include "twilio.h"

peony::twilio::Config::Config(const toml::table &root)
{
    std::optional<std::string> form = root["from"].value<std::string>();
    if (form)
    {
        this->from = form.value();
    }
    std::optional<std::string> account_sid =
        root["account-sid"].value<std::string>();
    if (account_sid)
    {
        this->account_sid = account_sid.value();
    }
    std::optional<std::string> auth_token =
        root["auth-token"].value<std::string>();
    if (auth_token)
    {
        this->auth_token = auth_token.value();
    }
}

peony::twilio::Config::operator toml::table() const
{
    toml::table root;
    root.insert("account-sid", this->account_sid);
    root.insert("auth-token", this->auth_token);
    root.insert("from", this->from);
    return root;
}
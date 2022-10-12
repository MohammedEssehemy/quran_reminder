# Quran Reminder

Send a random quran page to the through emails or whatsapp

ترسل صفحة عشوائية من القرآن عبر البريد الإلكتروني أو الواتساب

## config

```shell
TRANSPORTS # Method of notification - default is  email, whatsapp
LANGUAGE # en | ar - default is ar
QURAN_VERSION # madina | tajweed - default is madina
CRON_PATTERN # works only if you run the server crate - default is "10 10 6 * * * *"
# Required if whatsapp transport is enabled
WHATSAPP_ACCESS_TOKEN # whatsapp access token with permission whatsapp_business_messaging
WHATSAPP_SENDER_PHONE_ID # whatsapp sender phone id
RECIPIENT_PHONES # comma separated whatsapp recipient numbers

# Required if email transport is enabled
SENDGRID_API_KEY # api_key for send grid
SENDGRID_FROM # the email use as from in emails
RECIPIENT_EMAILS # comma separated recipients
```

## Getting started

- Create a `.env` file with the configuration above

- Run the app, use one of the following:

    1. Run once

        ```shell
        cargo run --bin main
        ```

    2. Long running process with cron job

        ```shell
        cargo run --bin server
        ```

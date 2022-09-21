# Quran Reminder

Send a random quran page to the specified emails
ترسل صفحة عشوائية من القرآن إلى البريد الإلكتروني المحدد

## config

```shell
SENDGRID_API_KEY= # api_key for send grid
SENDGRID_FROM= # the email use as from in emails
RECIPIENT_EMAILS= # recipients comma separated
EMAIL_LANGUAGE=ar # en | ar
QURAN_VERSION=madina # madina | tajweed
```

## Getting started

- create a `.env` file with the configuration above
- Run the application

```shell
cargo run
```

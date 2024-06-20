use email_encoding::headers::EmailWriter;

use super::{Header, HeaderName, HeaderValue};
use crate::{
    message::mailbox::{Mailbox, Mailboxes},
    BoxError,
};

/// Header which can contains multiple mailboxes
pub trait MailboxesHeader {
    fn join_mailboxes(&mut self, other: Self);
}

macro_rules! mailbox_header {
    ($(#[$doc:meta])*($type_name: ident, $header_name: expr)) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $type_name(Mailbox);

        impl Header for $type_name {
            fn name() -> HeaderName {
                HeaderName::new_from_ascii_str($header_name)
            }

            fn parse(s: &str) -> Result<Self, BoxError> {
                let mailbox: Mailbox = s.parse()?;
                Ok(Self(mailbox))
            }

            fn display(&self) -> HeaderValue {
                let mut encoded_value = String::new();
                let line_len = $header_name.len() + ": ".len();
                {
                    let mut w = EmailWriter::new(&mut encoded_value, line_len, 0, false, false);
                    self.0.encode(&mut w).expect("writing `Mailbox` returned an error");
                }

                HeaderValue::dangerous_new_pre_encoded(Self::name(), self.0.to_string(), encoded_value)
            }
        }

        impl std::convert::From<Mailbox> for $type_name {
            #[inline]
            fn from(mailbox: Mailbox) -> Self {
                Self(mailbox)
            }
        }

        impl std::convert::From<$type_name> for Mailbox {
            #[inline]
            fn from(this: $type_name) -> Mailbox {
                this.0
            }
        }
    };
}

macro_rules! mailboxes_header {
    ($(#[$doc:meta])*($type_name: ident, $header_name: expr)) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $type_name(pub(crate) Mailboxes);

        impl MailboxesHeader for $type_name {
            fn join_mailboxes(&mut self, other: Self) {
                self.0.extend(other.0);
            }
        }

        impl Header for $type_name {
            fn name() -> HeaderName {
                HeaderName::new_from_ascii_str($header_name)
            }

            fn parse(s: &str) -> Result<Self, BoxError> {
                let mailbox: Mailboxes = s.parse()?;
                Ok(Self(mailbox))
            }

            fn display(&self) -> HeaderValue {
                let mut encoded_value = String::new();
                let line_len = $header_name.len() + ": ".len();
                {
                    let mut w = EmailWriter::new(&mut encoded_value, line_len, 0, false, false);
                    self.0.encode(&mut w).expect("writing `Mailboxes` returned an error");
                }

                HeaderValue::dangerous_new_pre_encoded(Self::name(), self.0.to_string(), encoded_value)
            }
        }

        impl std::convert::From<Mailboxes> for $type_name {
            #[inline]
            fn from(mailboxes: Mailboxes) -> Self {
                Self(mailboxes)
            }
        }

        impl std::convert::From<$type_name> for Mailboxes {
            #[inline]
            fn from(this: $type_name) -> Mailboxes {
                this.0
            }
        }
    };
}

mailbox_header! {
    /**

    `Sender` header

    This header contains [`Mailbox`][self::Mailbox] associated with sender.

    ```no_test
    header::Sender("Mr. Sender <sender@example.com>".parse().unwrap())
    ```
     */
    (Sender, "Sender")
}

mailboxes_header! {
    /**

    `From` header

    This header contains [`Mailboxes`][self::Mailboxes].

     */
    (From, "From")
}

mailboxes_header! {
    /**

    `Reply-To` header

    This header contains [`Mailboxes`][self::Mailboxes].

     */
    (ReplyTo, "Reply-To")
}

mailboxes_header! {
    /**

    `To` header

    This header contains [`Mailboxes`][self::Mailboxes].

     */
    (To, "To")
}

mailboxes_header! {
    /**

    `Cc` header

    This header contains [`Mailboxes`][self::Mailboxes].

     */
    (Cc, "Cc")
}

mailboxes_header! {
    /**

    `Bcc` header

    This header contains [`Mailboxes`][self::Mailboxes].

     */
    (Bcc, "Bcc")
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::{From, Mailbox, Mailboxes};
    use crate::message::header::{HeaderName, HeaderValue, Headers};

    #[test]
    fn format_single_without_name() {
        let from = Mailboxes::new().with("kayo@example.com".parse().unwrap());

        let mut headers = Headers::new();
        headers.set(From(from));

        assert_eq!(headers.to_string(), "From: kayo@example.com\r\n");
    }

    #[test]
    fn format_single_with_name() {
        let from = Mailboxes::new().with("Kayo <kayo@example.com>".parse().unwrap());

        let mut headers = Headers::new();
        headers.set(From(from));

        assert_eq!(headers.to_string(), "From: Kayo <kayo@example.com>\r\n");
    }

    #[test]
    fn format_multi_without_name() {
        let from = Mailboxes::new()
            .with("kayo@example.com".parse().unwrap())
            .with("pony@domain.tld".parse().unwrap());

        let mut headers = Headers::new();
        headers.set(From(from));

        assert_eq!(
            headers.to_string(),
            "From: kayo@example.com, pony@domain.tld\r\n"
        );
    }

    #[test]
    fn format_multi_with_name() {
        let from = vec![
            "Kayo <kayo@example.com>".parse().unwrap(),
            "Pony P. <pony@domain.tld>".parse().unwrap(),
        ];

        let mut headers = Headers::new();
        headers.set(From(from.into()));

        assert_eq!(
            headers.to_string(),
            "From: Kayo <kayo@example.com>, \"Pony P.\" <pony@domain.tld>\r\n"
        );
    }

    #[test]
    fn format_single_with_utf8_name() {
        let from = vec!["Кайо <kayo@example.com>".parse().unwrap()];

        let mut headers = Headers::new();
        headers.set(From(from.into()));

        assert_eq!(
            headers.to_string(),
            "From: =?utf-8?b?0JrQsNC50L4=?= <kayo@example.com>\r\n"
        );
    }

    #[test]
    fn parse_single_without_name() {
        let from = vec!["kayo@example.com".parse().unwrap()].into();

        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "kayo@example.com".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), Some(From(from)));
    }

    #[test]
    fn parse_single_with_name() {
        let from = vec!["K. <kayo@example.com>".parse().unwrap()].into();

        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "K. <kayo@example.com>".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), Some(From(from)));
    }

    #[test]
    fn parse_multi_without_name() {
        let from: Vec<Mailbox> = vec![
            "kayo@example.com".parse().unwrap(),
            "pony@domain.tld".parse().unwrap(),
        ];

        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "kayo@example.com, pony@domain.tld".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), Some(From(from.into())));
    }

    #[test]
    fn parse_multi_with_name() {
        let from: Vec<Mailbox> = vec![
            "K. <kayo@example.com>".parse().unwrap(),
            "Pony P. <pony@domain.tld>".parse().unwrap(),
        ];

        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "K. <kayo@example.com>, Pony P. <pony@domain.tld>".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), Some(From(from.into())));
    }

    #[test]
    fn parse_multi_with_name_containing_comma() {
        let from: Vec<Mailbox> = vec![
            "Test, test <1@example.com>".parse().unwrap(),
            "Test2, test2 <2@example.com>".parse().unwrap(),
        ];

        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "Test, test <1@example.com>, Test2, test2 <2@example.com>".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), Some(From(from.into())));
    }

    #[test]
    fn parse_multi_with_name_containing_comma_last_broken() {
        let mut headers = Headers::new();
        headers.insert_raw(HeaderValue::new(
            HeaderName::new_from_ascii_str("From"),
            "Test, test <1@example.com>, Test2, test2".to_owned(),
        ));

        assert_eq!(headers.get::<From>(), None);
    }
}

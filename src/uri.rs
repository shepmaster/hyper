use std::borrow::Cow;
use std::fmt::{Display, self};
use std::str::FromStr;
use url::{self, Url};
use url::ParseError as UrlError;

use Error;

/// The Request-URI of a Request's StartLine.
///
/// From Section 5.3, Request Target:
/// > Once an inbound connection is obtained, the client sends an HTTP
/// > request message (Section 3) with a request-target derived from the
/// > target URI.  There are four distinct formats for the request-target,
/// > depending on both the method being requested and whether the request
/// > is to a proxy.
/// >
/// > ```notrust
/// > request-target = origin-form
/// >                / absolute-form
/// >                / authority-form
/// >                / asterisk-form
/// > ```
///
/// # Uri explanations
///
/// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
/// |-|   |-------------------------------||--------| |-------------------| |-----|
///  |                  |                       |               |              |
/// scheme          authority                 path            query         fragment
#[derive(Clone)]
pub struct Uri {
    source: Cow<'static, str>,
    scheme_end: Option<usize>,
    authority_end: Option<usize>,
    query: Option<usize>,
    fragment: Option<usize>,
}

impl Uri {
    /// Parse a string into a `Uri`.
    pub fn new(s: &str) -> Result<Uri, Error> {
        let bytes = s.as_bytes();
        if bytes.len() == 0 {
            Err(Error::Uri(UrlError::RelativeUrlWithoutBase))
        } else if bytes == b"*" {
            Ok(Uri {
                source: "*".into(),
                scheme_end: None,
                authority_end: None,
                query: None,
                fragment: None,
            })
        } else if bytes == b"/" {
            Ok(Uri::default())
        } else if bytes.starts_with(b"/") {
            let mut temp = "http://example.com".to_owned();
            temp.push_str(s);
            let url = try!(Url::parse(&temp));
            let query_len = url.query().unwrap_or("").len();
            let fragment_len = url.fragment().unwrap_or("").len();
            Ok(Uri {
                source: s.to_owned().into(),
                scheme_end: None,
                authority_end: None,
                query: if query_len > 0 { Some(query_len) } else { None },
                fragment: if fragment_len > 0 { Some(fragment_len) } else { None },
            })
        } else if s.contains("://") {
            let url = try!(Url::parse(s));
            let query_len = url.query().unwrap_or("").len();
            let v: Vec<&str> = s.split("://").collect();
            let authority_end = v.last().unwrap()
                                        .split(url.path())
                                        .next()
                                        .unwrap_or(s)
                                        .len() + if v.len() == 2 { v[0].len() + 3 } else { 0 };
            let fragment_len = url.fragment().unwrap_or("").len();
            match url.origin() {
                url::Origin::Opaque(_) => Err(Error::Method),
                url::Origin::Tuple(scheme, _, _) => {
                    Ok(Uri {
                        source: url.to_string().into(),
                        scheme_end: Some(scheme.len()),
                        authority_end: if authority_end > 0 { Some(authority_end) } else { None },
                        query: if query_len > 0 { Some(query_len) } else { None },
                        fragment: if fragment_len > 0 { Some(fragment_len) } else { None },
                    })
                }
            }
        } else {
            Ok(Uri {
                source: s.to_owned().into(),
                scheme_end: None,
                authority_end: Some(s.len()),
                query: None,
                fragment: None,
            })
        }
    }

    /// Get the path of this `Uri`.
    pub fn path(&self) -> &str {
        let index = self.authority_end.unwrap_or(self.scheme_end.unwrap_or(0));
        let query_len = self.query.unwrap_or(0);
        let fragment_len = self.fragment.unwrap_or(0);
        let end = self.source.len() - if query_len > 0 { query_len + 1 } else { 0 } -
            if fragment_len > 0 { fragment_len + 1 } else { 0 };
        if index >= end {
            ""
        } else {
            &self.source[index..end]
        }
    }

    /// Get the scheme of this `Uri`.
    pub fn scheme(&self) -> Option<&str> {
        if let Some(end) = self.scheme_end {
            Some(&self.source[..end])
        } else {
            None
        }
    }

    /// Get the authority of this `Uri`.
    pub fn authority(&self) -> Option<&str> {
        if let Some(end) = self.authority_end {
            let index = self.scheme_end.map(|i| i + 3).unwrap_or(0);
            Some(&self.source[index..end])
        } else {
            None
        }
    }

    /// Get the host of this `Uri`.
    pub fn host(&self) -> Option<&str> {
        if let Some(auth) = self.authority() {
            auth.split(":").next()
        } else {
            None
        }
    }

    /// Get the port of this `Uri.
    pub fn port(&self) -> Option<u16> {
        if let Some(auth) = self.authority() {
            let v: Vec<&str> = auth.split(":").collect();
            if v.len() == 2 {
                u16::from_str(v[1]).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the query string of this `Uri`, starting after the `?`.
    pub fn query(&self) -> Option<&str> {
        let fragment_len = self.fragment.unwrap_or(0);
        let fragment_len = if fragment_len > 0 { fragment_len + 1 } else { 0 };
        if let Some(len) = self.query {
            Some(&self.source[self.source.len() - len - fragment_len..self.source.len() - fragment_len])
        } else {
            None
        }
    }

    #[cfg(test)]
    fn fragment(&self) -> Option<&str> {
        if let Some(len) = self.fragment {
            Some(&self.source[self.source.len() - len..])
        } else {
            None
        }
    }
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Uri, Error> {
        Uri::new(s)
    }
}

impl From<Url> for Uri {
    fn from(url: Url) -> Uri {
        Uri::new(url.as_str()).expect("Uri::From<Url> failed")
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Uri) -> bool {
        self.source == other.source
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        &self.source
    }
}

impl Default for Uri {
    fn default() -> Uri {
        Uri {
            source: "/".into(),
            scheme_end: None,
            authority_end: None,
            query: None,
            fragment: None,
        }
    }
}

impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.source.as_ref(), f)
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.source)
    }
}

macro_rules! test_parse {
    (
        $test_name:ident,
        $str:expr,
        $($method:ident = $value:expr,)*
    ) => (
        #[test]
        fn $test_name() {
            let uri = Uri::new($str).unwrap();
            $(
            assert_eq!(uri.$method(), $value);
            )+
        }
    );
}

test_parse! {
    test_uri_parse_origin_form,
    "/some/path/here?and=then&hello#and-bye",

    scheme = None,
    authority = None,
    path = "/some/path/here",
    query = Some("and=then&hello"),
    fragment = Some("and-bye"),
}

test_parse! {
    test_uri_parse_absolute_form,
    "http://127.0.0.1:61761/chunks",

    scheme = Some("http"),
    authority = Some("127.0.0.1:61761"),
    path = "/chunks",
    query = None,
    fragment = None,
}

test_parse! {
    test_uri_parse_absolute_form_without_path,
    "https://127.0.0.1:61761",

    scheme = Some("https"),
    authority = Some("127.0.0.1:61761"),
    path = "/",
    query = None,
    fragment = None,
}

test_parse! {
    test_uri_parse_asterisk_form,
    "*",

    scheme = None,
    authority = None,
    path = "*",
    query = None,
    fragment = None,
}

test_parse! {
    test_uri_parse_authority_form,
    "localhost:3000",

    scheme = None,
    authority = Some("localhost:3000"),
    path = "",
    query = None,
    fragment = None,
}

#[test]
fn test_uri_parse_error() {
    fn err(s: &str) {
        Uri::new(s).unwrap_err();
    }

    err("http://");
    //TODO: these should error
    //err("htt:p//host");
    //err("hyper.rs/");
    //err("hyper.rs?key=val");
}

#[test]
fn test_uri_from_url() {
    let uri = Uri::from(Url::parse("http://test.com/nazghul?test=3").unwrap());
    assert_eq!(uri.path(), "/nazghul");
    assert_eq!(uri.authority(), Some("test.com"));
    assert_eq!(uri.scheme(), Some("http"));
    assert_eq!(uri.query(), Some("test=3"));
    assert_eq!(uri.as_ref(), "http://test.com/nazghul?test=3");
}

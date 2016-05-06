initSidebarItems({"enum":[["AccessControlAllowOrigin","The `Access-Control-Allow-Origin` response header, part of CORS"],["ByteRangeSpec","Each Range::Bytes header can contain one or more ByteRangeSpecs. Each ByteRangeSpec defines a range of bytes to fetch"],["CacheDirective","CacheControl contains a list of these directives."],["Charset","A Mime charset."],["ConnectionOption","Values that can be in the `Connection` header."],["ContentRangeSpec","Content-Range, described in RFC7233"],["DispositionParam","A parameter to the disposition type"],["DispositionType","The implied disposition of the content of the HTTP body"],["Encoding","A value to represent an encoding used in `Transfer-Encoding` or `Accept-Encoding` header."],["Expect","The `Expect` header."],["IfMatch","`If-Match` header, defined in RFC7232"],["IfNoneMatch","`If-None-Match` header, defined in RFC7232"],["IfRange","`If-Range` header, defined in RFC7233"],["Pragma","The `Pragma` header defined by HTTP/1.0."],["Preference","Prefer contains a list of these preferences."],["ProtocolName","A protocol name used to identify a spefic protocol. Names are case-sensitive except for the `WebSocket` value."],["Range","`Range` header, defined in RFC7233"],["RangeUnit","Range Units, described in RFC7233"],["Vary","`Vary` header, defined in RFC7231"]],"fn":[["q","Convenience function to create a `Quality` from a float."],["qitem","Convinience function to wrap a value in a `QualityItem` Sets `q` to the default 1.0"]],"mod":[["parsing","Utility functions for Header implementations."]],"struct":[["Accept","`Accept` header, defined in RFC7231"],["AcceptCharset","`Accept-Charset` header, defined in RFC7231"],["AcceptEncoding","`Accept-Encoding` header, defined in RFC7231"],["AcceptLanguage","`Accept-Language` header, defined in RFC7231"],["AcceptRanges","`Accept-Ranges` header, defined in RFC7233"],["AccessControlAllowCredentials","`Access-Control-Allow-Credentials` header, part of CORS"],["AccessControlAllowHeaders","`Access-Control-Allow-Headers` header, part of CORS"],["AccessControlAllowMethods","`Access-Control-Allow-Methods` header, part of CORS"],["AccessControlExposeHeaders","`Access-Control-Expose-Headers` header, part of CORS"],["AccessControlMaxAge","`Access-Control-Max-Age` header, part of CORS"],["AccessControlRequestHeaders","`Access-Control-Request-Headers` header, part of CORS"],["AccessControlRequestMethod","`Access-Control-Request-Method` header, part of CORS"],["Allow","`Allow` header, defined in RFC7231"],["Authorization","`Authorization` header, defined in RFC7235"],["Basic","Credential holder for Basic Authentication"],["Bearer","Token holder for Bearer Authentication, most often seen with oauth"],["CacheControl","`Cache-Control` header, defined in RFC7234"],["Connection","`Connection` header, defined in RFC7230"],["ContentDisposition","A `Content-Disposition` header, (re)defined in RFC6266"],["ContentEncoding","`Content-Encoding` header, defined in RFC7231"],["ContentLanguage","`Content-Language` header, defined in RFC7231"],["ContentLength","`Content-Length` header, defined in RFC7230"],["ContentRange","`Content-Range` header, defined in RFC7233"],["ContentType","`Content-Type` header, defined in RFC7231"],["Cookie","`Cookie` header, defined in RFC6265"],["CookieJar","A jar of cookies for managing a session"],["CookiePair",""],["Date","`Date` header, defined in RFC7231"],["ETag","`ETag` header, defined in RFC7232"],["EntityTag","An entity tag, defined in RFC7232"],["Expires","`Expires` header, defined in RFC7234"],["From","`From` header, defined in RFC7231"],["HeaderFormatter","A wrapper around any Header with a Display impl that calls fmt_header."],["HeaderView","Returned with the `HeadersItems` iterator."],["Headers","A map of header fields on requests and responses."],["HeadersItems","An `Iterator` over the fields in a `Headers` map."],["Host","The `Host` header."],["HttpDate","A `time::Time` with HTTP formatting and parsing"],["IfModifiedSince","`If-Modified-Since` header, defined in RFC7232"],["IfUnmodifiedSince","`If-Unmodified-Since` header, defined in RFC7232"],["LastModified","`Last-Modified` header, defined in RFC7232"],["Location","`Location` header, defined in RFC7231"],["Prefer","`Prefer` header, defined in RFC7240"],["PreferenceApplied","`Preference-Applied` header, defined in RFC7240"],["Protocol","Protocols that appear in the `Upgrade` header field"],["Quality","Represents a quality used in quality values."],["QualityItem","Represents an item with a quality value as defined in RFC7231."],["Referer","`Referer` header, defined in RFC7231"],["Server","`Server` header, defined in RFC7231"],["SetCookie","`Set-Cookie` header, defined RFC6265"],["StrictTransportSecurity","`StrictTransportSecurity` header, defined in RFC6797"],["TransferEncoding","`Transfer-Encoding` header, defined in RFC7230"],["Upgrade","`Upgrade` header, defined in RFC7230"],["UserAgent","`User-Agent` header, defined in RFC7231"]],"trait":[["Header","A trait for any object that will represent a header field and value."],["Scheme","An Authorization scheme to be used in the header."]]});
#[macro_export]
macro_rules! _parse_tree {
    ($method:ident, $headers:ident, $body:ident, headers: {$($field:tt: $value:expr),*}) => {
        let $headers = {
            let mut map = fetchlike::HeaderMap::new();
            $(map.insert(
                $field,
                $value.parse()?,
            );)*
            Some(map)
        };
    };
    ($method:ident, $headers:ident, $body:ident, headers: {$($field:tt: $value:expr),* $(,)*} $(,)*) => {
        fetchlike::_parse_tree!($method, $headers, $body, headers: {$($field: $value),*});
    };
    ($method:ident, $headers:ident, $body:ident, headers: {$($field:tt: $value:expr),*}, $($tree:tt)+) => {
        fetchlike::_parse_tree!($method, $headers, $body, headers: {$($field: $value),*});
        fetchlike::_parse_tree!($method, $headers, $body, $($tree)+);
    };

    ($method:ident, $headers:ident, $body:ident, $field:ident: $value:expr) => {
        fetchlike::_parse_field!($method, $headers, $body, $field: $value);
    };
    ($method:ident, $headers:ident, $body:ident, $field:ident: $value:expr $(,)*) => {
        fetchlike::_parse_tree!($method, $headers, $body, $field: $value);
    };
    ($method:ident, $headers:ident, $body:ident, $field:ident: $value:expr, $($tree:tt)+) => {
        fetchlike::_parse_tree!($method, $headers, $body, $field: $value);
        fetchlike::_parse_tree!($method, $headers, $body, $($tree)+);
    };
}

#[macro_export]
macro_rules! _parse_field {
    ($method:ident, $headers:ident, $body:ident, method: $value:expr) => {
        let $method = fetchlike::Method::from_bytes($value.as_bytes())?;
    };
    ($method:ident, $headers:ident, $body:ident, body: $value:expr) => {
        let $body = fetchlike::Body::from($value.to_string());
    };
}

#[macro_export]
macro_rules! fetch_macro {
    ($url:expr) => {
        fetchlike::fetch(fetchlike::Request {
            url: $url.parse()?,
            ..Default::default()
        })
    };
    ($url:expr, {$($tree:tt)+}) => {
        {
            let _method = fetchlike::Method::GET;
            let _headers: Option<fetchlike::HeaderMap> = None;
            let _body = fetchlike::Body::empty();
            fetchlike::_parse_tree!(_method, _headers, _body, $($tree)+);

            fetchlike::fetch(fetchlike::Request {
                url: $url.parse()?,
                method: _method,
                headers: _headers,
                body: _body
            })
        }
    };
}

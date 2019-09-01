#[macro_export]
macro_rules! uri_path {
    { $( $val:ident )/ * } => {
        $crate::uri::UriPath::new(vec![ $( stringify!($val) ),* ])
    }
}

#[macro_export]
macro_rules! uri_params {
    { $( $key:ident => $val:expr ),* } => {
        {
            use std::collections::HashMap;
            use std::borrow::Cow;
            let mut map: HashMap<&'static str, Cow<'_, str>> = HashMap::new();
            $( map.insert(stringify!($key), Cow::from($val)); )*
            $crate::uri::QueryParameters::new(map)
        }
    }
}

#[macro_export]
macro_rules! dbg_json {
    { $e:expr } => {
        {
            let val = &($e);
            if let serde_json::Value::Object(map) = val {
                eprintln!("[{}:{}] {}.keys() = {:?}",
                          file!(),
                          line!(),
                          stringify!($e),
                          map.keys().collect::<Vec<&std::string::String>>());
            } else {
                dbg!(val);
            }
        }
    }
}

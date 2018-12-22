macro_rules! map {
    ( $( $k: expr => $v: expr, )* ) => {{
        use std::collections::HashMap;

        let mut map: HashMap<String, Value> = HashMap::new();
        $( map.insert(String::from($k), $v.into()); )*
        map
    }}
}

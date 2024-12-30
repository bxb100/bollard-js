use std::collections::HashMap;

pub fn convert_vec_to_map(vec: Option<Vec<String>>) -> Option<HashMap<String, HashMap<(), ()>>> {
    vec.map(|v| {
        v.into_iter()
            .map(|s| (s, HashMap::<(), ()>::with_capacity(0)))
            .collect::<HashMap<String, HashMap<(), ()>>>()
    })
}

pub fn convert_map_to_vec(map: Option<HashMap<String, HashMap<(), ()>>>) -> Option<Vec<String>> {
    map.map(|m| m.into_keys().collect::<Vec<_>>())
}

pub fn convert_vec_to_vec<T, R>(vec: Option<Vec<T>>) -> Option<Vec<R>>
where
    T: Into<R>,
{
    vec.map(|v| v.into_iter().map(Into::into).collect::<Vec<_>>())
}

pub fn convert_map_vec_to_map_vec<V1, V2>(
    map: Option<HashMap<String, Option<Vec<V1>>>>,
) -> Option<HashMap<String, Option<Vec<V2>>>>
where
    V1: Into<V2>,
{
    map.map(|m| {
        m.into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.map(|o| o.into_iter().map(Into::into).collect::<Vec<_>>()),
                )
            })
            .collect::<HashMap<_, _>>()
    })
}

pub fn convert_map_to_map<V1, V2>(map: Option<HashMap<String, V1>>) -> Option<HashMap<String, V2>>
where
    V1: Into<V2>,
{
    map.map(|m| {
        m.into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<HashMap<_, _>>()
    })
}

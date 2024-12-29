use std::collections::HashMap;

pub fn convert_vec_to_map(vec: Option<Vec<String>>) -> Option<HashMap<String, HashMap<(), ()>>> {
    vec.map(|v| {
        v.into_iter()
            .map(|s| (s, HashMap::<(), ()>::with_capacity(0)))
            .collect::<HashMap<String, HashMap<(), ()>>>()
    })
}

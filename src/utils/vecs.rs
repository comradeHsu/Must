use std::convert::From;


pub fn flat_map<T, S>(data: Vec<S>) -> Vec<T>
where
    T: From<S>,
{
    let mut vec = Vec::with_capacity(data.len());
    for datum in data {
        vec.push(T::from(datum));
    }
    return vec;
}

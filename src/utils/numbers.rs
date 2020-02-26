#[inline]
pub fn f32_to_i32(val: f32) -> i32 {
    let bytes = val.to_be_bytes();
    return i32::from_be_bytes(bytes);
}

#[inline]
pub fn i32_to_f32(val: i32) -> f32 {
    let bytes = val.to_be_bytes();
    return f32::from_be_bytes(bytes);
}

#[inline]
pub fn f64_to_i64(val: f64) -> i64 {
    let bytes = val.to_be_bytes();
    return i64::from_be_bytes(bytes);
}

#[inline]
pub fn i64_to_f64(val: i64) -> f64 {
    let bytes = val.to_be_bytes();
    return f64::from_be_bytes(bytes);
}

#[inline]
pub fn i64_back_bytes_to_i32(val: i64) -> i32 {
    let bytes: [u8; 8] = val.to_be_bytes();
    let (_front, back) = bytes.split_at(4);
    let mut back_array = [0; 4];
    back_array.copy_from_slice(back);
    return i32::from_be_bytes(back_array);
}

#[inline]
pub fn i64_from_bytes(val: i32) -> i64 {
    let bytes: [u8; 4] = val.to_be_bytes();
    let mut raw_data = [0u8;8];
    for index in 0..4usize {
        raw_data[4+index] = bytes[index];
    }
    return i64::from_be_bytes(raw_data);
}

#[inline]
pub fn i64_from_i32_bytes(val: i32) -> i64 {
    let bytes: [u8; 4] = val.to_be_bytes();
    let mut array = [0u8; 8];
    for i in 4..8 {
        array[i] = bytes[i - 4];
    }
    return i64::from_be_bytes(array);
}

///Returns a power of two size for the given target capacity.
#[inline]
pub fn get_power_of_two(val: usize) -> usize {
    let mut n = val - 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    return n + 1;
}

#[inline]
pub fn i32_for_bool(val: bool) -> i32 {
    if val {
        return 1;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use crate::utils::numbers::{f32_to_i32, i32_to_f32};

    #[test]
    fn test_f32_to_i32() {
        let f: f32 = 3.14159;
        let i = f32_to_i32(f);
        println!("int is {}", i);
        println!("f32 is {}", i32_to_f32(i));
    }
}

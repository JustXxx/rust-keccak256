extern "C" {
    pub fn wasm_input(is_public: u32) -> u64;
    pub fn require(cond: bool);
}

fn read_public_input() -> u64 {
    unsafe { wasm_input(1) }
}

fn read_private_input() -> u64 {
    unsafe { wasm_input(0) }
}

use wasm_bindgen::prelude::*;


fn keccak256check(input: &[u8], output: &[u8]) {
    use tiny_keccak::{Hasher, Keccak};
    let mut k256 = Keccak::v256();
    let mut hash = [0; 32];
    k256.update(input);
    k256.finalize(&mut hash);
    for i in 0..32 {
        unsafe { require(hash[i] == output[i]) };
    }
}

fn test1() {
    let mut input = [197];
    let output = [
        21, 191, 54, 255, 99, 225, 69, 172, 52, 26, 134, 0, 126, 137, 21, 92, 243, 18, 222, 79, 162, 167, 211, 173, 63, 188, 75, 120, 1, 3, 35, 72,
    ];
    keccak256check(&input, &output);
}

fn test8() {
    let mut input = [249, 2, 45, 160, 75, 87, 39, 184];
    let output = [
        69, 102, 19, 84, 8, 182, 211, 54, 122, 24, 197, 90, 72, 208, 4, 65, 223, 196, 16, 224, 97,
        91, 5, 82, 175, 71, 76, 207, 81, 108, 180, 124,
    ];
    keccak256check(&input, &output);
}

fn test80() {
    let mut input = [
        249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75,
        87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2,
        45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184,
    ];
    let output = [
        224, 204, 76, 15, 86, 83, 29, 178, 68, 108, 44, 202, 181, 133, 37, 64, 9, 165, 226, 87, 92,
        102, 98, 232, 199, 43, 63, 132, 252, 185, 148, 35,
    ];
    keccak256check(&input, &output);
}

fn test800() {
    let mut input = [
        249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75,
        87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2,
        45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249,
        2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 222, 2, 45, 160, 75, 87, 39,
        184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184, 249, 2, 45, 160,
        75, 87, 39, 184, 249, 2, 45, 160, 75, 87, 39, 184,
    ];
    let output = [
        203, 252, 250, 225, 242, 208, 58, 199, 227, 201, 181, 219, 87, 62, 170, 198, 60, 240, 227,
        213, 146, 210, 62, 111, 101, 198, 234, 158, 174, 126, 9, 240,
    ];
    keccak256check(&input, &output);
}

#[wasm_bindgen]
pub fn zkmain() -> i64 {
    test1();
    0
}


#[wasm_bindgen]
pub fn zkmain_old() -> i64 {
    //empty
    let empty_output = [
        197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83,
        202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112,
    ];
    keccak256check(&vec![], &empty_output);

    // short
    let short_output = [
        56, 209, 138, 203, 103, 210, 92, 139, 185, 148, 39, 100, 182, 47, 24, 225, 112, 84, 246,
        106, 129, 123, 212, 41, 84, 35, 173, 249, 237, 152, 135, 62,
    ];
    let mut input:[u8; 6] = [102, 111, 111, 98, 97, 114];
    keccak256check(&input, &short_output);

    // long
    let long_output = [
        60, 227, 142, 8, 143, 135, 108, 85, 13, 254, 190, 58, 30, 106, 153, 194, 188, 6, 208, 49,
        16, 102, 150, 120, 100, 130, 224, 177, 64, 98, 53, 252,
    ];
    let mut input = [
        65, 108, 105, 99, 101, 32, 119, 97, 115, 32, 98, 101, 103, 105, 110, 110, 105, 110, 103,
        32, 116, 111, 32, 103, 101, 116, 32, 118, 101, 114, 121, 32, 116, 105, 114, 101, 100, 32,
        111, 102, 32, 115, 105, 116, 116, 105, 110, 103, 32, 98, 121, 32, 104, 101, 114, 32, 115,
        105, 115, 116, 101, 114, 32, 111, 110, 32, 116, 104, 101, 32, 98, 97, 110, 107, 44, 32, 97,
        110, 100, 32, 111, 102, 32, 104, 97, 118, 105, 110, 103, 32, 110, 111, 116, 104, 105, 110,
        103, 32, 116, 111, 32, 100, 111, 58, 32, 111, 110, 99, 101, 32, 111, 114, 32, 116, 119,
        105, 99, 101, 32, 115, 104, 101, 32, 104, 97, 100, 32, 112, 101, 101, 112, 101, 100, 32,
        105, 110, 116, 111, 32, 116, 104, 101, 32, 98, 111, 111, 107, 32, 104, 101, 114, 32, 115,
        105, 115, 116, 101, 114, 32, 119, 97, 115, 32, 114, 101, 97, 100, 105, 110, 103, 44, 32,
        98, 117, 116, 32, 105, 116, 32, 104, 97, 100, 32, 110, 111, 32, 112, 105, 99, 116, 117,
        114, 101, 115, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 97, 116, 105, 111, 110,
        115, 32, 105, 110, 32, 105, 116, 44, 32, 97, 110, 100, 32, 119, 104, 97, 116, 32, 105, 115,
        32, 116, 104, 101, 32, 117, 115, 101, 32, 111, 102, 32, 97, 32, 98, 111, 111, 107, 44, 32,
        116, 104, 111, 117, 103, 104, 116, 32, 65, 108, 105, 99, 101, 32, 119, 105, 116, 104, 111,
        117, 116, 32, 112, 105, 99, 116, 117, 114, 101, 115, 32, 111, 114, 32, 99, 111, 110, 118,
        101, 114, 115, 97, 116, 105, 111, 110, 115, 63,
    ];
    keccak256check(&input, &long_output);


    0
}

/*
#[test]
fn test_empty_input() {
    let output = [
        197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83,
        202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112,
    ];
    assert_eq!(keccak256(&[]), output);
}

#[test]
fn test_short_input() {
    let output = [
        56, 209, 138, 203, 103, 210, 92, 139, 185, 148, 39, 100, 182, 47, 24, 225, 112, 84, 246,
        106, 129, 123, 212, 41, 84, 35, 173, 249, 237, 152, 135, 62,
    ];
    assert_eq!(keccak256(&[102, 111, 111, 98, 97, 114]), output);
}

#[test]
fn test_long_input() {
    let input = [
        65, 108, 105, 99, 101, 32, 119, 97, 115, 32, 98, 101, 103, 105, 110, 110, 105, 110, 103,
        32, 116, 111, 32, 103, 101, 116, 32, 118, 101, 114, 121, 32, 116, 105, 114, 101, 100, 32,
        111, 102, 32, 115, 105, 116, 116, 105, 110, 103, 32, 98, 121, 32, 104, 101, 114, 32, 115,
        105, 115, 116, 101, 114, 32, 111, 110, 32, 116, 104, 101, 32, 98, 97, 110, 107, 44, 32, 97,
        110, 100, 32, 111, 102, 32, 104, 97, 118, 105, 110, 103, 32, 110, 111, 116, 104, 105, 110,
        103, 32, 116, 111, 32, 100, 111, 58, 32, 111, 110, 99, 101, 32, 111, 114, 32, 116, 119,
        105, 99, 101, 32, 115, 104, 101, 32, 104, 97, 100, 32, 112, 101, 101, 112, 101, 100, 32,
        105, 110, 116, 111, 32, 116, 104, 101, 32, 98, 111, 111, 107, 32, 104, 101, 114, 32, 115,
        105, 115, 116, 101, 114, 32, 119, 97, 115, 32, 114, 101, 97, 100, 105, 110, 103, 44, 32,
        98, 117, 116, 32, 105, 116, 32, 104, 97, 100, 32, 110, 111, 32, 112, 105, 99, 116, 117,
        114, 101, 115, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 97, 116, 105, 111, 110,
        115, 32, 105, 110, 32, 105, 116, 44, 32, 97, 110, 100, 32, 119, 104, 97, 116, 32, 105, 115,
        32, 116, 104, 101, 32, 117, 115, 101, 32, 111, 102, 32, 97, 32, 98, 111, 111, 107, 44, 32,
        116, 104, 111, 117, 103, 104, 116, 32, 65, 108, 105, 99, 101, 32, 119, 105, 116, 104, 111,
        117, 116, 32, 112, 105, 99, 116, 117, 114, 101, 115, 32, 111, 114, 32, 99, 111, 110, 118,
        101, 114, 115, 97, 116, 105, 111, 110, 115, 63,
    ];
    let output = [
        60, 227, 142, 8, 143, 135, 108, 85, 13, 254, 190, 58, 30, 106, 153, 194, 188, 6, 208, 49,
        16, 102, 150, 120, 100, 130, 224, 177, 64, 98, 53, 252,
    ];
    assert_eq!(keccak256(&input), output);
}
*/

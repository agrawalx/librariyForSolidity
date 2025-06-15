
#![no_main]
#![no_std]

extern crate alloc;
use static_alloc::Bump;

#[global_allocator]
static ALLOCATOR: Bump<[u8; 128 * 1024]> = Bump::uninit(); // Increased memory

use uapi::{HostFn, HostFnImpl as api, ReturnFlags};
use alloc::vec::Vec;
use alloc::vec;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("unimp");
        core::hint::unreachable_unchecked();
    }
}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

/// Main entry point. Decodes a function selector and dispatches to the correct handler.
#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
    let selector = u32::from_be_bytes(extract_selector());

    // Dispatch based on the selector. Each function has a unique ID.
    match selector {
        // --- Original Math & Geometry Functions ---
        0x01 => api::return_value(ReturnFlags::empty(), &modexp_handler()),
        0x02 => api::return_value(ReturnFlags::empty(), &square_handler()),
        0x03 => api::return_value(ReturnFlags::empty(), &square_root_handler()),
        0x04 => api::return_value(ReturnFlags::empty(), &mul_handler()),
        0x05 => api::return_value(ReturnFlags::empty(), &div_handler()),
        0x06 => api::return_value(ReturnFlags::empty(), &lerp_handler()),
        0x07 => api::return_value(ReturnFlags::empty(), &sin_handler()),
        0x08 => api::return_value(ReturnFlags::empty(), &cos_handler()),
        0x09 => api::return_value(ReturnFlags::empty(), &squared_distance_handler()),
        0x0A => api::return_value(ReturnFlags::empty(), &distance_between_handler()),
        0x0B => api::return_value(ReturnFlags::empty(), &dot_product_handler()),
        0x0C => api::return_value(ReturnFlags::empty(), &magnitude_handler()),
        0x0D => api::return_value(ReturnFlags::empty(), &cross_product_handler()),
        0x0E => api::return_value(ReturnFlags::empty(), &clamp_handler()),
        0x0F => api::return_value(ReturnFlags::empty(), &clamp_vector_magnitude_handler()),
        0x10 => api::return_value(ReturnFlags::empty(), &is_point_in_rect_handler()),
        0x11 => api::return_value(ReturnFlags::empty(), &is_point_in_circle_handler()),
        0x12 => api::return_value(ReturnFlags::empty(), &add_vectors_handler()),
        0x13 => api::return_value(ReturnFlags::empty(), &subtract_vectors_handler()),
        0x14 => api::return_value(ReturnFlags::empty(), &scale_vector_handler()),
        0x15 => api::return_value(ReturnFlags::empty(), &normalize_vector_handler()),
        0x16 => api::return_value(ReturnFlags::empty(), &rotate_vector_handler()),
        0x17 => api::return_value(ReturnFlags::empty(), &reflect_vector_handler()),
        0x18 => api::return_value(ReturnFlags::empty(), &is_point_in_triangle_handler()),

        // --- New Number Theory & Crypto Functions ---
        0x19 => api::return_value(ReturnFlags::empty(), &modinv_handler()),
        0x1A => api::return_value(ReturnFlags::empty(), &is_prime_handler()),
        0x1B => api::return_value(ReturnFlags::empty(), &gcd_handler()),
        0x1C => api::return_value(ReturnFlags::empty(), &lcm_handler()),
        0x1D => api::return_value(ReturnFlags::empty(), &factorial_handler()),
        0x1E => api::return_value(ReturnFlags::empty(), &n_choose_k_handler()),
        0x1F => api::return_value(ReturnFlags::empty(), &log2_floor_handler()),
        0x20 => api::return_value(ReturnFlags::empty(), &log10_floor_handler()),
        0x21 => api::return_value(ReturnFlags::empty(), &popcount_handler()),
        0x22 => api::return_value(ReturnFlags::empty(), &reverse_bits_handler()),
        0x23 => api::return_value(ReturnFlags::empty(), &phi_handler()),
        0x24 => api::return_value(ReturnFlags::empty(), &rotl64_handler()),
        0x25 => api::return_value(ReturnFlags::empty(), &rotr64_handler()),
        0x26 => api::return_value(ReturnFlags::empty(), &constant_time_eq_handler()),
        0x27 => api::return_value(ReturnFlags::empty(), &clmul_handler()),
        0x28 => api::return_value(ReturnFlags::empty(), &xorshift_next_handler()),
        0x29 => api::return_value(ReturnFlags::empty(), &point_add_handler()),
        0x2A => api::return_value(ReturnFlags::empty(), &point_double_handler()),
        0x2b => api::return_value(ReturnFlags::empty(), &get_projectile_trajectory_coefficients_handler()),
        
        _ => {
            // Default case for an unknown selector
            api::return_value(ReturnFlags::empty(), &[0u8; 32]);
        }
    };
}


// ==========================================================================================
//                                     HANDLER FUNCTIONS
// ==========================================================================================
// These functions parse inputs from calldata, call the logic, and encode the output.

// --- Original Math Handlers ---

fn modexp_handler() -> [u8; 32] {
    let inputs = read_inputs(3);
    let base = u64_from_abi_word(&inputs[0]);
    let exp = u64_from_abi_word(&inputs[1]);
    let modu = u64_from_abi_word(&inputs[2]);
    let result = modexp(base, exp, modu);
    u64_to_abi_word(result)
}

fn square_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = square(n);
    u64_to_abi_word(result)
}

fn square_root_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = square_root(n);
    u64_to_abi_word(result)
}

fn mul_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let a = u64_from_abi_word(&inputs[0]);
    let b = u64_from_abi_word(&inputs[1]);
    let result = mul(a, b);
    u64_to_abi_word(result)
}

fn div_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let a = u64_from_abi_word(&inputs[0]);
    let b = u64_from_abi_word(&inputs[1]);
    let result = div(a, b);
    u64_to_abi_word(result)
}

fn get_projectile_trajectory_coefficients_handler() -> Vec<u8> {
    let inputs = read_inputs(3);
    let angle_deg_times_10 = u32_from_abi_word(&inputs[0]);
    let initial_velocity = u64_from_abi_word(&inputs[1]);
    let gravity = u64_from_abi_word(&inputs[2]);
    
    let (c1, c2) = get_projectile_trajectory_coefficients(angle_deg_times_10, initial_velocity, gravity);
    
    let mut output = Vec::with_capacity(64);
    output.extend_from_slice(&i64_to_abi_word(c1));
    output.extend_from_slice(&i64_to_abi_word(c2));
    output
}

fn lerp_handler() -> [u8; 32] {
    let inputs = read_inputs(3);
    let start = u64_from_abi_word(&inputs[0]);
    let end = u64_from_abi_word(&inputs[1]);
    let t = u64_from_abi_word(&inputs[2]);
    let result = lerp(start, end, t);
    u64_to_abi_word(result)
}

fn sin_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let angle = u32_from_abi_word(&inputs[0]);
    let result = sin(angle);
    i64_to_abi_word(result)
}

fn cos_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let angle = u32_from_abi_word(&inputs[0]);
    let result = cos(angle);
    i64_to_abi_word(result)
}

fn squared_distance_handler() -> [u8; 32] {
    let inputs = read_inputs(4);
    let x1 = u64_from_abi_word(&inputs[0]);
    let y1 = u64_from_abi_word(&inputs[1]);
    let x2 = u64_from_abi_word(&inputs[2]);
    let y2 = u64_from_abi_word(&inputs[3]);
    let result = squared_distance(x1, y1, x2, y2);
    u64_to_abi_word(result)
}

fn distance_between_handler() -> [u8; 32] {
    let inputs = read_inputs(4);
    let x1 = u64_from_abi_word(&inputs[0]);
    let y1 = u64_from_abi_word(&inputs[1]);
    let x2 = u64_from_abi_word(&inputs[2]);
    let y2 = u64_from_abi_word(&inputs[3]);
    let result = distance_between(x1, y1, x2, y2);
    u64_to_abi_word(result)
}

fn dot_product_handler() -> [u8; 32] {
    let inputs = read_inputs(4);
    let vx1 = u64_from_abi_word(&inputs[0]);
    let vy1 = u64_from_abi_word(&inputs[1]);
    let vx2 = u64_from_abi_word(&inputs[2]);
    let vy2 = u64_from_abi_word(&inputs[3]);
    let result = dot_product(vx1, vy1, vx2, vy2);
    u64_to_abi_word(result)
}

fn magnitude_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let result = magnitude(vx, vy);
    u64_to_abi_word(result)
}

fn cross_product_handler() -> [u8; 32] {
    let inputs = read_inputs(4);
    let vx1 = u64_from_abi_word(&inputs[0]);
    let vy1 = u64_from_abi_word(&inputs[1]);
    let vx2 = u64_from_abi_word(&inputs[2]);
    let vy2 = u64_from_abi_word(&inputs[3]);
    let result = cross_product(vx1, vy1, vx2, vy2);
    i64_to_abi_word(result)
}

fn clamp_handler() -> [u8; 32] {
    let inputs = read_inputs(3);
    let value = u64_from_abi_word(&inputs[0]);
    let min = u64_from_abi_word(&inputs[1]);
    let max = u64_from_abi_word(&inputs[2]);
    let result = clamp(value, min, max);
    u64_to_abi_word(result)
}

fn clamp_vector_magnitude_handler() -> Vec<u8> {
    let inputs = read_inputs(3);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let max_length = u64_from_abi_word(&inputs[2]);
    let (new_vx, new_vy) = clamp_vector_magnitude(vx, vy, max_length);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_vx.to_be_bytes());
    output[56..64].copy_from_slice(&new_vy.to_be_bytes());
    output
}

fn is_point_in_rect_handler() -> [u8; 32] {
    let inputs = read_inputs(6);
    let px = u64_from_abi_word(&inputs[0]);
    let py = u64_from_abi_word(&inputs[1]);
    let rect_x = u64_from_abi_word(&inputs[2]);
    let rect_y = u64_from_abi_word(&inputs[3]);
    let rect_width = u64_from_abi_word(&inputs[4]);
    let rect_height = u64_from_abi_word(&inputs[5]);
    let result = is_point_in_rect(px, py, rect_x, rect_y, rect_width, rect_height);
    bool_to_abi_word(result)
}

fn is_point_in_circle_handler() -> [u8; 32] {
    let inputs = read_inputs(5);
    let px = u64_from_abi_word(&inputs[0]);
    let py = u64_from_abi_word(&inputs[1]);
    let circle_cx = u64_from_abi_word(&inputs[2]);
    let circle_cy = u64_from_abi_word(&inputs[3]);
    let circle_radius = u64_from_abi_word(&inputs[4]);
    let result = is_point_in_circle(px, py, circle_cx, circle_cy, circle_radius);
    bool_to_abi_word(result)
}

fn add_vectors_handler() -> Vec<u8> {
    let inputs = read_inputs(4);
    let vx1 = u64_from_abi_word(&inputs[0]);
    let vy1 = u64_from_abi_word(&inputs[1]);
    let vx2 = u64_from_abi_word(&inputs[2]);
    let vy2 = u64_from_abi_word(&inputs[3]);
    let (new_vx, new_vy) = add_vectors(vx1, vy1, vx2, vy2);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_vx.to_be_bytes());
    output[56..64].copy_from_slice(&new_vy.to_be_bytes());
    output
}

fn subtract_vectors_handler() -> Vec<u8> {
    let inputs = read_inputs(4);
    let vx1 = u64_from_abi_word(&inputs[0]);
    let vy1 = u64_from_abi_word(&inputs[1]);
    let vx2 = u64_from_abi_word(&inputs[2]);
    let vy2 = u64_from_abi_word(&inputs[3]);
    let (new_vx, new_vy) = subtract_vectors(vx1, vy1, vx2, vy2);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_vx.to_be_bytes());
    output[56..64].copy_from_slice(&new_vy.to_be_bytes());
    output
}

fn scale_vector_handler() -> Vec<u8> {
    let inputs = read_inputs(3);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let scalar = u64_from_abi_word(&inputs[2]);
    let (new_vx, new_vy) = scale_vector(vx, vy, scalar);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_vx.to_be_bytes());
    output[56..64].copy_from_slice(&new_vy.to_be_bytes());
    output
}

fn normalize_vector_handler() -> Vec<u8> {
    let inputs = read_inputs(2);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let (new_vx, new_vy) = normalize_vector(vx, vy);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_vx.to_be_bytes());
    output[56..64].copy_from_slice(&new_vy.to_be_bytes());
    output
}

fn rotate_vector_handler() -> Vec<u8> {
    let inputs = read_inputs(3);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let angle = u32_from_abi_word(&inputs[2]);
    let (new_x, new_y) = rotate_vector(vx, vy, angle);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&new_x.to_be_bytes());
    output[56..64].copy_from_slice(&new_y.to_be_bytes());
    output
}

fn reflect_vector_handler() -> Vec<u8> {
    let inputs = read_inputs(4);
    let vx = u64_from_abi_word(&inputs[0]);
    let vy = u64_from_abi_word(&inputs[1]);
    let normal_x = u64_from_abi_word(&inputs[2]);
    let normal_y = u64_from_abi_word(&inputs[3]);
    let (reflected_vx, reflected_vy) = reflect_vector(vx, vy, normal_x, normal_y);
    let mut output = vec![0u8; 64];
    output[24..32].copy_from_slice(&reflected_vx.to_be_bytes());
    output[56..64].copy_from_slice(&reflected_vy.to_be_bytes());
    output
}

fn is_point_in_triangle_handler() -> [u8; 32] {
    let inputs = read_inputs(8);
    let px = u64_from_abi_word(&inputs[0]);
    let py = u64_from_abi_word(&inputs[1]);
    let ax = u64_from_abi_word(&inputs[2]);
    let ay = u64_from_abi_word(&inputs[3]);
    let bx = u64_from_abi_word(&inputs[4]);
    let by = u64_from_abi_word(&inputs[5]);
    let cx = u64_from_abi_word(&inputs[6]);
    let cy = u64_from_abi_word(&inputs[7]);
    let result = is_point_in_triangle(px, py, ax, ay, bx, by, cx, cy);
    bool_to_abi_word(result)
}


// --- New Crypto & Number Theory Handlers ---

fn modinv_handler() -> Vec<u8> {
    let inputs = read_inputs(2);
    let a = i64_from_abi_word(&inputs[0]);
    let m = i64_from_abi_word(&inputs[1]);
    let result = modinv(a, m);
    encode_option_i64(result)
}

fn is_prime_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = is_prime(n);
    bool_to_abi_word(result)
}

fn gcd_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let a = u64_from_abi_word(&inputs[0]);
    let b = u64_from_abi_word(&inputs[1]);
    let result = gcd(a, b);
    u64_to_abi_word(result)
}

fn lcm_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let a = u64_from_abi_word(&inputs[0]);
    let b = u64_from_abi_word(&inputs[1]);
    let result = lcm(a, b);
    u64_to_abi_word(result)
}

fn factorial_handler() -> Vec<u8> {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = factorial(n);
    encode_option_u64(result)
}

fn n_choose_k_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let n = u64_from_abi_word(&inputs[0]);
    let k = u64_from_abi_word(&inputs[1]);
    let result = n_choose_k(n, k);
    u64_to_abi_word(result)
}

fn log2_floor_handler() -> Vec<u8> {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = log2_floor(n);
    encode_option_u32(result)
}

fn log10_floor_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = log10_floor(n);
    u32_to_abi_word(result)
}

fn popcount_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = popcount(n);
    u32_to_abi_word(result)
}

fn reverse_bits_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = reverse_bits(n);
    u64_to_abi_word(result)
}

fn phi_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let n = u64_from_abi_word(&inputs[0]);
    let result = phi(n);
    u64_to_abi_word(result)
}

fn rotl64_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let n = u64_from_abi_word(&inputs[0]);
    let k = u32_from_abi_word(&inputs[1]);
    let result = rotl64(n, k);
    u64_to_abi_word(result)
}

fn rotr64_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let n = u64_from_abi_word(&inputs[0]);
    let k = u32_from_abi_word(&inputs[1]);
    let result = rotr64(n, k);
    u64_to_abi_word(result)
}

fn constant_time_eq_handler() -> [u8; 32] {
    let inputs = read_inputs(2);
    let result = constant_time_eq(&inputs[0], &inputs[1]);
    bool_to_abi_word(result)
}

fn clmul_handler() -> Vec<u8> {
    let inputs = read_inputs(2);
    let a = u64_from_abi_word(&inputs[0]);
    let b = u64_from_abi_word(&inputs[1]);
    let result = clmul(a, b);
    
    let mut output = Vec::with_capacity(64);
    let high = (result >> 64) as u64;
    let low = result as u64;
    output.extend_from_slice(&u64_to_abi_word(high));
    output.extend_from_slice(&u64_to_abi_word(low));
    output
}

fn xorshift_next_handler() -> [u8; 32] {
    let inputs = read_inputs(1);
    let seed = u64_from_abi_word(&inputs[0]);
    let mut rng = Xorshift64Star::new(seed);
    let result = rng.next();
    u64_to_abi_word(result)
}

fn point_add_handler() -> Vec<u8> {
    let inputs = read_inputs(6);
    let p1 = point_from_abi_words(&inputs[0], &inputs[1]);
    let p2 = point_from_abi_words(&inputs[2], &inputs[3]);
    let a = u64_from_abi_word(&inputs[4]);
    let modulus = u64_from_abi_word(&inputs[5]);
    let result = point_add(p1, p2, a, modulus); 
    encode_option_point(result)
}

fn point_double_handler() -> Vec<u8> {
    let inputs = read_inputs(4);
    let p = point_from_abi_words(&inputs[0], &inputs[1]);
    let a = u64_from_abi_word(&inputs[2]);
    let modulus = u64_from_abi_word(&inputs[3]);
    let result = point_double(p, a, modulus);
    encode_option_point(result)
}


// ==========================================================================================
//                                 ABI ENCODING/DECODING HELPERS
// ==========================================================================================

fn read_inputs(num_args: usize) -> Vec<[u8; 32]> {
    let mut inputs = Vec::with_capacity(num_args);
    for i in 0..num_args {
        let mut buf = [0u8; 32];
        api::call_data_copy(&mut buf, (4 + i * 32).try_into().unwrap());
        inputs.push(buf);
    }
    inputs
}

fn u64_from_abi_word(word: &[u8; 32]) -> u64 {
    u64::from_be_bytes(word[24..].try_into().unwrap())
}

fn i64_from_abi_word(word: &[u8; 32]) -> i64 {
    i64::from_be_bytes(word[24..].try_into().unwrap())
}

fn u32_from_abi_word(word: &[u8; 32]) -> u32 {
    u32::from_be_bytes(word[28..].try_into().unwrap())
}

const fn u64_to_abi_word(value: u64) -> [u8; 32] {
    let mut output = [0u8; 32];
    let value_bytes = value.to_be_bytes();
    output[24] = value_bytes[0];
    output[25] = value_bytes[1];
    output[26] = value_bytes[2];
    output[27] = value_bytes[3];
    output[28] = value_bytes[4];
    output[29] = value_bytes[5];
    output[30] = value_bytes[6];
    output[31] = value_bytes[7];
    output
}

fn i64_to_abi_word(value: i64) -> [u8; 32] {
    let mut output = [0u8; 32];
    output[24..].copy_from_slice(&value.to_be_bytes());
    output
}

fn u32_to_abi_word(value: u32) -> [u8; 32] {
    let mut output = [0u8; 32];
    output[28..].copy_from_slice(&value.to_be_bytes());
    output
}

fn bool_to_abi_word(value: bool) -> [u8; 32] {
    let mut output = [0u8; 32];
    if value { output[31] = 1; }
    output
}

fn extract_selector() -> [u8; 4] {
    let mut selector = [0u8; 4];
    api::call_data_copy(&mut selector, 0);
    selector
}

fn encode_option_u64(opt: Option<u64>) -> Vec<u8> {
    let mut output = Vec::with_capacity(64);
    match opt {
        Some(val) => {
            output.extend_from_slice(&bool_to_abi_word(true));
            output.extend_from_slice(&u64_to_abi_word(val));
        },
        None => {
            output.extend_from_slice(&bool_to_abi_word(false));
            output.extend_from_slice(&[0; 32]);
        }
    }
    output
}

fn encode_option_i64(opt: Option<i64>) -> Vec<u8> {
    let mut output = Vec::with_capacity(64);
    match opt {
        Some(val) => {
            let mut word = [0u8; 32];
            word[24..].copy_from_slice(&val.to_be_bytes());
            output.extend_from_slice(&bool_to_abi_word(true));
            output.extend_from_slice(&word);
        },
        None => {
            output.extend_from_slice(&bool_to_abi_word(false));
            output.extend_from_slice(&[0; 32]);
        }
    }
    output
}

fn encode_option_u32(opt: Option<u32>) -> Vec<u8> {
    let mut output = Vec::with_capacity(64);
    match opt {
        Some(val) => {
            output.extend_from_slice(&bool_to_abi_word(true));
            output.extend_from_slice(&u32_to_abi_word(val));
        },
        None => {
            output.extend_from_slice(&bool_to_abi_word(false));
            output.extend_from_slice(&[0; 32]);
        }
    }
    output
}

const U64_MAX_WORD: [u8; 32] = u64_to_abi_word(u64::MAX);

fn point_from_abi_words(x_word: &[u8; 32], y_word: &[u8; 32]) -> Point {
    if x_word == &U64_MAX_WORD && y_word == &U64_MAX_WORD {
        Point::Infinity
    } else {
        Point::Coordinate { x: u64_from_abi_word(x_word), y: u64_from_abi_word(y_word) }
    }
}

fn encode_option_point(opt: Option<Point>) -> Vec<u8> {
    let mut output = Vec::with_capacity(96);
    match opt {
        Some(Point::Coordinate{x, y}) => {
            output.extend_from_slice(&bool_to_abi_word(true));
            output.extend_from_slice(&u64_to_abi_word(x));
            output.extend_from_slice(&u64_to_abi_word(y));
        },
        Some(Point::Infinity) => {
            output.extend_from_slice(&bool_to_abi_word(true));
            output.extend_from_slice(&U64_MAX_WORD);
            output.extend_from_slice(&U64_MAX_WORD);
        },
        None => {
            output.extend_from_slice(&bool_to_abi_word(false));
            output.extend_from_slice(&[0; 32]);
            output.extend_from_slice(&[0; 32]);
        }
    }
    output
}


// ==========================================================================================
//                                 CORE LOGIC FUNCTIONS
// ==========================================================================================

pub fn modexp(base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0; }
    let mut result: u128 = 1;
    let mut base_128 = (base % modulus) as u128;
    let modulus_128 = modulus as u128;
    while exp > 0 {
        if exp % 2 == 1 { result = (result * base_128) % modulus_128; }
        exp >>= 1;
        base_128 = (base_128 * base_128) % modulus_128;
    }
    result as u64
}

pub fn square(n: u64) -> u64 {
    n.saturating_mul(n) / 100
}

pub fn square_root(n: u64) -> u64 {
    let scaled_n = n.saturating_mul(100);
    if scaled_n == 0 { return 0; }
    let mut x = scaled_n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + scaled_n / x) / 2;
    }
    x
}

pub fn mul(a: u64, b: u64) -> u64 {
    a.saturating_mul(b) / 100
}

pub fn div(a: u64, b: u64) -> u64 {
    if b == 0 { return 0; }
    a.saturating_mul(100) / b
}

pub fn lerp(start: u64, end: u64, t: u64) -> u64 {
    let t_clamped = if t > 100 { 100 } else { t };
    if start < end {
        let delta = end - start;
        start + delta.saturating_mul(t_clamped) / 100
    } else {
        let delta = start - end;
        start - delta.saturating_mul(t_clamped) / 100
    }
}

const SINE_LUT_QUADRANT: [i64; 91] = [
    0, 175, 349, 523, 698, 872, 1045, 1219, 1392, 1564, 1736, 1908, 2079, 2250,
    2419, 2588, 2756, 2924, 3090, 3256, 3420, 3584, 3746, 3907, 4067, 4226, 4384,
    4540, 4695, 4848, 5000, 5150, 5299, 5446, 5592, 5736, 5878, 6018, 6157, 6293,
    6428, 6561, 6691, 6820, 6947, 7071, 7193, 7314, 7431, 7547, 7660, 7771, 7880,
    7986, 8090, 8192, 8290, 8387, 8480, 8572, 8660, 8746, 8829, 8910, 8988, 9063,
    9135, 9205, 9272, 9336, 9397, 9455, 9511, 9563, 9613, 9659, 9703, 9744, 9781,
    9816, 9848, 9877, 9903, 9925, 9945, 9962, 9976, 9986, 9994, 9998, 10000,
];

pub fn sin(angle_deg_times_10: u32) -> i64 {
    let angle = (angle_deg_times_10 % 3600) as usize;
    let quadrant = angle / 900;
    let index = angle % 900;
    let result = match quadrant {
        0 => SINE_LUT_QUADRANT[index / 10],
        1 => SINE_LUT_QUADRANT[90 - (index / 10)],
        2 => -SINE_LUT_QUADRANT[index / 10],
        _ => -SINE_LUT_QUADRANT[90 - (index / 10)],
    };
    result / 100
}

pub fn cos(angle_deg_times_10: u32) -> i64 {
    sin(angle_deg_times_10.wrapping_add(900))
}

pub fn squared_distance(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    let sq_dx = (dx.saturating_mul(dx)) / 100;
    let sq_dy = (dy.saturating_mul(dy)) / 100;
    sq_dx.saturating_add(sq_dy)
}

pub fn distance_between(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let sq_dist = squared_distance(x1, y1, x2, y2);
    square_root(sq_dist)
}

pub fn dot_product(vx1: u64, vy1: u64, vx2: u64, vy2: u64) -> u64 {
    let x_prod = (vx1.saturating_mul(vx2)) / 100;
    let y_prod = (vy1.saturating_mul(vy2)) / 100;
    x_prod.saturating_add(y_prod)
}

pub fn magnitude(vx: u64, vy: u64) -> u64 {
    let dot_self = dot_product(vx, vy, vx, vy);
    square_root(dot_self)
}

pub fn cross_product(vx1: u64, vy1: u64, vx2: u64, vy2: u64) -> i64 {
    let term1 = (vx1.saturating_mul(vy2)) / 100;
    let term2 = (vy1.saturating_mul(vx2)) / 100;
    (term1 as i64) - (term2 as i64)
}

pub fn clamp(value: u64, min: u64, max: u64) -> u64 {
    if value < min { min } else if value > max { max } else { value }
}

pub fn clamp_vector_magnitude(vx: u64, vy: u64, max_length: u64) -> (u64, u64) {
    let mag = magnitude(vx, vy);
    if mag > max_length {
        let new_vx = mul(vx, div(max_length, mag));
        let new_vy = mul(vy, div(max_length, mag));
        (new_vx, new_vy)
    } else {
        (vx, vy)
    }
}

pub fn is_point_in_rect(px: u64, py: u64, rect_x: u64, rect_y: u64, rect_width: u64, rect_height: u64) -> bool {
    px >= rect_x && px <= rect_x.saturating_add(rect_width) &&
    py >= rect_y && py <= rect_y.saturating_add(rect_height)
}

pub fn is_point_in_circle(px: u64, py: u64, circle_cx: u64, circle_cy: u64, circle_radius: u64) -> bool {
    let dx = if px > circle_cx { px - circle_cx } else { circle_cx - px };
    let dy = if py > circle_cy { py - circle_cy } else { circle_cy - py };
    
    let dx_sq = dx.saturating_mul(dx);
    let dy_sq = dy.saturating_mul(dy);
    let r_sq = circle_radius.saturating_mul(circle_radius);

    dx_sq.saturating_add(dy_sq) <= r_sq
}

pub fn add_vectors(vx1: u64, vy1: u64, vx2: u64, vy2: u64) -> (u64, u64) {
    (vx1.saturating_add(vx2), vy1.saturating_add(vy2))
}

pub fn subtract_vectors(vx1: u64, vy1: u64, vx2: u64, vy2: u64) -> (u64, u64) {
    (vx1.saturating_sub(vx2), vy1.saturating_sub(vy2))
}

pub fn scale_vector(vx: u64, vy: u64, scalar: u64) -> (u64, u64) {
    (mul(vx, scalar), mul(vy, scalar))
}

pub fn normalize_vector(vx: u64, vy: u64) -> (u64, u64) {
    let mag = magnitude(vx, vy);
    if mag == 0 { (0, 0) } else { (div(vx, mag), div(vy, mag)) }
}

pub fn rotate_vector(vx: u64, vy: u64, angle_deg_times_10: u32) -> (i64, i64) {
    let vx_i = vx as i64;
    let vy_i = vy as i64;
    let cos_a = cos(angle_deg_times_10);
    let sin_a = sin(angle_deg_times_10);
    let new_x = (vx_i.saturating_mul(cos_a) / 100) - (vy_i.saturating_mul(sin_a) / 100);
    let new_y = (vx_i.saturating_mul(sin_a) / 100) + (vy_i.saturating_mul(cos_a) / 100);
    (new_x, new_y)
}

pub fn reflect_vector(vx: u64, vy: u64, normal_x: u64, normal_y: u64) -> (i64, i64) {
    let dot_vn = dot_product(vx, vy, normal_x, normal_y);
    let change_x = (normal_x as i64).saturating_mul(2 * (dot_vn as i64)) / 100;
    let change_y = (normal_y as i64).saturating_mul(2 * (dot_vn as i64)) / 100;
    let reflected_vx = (vx as i64) - change_x;
    let reflected_vy = (vy as i64) - change_y;
    (reflected_vx, reflected_vy)
}

pub fn is_point_in_triangle(
    px: u64, py: u64,
    ax: u64, ay: u64,
    bx: u64, by: u64,
    cx: u64, cy: u64
) -> bool {
    let px_i = px as i64;
    let py_i = py as i64;
    let ax_i = ax as i64;
    let ay_i = ay as i64;
    let bx_i = bx as i64;
    let by_i = by as i64;
    let cx_i = cx as i64;
    let cy_i = cy as i64;

    let abx = bx_i - ax_i;
    let aby = by_i - ay_i;
    let apx = px_i - ax_i;
    let apy = py_i - ay_i;
    let cross_product1 = abx.saturating_mul(apy).saturating_sub(aby.saturating_mul(apx));

    let bcx = cx_i - bx_i;
    let bcy = cy_i - by_i;
    let bpx = px_i - bx_i;
    let bpy = py_i - by_i;
    let cross_product2 = bcx.saturating_mul(bpy).saturating_sub(bcy.saturating_mul(bpx));

    let cax = ax_i - cx_i;
    let cay = ay_i - cy_i;
    let cpx = px_i - cx_i;
    let cpy = py_i - cy_i;
    let cross_product3 = cax.saturating_mul(cpy).saturating_sub(cay.saturating_mul(cpx));

    let all_non_negative = cross_product1 >= 0 && cross_product2 >= 0 && cross_product3 >= 0;
    let all_non_positive = cross_product1 <= 0 && cross_product2 <= 0 && cross_product3 <= 0;

    all_non_negative || all_non_positive
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 { (b, 0, 1) } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn modinv(a: i64, m: i64) -> Option<i64> {
    if m == 0 { return None; }
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 { None } else { Some((x % m + m) % m) }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut d = n - 1;
    while d % 2 == 0 { d /= 2; }

    for &a in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == a { return true; }
        if modexp(a, d, n) != 1 {
            let mut t = modexp(a, d, n);
            let mut dt = d;
            let mut is_composite = true;
            while dt < n - 1 {
                if t == n - 1 { is_composite = false; break; }
                t = (t as u128 * t as u128 % n as u128) as u64;
                dt = dt.saturating_mul(2);
            }
            if is_composite { return false; }
        }
    }
    true
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    (a / gcd(a, b)).saturating_mul(b)
}

pub fn factorial(n: u64) -> Option<u64> {
    if n > 20 { return None; }
    let mut result = 1u64;
    for i in 2..=n { result = result.saturating_mul(i); }
    Some(result)
}

pub fn n_choose_k(n: u64, mut k: u64) -> u64 {
    if k > n { return 0; }
    if k == 0 || k == n { return 1; }
    if k > n / 2 { k = n - k; }
    let mut res: u64 = 1;
    for i in 0..k {
        res = res.saturating_mul(n - i) / (i + 1);
    }
    res
}

pub fn log2_floor(n: u64) -> Option<u32> {
    if n == 0 { None } else { Some(63 - n.leading_zeros()) }
}

pub fn log10_floor(mut n: u64) -> u32 {
    if n == 0 { return 0; }
    let mut count = 0;
    while n >= 10 { n /= 10; count += 1; }
    count
}

pub fn popcount(n: u64) -> u32 { n.count_ones() }
pub fn reverse_bits(n: u64) -> u64 { n.reverse_bits() }
pub fn phi(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 { result -= result / n; }
    result
}
pub fn rotl64(n: u64, k: u32) -> u64 { n.rotate_left(k) }
pub fn rotr64(n: u64, k: u32) -> u64 { n.rotate_right(k) }

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut result = 0;
    for (x, y) in a.iter().zip(b) { result |= x ^ y; }
    result == 0
}

pub fn clmul(a: u64, b: u64) -> u128 {
    let mut res: u128 = 0;
    for i in 0..64 {
        if (b >> i) & 1 == 1 { res ^= (a as u128) << i; }
    }
    res
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Coordinate { x: u64, y: u64 },
}

pub fn point_double(p: Point, a: u64, modulus: u64) -> Option<Point> {
    match p {
        Point::Infinity => Some(Point::Infinity),
        Point::Coordinate { x, y } => {
            if y == 0 || modulus == 0 { return Some(Point::Infinity); }
            let two_y_inv = match modinv(2 * y as i64, modulus as i64) {
                Some(val) => val as u64,
                None => return None,
            };
            let three_x_sq = modexp(x, 2, modulus).saturating_mul(3) % modulus;
            let lambda = ((three_x_sq.saturating_add(a)) % modulus).saturating_mul(two_y_inv) % modulus;
            let lambda_sq = modexp(lambda, 2, modulus);
            let two_x = (2 * x) % modulus;
            let x_r = (lambda_sq + modulus - two_x) % modulus;
            let y_r = (lambda.saturating_mul((x + modulus - x_r) % modulus) % modulus + modulus - y) % modulus;
            Some(Point::Coordinate { x: x_r, y: y_r })
        }
    }
}

pub fn point_add(p1: Point, p2: Point, a: u64, modulus: u64) -> Option<Point> {
    if modulus == 0 { return None; }
    match (p1, p2) {
        (Point::Infinity, _) => Some(p2),
        (_, Point::Infinity) => Some(p1),
        (Point::Coordinate { x: x1, y: y1 }, Point::Coordinate { x: x2, y: y2 }) => {
            if x1 == x2 {
                if y1 == y2 { return point_double(p1, a, modulus); }
                else { return Some(Point::Infinity); }
            }
            let x_diff = (x2 + modulus - x1) % modulus;
            let x_diff_inv = match modinv(x_diff as i64, modulus as i64) {
                Some(val) => val as u64,
                None => return None,
            };
            let y_diff = (y2 + modulus - y1) % modulus;
            let lambda = y_diff.saturating_mul(x_diff_inv) % modulus;
            let lambda_sq = modexp(lambda, 2, modulus);
            let x_sum = (x1 + x2) % modulus;
            let x_r = (lambda_sq + modulus - x_sum) % modulus;
            let y_r = (lambda.saturating_mul((x1 + modulus - x_r) % modulus) % modulus + modulus - y1) % modulus;
            Some(Point::Coordinate { x: x_r, y: y_r })
        }
    }
}
/// Fixed-point multiplication for two signed i64 values, scaled by 100.
fn mul_signed(a: i64, b: i64) -> i64 {
    (a as i128 * b as i128 / 100) as i64
}

/// Fixed-point division for two signed i64 values, scaled by 100.
fn div_signed(a: i64, b: i64) -> i64 {
    if b == 0 { return i64::MAX; } // Return max as error indicator
    (a as i128 * 100 / b as i128) as i64
}

/// Fixed-point square for a signed i64 value, scaled by 100.
fn square_signed(a: i64) -> i64 {
    (a as i128 * a as i128 / 100) as i64
}
pub fn get_projectile_trajectory_coefficients(
    angle_deg_times_10: u32,
    initial_velocity: u64,
    gravity: u64,
) -> (i64, i64) {
    let sin_val = sin(angle_deg_times_10);
    let cos_val = cos(angle_deg_times_10);

    let c1 = {
        if cos_val == 0 { i64::MAX } 
        else { div_signed(sin_val, cos_val) }
    };

    let c2 = {
        let v0_squared = square(initial_velocity);
        let cos_squared = square_signed(cos_val);
        let denom_part1 = mul_signed(v0_squared as i64, cos_squared);
        let denominator = denom_part1.saturating_mul(2);

        if denominator == 0 { i64::MIN } 
        else { div_signed(-(gravity as i64), denominator) }
    };
    
    if c1 == i64::MAX { (c1, 0) } 
    else { (c1, c2) }
}

#[derive(Clone)]
pub struct Xorshift64Star { state: u64, }
impl Xorshift64Star {
    pub fn new(seed: u64) -> Self {
        if seed == 0 { Xorshift64Star { state: 1 } }
        else { Xorshift64Star { state: seed } }
    }
    pub fn next(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545F4914F6CDD1D)
    }
}
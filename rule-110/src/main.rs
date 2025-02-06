use std::collections::HashMap;

// converts bits to string, then replaces 1's with *'s and 0's with .'s and prints it
fn first_row(bits: u8) -> u8 {
    // converts bits to string
    let mut first: String = format!("{:08b}", bits); //https://doc.rust-lang.org/std/fmt/index.html#fillalignment

    // replaces 1's with *'s and 0's with .'s
    first = first.replace("1", "*");
    first = first.replace("0", ".");
    println!("{}", first);

    bits
}

// HashMap to store Rule 110 conversions
fn convert(bits: &str) -> Option<String> {
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    let rules: HashMap<String, String> = HashMap::from([
        ("111".to_string(), "0".to_string()),
        ("110".to_string(), "1".to_string()),
        ("101".to_string(), "1".to_string()),
        ("100".to_string(), "0".to_string()),
        ("011".to_string(), "1".to_string()),
        ("010".to_string(), "1".to_string()),
        ("001".to_string(), "1".to_string()),
        ("000".to_string(), "0".to_string()),
    ]);

    rules.get(bits).cloned()
}

// computes next row of bits using Rule 110
fn rule110(bits: u8) -> u8 {
    // convert bits to string
    let mut bit_str: String = format!("{:08b}", bits); //https://doc.rust-lang.org/std/fmt/index.html#fillalignment

    // add the first bit to the front, and last bit to the end for easier computation
    let first = bit_str[0..1].to_string();
    let last = bit_str[7..8].to_string();
    let extended = format!("{}{}{}", last, bit_str, first);

    // create new string to store new row's bits
    let mut new = String::new();

    // https://doc.rust-lang.org/rust-by-example/flow_control/for.html

    // loop through to get the new bits for the new row
    for i in 0..8 {
        // go through all 8 bits of original line
        // get current plus next two bits for conversion
        let curr = &extended[i..i + 3];
        // convert the 3 bits from original line, to the new bit for new line
        let new_bit: Option<String> = convert(curr);
        new.push_str(&new_bit.unwrap());
    }

    bit_str = new.clone();

    // https://doc.rust-lang.org/std/primitive.u8.html#method.from_str_radix
    // found from_str_radix from chat GPT
    // convert new row's bits back to u8 to follow assignment's function signature of u8 -> u8
    let new_bits = u8::from_str_radix(&bit_str, 2).unwrap();

    //https://doc.rust-lang.org/std/primitive.str.html#method.replace
    // replace 1's and 0's with *'s and .'s
    new = new.replace("1", "*");
    new = new.replace("0", ".");
    println!("{}", new);

    new_bits
}

// run w cargo test
#[test]
fn test_1st_row() {
    assert_eq!(first_row(0b10100100), 0b10100100);
}

#[test]
fn test_convert() {
    assert_eq!(convert("111"), Some("0".to_string()));
    assert_eq!(convert("110"), Some("1".to_string()));
    assert_eq!(convert("101"), Some("1".to_string()));
    assert_eq!(convert("100"), Some("0".to_string()));
    assert_eq!(convert("011"), Some("1".to_string()));
    assert_eq!(convert("010"), Some("1".to_string()));
    assert_eq!(convert("001"), Some("1".to_string()));
    assert_eq!(convert("000"), Some("0".to_string()));
    assert_eq!(convert("123)"), None);
}

#[test]
fn test_2nd_row() {
    assert_eq!(rule110(0b10100100), 0b11101101);
}

#[test]
fn test_3rd_row() {
    assert_eq!(rule110(0b11101101), 0b00111111);
}

#[test]
fn test_4th_row() {
    assert_eq!(rule110(0b00111111), 0b01100001);
}

fn main() {
    // do first row
    first_row(0b10100100);
    let mut bits = 0b10100100;
    // do the next i rows, (currently 9)
    for _i in 0..9 {
        bits = rule110(bits);
    }
}

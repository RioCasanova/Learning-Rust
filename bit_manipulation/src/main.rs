/*
    Author: Rio Casanova
    Date: January 14, 2025

    Note:

        This will be confusing if not familiar with bitwise operations, bias and
        bitmasking.

    Purpose:

        Grow understanding of unpacking bytes/bits to understand
        the underlying architecture of how data is stored in
        memory, and deal with untyped bits.

    Summary

        We are going to isolate/extract the sign, exponent, and mantissa of a
        given floating point number, decode each value from its raw bit pattern to its value,
        convert from sci-notation to a ordinary number.

*/

fn main() {
    // ISOLATING THE SIGN BIT FROM AN F32
    // f32: [sign (1 bit)][exponent (8 bits)][mantissa (23 bits)]
    // f32 of 42.42: 0 | 10000100 | 01010010101000000000000 (n)
    // u32 of 42.42: 01000010 00101001 01010000 00000000 (n_bits)
    let n: f32 = 42.42;

    // interprets f32 bits as a u32 for manipulation - ONLY WAY
    // u32: 01000010 00101001 01010000 00000000
    let n_bits: u32 = n.to_bits();

    // shift bits 31 places right
    // after move: 00000000 00000000 00000000 00000000
    // its less of a 'move' and more of a 'push', because the entire
    // number shifts to isolate just the sign.
    let sign_bit = n_bits >> 31;

    let sign = if sign_bit == 0 {
        "positive"
    } else {
        "negative"
    };

    // The original number itself (mantissa, exponent, etc.)
    // is lost in this operation. You're not "moving the number";
    // you're specifically isolating the sign bit by discarding
    // everything else.

    // f32 can store decimals and u32 can only store whole numbers:
    // yes - in this instance though we only care about the sign bit

    // I can isolate the sign of an f32 without converting it to a u32:
    // yes, however you typically need to access the raw binary
    // representation of the floating point number to perform bitwise
    // manipulation - not just reading

    // ISOLATING THE EXPONENT - we are using 'n' and 'n_bits' from above
    // since 'n_bits' is prepared for manipulation.

    // This requires two bit manipulations - exponent isolation
    // - overwrite mantissa's bits: >> 23
    // - use AND mask (& 0xff) to exclude the sign bit

    let exponent_ = n_bits >> 23; // here is the overwrite of mantissa

    // bitwise AND operation that masks all but the least significant 8 bits
    // of the value stored in exponent_

    // 0xff is a hexadecimal value equivalent to 11111111 in binary,
    // which is 8 bits with all bits set to 1.

    // The & (bitwise AND) operator compares each corresponding
    // bit of the two operands (in this case, exponent_ and 0xff

    let exponent_ = exponent_ & 0xff; // sign exclusion

    // interpret exponent bits as a signed int and subtract bias (127)
    // as defined by the standard

    // a bias is a value subtracted from the exponent to allow for both
    // positive and negative exponents in a way that simplifies the hardware
    // representation.

    // This means the exponent is represented as an unsigned 8-bit integer,
    // and the actual exponent value is obtained by subtracting 127 from it.
    // This ensures that the exponent can represent both positive and negative
    // values, even though it's stored as an unsigned value.

    let exponent = (exponent_ as i32) - 127;
    /*
       Converts the exponent to a signed integer (i32) and then subtracts
       the bias (127) to get the actual exponent value in base 2, which can be
       positive or negative.

       Without subtracting the bias, you would not get the correct exponent
       value in the standard floating-point representation.
    */

    // ISOLATE THE MANTISSA - multiply each bit by its weight and sum result
    // First bit-weight: 0.5
    // Each subsequent bit weight: X = prevW * 0.5
    // Second bit-weight: .25 = 0.5 * 0.5
    let n_bits: u32 = n.to_bits();

    // The Mantissa: is 24 bits long, 23 of which are explicit, and 1 that is implicit
    // in memory there is only room for 23 bits but when interpreted by the system,
    // it accounts for an additional value or 24th bit with a value of 1.0
    // with most floating point integers - whatever number is in the 23 bits,
    // the system will put 1. in front of it in most cases
    // Bit Weight: value of each bit based on its positioning
    // base-2: each bit has a weight that is a power of 2
    // Bit Positioning: Rightmost (least weight), Leftmost (most weight), each position doubles in weight
    /*
       binary number: 1101

       1: 2^0 = 1
       0: 2^1 = 0 (because 0)
       1: 2^2 = 4
       1: 2^3 = 8
       value of 1101: 13 (integer)

       basically like exponents of 2


    */

    // This represents the implicit 24th bit interpreted by
    // the system - everything else will be calculated as a small decimal
    // that will be added to 1.0
    let mut mantissa: f32 = 1.0; // 2 to pow 0

    // 0 indexed - 23 bits in mantissa
    for i in 0..23 {
        // takes 1, in binary 00000000 00000000 00000000 00000001
        // moves it left 'i' spaces [0...0010, 0...0100, 0...1000]
        let mask = 1 << i;

        // Compares the binary representation of our f32 value to bit postion of
        // 'mask' - returns 1 if both operands contain 1 in the same position
        // otherwise 0 - should return u32 with 1 bit position with '1' or none
        let one_at_bit_i = n_bits & mask;
        // if there is a value / the AND found a bit value in common
        // take the position that evaluated to a value, coerce to use subtraction
        // calculate weight and add value to mantissa
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    let calc_exp = 2.0_f32.powf(exponent as f32);
    let value = 1.0 * mantissa * 2.0_f32.powf(exponent as f32);
    println!(
        "Sign: {}, Exponent: {}, Mantissa(Significand): {}, Radix: 2",
        sign, exponent, mantissa
    );
    println!(
        "{} 1 x {} x (radix)2^{} (or {}) = {}",
        sign, mantissa, exponent, calc_exp, value
    )
}

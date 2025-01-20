mod hexdump;
mod hexdump_iter;
mod serialize_eg;

fn main() {
    serialize_eg::run_serialize();
    hexdump::run_hexdump();
    hexdump_iter::run_hexdump_iter();
}

use bincode::serialize as to_bincode; // <1>
use serde_cbor::to_vec as to_cbor; // <1>
use serde_derive::Serialize;
use serde_json::to_string as to_json; // <1>

#[derive(Serialize)] // <2>
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

pub fn run_serialize() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    // Converting the calabar var to a JSON encoded string
    // bincode and CBOR are also formats - but harder to read
    let as_json = to_json(&calabar).unwrap(); // <3>
    let as_cbor = to_cbor(&calabar).unwrap(); // <3>
    let as_bincode = to_bincode(&calabar).unwrap(); // <3>

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!(
        "json (as UTF-8):\n{}\n",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!(
        "cbor (as UTF-8):\n{:?}\n",
        String::from_utf8_lossy(&as_cbor)
    );
    println!(
        "bincode (as UTF-8):\n{:?}\n",
        String::from_utf8_lossy(&as_bincode)
    );
}

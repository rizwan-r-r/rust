#![deny(uncommon_codepoints)]

const µ: f64 = 0.000001; //~ ERROR identifier contains an uncommon Unicode codepoint
//~| WARNING should have an upper case name

fn dĳkstra() {} //~ ERROR identifier contains an uncommon Unicode codepoint

fn main() {
    let ㇻㇲㇳ = "rust"; //~ ERROR identifier contains uncommon Unicode codepoints

    // using the same identifier the second time won't trigger the lint.
    println!("{}", ㇻㇲㇳ);
}

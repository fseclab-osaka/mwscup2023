extern crate regex;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct G1Affine {
    x: String,
    y: String,
    infinity: String,
}

#[derive(Debug, Clone)]
struct G2Affine {
    x: String,
    y: String,
    infinity: String,
}

#[derive(Debug, Clone)]
struct Proof {
    a: G1Affine,
    b: G2Affine,
    c: G1Affine,
}

fn parse_affine(text: &str) -> HashMap<String, String> {
    let re = Regex::new(r"(\w+):\s*(0x[0-9a-fA-F]+|\w+\(\d+\))").unwrap();
    let mut map = HashMap::new();
    for cap in re.captures_iter(text) {
        map.insert(cap[1].to_string(), cap[2].to_string());
    }
    map
}

fn parse_proof(input: &str) -> Option<Proof> {
    let re = Regex::new(r"a:\s*G1Affine\s*\{(.*?)\},\s*b:\s*G2Affine\s*\{(.*?)\},\s*c:\s*G1Affine\s*\{(.*?)\}").unwrap();
    let captures = re.captures(input)?;

    let a_data = parse_affine(&captures[1]);
    let a = G1Affine {
        x: a_data.get("x")?.clone(),
        y: a_data.get("y")?.clone(),
        infinity: a_data.get("infinity")?.clone(),
    };

    let b_data = parse_affine(&captures[2]);
    let b = G2Affine {
        x: b_data.get("x")?.clone(),
        y: b_data.get("y")?.clone(),
        infinity: b_data.get("infinity")?.clone(),
    };

    let c_data = parse_affine(&captures[3]);
    let c = G1Affine {
        x: c_data.get("x")?.clone(),
        y: c_data.get("y")?.clone(),
        infinity: c_data.get("infinity")?.clone(),
    };

    Some(Proof { a, b, c })
}

fn main() {
    let input = "Proof { a: G1Affine { x: 0x0ed5be380302722a, y: 0x04068866d9f17f77, infinity: Choice(0) }, b: G2Affine { x: 0x084e975645c08d32, y: 0x08d1edc621c76727, infinity: Choice(0) }, c: G1Affine { x: 0x160cc95dad7f0cb5, y: 0x052d208e0f4ea5aa, infinity: Choice(0) } }";
    let proof = parse_proof(input);
    
    match parse_proof(input) {
        Some(proof) => println!("{:?}", proof),
        None => println!("Failed to parse input."),
    }
    println!("{:?}", proof.unwrap().a.x);
}


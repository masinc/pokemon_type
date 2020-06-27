use itertools::Itertools;
use pokemon_type::types::Type;

use std::env;

fn list_types() {
    let types = Type::types();
    let c: Vec<(Vec<&Type>, f64)> = types
        .iter()
        .combinations(3)
        .into_iter()
        .map(|t| {
            (
                t.clone(),
                Type::blocks_rates(&t)
                    .into_iter()
                    .fold(0.0, |a, (_, b)| a + if b == 0.0 { b } else { b.log2() }),
            )
        })
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap())
        .collect();
    println!("types: {}", c.len());
    for x in c {
        println!("{:?} = {}", x.0, x.1);
    }
}

fn show_type(types: &[String]) {
    let types: Vec<_> = types
        .iter()
        .map(|ty| Type::from_str(ty).unwrap().clone())
        .collect();
    let types: Vec<_> = types.iter().map(|ty| ty).collect();
    println!("{:#?}", Type::blocks_rates(&types[..]));
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() == 0 {
        list_types();
    } else {
        show_type(&args[..])
    }
}

use bignum::*;
fn add_ix(a: &ix, b: &ix) -> ix{
    b_v = b.vals.clone();
    a_v = a.values.clone();
    a_s = a.sign.clone();
    b_s = b.sign.clone();
    for v in b_v{
        println!('line');
    }
}
fn sub_ix(a: &ix, b: &ix) -> ix{
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    return add_ix(&a, &b);
}



fn mul_ix(a: &ix, b: &ix) -> ix{
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a = h2i_ix(&args[1]);
    let b = h2i_ix(&args[2]);
    match args[3].as_str() {
        "ADD" => see_ix(&add_ix(&a, &b)),
        "SUB" => see_ix(&sub_ix(&a, &b)),
        "MUL" => todo!(),
        "DIV" => todo!(),
        "REM" => todo!(),
        &_    => println!("Operator not recognized: choose from ADD, SUB, MUL, DIV, REM"),
    }
}

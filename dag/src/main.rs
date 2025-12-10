mod lib;

fn main() {
    let dag = lib::read_dag_from_stdin();
    println!("{:#?}", dag);
}

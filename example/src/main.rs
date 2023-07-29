fn main() {
    println!("{}", foxes::fox().unwrap());
    println!("{}", foxes::fox().unwrap().width(150));
    println!("{}", foxes::fox().unwrap().width(150).height(150));
}

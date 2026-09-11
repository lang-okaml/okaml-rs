use okaml::Okml;

// Driver Code
fn main() {
    let file_path = "../example/syntax.okml";
    let parsed_ast:Vec<Okml> = Okml::read_from_file(file_path);
    println!("{:?}", parsed_ast);
}

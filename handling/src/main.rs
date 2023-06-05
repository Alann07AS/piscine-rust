use std::fs::File;
use std::io::Read;
use std::io::Write;

// use std::io::Write;
use handling::*;

fn main() {
    let path = "a.txt";
    File::create(path).unwrap();
    open_or_create(path, "content to be written");

    let mut file = File::open(path).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
// fn main() {
//     let mut file = match File::open("file.txt") {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Erreur lors de la création du fichier : {}", error);
//             return;
//         }
//     };

//     let content = "Contenu à écrire dans le fichier";

//     match file.write_all(content.as_bytes()) {
//         Ok(_) => println!("Écriture réussie dans le fichier"),
//         Err(error) => eprintln!("Erreur lors de l'écriture dans le fichier : {}", error),
//     }
// }
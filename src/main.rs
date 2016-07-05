use std::io ///

fn main() {

	println!("Escoge un numero");
	println!("Por favor introduzca su eleccion");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess).expect("Error al leer la linea");

	println!("Su eleccion: {}", guess);
}
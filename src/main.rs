use std::io::{self, Write};
use std::str::FromStr;

enum Operation {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Potencia,
    RaizCuadrada,
    Modulo,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "+" | "suma"          => Ok(Operation::Suma),
            "-" | "resta"         => Ok(Operation::Resta),
            "*" | "multiplicacion"=> Ok(Operation::Multiplicacion),
            "/" | "division"      => Ok(Operation::Division),
            "^" | "potencia"      => Ok(Operation::Potencia),
            "√" | "raiz" | "raizcuadrada" => Ok(Operation::RaizCuadrada),
            "%" | "modulo"        => Ok(Operation::Modulo),
            _                     => Err(()),
        }
    }
}

fn main() {
    let operation = read_operation();
    let result = match operation {
        Operation::RaizCuadrada => {
            let num = read_num("Ingresa el número: ");
            raiz_cuadrada(num)
        },
        _ => {
            let num1 = read_num("Ingresa Valor 1: ");
            let num2 = read_num("Ingresa Valor 2: ");
            match operation {
                Operation::Suma            => sum(num1, num2),
                Operation::Resta           => rest(num1, num2),
                Operation::Multiplicacion  => mult(num1, num2),
                Operation::Division        => divide(num1, num2),
                Operation::Potencia        => potencia(num1, num2),
                Operation::Modulo          => modulo(num1, num2),
                _ => unreachable!(),
            }
        }
    };

    println!("Resultado: {}", result);
}

fn sum(a: f64, b: f64) -> f64 {
    a + b
}

fn rest(a: f64, b: f64) -> f64 {
    a - b
}

fn mult(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: División por cero no permitida.");
        std::process::exit(1);
    }
    a / b
}

fn potencia(a: f64, b: f64) -> f64 {
    a.powf(b)
}

fn raiz_cuadrada(a: f64) -> f64 {
    if a < 0.0 {
        println!("Error: No se puede calcular la raíz cuadrada de un número negativo.");
        std::process::exit(1);
    }
    a.sqrt()
}

fn modulo(a: f64, b: f64) -> f64 {
    a % b
}


fn read_operation() -> Operation {
    loop {
        let mut entrada = String::new();
        println!("Ingresa Operacion\n(+, -, *, /, ^, raiz, %): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");

        match entrada.trim().parse() {
            Ok(op) => return op,
            Err(_) => println!("Operación no válida. Revisa e intenta de nuevo."),
        }
    }
}


fn read_num(message: &str) -> f64 {
    loop {
        let mut entrada = String::new();
        print!("{}", message);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        match entrada.trim().parse() {
            Ok(num) => return num,
            Err(_)  => println!("Número no válido, intenta otra vez."),
        }
    }
}

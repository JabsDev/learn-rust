fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn maior_numero(vetor: &[i32]) -> Option<&i32> {
    vetor.iter().max()
}

fn contador_vogais(texto: &str) -> usize {
    texto.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    fn area(self) -> u32 {
        self.largura * self.altura
    }
}

fn quadrado(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n * n),
        None => None,
    }
}

enum Forma {
    Circulo(f64),
    Retangulo(f64, f64),
}

fn area(forma: Forma) -> f64 {
    match forma {
        Forma::Circulo(raio) => 3.14 * raio * raio,
        Forma::Retangulo(larg, alt) => larg * alt,
    }
}

fn celsius_para_fahrenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
fn fahrenheit_para_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn main() {
    println!("Soma: {}", soma(4, 5));

    let numeros = vec![10, -100, 5, 33];
    match maior_numero(&numeros) {
        Some(max) => println!("Maior: {}", max),
        None => println!("Vetor Vazio"),
    }

    let palavra = "Eu dou o cuzinho gostoso";
    println!("Vogais: {}", contador_vogais(&palavra));

    let ret = Retangulo {
        largura: 10,
        altura: 5,
    };
    println!("Area: {}", ret.area());

    let num = Some(4);
    println!("Quadrado: {:?}", quadrado(num));

    let circulo = Forma::Circulo(3.0);
    println!("Area do circulo: {}", area(circulo));

    let celsius = 24.0;
    let fahrenheit = 86.0;
    println!(
        "Temperatura {}°F {}°C",
        celsius_para_fahrenheit(celsius),
        fahrenheit_para_celsius(fahrenheit)
    );
}

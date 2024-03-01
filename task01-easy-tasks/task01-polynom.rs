use std::io;

// Функция для ввода многочлена
fn input_polynomial() -> Vec<f64> {
    println!("Введите коэффициенты многочлена (начиная с коэффициента при старшей степени):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");

    let coefficients: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Ошибка при преобразовании в число"))
        .collect();

    coefficients
}

// Функция для вывода многочлена
fn print_polynomial(polynomial: &[f64]) {
    let mut first_term = true;
    for (index, &coeff) in polynomial.iter().enumerate() {
        if coeff != 0.0 {
            if !first_term {
                print!(" + ");
            }
            print!("{}", coeff);
            if index > 0 {
                print!("x^{}", index);
            }
            first_term = false;
        }
    }
    println!();
}

// Функция для сложения двух многочленов
fn add_polynomials(p1: &[f64], p2: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(p1.len().max(p2.len()));

    for i in 0..result.capacity() {
        let coeff1 = if i < p1.len() { p1[i] } else { 0.0 };
        let coeff2 = if i < p2.len() { p2[i] } else { 0.0 };
        result.push(coeff1 + coeff2);
    }

    result
}

// Функция для вычитания многочленов
fn subtract_polynomials(p1: &[f64], p2: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(p1.len().max(p2.len()));

    for i in 0..result.capacity() {
        let coeff1 = if i < p1.len() { p1[i] } else { 0.0 };
        let coeff2 = if i < p2.len() { p2[i] } else { 0.0 };
        result.push(coeff1 - coeff2);
    }

    result
}

// Функция для умножения многочленов
fn multiply_polynomials(p1: &[f64], p2: &[f64]) -> Vec<f64> {
    let mut result = vec![0.0; p1.len() + p2.len() - 1];

    for (i, &coeff1) in p1.iter().enumerate() {
        for (j, &coeff2) in p2.iter().enumerate() {
            result[i + j] += coeff1 * coeff2;
        }
    }

    result
}

// Функция для вычисления производной многочлена
fn differentiate_polynomial(p: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(p.len() - 1);

    for (index, &coeff) in p.iter().enumerate().skip(1) {
        result.push(coeff * index as f64);
    }

    result
}

// Функция для вычисления значения многочлена в заданной точке
fn evaluate_polynomial(p: &[f64], x: f64) -> f64 {
    p.iter()
        .enumerate()
        .map(|(index, &coeff)| coeff * x.powi(index as i32))
        .sum()
}

fn main() {
    println!("Программа для работы с многочленами");

    let poly1 = input_polynomial();
    let poly2 = input_polynomial();

    println!("Первый многочлен:");
    print_polynomial(&poly1);

    println!("Второй многочлен:");
    print_polynomial(&poly2);

    let sum = add_polynomials(&poly1, &poly2);
    println!("Сумма многочленов:");
    print_polynomial(&sum);

    let difference = subtract_polynomials(&poly1, &poly2);
    println!("Разность многочленов:");
    print_polynomial(&difference);

    let product = multiply_polynomials(&poly1, &poly2);
    println!("Произведение многочленов:");
    print_polynomial(&product);

    let derivative = differentiate_polynomial(&poly1);
    println!("Производная первого многочлена:");
    print_polynomial(&derivative);

    let x = 2.0; // Значение x для вычисления многочлена
    let value_at_x = evaluate_polynomial(&poly1, x);
    println!("Значение первого многочлена при x = {}: {}", x, value_at_x);
}

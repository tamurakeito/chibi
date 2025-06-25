use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Print(Box<Expr>),
}

// 字句解析: 入力をトークンに変換
fn tokenize(input: &str) -> Vec<String> {
    let mut spaced = String::new();
    for c in input.chars() {
        if "()+-*/".contains(c) {
            spaced.push(' ');
            spaced.push(c);
            spaced.push(' ');
        } else {
            spaced.push(c);
        }
    }
    spaced
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// 構文解析（中置記法）
fn parse_expr(tokens: &mut Vec<String>) -> Expr {
    parse_add_sub(tokens)
}

fn parse_add_sub(tokens: &mut Vec<String>) -> Expr {
    let mut expr = parse_mul_div(tokens);
    while let Some(op) = tokens.first() {
        if op == "+" || op == "-" {
            let op = tokens.remove(0);
            let rhs = parse_mul_div(tokens);
            expr = match op.as_str() {
                "+" => Expr::Add(Box::new(expr), Box::new(rhs)),
                "-" => Expr::Sub(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
        } else {
            break;
        }
    }
    expr
}

// トークン列から*,/を解析して優先的に処理
fn parse_mul_div(tokens: &mut Vec<String>) -> Expr {
    let mut expr = parse_primary(tokens);
    while let Some(op) = tokens.first() {
        if op == "*" || op == "/" {
            let op = tokens.remove(0);
            let rhs = parse_primary(tokens);
            expr = match op.as_str() {
                "*" => Expr::Mul(Box::new(expr), Box::new(rhs)),
                "/" => Expr::Div(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
        } else {
            break;
        }
    }
    expr
}

// 基本単位の解析
// ()の中を再帰的に解析する
fn parse_primary(tokens: &mut Vec<String>) -> Expr {
    let token = tokens.remove(0);
    if token == "print" {
        assert_eq!(tokens.remove(0), "(");
        let expr = parse_expr(tokens);
        assert_eq!(tokens.remove(0), ")");
        Expr::Print(Box::new(expr))
    } else if  token == "(" {
        let expr = parse_expr(tokens);
        assert_eq!(tokens.remove(0), ")");
        expr
    } else {
        Expr::Num(token.parse().unwrap())
    }
}

// 評価: ASTを計算
fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Div(a, b) => eval(a) / eval(b),
                Expr::Print(e) => {
            let v = eval(e);
            println!("{}", v);
            v
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "run" {
        let filename = &args[2];
        let content = fs::read_to_string(filename).expect("ファイル読み込み失敗");
        let mut tokens = tokenize(&content);
        let ast = parse_expr(&mut tokens);
        let result = eval(&ast);
        println!("= {}", result);
    } else {
        println!("使い方: chibi run <filename.chibi>");
    }
}

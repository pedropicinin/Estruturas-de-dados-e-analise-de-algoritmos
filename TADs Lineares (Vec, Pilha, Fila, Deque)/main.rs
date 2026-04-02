use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    ex1();
}

//
// ===== Grupo 1 =====
//

fn ex1() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut inv = Vec::new();

    while let Some(x) = v.pop() {
        inv.push(x);
    }

    println!("{:?}", inv);
}

fn ex2() {
    let texto = "banana";
    let mut contagem = std::collections::HashMap::new();

    for c in texto.chars() {
        *contagem.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", contagem);
}

fn ex3() {
    let mut v = vec![1,2,3,4,5,6];
    let mut i = 0;

    while i < v.len() {
        if v[i] % 2 == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }

    println!("{:?}", v);
}

fn ex4() {
    let a = vec![1,3,5];
    let b = vec![2,4,6];

    let mut c = Vec::new();
    c.extend(a);
    c.extend(b);
    c.sort();

    println!("{:?}", c);
}

//
// ===== Grupo 2 =====
//

fn ex5() {
    let expr = "3 4 + 2 *";
    let mut pilha: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a + b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a * b);
            }
            _ => pilha.push(token.parse().unwrap())
        }
    }

    println!("{}", pilha.pop().unwrap());
}

fn ex6() {
    let mut back: Vec<String> = Vec::new();
    let mut forward: Vec<String> = Vec::new();

    back.push("google".into());
    back.push("youtube".into());

    let atual = back.pop().unwrap();
    forward.push(atual);

    println!("{:?}", back);
}

fn ex7() {
    let mut texto = String::new();
    let mut undo: Vec<String> = Vec::new();

    texto.push_str("oi");
    undo.push(texto.clone());

    texto.push_str(" tudo bem");
    undo.push(texto.clone());

    texto = undo.pop().unwrap();

    println!("{}", texto);
}

fn ex8() {
    let s = "{[()]}";
    let mut pilha = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => pilha.push(c),
            ')' => if pilha.pop() != Some('(') { println!("erro"); return; },
            ']' => if pilha.pop() != Some('[') { println!("erro"); return; },
            '}' => if pilha.pop() != Some('{') { println!("erro"); return; },
            _ => {}
        }
    }

    println!("ok");
}

fn ex9() {
    let mut pilha = Vec::new();
    let mut min = Vec::new();

    pilha.push(3);
    min.push(3);

    pilha.push(1);
    min.push(1);

    println!("{:?}", min.last());
}

//
// ===== Grupo 3 =====
//

fn ex10() {
    let inicio = Instant::now();

    let mut fila: VecDeque<i32> = VecDeque::new();

    for i in 0..10 {
        fila.push_back(i);
    }

    while let Some(_) = fila.pop_front() {}

    println!("{:?}", inicio.elapsed());
}

fn ex11() {
    let mut fila = VecDeque::new();

    fila.push_back(("doc1", 5));
    fila.push_back(("doc2", 2));

    while let Some((nome, paginas)) = fila.pop_front() {
        println!("{} - {} páginas", nome, paginas);
    }
}

fn ex12() {
    let mut fila: VecDeque<i32> = VecDeque::new();
    let capacidade = 3;

    for i in 1..6 {
        if fila.len() == capacidade {
            fila.pop_front();
        }
        fila.push_back(i);
    }

    println!("{:?}", fila);
}

fn ex13() {
    let mut fila = vec![(1, "baixa"), (3, "alta"), (2, "media")];

    fila.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{:?}", fila);
}

//
// ===== Grupo 4 =====
//

fn ex14() {
    let s = "A man a plan a canal Panama";
    let mut d: VecDeque<char> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    while d.len() > 1 {
        if d.pop_front() != d.pop_back() {
            println!("não é");
            return;
        }
    }

    println!("é palíndromo");
}

fn ex15() {
    let v = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;

    for i in 0..=v.len()-k {
        let mut max = v[i];
        for j in i..i+k {
            if v[j] > max {
                max = v[j];
            }
        }
        println!("{}", max);
    }
}

fn ex16() {
    let mut d = VecDeque::new();

    d.push_back("normal");
    d.push_front("urgente");

    while let Some(t) = d.pop_front() {
        println!("{}", t);
    }
}

//
// ===== Grupo 5 =====
//

fn ex17() {
    let inicio = Instant::now();
    let mut v = Vec::new();

    for i in 0..10000 {
        v.push(i);
    }

    while !v.is_empty() {
        v.remove(0);
    }

    println!("{:?}", inicio.elapsed());
}

fn ex18() {
    println!("a) Pilha");
    println!("b) Fila");
    println!("c) Pilha");
    println!("d) Fila");
    println!("e) Deque");
}

fn ex19() {
    let mut fila: VecDeque<i32> = (1..10).collect();
    let lote = 3;

    while !fila.is_empty() {
        for _ in 0..lote {
            if let Some(x) = fila.pop_front() {
                print!("{} ", x);
            }
        }
        println!();
    }
}

fn ex20() {
    let mut fila: VecDeque<(i32, i32)> = VecDeque::new();

    fila.push_back((1, 5));
    fila.push_back((2, 3));

    let quantum = 2;

    while let Some((id, mut tempo)) = fila.pop_front() {
        if tempo > quantum {
            tempo -= quantum;
            fila.push_back((id, tempo));
        } else {
            println!("processo {} terminou", id);
        }
    }
}

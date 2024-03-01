use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Структура узла декартового дерева
#[derive(Debug)]
struct Node {
    key: i32,
    priority: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, priority: i32) -> Self {
        Node {
            key,
            priority,
            left: None,
            right: None,
        }
    }
}

// Вставка нового узла в декартово дерево
fn insert(root: Option<Box<Node>>, key: i32, priority: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node::new(key, priority))),
        Some(mut node) => {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    node.left = insert(node.left, key, priority);
                }
                Ordering::Greater => {
                    node.right = insert(node.right, key, priority);
                }
                Ordering::Equal => {} // Уже есть такой ключ
            }
            Some(node)
        }
    }
}

// Удаление узла из декартового дерева
fn delete(root: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
    match root {
        None => None,
        Some(mut node) => {
            match key.cmp(&node.key) {
                Ordering::Less => {
                    node.left = delete(node.left, key);
                }
                Ordering::Greater => {
                    node.right = delete(node.right, key);
                }
                Ordering::Equal => {
                    return merge(node.left, node.right);
                }
            }
            Some(node)
        }
    }
}

// Объединение двух декартовых деревьев
fn merge(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
    match (left, right) {
        (None, t) => t,
        (t, None) => t,
        (Some(mut left_node), Some(mut right_node)) => {
            if left_node.priority < right_node.priority {
                right_node.left = merge(Some(left_node), right_node.left);
                Some(right_node)
            } else {
                left_node.right = merge(left_node.right, Some(right_node));
                Some(left_node)
            }
        }
    }
}

// Поиск элемента в декартовом дереве
fn search(root: &Option<Box<Node>>, key: i32) -> Option<&Node> {
    match root {
        None => None,
        Some(node) => {
            match key.cmp(&node.key) {
                Ordering::Less => search(&node.left, key),
                Ordering::Greater => search(&node.right, key),
                Ordering::Equal => Some(node),
            }
        }
    }
}

// Вывод декартового дерева
fn display(root: &Option<Box<Node>>, level: usize) {
    if let Some(node) = root {
        display(&node.right, level + 1);
        println!("{}{}", " ".repeat(level * 4), node.key);
        display(&node.left, level + 1);
    }
}

// Вычисление высоты дерева
fn tree_height(root: &Option<Box<Node>>) -> usize {
    match root {
        None => 0,
        Some(node) => {
            let left_height = tree_height(&node.left);
            let right_height = tree_height(&node.right);
            1 + std::cmp::max(left_height, right_height)
        }
    }
}

// Вывод элементов на заданном уровне
fn display_level(root: &Option<Box<Node>>, level: usize, current_level: usize) {
    if let Some(node) = root {
        if current_level == level {
            println!("{}", node.key);
        } else {
            display_level(&node.left, level, current_level + 1);
            display_level(&node.right, level, current_level + 1);
        }
    }
}

fn main() {
    let mut root: Option<Box<Node>> = None;

    loop {
        println!("Выберите действие:");
        println!("1. Вставить элемент");
        println!("2. Удалить элемент");
        println!("3. Поиск элемента");
        println!("4. Вывести дерево");
        println!("5. Вычислить высоту дерева");
        println!("6. Вывести элементы на заданном уровне");
        println!("7. Выйти");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Неверный ввод. Пожалуйста, введите число от 1 до 7.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Введите ключ:");
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).expect("Ошибка ввода");
                let key: i32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный ввод. Пожалуйста, введите целое число.");
                        continue;
                    }
                };
                let priority = rand::thread_rng().gen_range(0..100); // Генерируем случайный приоритет
                root = insert(root, key, priority);
                println!("Элемент вставлен");
            }
            2 => {
                println!("Введите ключ элемента для удаления:");
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).expect("Ошибка ввода");
                let key: i32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный ввод. Пожалуйста, введите целое число.");
                        continue;
                    }
                };
                root = delete(root, key);
                println!("Элемент удален");
            }
            3 => {
                println!("Введите ключ элемента для поиска:");
                let mut key_input = String::new();
                io::stdin().read_line(&mut key_input).expect("Ошибка ввода");
                let key: i32 = match key_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный ввод. Пожалуйста, введите целое число.");
                        continue;
                    }
                };
                match search(&root, key) {
                    Some(node) => println!("Элемент найден: {:?}", node),
                    None => println!("Элемент не найден"),
                }
            }
            4 => {
                println!("Дерево:");
                display(&root, 0);
            }
            5 => {
                let height = tree_height(&root);
                println!("Высота дерева: {}", height);
            }
            6 => {
                println!("Введите уровень:");
                let mut level_input = String::new();
                io::stdin().read_line(&mut level_input).expect("Ошибка ввода");
                let level: usize = match level_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный ввод. Пожалуйста, введите целое число.");
                        continue;
                    }
                };
                if level == 0 {
                    println!("Уровень должен быть больше 0");
                    continue;
                }
                println!("Элементы на уровне {}: ", level);
                display_level(&root, level, 1);
            }
            7 => {
                println!("Программа завершена.");
                break;
            }
            _ => {
                println!("Неверный выбор. Пожалуйста, выберите число от 1 до 7.");
            }
        }
    }
}

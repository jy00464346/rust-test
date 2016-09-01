#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(unboxed_closures)]

use std::mem;
use std::panic;

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> i32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil")
        }
    }
}

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

#[allow(overflowing_literals, unused_mut, unreachable_code)]
fn main() {
    let a = 3.2f32;
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 -2 = {}", 1i32 - 2);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    let xs = [1, 2, 3, 4, 5];
    // `len` 返回数组的大小
    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&a));

    let result = panic::catch_unwind(|| {
        //        println!("test out of bound {}", &xs[5]);
    });
    //    assert!(result.is_err());
    println!("error :{:?}", result.is_err());

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // 编译器会对未使用变量绑定产生警告； 可在变量名加上下划线的前缀来消除这些警告内容。
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名加上下划线前缀消除警告

    println!("1000 = {:?}", 1000 as u8);

    let item = 5u8;
    let mut vec = Vec::new();

    vec.push(item);
    println!("{:?}", vec);

    let nanosecond: NanoSecond = 5 as u64_t;
    let inch: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanosecond,
             inch,
             nanosecond + inch);
    let n = 5;
    let if_args = if n == 0 {
        println!(", and is a small number, increase ten-fold");
        // 这条表达式返回一个 `i32` 类型。
        10 * n
    } else {
        println!(", and is a big number, reduce by two");
        // 这条表达式也必须返回一个 `i32` 类型。
        n / 2
        // 试一试 ^ 试着加上一个分号来结束这条表达式。
    };
    println!("if_args={}", if_args);

    'outer: loop {
        'inner: loop {
            println!("inner loop ");
            break 'outer;
        }
        println!("this code will never be reached");
    }

    /*let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }*/
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

    let ref _is_a_reference = 5;

    let value = 5;
    let mut mut_value = 6;

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("{}", m);
        }
    }
    let pair = (2, -2);
    match pair {
        (x, y) if x == y =>
            println!("These are twins"),
        (x, y) if x + y == 0 =>
            println!("antimatter ,kaboom"),
        _ =>
            println!("No correlation"),
    };
    let age = 15;
    match age {
        0 =>
            println!("i'm 0"),
        n @ 1...10 =>
            println!("1-10 :{}", n),
        n @ 11...20 =>
            println!("11-20:{}", n),
        n =>
            println!("im {}", n)
    };
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("number is {}", i);
    }
    if let Some(i) = letter {} else {
        println!("don't match a number. let 's go with a letter !");
    }
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count {}", count);
    };
    inc();
    inc();
    //    let reborrow = &mut count;
    let moveable = Box::new(3);
    let consume = || {
        println!("movable : {:?}", moveable);
        mem::drop(moveable);
    };
    consume();
    //    consume();
    fn apply<F>(f: F) where F: FnOnce() {
        f()
    }
    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // `greeting` 使用引用方式： 需要 `Fn`。
        println!("I said {}.", greeting);
        // 改变迫使 `farewell` 变成了通过可变引用来捕获。
        // （ 原文： Mutation forces `farewell` to be
        // captured by mutable reference.）
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        // 手动调用 drop 将 `farewell` 强制转成通过值来捕获。
        // （ 原文： Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.）
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    //    println!("3 doubled: {}", apply_to_3(double))

    fn create_fn() -> Box<Fn()> {
        let text = "Fn".to_owned();
        Box::new(move || println!("This is a : {}", text))
    }

    fn create_fnmut() -> Box<FnMut()> {
        let text = "FnMut".to_owned();
        Box::new(move || println!("This is a :{}", text))
    }

    let fn_plain = create_fn();
    let mut fnmut_plain = create_fnmut();
    fn_plain();
    fnmut_plain();

    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];
    println!("2 in vec1 : {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec1 : {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1 : {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2 : {}", array2.into_iter().any(|&x| x == 2));

    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];
    println!("2 in vec1 : {:?}", vec1.iter().find(|&&x| x == 2));
    println!("2 in vec1 : {:?}", vec2.into_iter().find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1 : {:?}", array1.iter().find(|&&x| x == 2));
    println!("2 in array2 : {:?}", array2.into_iter().find(|&&x| x == 2));

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style : {}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..).map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("functional style : {}", sum_of_squared_odd_numbers);
}

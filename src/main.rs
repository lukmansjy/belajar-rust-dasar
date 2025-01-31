use core::time;
use std::{fmt::format, mem::replace, result};

fn main() {
    println!("Hello, world!");
    println!("Hello Lukman Sanjaya");
}

#[test]
fn hello_test() {
    println!("Hello test");
}

#[test]
fn variable_test() {
    let name = "Lukman Sanjaya";
    // name = "lukman"; // error imutable variable tidak bisa diubah
    println!("Hello {}", name);
}

#[test]
fn mutable_test() {
    let mut name = "Lukman Sanjaya";
    println!("Hello {}", name);

    name = "lukman";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let mut name = "Lukman Sanjaya";
    println!("Hello {}", name);

    name = "Lukman";
    // name = 10; // error expected str, found integer
    println!("Hello {}", name);
}

#[test]
fn variable_shadowing() {
    let  name = "Lukman Sanjaya";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name); // akan tampil 10, karena variable sebelumnya tertutupi (shadowing)
}

#[test]
fn variable_explicit() {
    let  age:i32 = 20; // explicit type data variable disebutkan
    println!("{}", age);
}

#[test]
fn number() {
    let a:i8 = 10;
    println!("{}", a);

    let b:f32 = 10.5;
    println!("{}", b);
}


#[test]
fn number_conversion() {
    let a:i8 = 10;
    println!("{}", a);

    let b:i16 = a as i16;
    println!("{}", b);

    let c:i32 = b as i32;
    println!("{}", c);

    let d:i64 = 1000000000;
    let e:i8 = d as i8; // integer overflow, hasil konversi menjadi tidak sesuai expektasi
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a / b;
    let e = a + b;

    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
}

#[test]
fn aumented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b:bool = false;
    println!("{} {}", a, b);

}

#[test]
fn comparison() {
    let result:bool = 10 > 20;
    println!("{}", result);

    let result2:bool = 30 >= 20;
    println!("{}", result2);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let data:(i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    println!("{} {} {}", data.0, data.1, data.2);

    let (a, b, c) = data; // destructuring tuple
    println!("{} {} {}", a, b, c);

    let mut data2:(i32, f64, bool) = (10, 10.5, true); // mutable tuple
    println!("{:?}", data2);

    data2.0 = 50;
    data2.1 = 20.5;
    data2.2 = false;
    println!("{:?}", data2);
}

fn unit() {
    println!("Hello")
}

#[test]
fn test_unit(){
    let result:() = unit(); // return tuple kosong
    println!("{:?}", result);

    let test:() = (); //tuple kosong
    println!("{:?}", test)
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    let mut array_mut: [i32; 5] = [1, 2, 3, 4, 5]; // array mutable
    array_mut[0] = 10;
    array_mut[1] = 20;
    println!("{:?}", array_mut);

    let length: usize = array_mut.len();
    println!("{}", length)
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 5],
        [3, 4, 6]
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0])
}


const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM);
}

#[test]
fn variable_scope() {
    let lukman = 1; // variable scope

    { // inner scope
        println!("inner lukman: {}", lukman);

        let sanjaya = 2;
        println!("inner sanjaya: {}", sanjaya);
    }
        // println!("inner sanjaya: {}", sanjaya); // error
}


#[test]
fn steack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10; // datanya fix maka disimpan di stack
    let b = String::from("Lukman"); // datanya tidak fix (datanya bisa membesar & mengecil) maka disimpan di heap

    println!("{} {}", a, b)
}

fn function_b() {
    let a = 10; // datanya fix maka disimpan di stack
    let b = String::from("Sanjaya"); // datanya tidak fix maka disimpan di heap

    println!("{} {}", a, b)
}

/*
String
    &str => string slice (datanya fix), datanya disimpan di stack
    String => ukuran datanya bisa mengembang, datanya disimpan di heap
*/
#[test]
fn string() {
    let name: &str = "   Lukman Sanjaya   "; // disimpan dimemory stack
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "Lukman";
    username = "Sanjaya"; // sebenarnya data Lukman masih ada di memory. variable usename cuman berubah memegang nama Sanjaya (tidak ditimpa cuman mengganti)
    println!("{}", username);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Lukman"); // disimpan di memory heap
    name.push_str("Sanjaya"); // data akan langsung ditambah ke heap
    println!("{}", name);

    let name2 = name.replace("Lukman", "luk"); // replace tidak akan mengubah data di heap, tetapi akan bikin data heap baru
    println!("{}", name2);
}

/*
Ownership Rules
- Setiap value di Rust harus punya owner (variable pemilik value)
- Dalam satu waktu, hanya boleh ada satu owner
- Ketika owner keluar scopr, value akan dihapus
*/

#[test]
fn ownership_rules() {
    let a = 10;
    {
        let b = 20;
        println!("{}", b);
    } // scope b selesai, b dihapus dari memory, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus dari memory, a tidak bisa diakses lagi


#[test]
fn data_copy() { // # 1 konsep 1 value 1 owner
    let a = 10;
    let b = a; // copy data dari a lalu disimpan ke b, b adalah data stack baru, data b tidak akan ada hubungan dgn a

    println!("{} {}", a, b);
}


#[test]
fn ownership_movement() { // # 2 konsep 1 value 1 owner. ownership movement tidak berlaku data yg disimpan di stack (hanya di heap)
    let name1 = String::from("Lukman");
    let name2 = name1; // name1 tidak bisa diakses lagi (value dipindah)

    // println!("{}", name1); // error value borrowed here after move
    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Lukman");
    let name2 = name1.clone(); // membuat data baru (tiruan dari data name1), proses ini akan lebih berat

    println!("{} {}", name1, name2)
}

#[test]
fn if_expression() {
    let value = 9;

    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("Not bad");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Very bad");
    }

    // if let statement
    let result: &str;
    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Not bad";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very bad";
    }

    println!("{}", result);

    // let statement expression
    let result2 = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very bad"
    };

    println!("{}", result2);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("Counter: {}", counter)
    }
}

#[test]
fn loop_retutn_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result)
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i +=1;
            if i> 5 {
                break;
            }
        }
        number += 1;
    }
}


#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value: {}", value);
    }
}

#[test]
fn range() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5; //range exclusive
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Value: {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4; //range inclusive
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Value: {}", array[i]);
    }
}

// fn say_hello() {
//     println!("Hello");
// }

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", last_name, first_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Lukman", "Sanjaya");
    say_goodbye("Joko", "Susanto");
}


fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("Result: {}", result);

    let result = factorial_loop(-10);
    println!("Result: {}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value)
    }
    print_text(value, times - 1);
}

#[test]
fn test_print_test() {
    print_text(String::from("Lukman"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_revirsive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}


// ownership function
fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Lukman"); // krn type datanya String (disimpan di heap) maka datanya akan pindah ke function
    // intinya data yg disimpan di heapkalau dikirim ke parameter maka ownership nya akan pindah ke parameter tsb
    hi(name);
    // println!("{}", name); // error value borrowed here after move
}

// return value ownership
fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Lukman");
    let last_name = String::from("Sanjaya");

    let name = full_name(first_name, last_name);
    println!("{}", name);
    // println!("{}", first_name); // error
    // println!("{}", last_name); // error
}

// mengembalikan ownership
fn full_name2(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name2() {
    let first_name = String::from("Lukman");
    let last_name = String::from("Sanjaya");

    let (first_name, last_name, name) = full_name2(first_name, last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

// references
fn full_name3(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name3() {
    let first_name = String::from("Lukman");
    let last_name = String::from("Sanjaya");

    let name = full_name3(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

// borrowing (tidak boleh modifikasi value)
fn change_value(value: &String) {
    // value.push_str("test"); // error `value` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

#[test]
fn test_change_value() {
    let mut value = String::from("Lukman");
    change_value(&value);
    println!("{}", value);
}

// borrowing (boleh modifikasi value)
fn change_value2(value: &mut String) {
    value.push_str("test");
}

#[test]
fn test_change_value2() {
    let mut value = String::from("Lukman");
    change_value2(&mut value);
    println!("{}", value);
}

// dangling pointer (datanya sudah tidak ada di memory) / function mengembalikan data pointer tidak diperbolehkan
// fn get_full_name(first_name: &String, last_name: &String) -> &String { // error
//     let name = format!("{} {}", first_name, last_name);
//     return &name;
// }


// slice
#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[..]; // slice full range
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5]; // slice [start..end] yg diambil index 0 - 4 krn exclusive
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..]; // slice ambil dr index 5 - akhir
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("Lukman Sanjaya");
    let first_name: &str = &name[0..6];
    println!("{}", first_name);

    let last_name: &str = &name[7..];
    println!("{}", last_name);
}

// struct
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}


#[test]
fn struct_person() {
    let person: Person = Person {
        first_name: String::from("Lukman"),
        middle_name: String::from("S"),
        last_name: String::from("Sanjaya"),
        age: 17
    };

    print_person(&person);
}

//  struct init shorthand
#[test]
fn struct_person_shorthand() {
    let first_name = String::from("Lukman");
    let last_name = String::from("Sanjaya");

    let person: Person = Person {
        first_name, // ownership berpindah
        middle_name: String::from("S"),
        last_name, // ownership berpindah
        age: 17
    };

    // println!(last_name); //error ownership berpindah
    print_person(&person);
}

// struct update syntax (copy data person sebelumnya)
#[test]
fn struct_person_update_syntax() {
    let first_name = String::from("Lukman");
    let last_name = String::from("Sanjaya");

    let person: Person = Person {
        first_name,
        middle_name: String::from("S"),
        last_name,
        age: 17
    };

    print_person(&person);

    // let person2: Person = Person {..person}; // ini data yg di heap akan berpindah ownership, sebaiknya menggunakan clone
    let person2: Person = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);
}

// struct tuple
struct GeoPoint(f64, f64);

#[test]
fn struct_tuple() {
    let geo_point = GeoPoint(-6.20, 106.816);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}

// struct tanpa field
struct Nothing;
#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}

// method
impl Person { // membuat method didalam struct
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name)
    }
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Lukman"),
        middle_name: String::from("S"),
        last_name: String::from("Sanjaya"),
        age: 18,
    };

    person.say_hello("Joko");
}

// associated functions (diakses langsung dr NamaStruct::nama_funct)
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_method_new() {
    let geo_pint: GeoPoint = GeoPoint::new(-6.34, 106.81);
    println!("long: {}", geo_pint.0);
    println!("lat: {}", geo_pint.1);
}

// enum
enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let _level: Level = Level::Premium;
}

enum Payment {
    // cart number
    CreditCart(String),
    // bank name, account number
    BankTransfer(String, String),
    // ewallet name, ewallet number
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying amount {}", amount);
    }
}

#[test]
fn test_payment() {
    let payment1: Payment = Payment::BankTransfer(String::from("BCA"), String::from("232423513"));
    let _payment2: Payment = Payment::EWallet(String::from("Dana"), String::from("353232131"));
    let _payment3: Payment = Payment::CreditCart(String::from("3532332322131"));

    payment1.pay(10000);
}

// pattren matching
// pattren matching untuk enum
#[test]
fn test_enum_matching() {
    let level: Level = Level::Premium;
    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

// destructuring emum data patterns
impl Payment {
    fn pay2(&self, amount: u32) {
        match self {
            Payment::CreditCart(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with ewallet {} {} amount {}",wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment_2() {
    let payment1: Payment = Payment::BankTransfer(String::from("BCA"), String::from("232423513"));
    let _payment2: Payment = Payment::EWallet(String::from("Dana"), String::from("353232131"));
    let _payment3: Payment = Payment::CreditCart(String::from("3532332322131"));

    payment1.pay2(10000);
}

// pattern matching untuk value
#[test]
fn test_match_value() {
    let name: &str = "Lukman";

    match name {
        "Budi" => {
            println!("Hello Budi");
        }
        "Lukman" => {
            println!("Hello Lukman");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

// multiple patterns
#[test]
fn test_match_value_multiple() {
    let name: &str = "Budi";

    match name {
        "Budi" | "Joko" => {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

// range patterns
#[test]
fn test_range_patterns() {
    let value = 100;
    match value {
        75..=100 =>{
            println!("Great")
        }
        50..=74 =>{
            println!("Good")
        }
        25..=49 =>{
            println!("Not Bad")
        }
        0..=24 =>{
            println!("Bad")
        }
        other =>{
            println!("Invalid value {}", other)
        }
    }
}

// destructuring struct patterns
#[test]
fn test_struct_patterns() {
    let point = GeoPoint(0.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Lukman"),
        middle_name: String::from("S"),
        last_name: String::from("Sanjaya"),
        age: 20,
    };

    match person {
        Person{ first_name, last_name, ..} => {
            println!("First Name: {}, Last Name: {}", first_name, last_name);
        }
    }
}

// math expression
#[test]
fn test_match_expression() {
    let value: i32 = 2;
    let result: &str = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };

    println!("{}", result)
}

// type alias
type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("13142132"),
        name: String::from("Lukman"),
        age: 18,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age)
}

// module
mod model {
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.first_name)
        }
    }
}

#[test]
fn test_module() {
    let user = model::User{
        first_name: String::from("Lukman"),
        last_name: String::from("Sanjaya"),
        username: String::from("lukman"),
        email: String::from("luk@test.com"),
        age: 18,
    };

    user.say_hello("Joko");
}


mod first;
mod second;

use first::say_hello;
use second::say_hello as say_hello_second;
#[test]
fn test_use() {
    say_hello();
    say_hello_second();
}
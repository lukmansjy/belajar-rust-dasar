use crate::third::say_hello as say_hello_third;

pub fn say_hello(){
    println!("hello from first module");
    say_hello_third();
}

// super
mod second {
    mod third {
        pub fn say_hello() {
            // crate::first::say_hello(); // menggunakan create bearti diliat dr file main
            super::super::say_hello(); // menggunakan super berarti dilihat dr mod ini berada lalu naik ke blok mod atasnya
        }
    }
}
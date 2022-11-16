use std::collections::HashMap;

fn main() {
    let a = 5;
    let b = 12;
    println!("Printing value a: {0}, b: {1}", a, b);

    //unsigned integer
    //u8, u16, u32, u64, u128
    let value:u8 = 255;
    println!("This is unsigned integer: {}", value);

    //signed integer
    //i8, i16, i32, i64, i128
    let value:i8 = -128;
    println!("This is signed integer: {}", value);

    //float is used for decimals
    let value:f32 = 12.5;
    println!("This is float value: {}", value);

    //char
    let string:char = 'C';
    let emoji:char = '\u{1F600}';
    println!("One letter is: {0} with emoji {1}", string, emoji);

    //boolean
    let condition:bool = true;
    println!("Condition is: {}", condition);

    //array
    let empty_list:[u8; 5] = [0; 5];
    let defined_list:[u8; 2] = [12, 5];
    println!("Empty list is {:?} with length {1}", empty_list, empty_list.len());
    println!("{}", defined_list[0]);

    //tuple
    let structure:(u8, bool, f32) = (8, true, -15.2);
    println!("This is all tuple {:?}", structure);
    println!("This is element in tuple: {0}, {1}, {2}", structure.0, structure.1, structure.2);

    //destructuring
    let (a, b, c) = structure;
    println!("The value of destructuring is:");
    print!("b: {0}, a: {1}, c: {2}", b, a, c);

    //function
    let a:u8 = 8;
    println!("{0} is even? : {1}", a, even_odd(a));

    //mutability
    let mut value:u8 = 0; //undefined variable in rust will give error on operation before assigned
    println!("Value is {}", value);
    value = 5;
    println!("Changed Value is {}", value);

    //slice
    let list:[u8; 4] = [12, 1, 9, 6];
    let slice:&[u8] = &list[1 .. 3];

    borrowing_slice(list, slice);

    //string
    let mut phrase:String = String::from("Hello World");
    //string slice
    let slice:&str = &phrase[..5];
    println!("Sliced String is: {}", slice);
    phrase.push(',');
    phrase.push_str(" Nakhim!!");
    phrase.pop();
    phrase = phrase.replace("Hello", "Bye");
    println!("Result from String methods: {}", phrase);

    //if-else
    let value:u8 = 3;
    println!("Value {0} is even? : {1}", value, even_odd_condition(value));

    //for-loop
    for index in 0..5 {
        println!("Index: {}", index);
    }

    //while-loop
    let mut index:u8 = 0;
    while index < 5 {
        println!("Index: {}", index);
        if index == 4 {
            break;
        }
        index += 1;
    }

    //matching (switch-case)
    let value:i8 = 5;
    match value {
        0 => {
            println!("Value is 0.");
            println!("Print another!")
        },
        1 | 2 => {
            println!("Value is 1 or 2.")
        },
        3 ..= 5 => {
            println!("Value is in range from 3 to equal 5.")
        },
        _ => println!("Value is unmatched."),
    }

    //struct and impl
    let user = User {
        name: String::from("Nakhim"),
        age: 25,
        salary: 10000,
    };
    user.print_name();
    println!("Age: {0}", user.get_age());
    println!("Salary: {0}, Admin: {1}", user.get_salary(), user.is_admin());

    //enumeration
    let x_value:i32 = 12;
    let y_value:i32 = 5;
    let z_value:i32 = -2;
    let name:Math = Math::Name;
    let function:Math = Math::Function(x_value);
    let coordination:Math = Math::Coordination {x: 2*x_value, y: 3*y_value, z: 2*z_value};

    if let Math::Function(x) = function {
        println!("Name: {:?}, X of Function: {1}", name, x);
    }

    let mut new_x:i32 = 0;
    let mut new_y:i32 = 0;
    let mut new_z:i32 = 0;
    if let Math::Coordination {x, y, z} = coordination {
        new_x = x;
        new_y = y;
        new_z = z;
    }
    println!("New value of x, y, z: ({0}, {1}, {2})", new_x, new_y, new_z);

    //vector
    let mut vec:Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("Length of vector: {}", vec.len());
    println!("First index of vector: {}", vec[0]);
    vec.push(6);
    vec.remove(0);
    println!("Vector now is {:?}", vec);

    //mapping
    let mut map:HashMap<String, String> = HashMap::new();
    map.insert(String::from("Key1"), String::from("Value1"));
    map.insert(String::from("Key2"), String::from("Value2"));
    println!("Map: {:?}", map);

    match map.get("Key1") {
        Some(value) => println!("Got Value: {}", value),
        None => println!("Cannot find key in map"),
    }

    match map.get("Key3") {
        Some(value) => println!("Got Value: {}", value),
        None => println!("Cannot find key in map"),
    }

    //remove key-value
    map.remove("Key1");
    println!("Map: {:?}", map);

    //option-datatype
    println!("Has key2? : {:?}", map.get("Key2"));
    println!("Has key3? : {:?}", map.get("Key3"));

    //result-datatype
    if divide(4, 0).is_ok() {
        println!("Division of 4 and 0 is {}", divide(4, 0).unwrap());
    } else {
        println!("Cannot divide value with zero.");
    }
}

pub fn even_odd(value:u8) -> bool {
    return value % 2 == 0;
}

pub fn borrowing_slice(list:[u8; 4], slice:&[u8]) {
    println!("Array {:?}", list);
    println!("Slice {:?}", slice);
    println!("Length Slice {}", slice.len());
    println!("Slice Indexing {0}, {1}", slice[0], slice[1]);
}

pub fn even_odd_condition(value:u8) -> bool {
    return if value % 2 == 0 { true } else { false };
}

struct User {
    name:String,
    age:u8,
    salary:u64,
}

impl User {
    fn print_name(&self) {
        println!("Name: {}", self.name);
    }

    fn get_age(&self) -> u8 {
        return self.age;
    }
}

trait Employee {
    fn get_salary(&self) -> u64 {
        return 1000;
    }
    fn is_admin(&self) -> bool;
}

impl Employee for User {
    fn get_salary(&self) -> u64 {
        return self.salary;
    }

    fn is_admin(&self) -> bool {
        return true;
    }
}

#[derive(Debug)]
enum Math {
    Name,
    Function(i32),
    Coordination {
        x:i32,
        y:i32,
        z:i32,
    }
}

#[derive(Debug)]
enum CatchError {
    Error,
}

fn divide(dividend:i64, divisor:i64) -> Result<i64, CatchError>{
    return if divisor == 0 { Err(CatchError::Error) } else { Ok(dividend/divisor) }
}
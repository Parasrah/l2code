#![allow(dead_code)]
use std::fmt;

fn main() {
    custom_traits_example();
}

// camelCase

// PascalCase (CapitalCase)

// snake_case

// a brief word about spacing (.editorconfig)

// data types

fn numbers_example () {
    // numbers (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, fsize)
    #![allow(overflowing_literals)]
    let my_num: i8 = -129;
    println!("my_num: {}", my_num);
}

fn boolean_example () {
    let my_bool = true;
    println!("the opposite of {} is {}", my_bool, !my_bool);
    println!("5 is greater than 3 is {}", 5 > 3);
}

fn character_example () {
    let my_char = 'a';
    println!("my favorite character is {}", my_char);
}

fn strings_example () {
    let my_str = String::from("hello world!");
    println!("today I learned to write {}", my_str);
}

fn tuple_example () {
    // tuples have a fixed length, but can contain different types
    let my_tuple = (5, String::from("hello world"));
    println!("look! I made a tuple! {:#?}", my_tuple); // quick note - why the {:#?}?

    // for two tuples to have the same "type", they must contain the
    // same length, with the same types in the same order
    let my_similar_tuple = (10, false);
    let my_different_tuple = (11, true);
}

fn array_example () {
    // an array is a low level construct, not present in all
    // languages. And even in some languages that call their
    // type an "array", it's actually more similar to a "list"
    // (i.e. javascript)
    let my_array = [1, 2, 3];
    println!("I made an array! {:#?}", my_array);

    // an array typically has a fixed length, and must contain the
    // same type.
    println!("the third element is: {}", my_array[2]);
    // println!("the fourth element is: {}", my_array[3]);
}

fn list_example () {
    // lists have different names in different languages
    // they also have different implementations in different
    // languages, meaning they perform differently too!
    let mut my_list = vec![1, 2, 3];

    // in languages that allow mutations, lists often have a
    // variable length. a list must be made up of the same
    // type.
    my_list.push(4);
    println!("{:#?}", my_list);
}

fn function_example () {
    let input = 5;
    let my_fun = |x| x + 1;
    print!("adding 1 to {} gives me {}", input, my_fun(input));
}

fn get_fn () -> Box<dyn Fn() -> i32> {
    let input = 5;
    return Box::new(move || input + 1);
}

/*--------------------*/
/*     mutability     */
/*--------------------*/

// What is mutability?

// Rust is one of the few languages where
// it's easy to reason about mutability.
// That is why it's hard to learn.
// what is memory?

fn mut_example () {

}

/*-----------------------*/
/*     borrow system     */
/*-----------------------*/

// what is the stack?

// what is the heap?

// what is a garbage collector?

// what is ownership?

// From the Rust book:
// ===================
//
// * Each value in Rust has a variable that’s called its owner.
// * There can only be one owner at a time.
// * When the owner goes out of scope, the value will be dropped.

// The "borrow" operator is "&"

fn ownership_example () {
    let my_str = String::from("hello");
    let updated = own(my_str);
    // println!("before: {}", my_str);
    println!("after: {}", updated);
}

fn borrow (my_str: &str) -> String {
    let mut updated = String::from(my_str);
    updated.push_str(" world!");
    updated
}

fn own (mut my_str: String) -> String {
    my_str.push_str(" world!");
    my_str
}

// control flow

fn control_flow_example () {
    let my_var = 3;
    // if my_var == 1 {
    //     println!("my variable was 1!");
    // } else if my_var == 2 {
    //     println!("my variable was 2!");
    // } else {
    //     println!("my variable was not 1!");
    // }

    let output = match my_var {
        1 => String::from("was 1"),
        2 => String::from("was 2!"),
        _ => format!("was unkwon"),
    };

    println!("my_var {}", output);
}

// generics

fn length_plus_one <T> (list: Vec<T>) -> usize {
    list.len() + 1
}

// union types (often called enums)
// ! explain discriminating unions

enum Gender {
    Male,
    Female,
    Other,
}

enum PoliticalIdeology {
    Democracy(u16),
    Communism,
}

// structs -- allow us to make complex state
// also called "objects" or "records" in
// other languages

struct Country {
    name: String,
    political_ideology: PoliticalIdeology,
}

struct Person {
    name: String,
    age: u16,
    gender: Gender,
}

impl Person {
    pub fn new (name: String, years: u16, gender: Gender) -> Person {
        Person {
            name: name,
            age: years,
            // shortcut syntax
            gender,
        }
    }

    fn can_vote (&self, country: &Country) -> bool {
        // pattern matching (discriminating union)
        match country.political_ideology {
            PoliticalIdeology::Communism => false,
            PoliticalIdeology::Democracy(voting_age) => voting_age <= self.age,
        }
    }
}

fn get_people () -> Vec<Person> {
    vec![
        Person::new(String::from("Lexi"), 21, Gender::Female),
        Person::new(String::from("Meg"), 29, Gender::Other),
        Person::new(String::from("Hailee"), 9, Gender::Female),
    ]
}

fn complex_types_example () {
    let country = Country {
        name: String::from("Canada"),
        political_ideology: PoliticalIdeology::Democracy(18),
    };
    let person = Person::new(String::from("Lexi"), 12, Gender::Female);
    let can_or_cannot = match person.can_vote(&country) {
        true => "can",
        false => "cannot",
    };
    println!("{} {} vote in {}", person.name, can_or_cannot, country.name);
}

// traits

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gender_str = match self {
            Gender::Female => "female",
            Gender::Male => "male",
            Gender::Other => "other",
        };
        write!(f, "{}", gender_str)
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}, Age: {}, Gender: {}'", self.name, self.age, self.gender)
    }
}

fn traits_example () {
    let people = get_people();
    let person = match people.first() {
        Some(x) => x,
        None => panic!("I COULDN'T FIND A PERSON FOR MY EXAMPLE!!")
    };
    // now we can print!
    println!("{}", person);
}

trait Named {
    fn get_name (&self) -> &str;
}

impl Named for &Person {
    fn get_name (&self) -> &str {
        &self.name
    }
}

impl dyn Named {
    fn get_name_length <T: Named> (input: T) -> usize {
        input.get_name().len()
    }
}

trait Living {
    fn get_age (&self) -> u32;
}

impl Living for &Person {
    fn get_age (&self) -> u32 {
        self.age as u32
    }
}

fn custom_traits_example () {
    let people = get_people();
    let person = people.first().unwrap(); // do as I say, not as I do
    println!("summary: {}", get_summary(person));
}

fn get_summary <T: Named + Living> (entity: T) -> String {
    format!("{} is {} years old!", entity.get_name(), entity.get_age())
}

/*---------------------*/
/*     concurrency     */
/*---------------------*/

/*------------------*/
/*     closures     */
/*------------------*/

/*-------------------*/
/*     iterators     */
/*-------------------*/

/*------------------------*/
/*     error handling     */
/*------------------------*/

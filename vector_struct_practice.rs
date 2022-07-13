/* In this code I try to use a vector to hold some structs
   with a bit of succes */ 


// we are deriving some useful traits to use with our struct
#[derive(Debug, Clone)]
struct Person{
    name: String,
    age: u8,
}

fn main() {
// to use a variable as a vector index the type has to be usize
    let my_index:usize = 1;
    
    let person1 = Person{
        name: String::from("John"), 
        age: 32
    };

    


    let mut my_vector = vec![person1.clone(),];

    my_vector.push(build_person("Alice".to_string(), 32));   
    
    println!("{:?}", my_vector);

   
 /* Here I am trying diferent ways to acces the data of a
    struct and a vector of structs respectively */
    person1.print_name();
    person1.print_age();
    my_vector[my_index].print_age();
    my_vector[my_index].print_name();

    println!("This persons name is {:?}", my_vector[1].name)

}


impl Person {
    fn print_name(&self) {
        println!("{:?}", self.name);
    }
    fn print_age(&self) {
        println!("{:?}", self.age);
    }
}

/*This is a constructor function to isntatiate 
  tructs given the data */
fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age
    }
}

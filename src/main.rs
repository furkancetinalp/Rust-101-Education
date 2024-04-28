use core::num;
use std::{io, num::ParseIntError};

fn main() {
    //***************************** STRUCT / STRUCTURES ********************************** 
    //lets consider we want to store a person's informations which are name,age,height,weight,blood type
    //we can define these variables by:
    let person_name: String = String::from("Furkan Cetinalp");
    let persone_age: u8 = 18;
    let person_height: u8 = 178;
    let person_weight= 75_u8;
    let person_blood_type: String;


    //but defining each variable in seperate would be a problem when project is getting bigger. 
    //for this problem, a new approach arrived which is Structures
    // !!! structs or structures are user defined data types which can store different type of variables together

    //--- DEFINING CLASSİC STRUCT (C STRUCTS)

    // struct struct_name {
    //     field1: data_type,
    //     field2: data_type,
    //     field3: data_type
    // }


    struct Person{
        name:String,
        age:u8,
        height:u8,
        weight:u8,
        blood_type:String,
    }


    //instance of struct
    let person_one = Person{
        name:"Furkan Cetinalp".to_string(),
        age:28,
        height:175,
        weight:72,
        blood_type:"0-".to_string()
    };


    //accessing struct's variables

    let person_one_name = &person_one.name;
    println!("name of the person one is {}",person_one_name);


    // ----------- DEFINING TUPLE STRUCTS
    //a kind of struct which does not have specific names for their variables, they only have the data types
    // struct TupleStruct(data_type, data_type,...);

    //Example:
    struct PersonsForTupleStruct(String,u8,u8,u8,String); // we defined Person with same data types in tuple structs

    //instance of Tuple Structs
    let person_tuple_one = PersonsForTupleStruct("Furkan Cetinalp".to_string(),28,175,72,"0-".to_string());

    //the problem in tuple structs is that we need to memorize each of the variable's functionality. 
    //For example first index of structs represents name of person but we don't keep this info in struct.

    //accessing struct's variables
    let person_tuple_one_name = person_tuple_one.0;
    println!("name of the person TUPLE one is {}",person_tuple_one_name);

    
    //Tuple structs can be used to represent simple objects which does not require much complex to understand
    //For example, in RGB color representation, a color is defined via combination of the tonne of Red, Green and Blue which can have a value from
    //range of 0-255 :

    struct RgbColor(u16,u16,u16);

    let green = RgbColor(0,255,0);
    let red  =RgbColor(255,0,0);



    // --- UNIT STRUCTS
    //a kind of struct that does not have any fields.
    //They are generally not useful for single usage but they are used in generics and traits
    #[derive(Debug)] 
    struct PersonUnit;

    let person_unit_one = PersonUnit;
    println!("person in unit struct: {:?}",person_unit_one);



    // --- USE OF IMPL
    //Sometimes, when we create functions that interact with our structs they usually get mixed up with other functions. 
    //This process makes it hard to easily understand what our code does.
    //For this problem, Rust has a feature which is named 'impl'
    //impl allows developers to group functions which are directly related to specific Structs

    //USAGE
    /*
    impl StructName{
        fn function_name(parameters) -> return_value
    }
    */
    //EXAMPLE
    impl Person{
        fn create_person(name:String,age:u8,height:u8,weight:u8,blood_type:String) -> Self{
            Self{
                name,
                age,
                height,
                weight,
                blood_type
            }
        }

        fn show_person_info(&self) -> (){
            println!("Name: {}\nAge: {}\nHeight: {}\nWeight: {}\nBlood Type: {}",self.name,self.age,self.height,self.weight,self.blood_type)
        }
    }

    //Instance of the impl functions
    let new_person = Person::create_person("Furkan Cetinalp".to_string(), 28, 175, 72, "0-".to_string());

    //calling show_person_info
    new_person.show_person_info();




    //ENUMS or ENUMERATIONS
    //Enums are user defined data types that contains variants which are used to represent multiple distinct values within single data

    //lets consider adding days of week in a data type. 

    let sunday:String;
    let monday:String;
    let tuesday:String;
    let wednesday:String;
    let thursday:String;
    let friday:String;
    let saturday:String;

    //as we can see, we have to decleare variables for each of the day. In small projects, that might not be  seen as a huge problem.
    //However, when project is getting bigger, this usage becomes really unefficient way to choose and the code gets  much complex to handle and read.
    //For these problems, Rust represent a data type which is Enum. With enum, we can set multiple variables into one data type:

    //USAGE
    /*
    enum EnumName{
        value_one,
        value_two,
        .
        .
        .
    }
     */

    //EXAMPLE
    enum Day{
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday
    }

    //for the above: 
    //Day is an ENUM
    // Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday are the variants of the ENUM

    //Instance of Enum
    let selected_day = Day::Monday;


    //Enums whose variants contain variables

    //lets say we want to group all result messages into one structure. (Success, Warning, Error)
    //Also, lets consider we want to send a message and other data when result value is not success
    //For that case, we can use Enum to create our Message model:
    #[derive(Debug)]
    enum ResultMessage{
        Success,
        Warning{category:u32,message:String},
        Error(String),
    }

    //Above example shows that enum can contain Struct values which are Warning and Error

    //Lets go further and add another Enum inside Result message:
    #[derive(Debug)]
    enum HttpStatusCode{
        Success,
        BadRequest,
        UnAuthorized,
        Forbidden,
        NotFound,
        InternalServerError
    }

    #[derive(Debug)]
    struct ActionResult{
        StatusCode:HttpStatusCode,
        Message:ResultMessage,
    }
    

    let message = ActionResult{StatusCode:HttpStatusCode::BadRequest,Message:ResultMessage::Error("An error occurred".to_string())};
    println!("message: {:?}",message);



    //Lets create a scenario:
    //We take a number from user
    //If number is odd, status code and message  will be success
    //If number is even, status code will be bad request and message will be a warning,
    //If number is 0, status code will be UnAuthorized and Message will be an error
    //If given input is NOT a number, status code will be InternalServerError and Message will be an error

    

    let mut guess_game_result:ActionResult;
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()//use std::io;
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("You guessed: {guess}");

    // let guesss: Result<i32,ActionResult> = match guess.trim().parse::<i32>(){
    //     Ok(num) => Ok(num),
    //     Err(_) =>Err(ActionResult{StatusCode:HttpStatusCode::BadRequest,Message:ResultMessage::Success}),
    // };


    let guess = parse_guess(guess);

    match guess {
        Ok(num) => {
            if(num ==0){
                println!("{:?}",ActionResult{
                    StatusCode:HttpStatusCode::UnAuthorized,
                    Message:ResultMessage::Error("You typed 0 , please try again".to_string())
                })
            }
            else{
                match num % 2 {
                    1 => println!("{:?}",ActionResult{
                        StatusCode:HttpStatusCode::Success,
                        Message:ResultMessage::Success
                    }),
                    0 => println!("{:?}",ActionResult{
                        StatusCode:HttpStatusCode::BadRequest,
                        Message:ResultMessage::Warning { 
                            category: 1, 
                            message: "Please type an odd number".to_string() 
                        }
                    }),
                    _ => println!("{:?}",ActionResult{
                        StatusCode:HttpStatusCode::InternalServerError,
                        Message:ResultMessage::Error("An unexpected error occurred!".to_string())
                    })
                }
            }
        },
        Err(er) => println!("{:?}",ActionResult{
            StatusCode:HttpStatusCode::InternalServerError,
            Message:ResultMessage::Error("An unexpected error occurred!".to_string())
        })
    }
    
}
fn parse_guess(data:String) -> Result<i32,ParseIntError>{
    data.trim().parse::<i32>()
}

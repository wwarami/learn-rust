fn main() {
    // this variable is immutable and can not change
    let name = "bro";
    // this variable is mutable and can change
    let mut age = 17;

    println!("Hello {name} you are {age} years old.");
    // here we change age
    age = 18;
    println!("Now you are {age} years old.");

    // and here is a constant variable which can is
    // gonna be same over time
    // note that we should provide the type for a constant
    const IS_MAN: bool = true;
    println!("is man: {IS_MAN}");


    // shadowing
    let day = 1;
    println!("day is now {day}");
    
    let day = 2; // here we shadow the first day
    println!("new day is now {day}");

    // scopes 
    {
        let day = 3;
        println!("in inner scope day is now {day}");
    }

    println!("outer scope day is now {day}");

    // the deference between mutable variables and shadowing
    // is by shadowing each time we create new variables
    // but with mutable variables we just assign new values
    

}

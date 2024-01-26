fn main() {
    let mut number = 5;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("last iter {}", number);


    // loop over a collection 
    let a = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}" , a[index]);
         index = index + 1
    }

}

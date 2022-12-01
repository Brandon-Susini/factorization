//Take a number, split it into each possible tuple of factor pairs that exist for it
//For all numbers N, N = (a x b), where a x b are both real whole numbers whos product is N
//If N is prime, result = (N x 1)
//If N is not prime, result equals (a, b)

use std::env;
struct FactorData{
    input: i32
}
fn main() {
    let f = FactorData{input:10};
    println!("Factors: {:?}", get_factors(&f.input));
}

fn get_factors(n: &i32) -> Vec<(i32,i32)>{
    let mut factors: Vec<(i32,i32)> = Vec::new();
    let mut counter = *n - 1;
    while counter >1{
        //If n is divisible by counter, and factor pair is not already in factors vector.
        if n % counter == 0 && !factors.contains(&(*n/counter,counter)){
            //No remainder means this is a factor
            //Divide by counter, and return the value of counter and quotient.
            let tuple:(i32,i32) = (counter, *n/counter);
            factors.push(tuple);
        }
        counter -= 1;
    }
    return factors;
}



/* fn get_factors_rec(n: &u16) -> (u16,u16){
    if(n == 2){
        //Base case is two because that is the smallest prime number
        //Any number lower, will not have any factorizations at all
        //Return 2, 1
        return (2,1);
    }
    //Otherwise, call get_factors_rec on the next value where there is no 
    //remainder in the division
    //WE LITERALLY cant do trivial factorizations
    
    return (1,1);
}
 */
// use std::rc::Rc;

/* The branches in this function exhibit Rust's optional implicit return
   values, which can be utilized where a more "functional" style is preferred.
   Unlike C++ and related languages, Rust's `if` construct is an expression
   rather than a statement, and thus has a return value of its own. */
fn recursive_factorial(n: int) -> int {
    if n <= 1 { 1 }
    else { n * recursive_factorial(n-1) }
}
 
fn iterative_factorial(n: int) -> int {
    // Variables (or more correctly, bindings) are declared with `let`.
    // The `mut` keyword allows these variables to be mutated.
    let mut i = 1;
    let mut result = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    return result; // An explicit return, in contrast to the prior function.
}
 
// struct Blub {
//     a : int,
//     b : int,
//     // other : Rc<RefCell<Blub>>
// }
 /*
impl Drop for Blub {
    fn drop(&mut self) {
      println!( "drop\n" );
      
    }
    
}
 */
 
fn main() {
    println!("Recursive result: {:i}", recursive_factorial(10));
    println!("Iterative result: {:i}", iterative_factorial(10));

    let s : (~str) = ~"blub";
    let sr : (&str) = s; // this works -> ok to borrow from owned box

    let st = (~"blub", 1i );
    let (st_r, i) : (&str, int) = st; // compiler error: mismatched types: expected `(&str,int)` but found `(~str,int)` (str storage differs: expected `&` but found `~`)

    println!( "{} {} {} {}", s, sr, st_r, i);
}

// compute FIbonacci number

// # panics
// panics on 0

//use std::result;


// examples
// fib 1 = 1
// fib 15 =
fn fib(n:u32)->u64{
    
    // wr0ng? should be 0=>1, 1=>1
    match n {
        0 => panic!("fib 0"),
        1 => 1,
        2 => 1,
        n => fib(n-1)+fib(n-2),
    }
}


fn strings(){
    let targets = [
        "world",
        "class",
        "you",

    ];
    for t in targets {
        let  t = t.to_string();
        println!("helo, {} ", t);
    }
}



// run w cargo test
#[test]
fn test_fib(){
    assert_eq!(fib(15),610);
    assert_eq!(fib(16),987);
    
}


fn arrays(a: [String;2]){
    for b in a {
        println!("{}", b);
    };
}


// 1/23/2025
// look at const vs static
const SIZE: usize = 10_000; // 100_000 makes my thing overflow
#[derive(Clone,Copy)]
struct S{
    contents:[u64;SIZE]

}

impl S {
    fn new() ->Self{
        Self {contents: [0;SIZE]}
    }
}

fn fs(s:S){
    println!("{}",s.contents[SIZE-1]);
}


/*
lets say
m = storage;
let n = m; // m's storage is transfered to n, m has nothing now?
all values at all times can only have one owner, it can be transfered tho

one way of init array of size 10
let mut result = Vec::with_capacity(10);

look into std::rc,
like let s = Rc::new(S::new()) // runtime checker checks how many refs this has,
                          // if 0, drops it

}




 */
fn main() {
    //println!("Hello, world!");
    println!("fib 16 = {}", fib(16));
    strings();
    
    let a: [String;2] = ["x".to_string(),"z".to_string()];
    arrays(a);

    let s =S::new();
    fs(s);
    //fs(s.clone());

    //fs(s);

    

}

use std::process;

fn main(){
 
  let mut r = 0;

  let mut sum = 0;
  
  let mut n = get_input().trim().parse::<i64>().unwrap();
  
  let temp = n;
  

  while n > 0 { 

    r=n%10; 

    sum=sum+(r*r*r);  

    n=n/10; 
    
    }

    if temp==sum {

      print!("This Number is an Armstrong  Number",);

    }
    else{

      print!("This Number is Not an Armstrong  Number",);

    }

    process::exit(1);
}

fn get_input() -> String {
    
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).expect ("Failed");

    buffer
}
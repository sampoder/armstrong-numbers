use std::process;

fn main(){
 
  let mut r = 0;

  let mut sum = 0;
  
  let mut n = get_input().trim().parse::<i64>().unwrap();
  
  let temp = n;

  let mut x = n;

  let mut b = 0;
  
  while x > 0 { 

    x/=10; 

    b+=1;
    
    }


  while n > 0 { 

    r=n%10; 

    let mut bb = b;

    let mut rt = r;

    while bb > 1 {rt*=r;bb-=1;}

    sum=sum+(rt);  

    n=n/10; 
    
    }

    if temp==sum {

      print!("This Number is an Armstrong  Number",);

    }
    else{

      print!("This Number is Not an Armstrong  Number",);

    }

   
}

fn get_input() -> String {
    
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).expect ("Failed");

    buffer
}

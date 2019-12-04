

#[macro_use] extern crate text_io;

fn main(){

    // print!("Enter a number:",);
   
    let mut r;
  
    let mut sum = 0;

    let mut n: i32 = read!();
    
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
  
        print!("This Number is an Armstrong  Number\n",);
  
      }
      else{
  
        print!("This Number is Not an Armstrong  Number\n",);
  
      }
  
     
  }
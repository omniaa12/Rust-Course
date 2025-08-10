fn main() {
   if 5==1
   {println!(" 5 is = 1");}
    else if 1==1
    {println!(" 1 is = 1");}
   else
   {println!(" 5 is ! = 1");}

   let x=if 5==1{10}
   else {20};
   println!("{}",x);

     isEven(60);
     println!("{}", isOdd(60));

    

}

fn isEven (number:i32)
{
    if number%2==0
    { println!(" number is {} evrn",number);}
    else{ println!(" number is {}odd",number);}
}

fn isOdd(number:i32)->bool
{number%2==0}

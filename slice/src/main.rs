fn main() {
   let nam=String::from("Omnia Najaty");
   let firstName=&nam[0..6];
   let lasttName :&str=&nam[7..];
   println!("{}",firstName);
   println!("{}",lasttName);

   let res =first_word(&nam);
    println!("{}",res);

let name :&str ="omnia najaty";
let namr:String=String::from("Menna Tawfeek"); 

let mut num[i32]=[2,8,4,6];
mult($ mut num);
}


fn first_word(s: &String) -> &str {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return &s[..i]; // slice من أول النص لحد قبل المسافة
        }
    }
    &s[..] // لو مفيش مسافة، يرجع النص كله
}

fn mult( num:$ mut[i32])->$[i32]
{
    for (number in num)
    {
      *number*=2;
    }
}

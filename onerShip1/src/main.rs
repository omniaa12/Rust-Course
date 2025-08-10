fn main() {
  let mut nam =String::from("najaty");
  nam.push_str(" omnia");

  println!("{}",nam);

  let otherNam=nam;
  // println!("{}",nam); error
   println!("{}",otherNam);

  let mut otheOtherNam=otherNam.clone();
    println!("{}",otherNam);
     println!("{}",otheOtherNam);
     
    otheOtherNam.push_str("Omnia Najaty");
    outer.push_str("Ahmed");
    println!("{}",otherNam);
     println!("{}",otheOtherNam);

     let age=21;
    taskeOnerShip(otherNam);
    makeCopy(age);
   

}

fn taskeOnerShip(somString:String)
{
    println!("{}",somString);
}

fn makeCopy(somInt:i32)
{
    println!("{}",somInt);
}


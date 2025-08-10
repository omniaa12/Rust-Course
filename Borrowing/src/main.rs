fn main() {
    let name= String::from("Omnia");
    let mut  len=calc(name);
    println!("{}",len.0);
    println!("{}",len.1);

   

}

fn calc (s:String) -> (usize,String)
{
   (s.len(),s)
}
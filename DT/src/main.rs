fn main() {
    // int , float , double , bool, char 
    let mut age :i32=21;

    //i8,i16,i32,i64,i128
    //u8,u16,u32,u64,u128
    //isize , usize 
    //f32 ,f64

    let age :u16=15;
     let char_value = b'A';

     println!("{}",char_value);
     println!("{}",char_value as char);

     let legnth=3.9f32;

     //bool
     let x= true;
     let y= false & true;
     let z= false | true;
     println!("{} {} {}", x,y,z);

     // tuple , arrays

     //let myData:(i32,char,bool)=(32,'A',true);
     let myData=(32,'A',true);

     println!("{:?} {}",myData,myData.1);

     let  unit_type:()=();

     //array 
     let my_array=[1,2,3,5];
      println!("{}", my_array[3]);
     
      
      let arr:[i32;4]=[1,5,8,9];
      println!("{}", arr[3]);


      let userIndex  env::=var("user_index").parse()
      .expect("expect number in user index ");

      

     
}

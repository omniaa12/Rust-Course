fn main() {
    let mut c = 0;
    loop {
        println!("hi");
        c += 1;
        if c == 5 {
            break;
        }
    }

    let mut n = 0;
    let res = loop {
        n += 1;
        if n == 5 {
            break 20;
        }
    };
    println!("res = {}", res);

    // 'mainLoop: loop {
    //     let mut c = 0;
    //     'innerLoop: loop {
    //         println!("Inner Loop");
    //         c += 1;
    //         if c == 3 {
    //             break 'mainLoop;
    //         }
    //         break ;
    //     }
    //     println!("outer Loop");
    

     let mut num=3;
     while num !=0
    {
         println!("number is {}", num);
         num-=1;
    }
}


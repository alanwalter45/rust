fn main(){
    let mut count = 0;
    'loop1: loop{
        count += 1;
        println!("L1: {}",count);
        let mut count2 = 0;
        'loop2: loop{
            count2+=1;
            println!("L2: {}",count2);
            if count2 >=10{
                break 'loop1;
            }
        }
    }
}
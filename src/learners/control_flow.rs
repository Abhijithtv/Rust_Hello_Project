use crate::helpers::display_helper;

const CONTEXT:&str = "CONTROL-FLOW";

pub fn learn_control_flow(){
    display_helper::start(CONTEXT);
    learn_if();
    learn_loop();
    display_helper::end(CONTEXT);
}

fn learn_loop(){
    let arr = &[1, 2, 3, 4, 5];

    //for loop
    {
        let mut sum = 0;
        for val in arr{
            sum += val;
        }
        println!("sum via for loop - {sum}");
    }

    //while
    {
        let mut sum = 0;
        let mut i = 0;

        while i<arr.len(){
            sum += arr[i];
            i+=1;
        }
        println!("sum via while loop - {sum}");
    }

    //loop
    {
        let mut sum = 0;
        let mut i = 0;

        loop{
            if i>=arr.len() {
                break;
            }
            sum += arr[i];
            i+=1;
        }
        println!("sum via loop - {sum}");
    }


    //break loop by name
    {
        let mut i = 0;
        let mut sum = 0;
        'loop1 : loop{
            'loop2 : loop {
                if sum>9
                {
                    break 'loop1;
                }
                if i >= arr.len(){
                    break  'loop2;
                }
                sum += arr[i];
                i+=1;
            }
        };
         println!("sum via named loop exit - {sum}");
    }
}

fn learn_if(){
    let x = 14;

    if x%2==0{
        println!("{x} is even");
    }
    else{
        println!("{x} is odd");
    }

    //another way
    let is_good = if x%2==0 { "I am good" } else { "not good"};
    println!("{x} is {is_good}");
}


fn main() {
    let mut user_input = String::new();

    println!("Please input a number:");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input");

    let user_num:i32 = user_input
        .trim()
        .parse()
        .expect("failed to parse string into i32");

    let mut counter:i32 = 0;
    let mut running_sum:i32 = 0;
    let square_of_sums = loop {
        counter +=1;
        running_sum += counter;
        if counter == user_num{
            break running_sum.pow(2);
        }
    };



    let mut counter:i32 = 0;
    let mut running_sum:i32 = 0;
    let sum_of_squares = loop {
        counter += 1;
        running_sum += counter.pow(2);
        if counter == user_num{
            break running_sum;
        };
    };


    println!("The difference between {} and {} is {}", square_of_sums, sum_of_squares, square_of_sums-sum_of_squares);


}

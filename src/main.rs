use std::io;

fn main()
{
    println!("Calculator:");

    //starting values
    let mut x1 = String::new();
    io::stdin()
        .read_line(&mut x1)
        .expect("failed to read from stdin");
    let mut value1 = x1.trim().parse::<u32>().unwrap();
    let mut result: u32;

    loop
    {
        //input sign
        let mut sign1 = String::new();
        io::stdin()
        .read_line(&mut sign1)
        .expect("failed to read from stdin");
        let sign: char = sign1.trim().parse::<char>().unwrap();

        //input value2
        let mut y1 = String::new();
        io::stdin()
        .read_line(&mut y1)
        .expect("failed to read from stdin");
        let value2 = y1.trim().parse::<u32>().unwrap();
        
        //counting
        match &sign
        {
            '+' => result = value1 + value2,
            '-' => result = value1 - value2,
            '*' => result = value1 * value2,
            '/' => result = value1 / value2,
            '%' => result = value1 % value2,
            _ => break,
        }

        //result
        println!("Result:{}", result);
        value1 = result;
    }
}

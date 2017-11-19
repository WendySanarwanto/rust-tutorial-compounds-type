fn main() {
    println!("\nRust Compound Data Type's Demo.");
    println!("===============================\n");

    let tupple = (500, 6.4, 1);
    println!("Given, a tupple is {:?}\n", tupple);

    println!("Content of the tupple on tupple.0 is {}; on tupple.1 is {}; on tupple.2 is {}\n", tupple.0, tupple.1, tupple.2);

    let (x, y, z) = tupple;
    println!("Destructed the tupple into:\nx = {}\ny = {}\nz = {}\n", x, y, z);

    println!("---------------------------------\n");
    let array_num = [1, 2, 3, 4, 5];
    println!("Given, an array of number is {:?}\n", array_num);
    println!("The result of the addition between 1st & 3rd elements is {}\n", array_num[0] + array_num[2]);
}

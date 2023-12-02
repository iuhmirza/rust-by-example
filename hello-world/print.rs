fn main() {
    println!("{} lectures", 8);

    println!("{0}, this is {1}. {1}, this is {0}.", "Harold", "John");

    println!("{subject} {verb} {object}", subject="the man", verb="who sold", object={"the world"});

    println!("Binary: {:b}", 0xDEAD);
    println!("Hexadecimal: {:x}", 151251386);

    println!("Padded {number:0>5}", number=1);

    let pi = 3.141592;
    println!("Pi is roughly {pi:.4}")
}
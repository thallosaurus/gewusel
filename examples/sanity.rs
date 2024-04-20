fn main() {
    let x = 44;
    let y = 44;
    let width = 180;
    let height = 90;
    println!("{}", x > width * -1);
    println!("{}", x < width);
    println!("{}", y > height * -1);
    println!("{}", y < height);

    if (x > width * -1 && x < width) && (y > height * -1 && y < height) {
        println!("Safe to go");
    } else {
        println!("Not Safe to go");
    }
    /*if (x < (self.width * -1) && x < self.width) && (y < (self.height * -1) && y < self.height) {
        return None;
    }*/
}
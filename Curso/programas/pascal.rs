fn main() {
    const SIZE: usize = 10;
    const MAX: usize = SIZE+1;
    let mut pascal = [[0; MAX]; MAX];
    
    for col in 0..MAX {
        for row in 0..MAX {
            if (col + row) >= MAX { continue };
            if (col > 0) && (row > 0) {
                pascal[col][row]  = pascal[col - 1][row] + pascal[col][row - 1];
            } else {
                pascal[col][row] = 1;
            }
            
            print!("{:5}", pascal[col][row]);
        }
        println!("\n");
    }
}

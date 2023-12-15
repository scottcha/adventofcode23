use std::time::Instant;
use std::io::{self, BufRead};

fn main() -> io::Result<()>{
    // Iteration: 17761 load: 85117
    // Iteration: 17762 load: 85133
    // Iteration: 17763 load: 85157
    // Iteration: 17764 load: 85192
    // Iteration: 17765 load: 85224
    // Iteration: 17766 load: 85245
    // Iteration: 17767 load: 85260
    // Iteration: 17768 load: 85270
    // Iteration: 17769 load: 85261
    // Iteration: 17770 load: 85247
    // Iteration: 17771 load: 85223
    // Iteration: 17772 load: 85206
    // Iteration: 17773 load: 85175
    // Iteration: 17774 load: 85155
    // Iteration: 17775 load: 85125
    // Iteration: 17776 load: 85115
    // Iteration: 17777 load: 85111
    // Iteration: 17778 load: 85120
    // Iteration: 17779 load: 85130
    // Iteration: 17780 load: 85160
    // Iteration: 17781 load: 85189
    // Iteration: 17782 load: 85227
    // Iteration: 17783 load: 85242
    // Iteration: 17784 load: 85263
    // Iteration: 17785 load: 85267
    // Iteration: 17786 load: 85264
    // Iteration: 17787 load: 85244
    // Iteration: 17788 load: 85226
    // Iteration: 17789 load: 85203
    // Iteration: 17790 load: 85178
    // Iteration: 17791 load: 85152
    // Iteration: 17792 load: 85128
    // Iteration: 17793 load: 85112
    // Iteration: 17794 load: 85114
    //store the above in an array
    let mut arr = [85117, 85133, 85157, 85192, 85224, 85245, 85260, 85270, 85261, 85247, 85223, 85206, 85175, 85155, 85125, 85115, 85111, 85120, 85130, 85160, 85189, 85227, 85242, 85263, 85267, 85264, 85244, 85226, 85203, 85178, 85152, 85128, 85112, 85114];
    let mut j = 0;
    let n = 1000000000;
    for i in 930..n {
        if i > n-40
        {
            println!("Iteration: {} load: {}", i, arr[j%arr.len()]);
        }
        j += 1;
    }
Ok(())
}
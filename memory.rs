/* NOTES:
 * Rust was created to make the handling of memory easier
 * for programmers, so I decided to utilize an array example
 * from a tutorial to showcase what Rust can do as arrays
 * are a staple data type that cannot be used unless the user
 * knows at least something about how memory works.
 *
 * In other words, you cannot use arrays in programming
 * for anything interesting unless you know how they work
 * in memory, especially with pointers.
 */


//input is list containing 32-bit integers
//return is a single 32-bit integer
//function returns sum of the array
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    //in Rust, rather than saying "return res;"
    //you simply put the variable you wish to return
    //without a semicolon
    res
}

fn main() {
    let arr = [10, 20, 30, 40];

    //print all values of array, then the lenght of the array
    //showcasing for loop with range
    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());

    //giving the address of the array to the function
    //memory usage
    let res = sum(&arr);
    println!("sum {}", res);
}
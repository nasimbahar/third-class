
pub fn starting_point(){
    sliceEntireRecord();
    SliceRange();
    immutableSlice();
   
}

fn sliceEntireRecord() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..]; // Borrowing the whole array

    println!("The slice contains: {:?}", slice);
}
fn SliceRange() {
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..2]; // Slice from index 1 to 3 (20, 30, 40)

    println!("The slice contains: {:?}", slice);
}

fn immutableSlice() {
    let static_slice: &'static [i32] = &[1, 2, 3, 4, 5];
    // This is a 'static slice, stored directly in the binary
    //let array: [i32; 5] = [1, 2, 3, 4, 5];
//let static_slice: &'static [i32] = &array;


    println!("The static slice contains: {:?}", static_slice);
}
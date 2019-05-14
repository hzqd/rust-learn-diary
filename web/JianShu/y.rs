//rust--array和slice的区别

// 首先array和slice的表现形式都是[], 但是它们却截然不同.
// array 在rust中是固定大小的, 定义好了之后就不能新增或删除array中的元素.
// 而slice是未知大小的, 因此一般情况下slice都必须要结合&来使用.

// 表现形式:
// array: [T; N]
// slice: [T]

// slice的几种表现形式:
// &[T]:        'share slice', often just called 'slice'.
// &mut [T]:    'mutable slice'
// Box<[T]>:    'boxed slice'


fn array_example() {
    let s: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", s);
}


fn slice_example() {
    let s: [i32; 4] = [1, 2, 3, 4];
    let y: &[i32] = &s[0..2];
    println!("{:?}", y);
}


fn main() {
    array_example();
    slice_example();
}
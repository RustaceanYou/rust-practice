// fn main() {
//     let a: f64 = 1.0;
//     // a = 2.0;
//     println!("{:?}", a);
//     println!("Hello, world!");
// }

// fn main() {
//     let mut a1: f64 = 1.0;
//     let _b = 2.0f32;
//     drop(a1);

//     //改变 a 的绑定
//     a1 = 2.0;
//     println!("{:?}", a1);

//     //重新绑定为不可变
//     // let a = a;

//     //不能赋值
//     //a = 3.0;

//     //类型不匹配
//     //assert_eq!(a, b);
// }

/**
 * let 解构
 */
// fn main() {
//     let (a, mut b): (bool,bool) = (true, false);
//     println!("a = {:?}, b = {:?}", a, b);
//     //a 不可变绑定
//     //a = false;

//     //b 可变绑定
//     b = true;
//     assert_eq!(a, b);
// }

// 变量
fn main () {
    // boolean type
    let t = true;
    let f: bool = false;

    // char type
    let c = 'c';

    // numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex = 0xf23a_b049;

    // string types
    let str = "Hello, world!";
    let mut string = str.to_string();

    // arrays and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    // functions
    fn foo(x: i32) -> i32 { x }
    let bar: fn(i32) -> i32 = foo;
}
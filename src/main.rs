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
// fn main () {
    // boolean type
    // let t = true;
    // let f: bool = false;

    // char type
    // let c = 'c';

    // numeric types
    // let x = 42;
    // let y: u32 = 123_456;
    // let z: f64 = 1.23e+2;
    // let zero = z.abs_sub(123.4);
    // let bin = 0b1111_0000;
    // let oct = 0o7320_1546;
    // let hex = 0xf23a_b049;

    // string types
    // let str = "Hello, world!";
    // let mut string = str.to_string();

    // arrays and slices
    // let a = [0, 1, 2, 3, 4];
    // let middle = &a[1..4];
    // let mut ten_zeros: [i64; 10] = [0; 10];

    // tuples
    // let tuple: (i32, &str) = (50, "hello");
    // let (fifty, _) = tuple;
    // let hello = tuple.1;

    // raw pointers
    // let x = 5;
    // let raw = &x as *const i32;
    // let points_at = unsafe { *raw };

    // functions
    // fn foo(x: i32) -> i32 { x }
    // let bar: fn(i32) -> i32 = foo;
// }

// 数组
// fn main () {
//     // arrays and slices
//     // Rust 使用数组存储相同类型的数据集。 [T; N]表示一个拥有 T 类型，N 个元素的数组。数组的大小是固定
//     let a = [0, 1, 2, 3, 4];
//     let middle = &a[1..4];
//     // 遍历数组下标从1~4的数组元素
//     for x in middle {
//         println!("{} ", x);
//     }
//     // 遍历数组下标所有元素
//     for x in &a {
//         println!("{}", x)
//     }
//     // 比较数组前两个元素的大小
//     assert_eq!([0, 1], &a[0..2]);

//     // 动态数组 vec
//     // 创建空的 vec
//     let _v: Vec<i32> = Vec::new();
//     // 使用宏创建空的 Vec
//     let _v: Vec<i64> = vec![];
//     // 创建五个元素的 vec
//     let _v = vec![0,1,2,3,4];
//     for x in &_v {
//         println!("{}", x);
//     }
//     // 创建可变数组 vec
//     let mut _p = vec![10, 11];
//     _p.push(12);
//     for m in &_p {
//         println!("{}", m);
//     }
    // 创建两个元素的vec，并弹出一个元素
    // let mut _q = vec![20, 21];
    // _q.pop();
    // for q in &_q {
    //     println!("{}", q);
    // }
    // 创建三个元素的vec，并索引一个值，修改一个值
    // let mut _m = vec![30, 31, 32];
    // let _m_value =  _m[0];
    // _m[1] = _m_value + 3;
    // for m in &_m {
    //     println!("{}", m);
    // }
// }

// 字符串
// fn main () {
//     // 字符串字面值
//     let _str = "hello world";
//     println!("{}", _str);
//     // 附带显式类型标识
//     let _public_str: &'static str = "public hello world";
//     println!("{}", _public_str);
//     // 创建一个空的字符串
//     let _empty_string = String::new();
//     println!("{}", _empty_string);
//     // 从 `&str` 类型转化成 `String` 类型
//     let mut _hello = String::from("hello, ");
//     // 压入字符
//     _hello.push('d');
//     // 压入字符碎片
//     _hello.push_str(" world");
//     println!("{}", _hello);
//     // 弹出字符
//     let mut s = String::from("foo");
//     assert_eq!(s.pop(), Some('o'));
//     assert_eq!(s.pop(), Some('o'));
//     assert_eq!(s.pop(), Some('f'));
//     assert_eq!(s.pop(), None);
// }
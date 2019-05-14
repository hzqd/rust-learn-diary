//rust数据结构
//在rust中被广泛使用的三种数据结构：Vector、String和Hash Map，下面将简要记录他们的部分操作方法。
 
//Vector
//vector是一个列表类型的数据。
//vector只能存储相同类型的值。
//vector在内存中彼此相邻的排列所有的值。


//定义一个Vec的几种形式

//# 全量写法
fn main() {
    let v: Vec<i32> = Vec::new();
}

//# 宏写法
fn main() {
    let v = vec![];
}

//# 数字默认值, 泛型自动识别元素类型
fn main() {
    let v = vec![1, 2, 3, 4, 5]
}

//# 字符窜默认值, 泛型自动识别元素类型
fn main() {
    let v = vec!["a", "b", "c", "d"]
}

//添加元素(按顺序的往后添加)
fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.push("e");
    v.push("f");
    v.push("g");
    println!("{:?}", v)
}

//插入元素(按指定元素位置插入)
fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.insert(0, "e");
    println!("{:?}", v);    // ["e", "a", "b", "c", "d"]
}

//移除元素(按顺序的从后删除)
fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.pop();
    println!("{:?}", v);  // ["a", "b", "c"]
}

//移除元素(按指定位置移除)
fn main() {
    let mut v = vec!["a", "b", "c", "d"];
    v.remove(2);            // 从0开始数, 2表示第三个元素.
    println!("{:?}", v);    // ["a", "b", "d"]
}

//利用分片(slice)读取元素
fn main() {
    let v = vec!["a", "b", "c", "d", "e", "f"];
    println!("{:?}", v[3])    // "d"
}

//# 分片超出范围, 会报错(panic)并退出程序.
fn main() {
    let v = vec!["a", "b", "c", "d", "e", "f"];
    println!("{:?}", v[100])
}

//利用get方法读取元素
fn main() {
    let v = vec!["a", "b", "c", "d", "e", "f"];
    println!("{:?}", v.get(3))  // "d"
}

//# get超出范围并不会报错，而是返回None
fn main() {
    let v = vec!["a", "b", "c", "d", "e", "f"];
    println!("{:?}", v.get(100))
}

//循环遍历读取
fn main() {
    let v = vec!["a", "b", "c", "d"];
    for i in &v[0..4] {
        println!("{}", i);
    }
}

//# 另外一种写法
fn main() {
    let v = vec!["a", "b", "c", "d"];
    for i in &v[0..v.len()] {   // len(): 获取元素总个数
        println!("{}", i);
    }
}
 
 
//String

//String存储在堆(Heap)上，可以在运行时动态增减String的内容.
//String于Vector的方法相似, 因为String是在Vector的基础上进行的二次封装.
//String分片并非统一标准, 它取决于字符对应在Unicode编码表中的占位长度，例如: ASCII每个字符占1个字节、Literal每个字符占2个字节、象形文字每个字符占3个字节，因此每个不同形态的String分片都要采用不同的size.


//定义字符串
fn main() {
    let s = String::new();  // 空字符串对象
}

//定义默认值字符串
fn main() {
    let s = String::from("hello world!")
}

//定义默认值字符串(由literals转成字符串)
fn main() {
    let ic = "initial contents";
    let s = ic.to_string();   // 等同于String::from
}

//追加内容
fn main() {
    let mut s = String::new();
    s.push_str("hello");
    s.push_str(" world!");
    println!("{}", s)       // hello world!
}

//插入内容
fn main() {
    let mut s = String::from("hello world!");
    s.insert_str(3, " zt ");
    println!("{}", s)       // hel zt lo world!
}

//移除元素(按顺序从末尾移除)
fn main() {
    let mut s = String::new();
    s.push_str("hello world!");
    s.pop();
    s.pop();
    println!("{}", s)       // hello worl
}

//移除元素(按指定位置移除)
fn main() {
    let mut s = String::new();
    s.push_str("hello world!");
    s.remove(5);
    println!("{}", s)       // helloworld!
}

//分片读取元素(Unicode.ASCII: bytes/字节)
fn main() {
    let hello = "abcdefg1234567!@#$%^&";
    // ASCII 在Unicode中以1个字节为单位存储，
    // 因此最小分片单位间隔为1。
    let a = &hello[0..1];
    let b = &hello[1..2];
    let c = &hello[2..10];
    println!("{:?} {:?} {:?}", a, b, c);    // 输出: "a" "b" "cdefg123"
}

//分片读取元素(Unicode.Literal: scalar values/标量值)
fn main() {
    let hello = "Здравствуйте";             // 俄罗斯语: 你好
    // Literal 在Unicode中以2个字节为单位存储，
    // 因此最小分片单位间隔为2。
    let a = &hello[0..2];
    let b = &hello[2..4];
    let c = &hello[4..10];
    println!("{:?} {:?} {:?}", a, b, c);    // 输出: "З" "д" "рав"
}


fn main() {
    let hello = "السلام عليكم";             // 阿拉伯语: 你好
    // Literal 在Unicode中以2个字节为单位存储，
    // 因此最小分片单位间隔为2。
    let a = &hello[0..2];
    let b = &hello[2..4];
    let c = &hello[4..10];
    println!("{:?} {:?} {:?}", a, b, c);    // 输出: "ا" "ل" "سلا"
}

//分片读取元素(Unicode.Char: grapheme/象形)
fn main() {
    let hello = "今天是晴天";             // 中文
    // Char 在Unicode中以3个字节为单位存储，
    // 因此最小分片单位间隔为3。
    let a = &hello[0..3];
    let b = &hello[3..6];
    let c = &hello[6..15];
    println!("{:?} {:?} {:?}", a, b, c);    // 输出: "今" "天" "是晴天"
}

//自动识别Unicode存储单位
fn main() {
    // ASCII: 字节值
    let hello = "abcdefg1234567!@#$%^&";
    for i in hello.chars() {
        // 输出:
        // 'a', 'b', 'c', 'd', 'e', 'f', 'g',
        // '1', '2', '3', '4', '5', '6', '7',
        // '!', '@', '#', '$', '%', '^', '&'
        println!("{:?}", i)
    }

    // 俄罗斯语: 标量值
    let hello = "Здравствуйте";
    for i in hello.chars() {
        // 输出:
        // 'З', 'д', 'р', 'а', 'в', 'с', 'т'
        // 'в', 'у', 'й', 'т', 'е'
        println!("{:?}", i)
    }

    // 中文: 象形文字值
    let hello = "今天是晴天";
    for i in hello.chars() {
        // 输出:
        // '今', '天', '是', '晴', '天'
        println!("{:?}", i)
    }
}
 
 
//Hash Map

//Hash Map是由一个或多个key和value组成.
//Hash Map的所有key必须是相同类型.
//Hash Map的所有value必须是相同类型.

//定义一个Hash Map
use std::collections::HashMap;

fn main() {

    let mut hm = HashMap::new();  // 定义一个HashMap（创建一个hashmap的实例）

    hm.insert("a", 10);           // HashMap不运行实例创建后是一个空的对象
    hm.insert("b", 20);           // 所以这里对hm实例对象进行了数据插入行为
    hm.insert("c", 30);

    println!("{:?}", hm)
}

//覆盖一个键值
use std::collections::HashMap;

fn main() {

    let mut hm = HashMap::new();  // 定义一个HashMap（创建一个hashmap的实例）

    hm.insert("a", 10);
    hm.insert("b", 20);
    println!("{:?}", hm);         // 输出: {"a": 10, "b": 20}
    hm.insert("b", 30);           // 覆盖掉原来的"b"键
    println!("{:?}", hm)          // 输出: {"a": 10, "b": 30}
}

//当键不存在时插入
fn main() {

    let mut hm = HashMap::new();  // 定义一个HashMap（创建一个hashmap的实例）

    hm.insert("a", 10);
    hm.entry("a").or_insert(30);   // "a" 已存在, 所以这行代码并没有落实.
    println!("{:?}", hm);          // 输出: {"a": 10}
    hm.entry("b").or_insert(40);   // "b" 尚未存在, 所以会插入一个键为"b"和值为40的元素
    println!("{:?}", hm);          // 输出: {"b": 40, "a": 10}
}

//更新一个键值
use std::collections::HashMap;

// 数字
fn main() {

    let mut hm = HashMap::new();

    hm.insert("a", 10);            // 数字类型的值

    {                              // 开启一个临时作用域
        let a_ = hm.entry("a")     // 返回该键的值的可变引用
            .or_insert(0);
        *a_ += 10;                 // *解引用，默认情况下 &mut {integer} 是不允许做 += 操作的
    }                              // 退出作用域

    println!("{:?}", hm)           // 因为退出作用域, 所以就不会出现二次borrow问题
}


// 字符串
fn main() {

    let mut hm = HashMap::new();

    hm.insert(
        "a",
        String::from("hello")      // 字符串类型
    );

    {                              // 开启一个临时作用域
        let a_ = hm.entry("a")     // 返回该键的值的可变引用
            .or_insert(String::from("hello"));
        a_.push_str(" world!");
    }                              // 退出作用域

    println!("{:?}", hm)           // 因为退出作用域, 所以就不会出现二次borrow问题
}

//获取一个键值
use std::collections::HashMap;

fn main() {

    let mut hm = HashMap::new();

    hm.insert("a", 10);            // 数字类型的值
    hm.insert("b", 20);
    hm.insert("c", 30);

    println!("{:?}", hm.get("b"))  // 输出: Some(20)
}

//遍历HashMap
fn main() {

    let mut hm = HashMap::new();

    hm.insert("a", 10);            // 数字类型的值
    hm.insert("b", 20);
    hm.insert("c", 30);

    for i in &hm {
        println!("{:?}", i)        // 输出: ("a", 10), ("c", 30), ("b", 20)
    }

    for (key, value) in &hm {
        println!("{:?}", value)    // 输出: 20, 30, 10
    }
}
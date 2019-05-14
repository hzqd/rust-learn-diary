//rust模块
//库(crate)

//Rust中有四种库(crate)：core crate(核心库)、std crate(标准库)、binary crate(二进制库)、extern crate(外部库)，其中核心库的代码提供了很多特殊的方法让我们编写代码时无需显式导入就可以直接使用例如: println!、Some、enum、str、fn、impl等; 标准库和外部库代码是需要自行导入才能使用，它们两的区别是标准库由语言本身自带(由rust官方团队来维护)，而外部库库是一个外部公共库(由开发者们来自己维护).
//Rust中创建一个模块需要使用Cargo来完成；与之前利用Cargo来创建一个二进制项目代码(cargo new --bin PROJECT_NAME)所需提供的参数不同; Cargo默认情况下创建的并不是二进制项目代码，而是外部库(cargo new CRATE_NAME)。

 
 
//创建一个外部库
//# 创建一个二进制库
//[zhengtong@localhost ~]$ cargo new --bin learn_rust
//[zhengtong@localhost ~]$ cd learn_rust

//# 创建一个外部库
//[zhengtong@localhost learn_rust]$ cargo new communicator

//# 目录结构
//[zhengtong@localhost learn_rust]$ tree ../learn_rust
//../learn_rust/
//├── Cargo.toml
//├── communicator
//│   ├── Cargo.toml
//│   └── src
//│       └── lib.rs
//├── src
//│   └── main.rs

//# 查看样例代码
//[zhengtong@localhost learn_rust]$ cat communicator/src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


//上面这个代码片段创建了一个二进制库(learn_rust)和一个外部库(communicator)。
//binary crate：src/main.rs
//extern crate：src/lib.rs
//rust只允许binary crate 代码入口在src/main.rs中，
//rust只允许extern crate 代码入口在src/lib.rs中.

 
 
//定义一个模块
//
//备注：Rust允许函数块中不填写任何代码。
//
//[zhengtong@localhost learn_rust]$ vim communicator/src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

mod network {   // 模块: 不需要圆括号

    mod server {
        fn start() {
        }

        fn receive() {
        }

        fn send() {
        }
    }

    mod client {
        fn connect() {
        }

        fn send() {
        }

        fn receive() {
        }
    }

}

 
 
//公开模块
//
//Rust中所有模块默认都是私有的，需要指定pub之后才能被外部调用.
//
//[zhengtong@localhost learn_rust]$ vim communicator/src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod network {           // 公开模块

    pub mod server {        // 公开模块

        pub fn start() {    // 公开函数
            println!("start the server!")
        }

        pub fn receive() {  // 公开函数
            println!("receive data")
        }

        pub fn send() {     // 公开函数
            println!("send data back to client")
        }

    }

    pub mod client {

        pub fn connect() {
            println!("connect to server")
        }

        pub fn send() {
            println!("send data to server")
        }

        pub fn receive() {
            println!("receive data from server")
        }
    }

}


//在learn_rust二进制库中引用communicator外部库.
//
//# 编辑Cargo.toml配置文件
//[zhengtong@localhost learn_rust]$ vim Cargo.toml
//[package]
//name = "learn_rust"
//version = "0.1.0"
//authors = ["zhengtong"]
//
//[dependencies]
//communicator = { path = "./communicator" }  //添加这行



//# 引用communicator模块
//[zhengtong@localhost learn_rust]$ vim src/main.rs

extern crate communicator;  // 引入外部库


fn main() {
    communicator::network::server::start();  // 调用外部库的server模块的start函数.
    println!("hello world!")
}



//运行结果
//
//cargo run
//Compiling communicator v0.1.0 (file:///Users/zhengtong/PycharmProjects/learn_rust/communicator)
//Compiling learn_rust v0.1.0 (file:///Users/zhengtong/PycharmProjects/learn_rust)
//Finished dev [unoptimized + debuginfo] target(s) in 0.47 secs
//Running `target/debug/learn_rust`
//start the server!
//hello world!


 
 
/*将模块代码拆分到另外一个文件中
communicator/src/lib.rs
[zhengtong@localhost learn_rust]$ vim communicator/src/lib.rs*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod network;    // 这里发生变化

//communicator/src/network.rs
//[zhengtong@localhost learn_rust]$ vim communicator/src/network.rs

pub mod server {

    pub fn start() {
        println!("start the server!")
    }

    pub fn receive() {
        println!("receive data")
    }

    pub fn send() {
        println!("send data back to client")
    }

}

pub mod client {

    pub fn connect() {
        println!("connect to server")
    }

    pub fn send() {
        println!("send data to server")
    }

    pub fn receive() {
        println!("receive data from server")
    }
}


 
 
//深度拆分模块代码
//communicator/src/lib.rs
//[zhengtong@localhost learn_rust]$ vim communicator/src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod network;    // 这里发生变化

communicator/src/network/mod.rs
pub mod server;     // 这里发生变化

pub mod client;     // 这里发生变化

communicator/src/network/server.rs
pub fn start() {
    println!("start the server!")
}

pub fn receive() {
    println!("receive data")
}

pub fn send() {
    println!("send data back to client")
}

communicator/src/network/client.rs
pub fn connect() {
    println!("connect to server")
}

pub fn send() {
    println!("send data to server")
}

pub fn receive() {
    println!("receive data from server")
}
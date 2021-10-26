//导入std包
extern crate std;

//引用类库 io、net 用来完成TCP监听读取
use std::io::Write;
use std::net::{TcpListener, TcpStream, Shutdown};
// 引入 thread 类库用来多线程处理
use std::thread;


//定义handle_clinet
fn handle_client(mut stream: TcpStream) {
    //用512byte buffer
    let mut data = [0 as u8; 512];
    while match stream.write(&mut data) {
        Ok(size) => {
            //echo 所有
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_e) => {
            println!("有错误，终止程序：{}。", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}


fn main() {
    //定义一个请求地址
    let addr = "127.0.0.1.8080".to_string();
    //创建一个Tcp监听，通过字符串讲addr传入
    let listener = TcpListener::bind(&addr).unwrap();
    //调用incoming()方法接受客户端的链接信息，如果有新的信息进来返回Result
    for stream in listener.incoming() {
        //如果有客户端链接通过127.0.0.1.8080
        println!("debug::有新的链接进入，这行字就会打印------");
        //模式匹配
        match stream {
            //当Result类型匹配ok
            Ok(stream) => {
                //开启一个新的线程
                thread::spawn(move|| {
                    //将客户端处理信息给handle_client中，移交stream
                    handle_client(stream);
                });
                }
                //匹配错误
                Err(e) => {
                    panic!("出现错误 {:?}", e)
                }

            }
        }
        //关闭Tcp监听链接
        drop(listener);
    }

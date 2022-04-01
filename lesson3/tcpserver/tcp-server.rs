//导入std库中的网络通信类：tcp数据流，tcp监听器
use std::net::{TcpListener, TcpStream};
//导入线程类
use std::thread;
//导入std库中的io类，用于数据流、内存缓冲区的读写
use std::io::{Read, Write, Error};


// 客户端通信线程的处理函数， 每个客户端连接独立一个线程
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    //打印对方IP地址+端口
    println!("Incoming connection from: {}", stream.peer_addr()?);
    //定义接收缓冲区，初始化成512个0
    let mut buf = [0; 512];
    //死循环，等待数据，一旦有数据就读取全部数据，直到socket接收缓冲区中的数据全部被读出才退出。
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()); }
        //读到的数据全部写入流，发送给对方， 失败会返回错误
        stream.write(&buf[..bytes_read])?;
    }
}

//主函数
fn main() {
    //启动tcp监听服务器，绑定本机地址，监听端口是8888。如果出现绑定失败就提示异常信息
    let listener = TcpListener::bind("0.0.0.0:8888")
    .expect("Could not bind");

    //可能同时到达多个客户端连接请求，循环处理，每个连接建立一个单独的线程进行通信。
    for stream in listener.incoming() {
    // 检查socket连接请求成功或失败   
    match stream {
        Err(e) => { eprintln!("failed: {}", e) }
        //连接成功，
        Ok(stream) => {
            //创建一个线程，指定处理函数。处理函数执行失败会显示错误信息
            thread::spawn(move || {
                handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
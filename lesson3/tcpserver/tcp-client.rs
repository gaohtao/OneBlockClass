//导入std库中的网络通信io类、tcp数据流
use std::net::TcpStream;
//导入std库中str类
use std::str;
//导入std库中的io类，用于数据流、内存缓冲区的读写
use std::io::{self, BufRead, BufReader, Write};
 
fn main(){
    // 主动连接tcp服务器，服务器的监听端口8888，连接失败提示异常信息。成功返回tcp数据流
    let mut stream = TcpStream::connect("127.0.0.1:8888")
    .expect("could not connect to server");
 
    //死循环，直到用户输入exit才退出，结束客户端程序
    loop{
        //字符串作为用户输入缓冲区
        let mut input = String::new();
        //接收服务器返回信息的缓冲区
        let mut buffer: Vec<u8> = Vec::new();

        //标准IO读取一行字符串，直到换行符才结束。
        io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");
 
        //判断是否输入exit，是就跳出循环， 否则打印用户输入的信息
        match input.trim() == "exit"{
            true => break,
            _ => {
                println!("input is {}", input);
            },
        }
        
        // 另一种跳出循环的判断方法
        // if input.trim() == "exit"{
        //     println!("bye :-)");
        //     break;
        // }
        
        //字符串转为字节数组，写入数据流，发送给服务器。 如果写失败提示异常信息
        stream.write(input.as_bytes())
        .expect("faild to write to server");

        //定义缓冲区，用于接受服务器的返回信息，把tcp数据流包装成缓冲区流
        let mut reader = BufReader::new(&stream);
        //读取数据，直到换行符才结束。
        reader.read_until(b'\n', &mut buffer)
        .expect("Could not read into buffer");
        //收到的数据是字节数组，转换成字符串打印出来。转换失败提示异常信息
        print!("{}", str::from_utf8(&buffer)
        .expect("Could not write buffer as string"));
 
    }
}
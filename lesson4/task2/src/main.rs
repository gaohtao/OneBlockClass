/*  实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
 * 
 */

fn main() {

    //创建数据向量，填充数据
    let mut data_vec:Vec<u32> = Vec::new();
    for i in 1..101 {
        data_vec.push(i);
    }
    data_vec.push(0xFFFFFFF0);
    data_vec.push(0x0000000F);
    data_vec.push(0x00000001);
    println!("data_vec={:?}",data_vec);

    //求和无溢出
    let result = sum(&data_vec[0..100]);
    if result == None{
        println!("sum = none");

    }else {
        println!("sum = {}",result.unwrap());
    
    }
    //求和溢出，返回None
    match sum(&data_vec[100..]){
        None => println!("sum = none"),
        Some(val) => println!("sum = {}",val),
    }
}

//计算数组累加和，出现溢出就返回None
pub fn sum( data:&[u32]) -> Option<u32>{
    let mut result:Option<u32> = Some(0);
    for i in data{
        result = result.unwrap().checked_add(*i);
    }
    result
}


/* 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
*/ 

fn main() {
    println!("{:?}: duration time =  {}", TrafficLight::Red, TrafficLight::Red.get_duration_time());
    println!("{:?}: duration time =  {}", TrafficLight::Green, TrafficLight::Green.get_duration_time());
    println!("{:?}: duration time =  {}", TrafficLight::Yellow, TrafficLight::Yellow.get_duration_time());

}

#[derive(Debug)]
enum TrafficLight{
    Red,
    Green,
    Yellow,
}

//定义特型, 
trait Duration {
    //定义信号灯持续时间函数
    fn get_duration_time(&self) -> i32;

}

impl Duration for TrafficLight{
    fn get_duration_time(&self) -> i32{        
        match self {
            TrafficLight::Red =>  60,
            TrafficLight::Green =>  20,
            TrafficLight::Yellow =>  3,
        }
    }
}

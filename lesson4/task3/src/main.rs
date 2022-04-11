/* 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束，
*/


fn main() {

    //定义三种类型对象
    let mut circle = Circle::new( 1.0);
    let mut rectangle = Rectangle::new(3.0, 4.0);
    let mut triangle = Triangle::new(3.0, 4.0, 5.0);
    println!("{:?}, {:?}, {:?} ",circle, rectangle,triangle);

    //单独计算各自的面积
    println!("circle.area = {}", circle.calculate_area());
    println!("rectangle.area = {}", rectangle.calculate_area());
    println!("triangle.area = {}", triangle.calculate_area());


    //使用泛型函数计算面积
    all_shape_area(&mut circle);
    all_shape_area(&mut rectangle);
    all_shape_area(&mut triangle);

}

//定义特型
trait Calculate {
    //定义计算面积函数
    fn calculate_area(&mut self) -> f64;

}

//定义实现特型的三个具体类
#[derive(Debug)]
struct Circle {
    area : f64, 
    r : f64,
}
impl Circle{
    fn new(r:f64) -> Self {
        Self{area:0.0,r}
    }
}
impl Calculate for Circle {
    fn calculate_area(&mut self) -> f64 {
        self.area = self.r*self.r*std::f64::consts::PI;
        self.area
    }    
}

#[derive(Debug)]
struct Triangle {
    area : f64, 
    a : f64,
    b : f64,
    c : f64,
}
impl Triangle{
    fn new(a:f64,b:f64,c:f64) -> Self {
        Self{area:0.0,a,b,c}
    }
}
impl Calculate for Triangle {
    //根据三边长计算面积
    fn calculate_area(&mut self) -> f64 {
        self.area = 0.25* ((self.a+self.b+self.c)*(self.a+self.b-self.c)*(self.a+self.c-self.b)*(self.b+self.c-self.a)).sqrt();
        self.area
    }
}


#[derive(Debug)]
struct Rectangle {
    area : f64, 
    w : f64,
    h : f64,
}
impl Rectangle{
    fn new(w:f64,h:f64) -> Self {
        Self{area:0.0,w,h}
    }
}
impl Calculate for Rectangle {
    fn calculate_area(&mut self) -> f64 {
        self.area = self.w*self.h;
        self.area
    }
}

//定义实现泛型约束的函数
fn all_shape_area<T:Calculate>(t:&mut T) {
   println!("shape'area = {}", t.calculate_area());
}

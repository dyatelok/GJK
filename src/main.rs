use raylib::prelude::*;      
use euler::{Vec2,vec2,Vec3,vec3};

const origin : Vec2 = Vec2{ x :  800.0, y : 450.0};   
const screen : Vec2 = Vec2{ x : 1600.0, y : 900.0};  
const scale : f32 = 100.0;

#[derive(Clone,Copy)]
enum Shapes {
    Circ(Circle),
    Romb(Rombus),
}

impl Shapes {
    fn draw(&self, d : &mut RaylibDrawHandle) {
        match self {
            Shapes::Circ(Circle) => {Circle.draw(d);}
            Shapes::Romb(Rombus) => {Rombus.draw(d);}
        }
    }
    fn point_in_dir(&self, d : Vec2) -> Vec2 {
        match self {
            Shapes::Circ(Circle) => {Circle.point_in_dir(d)}
            Shapes::Romb(Rombus) => {Rombus.point_in_dir(d)}
        }
    }
}

#[derive(Clone,Copy)]
struct Circle {
    center : Vec2,
    radius : f32,
    color  : Color,
}

impl Circle {
    fn new(center : Vec2, radius : f32, color : Color) -> Circle {
        Circle {
            center,
            radius,
            color,
        }
    }
    fn draw(&self, d : &mut RaylibDrawHandle) {
        d.draw_circle((origin.x + self.center.x * scale) as i32,(origin.y - self.center.y * scale) as i32,self.radius * scale, self.color);
    }
    fn point_in_dir(&self,d : Vec2) -> Vec2 {
        self.center + d.normalize() * self.radius
    }
}

#[derive(Clone,Copy)]
struct Rombus {
    center : Vec2,
    diagonal : f32,
    color  : Color,
}

impl Rombus {
    fn new(center : Vec2, diagonal : f32, color : Color) -> Rombus {
        Rombus {
            center,
            diagonal,
            color,
        }
    }
    fn draw(&self, d : &mut RaylibDrawHandle) {
        d.draw_rectangle_pro(Rectangle{x     : origin.x + self.center.x * scale,     y      : origin.y - self.center.y * scale, 
                                       width : self.diagonal * 2_f32.sqrt() * scale, height : self.diagonal * 2_f32.sqrt() * scale},
                             Vector2{x : self.diagonal / 2_f32.sqrt() * scale, y : self.diagonal / 2_f32.sqrt() * scale}, 45.0 ,self.color);
    }
    fn point_in_dir(&self,d : Vec2) -> Vec2 {
        let pi = PI as f32;
        let mut ang = d.angle(vec2!(1.0,0.0));
        if d.y < 0.0 {ang = - ang;}
        if ang < -0.75 * pi || 0.75 * pi < ang  {return self.center + vec2!(-self.diagonal,0.0);}
        if -0.75 * pi <= ang && ang <= -0.25 * pi {return self.center + vec2!(0.0,-self.diagonal);}
        if -0.25 * pi <= ang && ang <=  0.25 * pi {return self.center + vec2!( self.diagonal,0.0);}
        if  0.25 * pi <= ang && ang <=  0.75 * pi {return self.center + vec2!(0.0, self.diagonal);}
        self.center
    }
}

fn point_in_dir(S1 : &Shapes, S2 : &Shapes,d : Vec2) -> Vec2 {
    S1.point_in_dir(d) - S2.point_in_dir(vec2!()-d)
} 

fn in_dir_of_center(A : Vec2, B : Vec2) -> Vec2 {
    let AB = vec3!(B.x-A.x,B.y-A.y,0.0);
    let AO = vec3!(-A.x,-A.y,0.0);
    let PerA = AB.cross(AO);
    let Res = PerA.cross(AB);
    vec2!(Res.x,Res.y).normalize()
}

fn GJK(S1 : Shapes, S2 : Shapes) -> bool {
    let mut A = point_in_dir(&S1,&S2,vec2!(1.0,0.0));
    let mut B = point_in_dir(&S1,&S2,vec2!()-A);
    loop {
        let dir = in_dir_of_center(A,B);
        let C = point_in_dir(&S1,&S2,dir);
        if dir.dot(C) < 0.0 {return false;}
        let dir1 = in_dir_of_center(A,C);
        let dir2 = in_dir_of_center(B,C);
        if dir1.dot(vec2!()-A) < 0.0 {B = C;} else {
            if dir2.dot(vec2!()-B) < 0.0 {A = C;} else {
                return true;
            }
        }
   }
}


fn main() {   
    let (mut rl, thread) = raylib::init()
        .size(screen.x as i32, screen.y as i32)
        .title("Collisions")
        .build();
    let backgroung = Color::WHITE;
    let axes = Color::BLACK;
    //Размер штрихов на осях 
    let shir = 1000.0;
    
    let c = Shapes::Circ(Circle::new(vec2!(1.0),1.0,Color::from((255,  0,  0,100))));
    let r = Shapes::Romb(Rombus::new(vec2!(2.0),1.0,Color::from((  0,  0,255,100))));
    
    let mut P : Vec<Vec2> = vec!();
    
    let mut t : f32 = 0.0;
   
    while t < 6.3 {
        P.push(point_in_dir(&c,&r,vec2!(t.cos(),t.sin())));
        t += 0.01;
    }
    
    let mut A = point_in_dir(&c,&r,vec2!(1.0,0.0));
    let mut B = point_in_dir(&c,&r,vec2!()-A);
    let mut C = vec2!(-100.0);
    let mut res = true;

    rl.set_target_fps(5);
    while !rl.window_should_close() {
        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_SPACE) => {res = false;}
            _ => {}
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(backgroung);
        d.draw_line_ex(Vector2{ x : origin.x, y : 0.0     },Vector2{ x : origin.x , y : screen.y},5.0,axes);   
        d.draw_line_ex(Vector2{ x : 0.0,      y : origin.y},Vector2{ x : screen.x , y : origin.y},5.0,axes);
        
        let k1 : i32 = ((origin.y - 0.0     ) / scale + 1.0) as i32;
        let k2 : i32 = ((screen.y - origin.y) / scale + 1.0) as i32;
        for i in -k1..k2 {  
            d.draw_line_ex(Vector2{ x : (origin.x - shir), y : (origin.y + scale * i as f32)},Vector2{ x : (origin.x  + shir), y : (origin.y + scale * i as f32)},2.0,axes);
        }
        let l1 : i32 = ((origin.x - 0.0     ) / scale + 1.0) as i32;
        let l2 : i32 = ((screen.x - origin.x) / scale + 1.0) as i32;
        for j in -l1..l2 { 
            d.draw_line_ex(Vector2{ x : (origin.x + scale * j as f32), y : (origin.y - shir)},Vector2{ x : (origin.x + scale * j as f32), y : (origin.y  + shir)},2.0,axes);
        }

        c.draw(&mut d);
        r.draw(&mut d);
        
        for p in &P {
            d.draw_circle((origin.x + p.x * scale) as i32,(origin.y - p.y * scale) as i32,5.0,Color::VIOLET);
        }
        
        d.draw_circle((origin.x + A.x * scale) as i32,(origin.y - A.y * scale) as i32,10.0,Color::RED);
        d.draw_circle((origin.x + B.x * scale) as i32,(origin.y - B.y * scale) as i32,10.0,Color::GREEN);
        
        if res == false {
            let dir = in_dir_of_center(A,B);
            
            C = point_in_dir(&c,&r,dir);
            if dir.dot(C) < 0.0 {println!("no"); res = true;} else {
                let dir1 = in_dir_of_center(A,C);
                let dir2 = in_dir_of_center(B,C);
                if dir1.dot(vec2!()-A) < 0.0 {B = C;} else {
                    if dir2.dot(vec2!()-B) < 0.0 {A = C;} else {
                        println!("yes");
                    }
                }
                res = true;
            }
        }

        d.draw_circle((origin.x + C.x * scale) as i32,(origin.y - C.y * scale) as i32,10.0,Color::BLUE);
        

        println!("{}",GJK(c,r));

    }
}

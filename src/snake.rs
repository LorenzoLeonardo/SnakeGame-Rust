use crate::position::Coordinates;


#[derive(PartialEq)]
pub enum SnakeDirection
{
    UP,
    DOWN,
    RIGHT,
    LEFT,
}
impl Copy for SnakeDirection {}
impl Clone for SnakeDirection {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Snake
{
    pub _snake_body: Vec<Coordinates>,
    pub _head: Coordinates,
    pub _tail: Coordinates,
    pub _direction: SnakeDirection,
    pub _length: usize,
    pub _is_alive: bool,
    pub _xy_limit: Coordinates,
}

impl Snake
{
    pub fn new () -> Snake 
    {
        Snake {_snake_body: Vec::new(), _head: Coordinates{_x:0, _y:0},
                _tail: Coordinates{_x:0, _y:0}, _direction: SnakeDirection::RIGHT,
                _length: 0, _is_alive:true, _xy_limit: Coordinates { _x: 0, _y: 0 }}
    }

    pub fn init_snake (&mut self, xy_limit: Coordinates) 
    {
        self._snake_body.push(Coordinates{_x: 5,_y: 2});
        self._snake_body.push(Coordinates{_x: 4,_y: 2});
        self._snake_body.push(Coordinates{_x: 3,_y: 2});
        self._snake_body.push(Coordinates{_x: 2,_y: 2});
        self._snake_body.push(Coordinates{_x: 1,_y: 2});
        self._length = self._snake_body.len();
        self._xy_limit = xy_limit.clone();
        self._head = self._snake_body[0].clone();
        self._tail= self._snake_body[self._length - 1].clone();
    }

    pub fn display_snake(&mut self)
    {
        let mut i = 0;
        while i < self._snake_body.len()
        {
            eprint!("{}{}",termion::cursor::Hide, termion::cursor::Goto(self._snake_body[i]._x.try_into().unwrap(), self._snake_body[i]._y.try_into().unwrap()));
            eprint!("@");
            i +=1 ;
        }
    }

    pub fn crawl_snake(&mut self)
    {
        match self._direction {
            SnakeDirection::RIGHT => self.crawl_right(),
            SnakeDirection::LEFT => self.crawl_left(),
            SnakeDirection::UP => self.crawl_up(),
            SnakeDirection::DOWN => self.crawl_down(),
            _ => println!("Others"),
        }
        self._head = self._snake_body[0].clone();
        self._tail= self._snake_body[self._length - 1].clone();

        self.check_body_collision();
    }

    pub fn set_direction(&mut self, dir: SnakeDirection) {
        self._direction = dir;
    }

    pub fn grow_snake(&mut self, pos: Coordinates) {
        self._snake_body.push(pos.clone());
        self._tail = pos.clone();
        self._length = self._snake_body.len();
    }

    fn crawl_right(&mut self) {
        let mut i = self._length - 1;
        while i > 0 {
            self._snake_body[i] = self._snake_body[i - 1].clone();
            i -= 1;
        }
        self._snake_body[0]._x += 1;
        if self._snake_body[0]._x >= self._xy_limit._x
        {
            self._snake_body[0]._x = 2;
        }
    }

    fn crawl_left(&mut self) {
        let mut i = self._length - 1;
        while i > 0 {
            self._snake_body[i] = self._snake_body[i - 1].clone();
            i -= 1;
        }
        self._snake_body[0]._x -= 1;
        if self._snake_body[0]._x < 2
        {
            self._snake_body[0]._x =  self._xy_limit._x - 1;
        }
    }

    fn crawl_up(&mut self) {
        let mut i = self._length - 1;
        while i > 0 {
            self._snake_body[i] = self._snake_body[i - 1].clone();
             i -= 1;
        }
        self._snake_body[0]._y -= 1;
        if self._snake_body[0]._y < 2
        {
            self._snake_body[0]._y =  self._xy_limit._y - 1;
        }
    }

    fn crawl_down(&mut self) {
        let mut i = self._length - 1;
        while i > 0 {
            self._snake_body[i] = self._snake_body[i - 1].clone();
            i -= 1;
        }
        self._snake_body[0]._y += 1;
        if self._snake_body[0]._y >= self._xy_limit._y
        {
            self._snake_body[0]._y = 2;
        }
    }

    fn check_body_collision(&mut self)
    {
        let mut i = 1;
        while  i < self._length
        {
            if (self._snake_body[i]._x == self._head._x) &&
                (self._snake_body[i]._y == self._head._y)
            {
                self._is_alive = false;
                break;
            }
            i += 1;
        }
    }
}
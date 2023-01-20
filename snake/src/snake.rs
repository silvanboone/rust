// use std::collections::VecDeque;
// use std::ops;

// pub struct Snake {
//     pos: VecDeque<Point>,
//     len: i32,
//     is_growing: bool,
// }

// impl Snake {
//     fn move_to(&mut self, point: Point) {
//         if !self.is_growing {
//             self.pos.pop_back();
//         }

//         let head = self.pos.front().unwrap();
//         self.pos.push_front(head + &point);
//     }
// }
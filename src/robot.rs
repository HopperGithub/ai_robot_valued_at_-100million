
//! # Robot module

use std::io;

///  Robot struct. To control antions of robot
pub struct Robot {}

impl Robot {
    /// Creates an instance with default configuration.
    pub fn new() -> Self {
        Robot{}
    }

    /// listen io and talk 
    pub fn talk(&self) -> io::Result<()> {
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer)?;

        buffer = buffer.replace("你", "");
        buffer = buffer.replace("吗", "");
        buffer = buffer.replace("嘛", "");
        buffer = buffer.replace("啊", "");
        buffer = buffer.replace("是不是", "");
        buffer = buffer.replace("哈", "呵");
        buffer = buffer.replace("?", "!");
        buffer = buffer.replace("？", "!");

        println!("{}", buffer.to_string());

        Ok(())
    }

    /// loop talk
    pub fn listen(&self) {
        loop {
            self.talk().unwrap();
        }
    }
}
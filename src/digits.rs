// digits.rs
// Digit matrix and digit size

pub const DIGIT_SIZE: usize = 5;

pub const NUMBER: [[bool; DIGIT_SIZE * DIGIT_SIZE]; 10] = [
    [true,true,true,true,true,
     true,false,false,false,true,
     true,false,false,false,true,
     true,false,false,false,true,
     true,true,true,true,true], // 0
    [false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false], // 1
    [true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true], // 2
    [true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true], // 3
    [true,false,false,false,true,
     true,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     false,false,false,false,true], // 4
    [true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true], // 5
    [true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true], // 6
    [true,true,true,true,true,
     false,false,false,false,true,
     false,false,false,false,true,
     false,false,false,false,true,
     false,false,false,false,true], // 7
    [true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true], // 8
    [true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true], // 9
];

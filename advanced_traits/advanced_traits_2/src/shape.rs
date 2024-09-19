/*
Disiapkan suatu trait bernama shape::Shape.
Trait ini memiliki satu associated types bernama Area.
Dan memiliki sebuah definisi method header area yang gunanya untuk menghitung luas bangun datar (shape).
*/

pub trait Shape {
    type Area;

    fn area(&self) -> Self::Area;
}

#![allow(unused)]

fn f1<T:Copy>(v: &T) {
}
fn f2<T>(v: &T) 
	where T: Copy,
{
}
fn f3<T:Copy + std::fmt::Display>(v: &T) {
}
fn f4<T>(v: &T) 
	where T: Copy + std::fmt::Display,
{
}
fn f5<T: Copy + std::fmt::Display, U: std::fmt::Debug>(v1: &T, v2: &U) {
}
fn f6<T, U>(v1: &T, v2: &U) 
	where T: Copy + std::fmt::Display, U: std::fmt::Debug,
{
}

fn main() {
}

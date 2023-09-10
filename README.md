# Rust-study

## Index

1. [Notes](#notes)
1. [Exercises](#exercises)

## Notes

### Basic

1. 类型转换必须是显式的，不会出现隐式自动转换
1. 数值上可以使用方法，如13.14_f32.round()，这里使用了类型后缀，因为编译器需要知道 13.14 的具体类型
1. 有专门检测溢出的函数如 overflowing_*，wrapping_*，checked_*，saturating_* 等，另外 debug 下溢出会 panic，release 下溢出会补码计算舍弃溢出位导致结果数值和预期不同
1. 有 NAN 数值，可以使用 is_nan() 检查
1. 所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型，由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
1. 单元类型(unit type)就是 ()，唯一的值也是 ()，不占用内存，size 为 0，fn main() 函数返回这个单元类型 ()，你不能说 main 函数无返回值，因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。例如常见的 println!() 的返回值也是单元类型 ()。
1. 还可以使用单元类型 () 作为 map 的值，表示我们不关注具体的值，只关注 key，这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。

## Exercises

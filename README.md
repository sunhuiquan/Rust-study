# Rust-study

## Index

1. [Notes](#notes)
1. [Exercises](#exercises)

## Notes

### Type

1. 类型转换必须是显式的，不会出现隐式自动转换
1. 数值上可以使用方法，如13.14_f32.round()，这里使用了类型后缀，因为编译器需要知道 13.14 的具体类型
1. 有专门检测溢出的函数如 overflowing_*，wrapping_*，checked_*，saturating_* 等，另外 debug 下溢出会 panic，release 下溢出会补码计算舍弃溢出位导致结果数值和预期不同
1. 有 NAN 数值，可以使用 is_nan() 检查
1. 所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型，由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
1. 单元类型(unit type)就是 ()，唯一的值也是 ()，不占用内存，size 为 0，fn main() 函数返回这个单元类型 ()，你不能说 main 函数无返回值，因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。例如常见的 println!() 的返回值也是单元类型 ()。
1. 还可以使用单元类型 () 作为 map 的值，表示我们不关注具体的值，只关注 key，这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。

### Expression & Statement
1. 表达式 expression 一定返回值。而对于语句，它们完成了一个具体的操作，但是没有返回值，因此是语句。在 Rust 中需要注意区分 expression 和 statement 的概念。如下代码中，由于 let 是语句，因此不能将 let 语句赋值给其它值。
    ```rust
    let b = (let a = 8);

    error: expected expression, found statement (`let`)
     --> src/main.rs:2:13
      |
    2 |     let b = let a = 8;
      |             ^^^^^^^^^
      |
      = note: variable declaration using `let` is a statement
    ```
1. 表达式会进行求值，然后返回一个值。
1. 表达式可以成为语句的一部分，如 let y = 6 中，6 就是一个表达式，它在求值后返回一个值 6。
1. 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式。
    ```rust
    let y = {
        let x = 3;
        x + 1
    };
    // 其中用花括号包裹最终返回一个值的语句块
    // {
    //     let x = 3;
    //     x + 1
    // };
    // 就是一个表达式。
    ```
1. 该语句块是表达式的原因是：它的最后一行是表达式，返回了 x + 1 的值，注意 x + 1 不能以分号结尾，否则就会从表达式变成语句，表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值。
1. 表达式如果不返回任何值，会隐式地返回一个 ()，所以 () 类型很重要，返回 () 也是返回值，也是一个表达式。
1. if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    ```rust
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 因为 "odd" 和 "even" 后面没有跟 ';' 所以不是一个语句，是表达式可以返回值

    let y = if x % 2 == 1 {
        "odd";
    } else {
        "even";
    };
    // 这样写的话就是表达式不返回任何值，会隐式地返回一个 () 的情况，y 会被赋值为 ()
    ```

### Function

1. 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
每个函数参数都需要标注类型
1. Rust 中函数就是表达式（除了发散函数），因此我们可以把函数的返回值直接赋给调用者
1. 函数的返回值就是函数体最后一条表达式的返回值，当然也可以使用 return 提前返回
1. 单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值（但实际上在 Rust 眼里是有返回值的，即一个()）
1. 在实际编程中，你会经常在错误提示中看到该 () 的身影出没，假如你的函数需要返回一个 u32 值，但是如果你不幸的以 表达式; 的方式作为函数的最后一行代码，就会报错：
    ```rust
    fn add(x:u32,y:u32) -> u32 {
        x + y;
    }

    // 错误如下：
    error[E0308]: mismatched types
    --> src/main.rs:6:24
      |
    6 | fn add(x:u32,y:u32) -> u32 {
      |    ---                 ^^^ expected `u32`, found `()` // 期望返回u32,却返回()
      |    |
      |    implicitly returns `()` as its body has no tail or `return` expression
    7 |     x + y;
      |          - help: consider removing this semicolon
    ```
    只有表达式能返回值，而 ; 结尾的是语句，所以这里返回的最后一条表达式是自动生成的()。在 Rust 中，一定要严格区分表达式和语句的区别，这个在其它语言中往往是被忽视的点。
1. 发散函数：当用 ! 作函数返回类型的时候，表示该函数永不返回(diverge function)，这种语法往往用做会导致程序崩溃的函数或者无限循环：
    ```rust
    fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
    }

    // 面的函数创建了一个无限循环，该循环永不跳出，因此函数也永不返回
    fn forever() -> ! {
    loop {
        //...
    };
    }
1. 发散函数无法返回值，因为返回的类型不可能与 ! 这个匹配，如下
    ```rust
    fn test() -> ! {
    }

    Compiling function v0.1.0 (/home/roxy/workplace/Rust-study/rust-course/function)
    error[E0308]: mismatched types
    --> src/main.rs:10:14
       |
    10 | fn test() -> ! {
       |    ----      ^ expected `!`, found `()`
       |    |
       |    implicitly returns `()` as its body has no tail or `return` expression
       |
    = note:   expected type `!`
            found unit type `()`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `function` (bin "function") due to previous error
    ```

### Ownership

1. Rust的所有权系统允许在编译时进行内存管理，通过借用（borrowing）、引用（references）和生命周期（lifetimes）等机制，使得在编译时就可以确定何时分配和释放内存，以及何时可以访问数据。这种方式可以避免运行时的内存泄漏、空指针引用和数据竞争等问题，而且不需要额外的运行时开销。总的来说，Rust的所有权系统在编译时进行检查，因为这种检查只发生在编译期，因此对于程序运行期，不会引入性能损失，反而有助于提高代码的性能和安全性。这使得Rust成为一种适用于系统级编程和高性能应用程序开发的语言。
1. 栈中的所有数据都必须占用已知且固定大小的内存空间，因为栈内存的管理方式是基于栈指针（stack pointer）的，栈的大小通常是固定的，且在程序运行时不会动态改变。栈内存用于存储函数调用的局部变量、函数参数、返回地址等数据，因此栈中的数据通常具有已知的大小，编译器在编译时可以静态地确定栈帧的大小。栈内存的固定大小有一些好处，例如在编译时可以进行一些优化，同时栈内存的分配和释放也非常高效，只需要移动栈指针即可。这种固定大小的栈内存通常被称为栈帧（stack frame）。
1. 然而，如果需要在运行时动态分配内存或者数据的大小未知或者可变，通常会使用堆内存（heap memory）来进行分配，例如使用malloc()、new等函数。堆内存的分配和释放通常需要更复杂的管理，因为大小是动态的，需要在运行时进行内存分配和释放的管理，这通常由开发人员来手动控制。所以，虚拟内存中的数据分配方式取决于你的需求和编程语言的特性。栈用于通常固定大小的数据，而堆用于需要动态大小的数据分配。
1. 写入方面：入栈比在堆上分配内存要快，因为入栈时操作系统无需分配新的空间，只需要将新数据放入栈顶即可。相比之下，在堆上分配内存则需要更多的工作，这是因为操作系统必须首先找到一块足够存放数据的内存空间，接着做一些记录为下一次分配做准备。
1. 读取方面：得益于 CPU 高速缓存，使得处理器可以减少对内存的访问，小块的栈数据往往可以直接缓存在 CPU 高速缓存中，而堆数据一般只能存储在内存中(对于堆上的数据，性能可能受到高速缓存的影响较小。堆上的数据通常是动态分配的，其分布可能比较分散，而且数据的大小和生命周期不确定。因此，CPU高速缓存不太能够有效地预取堆上的数据，因为缓存预取通常基于局部性原理工作。)，所以访问堆上的数据比访问栈上的数据慢，这种情况下利用 CPU 高速缓存性能会差出一个数量级。此外堆上的数据通常需要通过指针间接引用，从堆内存中加载数据，这个额外的间接访问步骤会引入一定的性能开销，不过开销并不大。
1. 但实际中基本不存在这种性能问题，因为堆的大小很小，首先放不下大块的数据，然后堆上的生命周期同样不一定适用，而且对于大块的数据也不会被缓存到 CPU chache 中，所以性能和堆上访问基本一样。而对于小块的数据，那点性能差异基本无关紧要。
1. 当你的代码调用一个函数时，传递给函数的参数（包括可能指向堆上数据的指针和函数的局部变量）依次被压入栈中，当函数调用结束时，这些值将被从栈中按照相反的顺序依次移除。而堆上的数据缺乏组织，因此跟踪这些数据何时分配和释放是非常重要的，否则堆上的数据将产生内存泄漏，这也是 Rust 所有权要处理的。
1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者；一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者；当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)。

硬编码进程序里的字符串值（类型为 &str ）。字符串字面值是很方便的，但是它并不适用于所有场景。原因有二：

字符串字面值是不可变的，因为被硬编码到程序代码中
并非所有字符串的值都能在编写代码时得知

Rust 为我们提供动态字符串类型: String, 该类型被分配到堆上，因此可以动态伸缩，也就能存储在编译时大小未知的文本。

可以使用下面的方法基于字符串字面量来创建 String 类型：

let s = String::from("hello");
:: 是一种调用操作符，这里表示调用 String 中的 from 方法，因为 String 存储在堆上是动态的，你可以这样修改它：

let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 将打印 `hello, world!`

let x = 5;
let y = x;
代码背后的逻辑很简单, 将 5 绑定到变量 x；接着拷贝 x 的值赋给 y，最终 x 和 y 都等于 5，因为整数是 Rust 基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存。

可能有同学会有疑问：这种拷贝不消耗性能吗？实际上，这种栈上的数据足够简单，而且拷贝非常非常快，只需要复制一个整数大小（i32，4 个字节）的内存即可，因此在这种情况下，拷贝的速度远比在堆上创建内存来得快的多。实际上，上一章我们讲到的 Rust 基本类型都是通过自动拷贝的方式来赋值的，就像上面代码一样。

然后再来看一段代码：

let s1 = String::from("hello");
let s2 = s1;
此时，可能某个大聪明(善意昵称)已经想到了：嗯，把 s1 的内容拷贝一份赋值给 s2，实际上，并不是这样。之前也提到了，对于基本类型（存储在栈上），Rust 会自动拷贝，但是 String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝。

实际上， String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量，如果你有 Go 语言的经验，这里就很好理解：容量是堆内存分配空间的大小，长度是目前已经使用的大小。

总之 String 类型指向了一个堆上的空间，这里存储着它的真实数据，下面对上面代码中的 let s2 = s1 分成两种情况讨论：

拷贝 String 和存储在堆上的字节数组 如果该语句是拷贝所有数据(深拷贝)，那么无论是 String 本身还是底层的堆上数据，都会被全部拷贝，这对于性能而言会造成非常大的影响

只拷贝 String 本身 这样的拷贝非常快，因为在 64 位机器上就拷贝了 8字节的指针、8字节的长度、8字节的容量，总计 24 字节，但是带来了新的问题，还记得我们之前提到的所有权规则吧？其中有一条就是：一个值只允许有一个所有者，而现在这个值（堆上的真实字符串数据）有了两个所有者：s1 和 s2。

好吧，就假定一个值可以拥有两个所有者，会发生什么呢？

当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。不过由于两个 String 变量指向了同一位置。这就有了一个问题：当 s1 和 s2 离开作用域，它们都会尝试释放相同的内存。这是一个叫做 二次释放（double free） 的错误，也是之前提到过的内存安全性 BUG 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。

再来看看，在所有权转移后再来使用旧的所有者，会发生什么：

let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
由于 Rust 禁止你使用无效的引用，你会看到以下的错误：

error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait

  听起来就像浅拷贝，但是又因为 Rust 同时使第一个变量 s1 无效了，因此这个操作被称为 移动(move)，而不是浅拷贝。上面的例子可以解读为 s1 被移动到了 s2 中

  再来看一段代码:

fn main() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
这段代码，大家觉得会否报错？如果参考之前的 String 所有权转移的例子，那这段代码也应该报错才是，但是实际上呢？

这段代码和之前的 String 有一个本质上的区别：在 String 的例子中 s1 持有了通过String::from("hello") 创建的值的所有权，而这个例子中，x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权。

因此 let y = x 中，仅仅是对该引用进行了拷贝，此时 y 和 x 都引用了同一个字符串。如果还不理解也没关系，当学习了下一章节 "引用与借用" 后，大家自然而言就会理解。

克隆(深拷贝)
首先，Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。

如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
这段代码能够正常运行，因此说明 s2 确实完整的复制了 s1 的数据。

如果代码性能无关紧要，例如初始化程序时，或者在某段时间只会执行一次时，你可以使用 clone 来简化编程。但是对于执行较为频繁的代码(热点路径)，使用 clone 会极大的降低程序性能，需要小心使用！

拷贝(浅拷贝)
浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。

再回到之前看过的例子:


let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，不过依然实现了类似深拷贝的效果 —— 没有报所有权的错误。

原因是像整型这样的基本类型在编译时是已知大小的，会被存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效（x、y 都仍然有效）。换句话说，这里没有深浅拷贝的区别，因此这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它（可以理解成在栈上做了深拷贝）。

Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。

那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则： 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

所有整数类型，比如 u32
布尔类型，bool，它的值是 true 和 false
所有浮点数类型，比如 f64
字符类型，char
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的

## Exercises

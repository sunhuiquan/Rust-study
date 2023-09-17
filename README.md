# Rust-study

## Index

1. [Notes](#notes)
    1. [Type](#type)
    1. [Expression & Statement](#expression--statement)
    1. [Function](#function)
    1. [Ownership](#ownership)
    1. [Reference & Borrow](#reference--borrowing)
    1. [String & Slice](#string--slice)
    1. [Tuple](#tuple)
    1. [Struct](#struct)
    1. [Enum](#enum)
    1. [Array](#array)
    1. [Flow Control](#flow-control)
    1. [Match & If let](#match--if-let)
    1. [Method](#method)
    1. [Generic](#generic)
    1. [Trait](#trait)
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
1. 字符串字面值（类型为 &str ）。字符串字面值是不可变的，因为被硬编码到程序代码中，而且并非所有字符串的值都能在编写代码时得知。
1. 动态字符串类型 String, 该类型被分配到堆上，因此可以动态伸缩，也就能存储在编译时大小未知的文本。
1. 对于基本类型，数据足够简单且存储在栈上(不涉及在堆上分配内存)，而且拷贝非常非常快，所以基本类型都是自动拷贝，不涉及下面的移动和克隆。
1. 这种特性叫做一个类型具有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。那么什么类型是可 Copy 的呢，可以查看给定类型的文档来确认，不过作为一个通用的规则：任何基本类型的组合可以 Copy，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：
    ```txt
    所有整数类型，比如 u32
    布尔类型，bool，它的值是 true 和 false
    所有浮点数类型，比如 f64
    字符类型，char
    元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
    不可变引用 &T，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
    ```
1. 具有 Copy 特征意味着 let a = b; 后 b 仍然有效；无 Copy 特征意味着具有移动语义，let a = b; 后 b 移动到 a，b 之后无效无法再被使用。<!-- TODO: 这个对吗，let a = b 是只有这两种情况吗 -->
1. 注意不可变引用 &T 是具有拷贝特征的，如：
    ```rust
    fn main() {
        let x: &str = "hello, world";
        let y = x;
        println!("{},{}",x,y);
    }
    ```
    这个例子中，x 只是**引用**了存储在二进制中的字符串 "hello, world"，并没有持有所有权，因此 let y = x 中，**仅仅是对该引用进行了拷贝**，此时 y 和 x 都引用了同一个字符串。
1. 对于未实现拷贝特征的（not implement the `Copy` trait），如 String 类型，不能自动拷贝。比如存储在堆上的类型基本都是这样。
    ```rust
    let s1 = String::from("hello");
    let s2 = s1;
    ```
    实际上， String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针指向了真实存储字符串内容的堆内存。如果拷贝所有数据(深拷贝)，那么无论是 String 本身还是底层的堆上数据，都会被全部拷贝，这对于性能而言会造成非常大的影响；而只拷贝 String 本身 这样的拷贝非常快，因为在 64 位机器上就拷贝了 8字节的指针、8字节的长度、8字节的容量，总计 24 字节。  
    但是带来了新的问题，Rust 一个值只允许有一个所有者，这个值（堆上的真实字符串数据）不能有两个所有者 s1 和 s2。因为当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存，如果有两个所有者，那么会导致二次释放（double free）的错误。  
    因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。
    如果在所有权转移后再来使用旧的所有者，会发生什么：
    ```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    // 由于 Rust 禁止你使用无效的引用，你会看到以下的错误：
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
    ```
    听起来就像浅拷贝，但是又因为 Rust 同时使第一个变量 s1 无效了，因此这个操作被称为移动(move)，而不是浅拷贝。上面的例子可以解读为 s1 被移动到了 s2 中，与 C++ 的移动含义相同。
1. 克隆: Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。如果我们确实需要复制 String 堆指针指向的堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    ```
    这段代码能够正常运行，因此说明分配了一个新的内存对象，复制了 s1 的数据，且 s2 有着该内存对象的所有权。
1. 同 C++ 的变量释放顺序一样，因为实现原理是栈，所以函数和代码块中遵循后进先出，后面创建的对象会先离开作用域被释放
    ```rust
    {
        let a = 1;
        let b = 1;
    }
    // a 在 b 之前声明，因此在作用域结束时，b 会首先被释放，然后是 a
    ```
1. 函数的参数同样遵循着拷贝、移动、克隆的规则，如下：
    ```rust
    fn main() {
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里，所以后面 s 不再有效
        // println!("{}", s);           // compile error

        let x = 5;                      // x 进入作用域
        makes_copy(x);                  // x 拷贝到函数里，所以在后面可继续使用 x
        println!("{}", x);

        let s1 = gives_ownership();         // gives_ownership 将返回值移动给 s1

        let s2 = String::from("hello");     // s2 进入作用域

        let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中，然后该函数也将返回值移给 s3
    }
    // 同 C++ 的释放顺序一样，栈上创建的变量后进先出，所以后面创建的对象先要离开作用域释放
    // 这里, s3 首先移出作用域并丢弃释放内存；然后 s2 也移出作用域，但已被移走，所以什么也不会发生；然后 s1 移出作用域并被丢弃；然后 x 移出了作用域并丢弃；最后是 s，但 s 的已被移动走，所以不会再释放

    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

    fn makes_copy(some_integer: i32) { // some_integer 进入作用域
        println!("{}", some_integer);
    } // 这里，some_integer 移出作用域。不会有特殊操作

    fn gives_ownership() -> String {              // gives_ownership 将返回值移动给调用它的函数
        let some_string = String::from("hello");  // some_string 进入作用域.
        some_string                               // 返回 some_string 并移出给调用的函数
    }

    fn takes_and_gives_back(a_string: String) -> String { // takes_and_gives_back 将传入字符串并返回该值
        a_string                                          // 返回 a_string 并移出给调用的函数
    }
    ```

### Reference & Borrowing

1. Rust 通过借用(Borrowing)获取变量的引用。
1. 常规引用是一个指针类型，指向了对象存储的内存地址，& 符号即是引用，它们允许你使用值，但是不获取所有权。
1. 通过 &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃，只是 s1 本身(保存着指向的值的地址)被释放。
1. 正如变量默认不可变一样，引用指向的值默认也是不可变的，即默认是不可变引用。
1. 同一引用作用域，特定数据只能有一个可变引用，且可变引用和不可变引用不可以同时存在，即**一个引用作用域只能拥有一个可变引用, 或者任意多个不可变引用**。
1. 因为**无法同时借用可变和不可变**，这样每个引用都只读这一份数据而不做修改，因此不用担心数据被污染，这种限制的好处就是使 Rust **在编译期就避免了数据竞争**。
1. **引用作用域**从引用创建开始，一直持续到它**最后一次使用的地方（作为左值被修引用指向不算）**，这个**跟变量的作用域不同**，变量的作用域从创建持续到某一个花括号 }。  
Rust 的编译器一直在优化，早期的时候，引用的作用域跟变量作用域是一致的，这对日常使用带来了很大的困扰，你必须非常小心的去安排可变、不可变变量的借用，免得无法通过编译，例如以下代码：
    ```rust
    fn main() {
    let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 新编译器中，r1,r2作用域在这里结束

        let r3 = &mut s;
        println!("{}", r3);
    } // 老编译器中，r1、r2 作用域在这里结束
    ```
    在老版本的编译器中（Rust 1.31 前），将会报错，因为 r1 和 r2 的作用域在花括号 } 处结束，那么 r3 的借用就会触发 无法同时借用可变和不可变的规则。  
    但是在新的编译器中，该代码将顺利通过，因为 引用作用域的结束位置从花括号变成最后一次使用的位置，因此 r1 借用和 r2 借用在 println! 后，就结束了，此时 r3 可以顺利借用到可变引用。
1. NLL: 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。
1. 很多时候，大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域：
    ```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    ```
1. 悬垂引用(Dangling References): 意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在。在 Rust 中**编译器可以确保引用永远也不会变成悬垂状态**，当你获取数据的引用后，**编译器可以确保数据不会在引用结束前被释放(或者被移动)，要想释放数据，必须先停止其引用的使用**，Rust 编译器通过分析生命周期来保证引用总是有效的。

### String & Slice

1. #![allow(unused_variables)] 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告。
1. unimplemented!() 告诉编译器该函数尚未实现，unimplemented!() 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码，类似的标记还有 todo!()，当代码执行到这种未实现的地方时，程序会直接报错。
1. 切片：使用方括号包括的一个序列，[开始索引..终止索引]，其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置，也就是**左闭右开**区间。在**切片数据结构内部会保存开始的位置和切片的长度**，其中长度是通过终止索引 - 开始索引的方式计算得来的。
1. 如果你的切片想要包含 String 的最后一个字节，则可以这样使用 [idx..]，不写终止索引，同理还存在 [..idx] 甚至 [..] 的形式。
1. 字符串中的一个字符这里指 "a啊の" 这里能选中的最小单元，比如 'a'，'啊'，'の' 这就是三个字符。
1. 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置(**Rust 中的 char 是 Unicode 类型每个字符占据 4 个字节内存空间，但是在字符串中字符串是 UTF-8 编码，这样可以减少所占空间**)，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：
    ```rust
    let s = "中国人";
    let a = &s[0..2];
    println!("{}",a);
    ```
    因为我们只取 s 字符串的前两个字节，但是本例中每个汉字占用三个字节，因此没有落在字符的边界处，也就是连第一个字符'中'字都取不完整，此时程序会直接崩溃退出，如果改成 &s[0..3]，则可以正常通过编译。
1. 字符串切片的类型标识是 &str，也是字符串字面值的类型。
1. Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。  
str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且**具有所有权**的 UTF-8 编码字符串，当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
1. 如何将 String 类型转为 &str 类型，取引用即可，这种灵活用法是因为 deref 隐式强制转换，或者使用 as_str() 函数。<!-- TODO -->
    ```rust
    fn main() {
        let s = String::from("hello,world!");
        say_hello(&s);
        say_hello(&s[..]);
        say_hello(s.as_str());
    }

    fn say_hello(s: &str) {
        println!("{}",s);
    }
    ```
1. 在其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错：
    ```rust
    let s1 = String::from("hello");
    let h = s1[0];

    3 |     let h = s1[0];
    |             ^^^^^ `String` cannot be indexed by `{integer}`
    |
    = help: the trait `Index<{integer}>` is not implemented for `String`
    ```
    这首先是因为一个字符可能占有多个字节；第二是索引操作，我们总是期望它的性能表现是 O(1)，然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符降低效率。
1. 字符串切片是非常危险的操作，因为**切片的索引是通过字节来进行，所以索引也要在字符的边界上**：
    ```rust
    let hello = "中国人";
    let s = &hello[0..2];
    // 运行上面的程序，会直接造成崩溃：
    thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国人`, src/main.rs:4:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // 这里提示的很清楚，我们索引的字节落在了'中'字符的内部，这种返回没有任何意义。
    ```
1. 无法使用 Index<{integer}> 索引字符串字符，那么该如何遍历一个 String 呢？这可以通过 **chars() 和 bytes() 函数分别对字符串使用字符和字节的形式进行遍历**。chars() 会以 Unicode 字符的方式遍历字符串；bytes() 以返回字符串的底层字节数组的方式遍历字符串。
    ```rust
    for c in "中国人".chars() {
        println!("{}", c);
    }

    // output:
    // 中
    // 国
    // 人

    for b in "中国人".bytes() {
        println!("{}", b);
    }

    // output:
    // 228
    // 184
    // 173
    // 229
    // 155
    // 189
    // 228
    // 186
    // 186
    ```
1. 可以通过使用 '\' 输出 ASCII 和 Unicode 字符来进行转义，'\\\\' 相当于 '\\'，另外可以通过 r#"something ..."# 来创建原始字符串字面量（raw string literals）允许在字符串中包含特殊字符（比如反斜杠'\\'）而无需进行转义。
1. **连接 (Concatenate)：使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型**。
1. **当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法（fn add(self, s: &str) -> String），这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型，不能直接传递 String 类型。该操作返回一个新的字符串。**
    ```rust
    fn main() {
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        // &string_rust会自动解引用为&str
        let result = string_append + &string_rust;
        let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
        result += "!!!";

        println!("连接字符串 + -> {}", result);
    }
    ```
1. 因为 add 方法涉及第一个参数是 String，可能涉及到所有权问题，所以 + 也要注意这个情况。s1 这个变量通过调用 add() 方法后，所有权被转移到 add() 方法里面，然后 add() 方法调用完后就被释放了。
```rust
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}
```
1. self 是 String 类型的字符串，该函数说明，只能将 &str 类型的字符串切片添加到 String 类型的变量，然后返回一个新的 String 类型，所以 let s3 = s1 + &s2; 就很好解释了，将 String 类型的 s1 与 &str 类型的 s2 进行相加，最终得到 String 类型的 s3。由此可推，以下代码也是合法的，其中String + &str返回一个 String，然后再继续跟一个 &str 进行 + 操作，返回一个 String 类型，不断循环，最终生成一个 s，也是 String 类型。
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
1. 使用 format! 连接字符串：format! 这种方式适用于 String 和 &str，用法与 print! 的用法类似。这种情况下 s1 和 s2 可以为 &str 和 String 类型，且 String 类型所有权未发生过转移。
```rust
fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
    println!("{}",s1);
    println!("{}",s2);
}
```

1. 为什么 String 可变，而字符串字面值 str 却不可以？**就字符串字面值来说，在编译时就知道其内容，最终字面值文本会被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性**。
1. **对于 String 类型，为了支持一个可变、可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容，这些都是在程序运行时完成的**。首先向操作系统请求内存来存放 String 对象，在使用完成后，将内存释放，归还给操作系统。  
    其中第一部分由 String::from 完成，它创建了一个全新的 String。到了第二部分，就是百家齐放的环节，在有垃圾回收 GC 的语言中，GC 来负责标记并清除这些不再使用的内存对象，这个过程都是自动完成，无需开发者关心，非常简单好用；但是在无 GC 的语言中，需要开发者手动去释放这些内存对象，就像创建对象需要通过编写代码来完成一样，未能正确释放对象造成的后果简直不可估量。  
    对于 Rust 而言，**安全和性能是核心特性**，如果**使用 GC，那么会牺牲性能；如果使用手动管理内存，那么会牺牲安全**，这该怎么办？为此，Rust 的开发者想出了一个无比惊艳的办法：**变量在离开作用域后，就自动释放其占用的内存**。
    ```rust
    {
        let s = String::from("hello"); // 从此处起，s 是有效的
    }                                  // 此作用域已结束，s 不再有效，内存被释放
    ```
    与其它系统编程语言的 free 函数相同，**Rust 也提供了一个释放内存的函数 drop**，但是**不同的是，其它语言要手动调用 free 来释放每一个变量占用的内存，而 Rust 则在变量离开作用域时，自动调用 drop 函数: 上面代码中，Rust 在结尾的 } 处自动调用 drop**。  
    其实，在 C++ 中，也有这种概念: **Resource Acquisition Is Initialization (RAII)**。如果你使用过 RAII 模式的话应该对 Rust 的 drop 函数并不陌生。

### Tuple

1. 元组是由多个类型（可以不同）组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的，可以使用模式匹配或者 . 操作符来获取元组中的值。
1. 用**模式匹配来解构**（解构：用和对象的相同的形式把复杂对象中的值匹配出来）元组，依次把对应位置的值绑定到变量。
    ```rust
    fn main() {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
    }
    ```
1. 模式匹配可以让我们一次性把元组中的值全部或者部分获取出来，如果只想要访问某个特定元素，那模式匹配就略显繁琐，可以**使用 . 访问，元组的索引也从 0 开始**。
    ```rust
    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let a = x.0;
        let b = x.1;
        let c = x.2;
    }
    ```
1. 元组一个非常常见的使用场景就是**函数利用元组返回多个值**。
    ```rust
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    fn main() {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
    ```

### Struct

1. 初始化实例时，每个字段都需要进行初始化，字段顺序不需要和结构体定义时的顺序一致，通过 . 操作符即可访问结构体实例内部的字段值。注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，而且 **Rust 不支持将某个字段标记为可变，只能把整个 struct 的变量声明为可变**。
    ```rust
    #![allow(unused)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main() {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");
    }
    ```
1. 可以通过写构建函数的方式简化结构体的构建，另外当函数参数和结构体字段同名时，可以只使用函数参数名即可进行初始化。
    ```rust
    fn build_user(email: String, username: String) -> User {
        User {
            email,       // i.e. email: email
            username,    // i.e. username: username
            active: true,
            sign_in_count: 1,
        }
    }
    ```
1. 根据已有的结构体实例，创建新的结构体实例，可以使用结构体更新语法来简化。..user1 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取，需要注意的是 .. 必须在结构体的尾部使用。
    ```rust
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    ```
1. 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。
1. 元组结构体(Tuple Struct)：结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体，比元组多了个结构体名字来代表类型的含义。
    ```rust
    // Point 元组结构体，众所周知 3D 点是 (x, y, z) 形式的坐标点，因此我们无需再为内部的字段逐一命名为：x, y, z
    #![allow(unused)]
    fn main() {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    ```
1. 单元结构体(Unit-like Struct)：单元结构体就跟单元类型很像，**没有任何字段和属性**，但是好在，它还挺有用。如果你**定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用单元结构体**。
    ```rust
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    impl SomeTrait for AlwaysEqual {

    }
    ```
1. 你也可以让 User 结构体从其它对象借用数据，不过这么做，就需要引入生命周期(lifetimes)这个新概念，简而言之，生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小。如果你想在结构体中使用一个引用，就必须加上生命周期，否则就会报错,未来在生命周期中会讲到如何修复这个问题以便在结构体中存储引用，不过在那之前，我们会避免在结构体中使用引用类型。
1. 直接使用 {} 格式化打印一个对象，需要该对象类型实现 Display 特征，用于输出打印。**基本类型都默认实现了该特征，但结构体不默认实现 Display 特征**，原因在于结构体较为复杂，如：你想要逗号对字段进行分割吗？需要括号吗？加在什么地方？所有的字段都应该显示？由于这种复杂性，**如果要用 {} 的方式打印结构体，那就自己实现 Display 特征**。
1. **使用 {:?} 打印一个对象，需要该对象类型实现 Debug 特征，和 Display 功能一致但生成更多的信息，通常用于调试目的。Rust 默认不会为结构体实现 Debug 特征，需要手动实现或使用 derive 派生**。derive 派生 Rust 自动为我们提供的实现，看上基本就跟结构体的定义形式一样，**当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 {:#?} 来替代 {:?}，这个会自动分行使内容更清楚**。如果还是不满足，那还是自己实现 Display 特征，以向用户更美的展示你的结构体内容。
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {:?}", rect1);
    }
    ```
1. 还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏。dbg! 宏**需要 Debug 特征**，它会拿走表达式的所有权，然后打印出**相应的文件名、行号、表达式的求值结果等 debug 信息，最终还会把表达式值的所有权返回**。
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 最终还会把表达式值的所有权返回，所以 width 可以被绑定，这个写法可以使用
        height: 50,
    };

    dbg!(&rect1);
}

$ cargo run
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```
1. dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout。

### Enum

1. 可以将数据关联到枚举成员上，而避免写 Struct，而且同一个枚举类型下的不同成员还能持有不同的数据类型。
```rust
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);
}

// 同一个枚举类型下的不同成员可以持有不同的数据类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}
```
1. 任何类型的数据都可以放入枚举成员中: 例如字符串、数值、元组、结构体、匿名结构体甚至另一个枚举。
1. 例如某个函数它的功能是接受消息并进行发送，那么用枚举的方式，让函数接受的参数是一个枚举类型，该枚举类型包含着许多可能的消息类型，就可以通过一个参数接收不同类型的消息，可以实现多态。
```rust
struct AContent {
    val: u32,
}

struct BContent {
    val: u32,
}

enum MessageContent {
    Message1(AContent),
    Message2(BContent),
}

struct Message {
    msg_id: u32,
    content: MessageContent,
}

fn func(msg: Message) {
    let content = msg.content;
    match content {
        MessageContent::Message1(a_content) => {
            println!("Message type1: {} - {}", msg.msg_id, a_content.val)
        }
        MessageContent::Message2(b_content) => {
            println!("Message type2: {} - {}", msg.msg_id, b_content.val)
        }
    }
}

fn main() {
    let msg = Message {
        msg_id: 1,
        content: MessageContent::Message1(AContent { val: 1 }),
    };
    func(msg);

    let msg = Message {
        msg_id: 2,
        content: MessageContent::Message2(BContent { val: 2 }),
    };
    func(msg);
}
```
1. 其他语言中当你对 null 进行操作时，例如调用一个方法，就会直接抛出 null 异常，导致程序的崩溃，因此我们在编程时需要格外的小心去处理这些 null 空值。  
尽管如此，空值的表达依然非常有意义，因为空值表示当前时刻变量的值是缺失的。有鉴于此，Rust 吸取了众多教训，决定抛弃 null，而改为使用 Option 枚举变量来表述这种结果。
1. 用 Option 枚举用于处理空值，T 是泛型参数，Some(T)表示该枚举成员的数据类型是 T，换句话说，**Some 可以包含任何类型的数据**，Option<T> 枚举是如此有用以至于它被包含在了 prelude（prelude 属于 Rust 标准库，Rust 会将最常用的类型、函数等提前引入其中，省得我们再手动引入）之中，所以你不需要将其显式引入作用域，因此实际使用中可以省略 Option:: 前缀。
```rust
// Option<T> 定义
// enum Option<T> {
//     Some(T),
//     None,
// }

#![allow(unused)]
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
```
1. 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
1. 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。那么，Option<T> 为什么就比空值要好呢？  
    因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，例如，这段代码不能编译，因为它尝试将 Option<i8>(Option<T>) 与 i8(T) 相加：
    ```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
    -->
      |
    5 |     let sum = x + y;
      |                 ^ no implementation for `i8 + std::option::Option<i8>`
      |
    ```
    很好！事实上，错误信息意味着 Rust 不知道该如何将 Option<i8> 与 i8 相加，因为它们的类型不同。当在 Rust 中拥有一个像 i8 这样类型的值时，编译器确保它总是有一个有效的值，我们可以放心使用而无需做空值检查。只有当使用 Option<i8>（或者任何用到的类型）的时候才需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。
1. 换句话说，在对 Option<T> 进行 T 的运算之前必须将其转换为 T。通常这能帮助我们捕获到空值最常见的问题之一：期望某值不为空但实际上为空的情况。不再担心会错误的使用一个空值，会让你对代码更加有信心。**为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 Option<T> 中。接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是 Option<T> 类型，你就可以安全的认定它的值不为空。**这是 Rust 的一个经过深思熟虑的设计决策，来**限制空值的泛滥以增加 Rust 代码的安全性**。
1. **为了使用 Option<T> 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。也希望一些代码在值为 None 时运行，这些代码并没有一个可用的 T 值。**
1. match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn main() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    ```

### Array

### Flow Control

1. 可以非常方便地用 if else 实现三元表达式和更复杂的变量绑定形式。这里的原理是 if else 语句块是表达式，表达式而非语句代表着有返回值（不写的话自动返回 ()），所以这里我们可以使用 if 表达式的返回值进行变量绑定。一般要求每个分支返回的类型一样，不过一些特殊情况允许在不同分支中返回不同类型的值。
    ```rust
    let number = if condition {
            5
        } else {
            6
        };

    // 结合循环使用
    let mut v = 0;
    for i in 1..10 {
        v = if i == 9 {
            continue
        } else {
            i
        }
    }
    println!("{}", v);
    ```
1. **使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）。如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了**。  
    对于实现了 copy 特征的数组（比如 i32 数组可以，String 数组不行）而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，显然这也应该使用引用来避免的开销。
    ```rust
    for item in &container {
    // ...
    }
    ```
1. 如果想在循环中，修改该元素，使用 mut 关键字。
    ```rust
    for item in &mut container {
    // ...
    }
    ```
1. 循环中获取元素的索引。
    ```rust
    fn main() {
        let a = [4, 3, 2, 1];
        for (i, v) in a.iter().enumerate() {
            println!("第{}个元素是{}", i + 1, v);
        }
    }
    ```
1. 当索引无用时可以用 _ 在 for 循环中来忽略，**在 Rust 中 _ 的含义是忽略该值或者类型的意思**，如果不使用 _，那么编译器会给你一个 变量未使用的 的警告
    ```rust
    for _ in 0..10 {
    }
    ```
1. **在循环中应该使用 in 而非通过下标索引访问**
    ```rust
    // Bad
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
    let item = collection[i];
    }

    // Good
    for item in collection {
    }
    ```
    代码更简洁：第二段代码使用了 Rust 中的迭代器（iterator）机制，这使得代码更加简洁和易读。它使用 for item in collection 的语法，不需要显式地使用索引来访问集合中的元素，这降低了代码的复杂性。  
    避免了越界错误：在第一段代码中，你使用了一个循环来遍历集合，通过索引来访问元素。这种方式存在越界错误的风险，因为你需要确保索引不会超出集合的边界。如果索引超出了集合的长度，就会导致运行时错误。第二段代码使用迭代器，完全避免了这种风险，因为它会自动处理集合的边界。    
    更好的性能：使用迭代器通常会比手动管理索引的方式更高效，因为迭代器可以根据集合的具体类型和实现来选择最优的遍历方式，而且可以避开检查索引是否越界的开销。
1. 如果你需要一个条件来循环，当该条件为 true 时，继续循环，条件为 false，跳出循环，那么 while 就非常适用。
1. loop 就是一个简单的无限循环，你可以在内部实现逻辑通过 break 关键字来控制循环何时结束。因为只能通过 break 跳出，所以这里 break 可以单独使用，也可以带一个返回值，有些类似 return，这也使得 loop 是一个表达式，因此可以返回一个值。
    ```rust
    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }
    ```

### Match & If let

1. match 类似与其他语言中的 switch，但是更强大。**match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性；match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同。**
    ```rust
    enum Direction {
        East,
        West,
        North,
        South,
    }

    fn main() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            },
            _ => println!("West"),
        };
    }
    ```
<!-- TODO 1. match 中 X | Y，类似逻辑运算符或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可。 -->
1. match 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行对应的代码。match 后的表达式返回值可以是任意类型，也可以返回默认返回 () 类型当作无返回值。
    ```txt
    match target {
        模式1 => 表达式1,
        模式2 => {
            语句1;
            语句2;
            表达式2
        },
        _ => 表达式3
    }

    // example:
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny =>  {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    ```
1. 接下来是 match 的分支。一个分支有两个部分：一个模式和针对该模式的处理代码。每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。如果分支有多行代码，那么需要用 {} 包裹。
1. 当 match 表达式执行时，它将目标值 coin 按顺序依次与每一个分支的模式相比较，如果模式匹配了这个值，那么模式之后的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支。
1. match 本身也是一个表达式，因此可以用它来赋值。
    ```rust
    enum IpAddr {
    Ipv4,
    Ipv6
    }

    fn main() {
        let ip1 = IpAddr::Ipv6;
        let ip_str = match ip1 {
            IpAddr::Ipv4 => "127.0.0.1",
            _ => "::1",
        };

        println!("{}", ip_str);
    }
    ```
<!-- 1. 模式匹配的另外一个重要功能是从模式中取出绑定的值，例如：

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}
其中 Coin::Quarter 成员还存放了一个值：美国的某个州（因为在 1999 年到 2008 年间，美国在 25 美分(Quarter)硬币的背后为 50 个州印刷了不同的标记，其它硬币都没有这样的设计）。

接下来，我们希望在模式匹配中，获取到 25 美分硬币上刻印的州的名称：

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
上面代码中，在匹配 Coin::Quarter(state) 模式时，我们把它内部存储的值绑定到了 state 变量上，因此 state 变量就是对应的 UsState 枚举类型。

例如有一个印了阿拉斯加州标记的 25 分硬币：Coin::Quarter(UsState::Alaska), 它在匹配时，state 变量将被绑定 UsState::Alaska 的枚举值。

再来看一个更复杂的例子：

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
运行后输出：


$ cargo run
   Compiling world_hello v0.1.0 (/Users/sunfei/development/rust/world_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/world_hello`
Hello Rust
point from (0, 0) move to (1, 2)
change color into '(r:255, g:255, b:0)', 'b' has been ignored

1. 穷尽匹配
在文章的开头，我们简单总结过 match 的匹配必须穷尽所有情况，下面来举例说明，例如：

enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}
我们没有处理 Direction::West 的情况，因此会报错：

error[E0004]: non-exhaustive patterns: `West` not covered // 非穷尽匹配，`West` 没有被覆盖
  -> src/main.rs:10:11
   |
1  | / enum Direction {
2  | |     East,
3  | |     West,
   | |     ---- not covered
4  | |     North,
5  | |     South,
6  | | }
   | |_- `Direction` defined here
...
10 |       match dire {
   |             ^^^^ pattern `West` not covered // 模式 `West` 没有被覆盖
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `Direction`
不禁想感叹，Rust 的编译器真强大，忍不住想爆粗口了，sorry，如果你以后进一步深入使用 Rust 也会像我这样感叹的。Rust 编译器清晰地知道 match 中有哪些分支没有被覆盖, 这种行为能强制我们处理所有的可能性，有效避免传说中价值十亿美金的 null 陷阱。

1. _ 通配符
当我们不想在匹配时列出所有值的时候，可以使用 Rust 提供的一个特殊模式，例如，u8 可以拥有 0 到 255 的有效的值，但是我们只关心 1、3、5 和 7 这几个值，不想列出其它的 0、2、4、6、8、9 一直到 255 的值。那么, 我们不必一个一个列出所有值, 因为可以使用特殊的模式 _ 替代：

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
通过将 _ 其放置于其他分支后，_ 将会匹配所有遗漏的值。() 表示返回单元类型与所有分支返回值的类型相同，所以当匹配到 _ 后，什么也不会发生。

除了_通配符，用一个变量来承载其他情况也是可以的。

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}
然而，在某些场景下，我们其实只关心某一个值是否存在，此时 match 就显得过于啰嗦。 -->

1. if let 匹配
<!-- TODO -->

### Method

1. 在 Rust 中，方法往往和对象成对出现，如 object.method()。例如读取一个文件写入缓冲区，如果用函数的写法 read(f, buffer)，用方法的写法 f.read(buffer)。Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用。
1.  Rust 的对象定义和方法定义是分离的，这种数据和使用分离的方式，会给予使用者极高的灵活度。
    ![IMG](/images/struct1.png)
1. **Rust 使用 impl 来定义一个类型的方法**。
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    fn main() {
        let rect1 = Rectangle { width: 30, height: 50 };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
    ```
1. self、&self 和 &mut self：在方法的签名中，**&self 其实是 self: &Self 的简写，在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例，换句话说，self 指代的是 Rectangle 结构体实例**，这样的写法会让我们的代码简洁很多，而且非常便于理解：我们为哪个结构体实现方法，那么 self 就是指代哪个结构体的实例。
1. **self 依然有所有权的概念，self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少，通常用于将当前的对象转成另外一个对象时使用，可以防止对之前对象的误调用；&self 表示该方法对 Rectangle 的不可变借用
&mut self 表示可变借用**。总之，self 的使用就跟函数参数一样，要严格遵守 Rust 的所有权规则。
1. **仅仅通过使用 self 作为第一个参数来使方法获取实例的所有权是很少见的，这种使用方式往往用于把当前的对象转成另外一个对象时使用，转换完后，就不再关注之前的对象，且可以防止对之前对象的误调用**。
1. 使用方法代替函数有以下好处：不用在函数签名中重复书写 self 对应的类型，代码的组织性和内聚性更强，对于代码维护和阅读来说，好处巨大。
1. 允许方法名跟结构体的字段名相同，方法跟字段同名，往往适用于实现 getter 访问器。
    ```rust
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        pub fn width(&self) -> u32 {
            return self.width;
        }
    }

    fn main() {
        let rect1 = Rectangle::new(30, 50);
        println!("{}", rect1.width());
    }
    ```
1. **Rust 并没有一个与 -> 等效的运算符，取而代之的是 Rust 有<u>自动引用和解引用</u>，方法调用是 Rust 中少数几个拥有这种行为的地方。当使用 object.something() 调用方法时，Rust 会<u>自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配</u>**。
    ```rust
    // 这两行是等价的
    p1.distance(&p2);
    (&p1).distance(&p2);
    ```
    **这种自动引用的行为之所以有效，是因为方法有一个明确的接收者 self，在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）**。

1. 关联函数：通过**在 impl 代码块中写参数不包含 self 的函数即可为一个结构体定义一个构造器方法，也就是接受参数，构造并返回该结构体的实例**。这种定义在 impl 中且没有 self 的函数被称之为关联函数： 因为它没有 self，不能用 f.read() 的形式调用，因此它是一个函数而不是方法，它又在 impl 中，与结构体紧密关联，因此称为关联函数。
    ```rust
    impl Rectangle {
        fn new(w: u32, h: u32) -> Rectangle {
            Rectangle { width: w, height: h }
        }
    }
    ```
    **Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称，出于设计上的考虑，Rust 特地没有用 new 作为关键字**。  
    因为是函数，所以不能用 . 的方式来调用，我们**需要用 :: 来调用**，例如 let sq = Rectangle::new(3, 3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。
1. Rust 允许我们为一个结构体定义多个 impl 块，目的是提供**更多的灵活性和代码组织性，例如当方法多了后，可以把相关的方法组织在同一个 impl 块中，那么就可以形成多个 impl 块，分别完成不同功能部分**。
    ```rust
    // 就这个例子而言，我们没必要使用两个 impl 块，这里只是为了演示方便。
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    ```
1. 枚举类型之所以强大，不仅仅在于它好用、可以同一化类型，还在于，我们可以像结构体一样，为枚举实现方法：
    ```rust
    #![allow(unused)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    fn main() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }
    ```
1. **除了结构体和枚举，我们还能为特征(trait)实现方法**

### Generic

1. 泛型是一种多态形式，用同一功能的函数处理不同类型的数据，例如两个数的加法，无论是整数还是浮点数，甚至是自定义类型，都能进行支持。在不支持泛型的编程语言中，通常需要为每一种类型编写一个函数。
1. 可以在函数中使用泛型，**泛型参数的名称你可以任意起，但是出于惯例，我们都用 T ( T 是 type 的首字母)来作为首选，这个名称越短越好，除非需要表达含义，否则一个字母是最完美的**。
    ```rust
    fn largest<T>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    // 运行后报错：
    error[E0369]: binary operation `>` cannot be applied to type `T`
    -> src/main.rs:5:17
      |
    5 |         if item > largest {
      |            ---- ^ ------- T
      |            |
      |            T
      |
    help: consider restricting type parameter `T`
      |
    1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
      |             ++++++++++++++++++++++
    ```
    **因为 T 可以是任何类型，但不是所有的类型都能进行比较，因此上面的错误中，编译器建议我们给 T 添加一个类型限制**，使用 std::cmp::PartialOrd 特征（Trait）对 T 进行限制，特征在下一节会详细介绍，现在你只要理解，该特征的目的就是让类型实现可比较的功能。
1. **结构体中的字段类型也可以用泛型来定义**，下面代码定义了一个坐标点 Point，它可以存放任何类型的坐标值：
    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }
    ```
    提前声明，跟泛型函数定义类似，首先我们在使用泛型参数之前必需要进行声明 Point<T>，接着就可以在结构体的字段类型中使用 T 来替代具体的类型，注意一个泛型类型只能代表一个类型，如果多个字段使用同一个泛型类型，那么这些字段必须是同一类型。
1. 如果想让不同字段如 x 和 y 既能类型相同，又能类型不同，就需要使用不同的泛型参数，但泛型类型不易过多，这会显著降低代码可读性。
    ```rust
    struct Point<T,U> {
        x: T,
        y: U,
    }

    fn main() {
        let p = Point{x: 1, y :1.1};
    }
    ```
1. Option<T> 是一个拥有泛型 T 的枚举类型，它第一个成员是 Some(T)，存放了一个类型为 T 的值。得益于泛型的引入，我们**可以在任何一个需要返回值的函数中，去使用 Option<T> 枚举类型来做为返回值，用于返回一个任意类型的值 Some(T)，或者没有值 None**。
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```
1. Result<T, E> 枚举和 Option 一样，主要用于函数返回值，与 Option 用于值的存在与否不同，**Result 关注的主要是值的正确性**。
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
    函数正常运行，则最后返回一个 Ok(T)，如果函数异常运行，则返回一个 Err(E)，E 是错误类型。  
    例如打开一个文件：如果成功打开文件，则返回 Ok(std::fs::File)，因此 T 对应的是 std::fs::File 类型；而当打开文件时出现问题时，返回 Err(std::io::Error)，E 对应的就是 std::io::Error 类型。
1. 实现泛型结构体的方法，需要先在 impl<T> 中声明。
    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> { // 这里 Point<T> 不是泛型声明，而是一个结构体类型，因为定义的结构体就是 Point<T> 而不是 Point
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
    ```
1. **方法本身也可以额外实现泛型，除了结构体中的泛型参数，我们还能在该结构体的方法中定义额外的泛型参数，就跟泛型函数一样，这里方法的泛型参数和结构体是不是泛型有没有泛型参数无关**。
    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c'};

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    ```
    T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，它们并不冲突，说白了，你可以理解为，一个是结构体泛型，一个是函数泛型。
1. **为具体的泛型类型实现方法，对于 Point<T> 类型，你不仅能定义基于 T 的方法，还能针对特定的具体类型，进行方法定义**。
    ```rust
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    ```
    这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而**其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法**。这个方法计算点实例与坐标(0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。**这样我们就能针对特定的泛型类型实现某个特定的方法，对于其它泛型类型则没有定义该方法**。
1. **const 泛型**：在之前的泛型中，可以抽象为一句话：针对类型实现的泛型，所有的泛型都是为了抽象不同的类型，那有没有**针对值的泛型**？  
    在数组那节，有提到过很重要的一点：[i32; 2] 和 [i32; 3] 是不同的数组类型，比如下面的代码：
    ```rust
    fn display_array(arr: [i32; 3]) {
        println!("{:?}", arr);
    }

    fn main() {
        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);

        let arr: [i32;2] = [1,2];
        display_array(arr);
    }

    error[E0308]: mismatched types // 类型不匹配
    --> src/main.rs:10:19
       |
    10 |     display_array(arr);
       |                   ^^^ expected an array with a fixed size of 3 elements, found one with 2 elements
    ```
    结合代码和报错，可以很清楚的看出，[i32; 3] 和 [i32; 2] 确实是两个完全不同的类型，因此无法用同一个函数调用。
1. 首先，让我们修改代码，让 display_array 能打印任意长度的 i32 数组，只要使用数组切片，然后传入 arr 的不可变引用即可。
    ```rust
    fn display_array(arr: &[i32]) {
        println!("{:?}", arr);
    }

    fn main() {
        let arr: [i32; 3] = [1, 2, 3];
        display_array(&arr);

        let arr: [i32;2] = [1,2];
        display_array(&arr);
    }
    ```
1. 接着，将 i32 改成所有类型的数组：
    ```rust
    fn display_array<T: std::fmt::Debug>(arr: &[T]) {
        println!("{:?}", arr);
    }

    fn main() {
        let arr: [i32; 3] = [1, 2, 3];
        display_array(&arr);

        let arr: [i32;2] = [1,2];
        display_array(&arr);
    }
    ```
1. **通过引用，我们可以很轻松的解决处理任何类型数组的问题，但是如果在某些场景下引用不适宜用或者干脆不能用呢**？  
    好在现在有了 **const 泛型，也就是针对值的泛型，可以用于处理数组长度的问题**。
    ```rust
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    fn main() {
        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);

        let arr: [i32; 2] = [1, 2];
        display_array(arr);
    }
    ```
    <!-- TODO 你们知道为什么以前 Rust 的一些数组库，在使用的时候都限定长度不超过 32 吗？因为它们会为每个长度都单独实现一个函数，简直。。。毫无人性。难道没有什么办法可以解决这个问题吗？ -->
1. 如上所示，我们定义了一个类型为 [T; N] 的数组，其中 T 是一个基于类型的泛型参数，而**重点在于 N 这个泛型参数，const N: usize，它是一个基于值的泛型参数，这里类型是确定的，值是任意的**，可以用它来替代数组的长度。
1. **const 泛型表达式：假设我们某段代码需要在内存很小的平台上工作，因此需要限制函数参数占用的内存大小，此时就可以使用 const 泛型表达式来实现**。
    <!-- TODO 限制函数参数占用的内存大小？ -->
<!-- // 目前只能在nightly版本下使用
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn something<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
{
    //
}

fn main() {
    something([0u8; 0]); // ok
    something([0u8; 512]); // ok
    something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
}

// ---

pub enum Assert<const CHECK: bool> {
    //
}

pub trait IsTrue {
    //
}

impl IsTrue for Assert<true> {
    //
} -->

1. 泛型的性能：**在 Rust 中泛型是零成本的抽象，因为是一种编译期多态的实现，意味着你在使用泛型时，完全不用担心性能上的问题**，但是任何选择都是权衡得失的，既然我们获得了性能上的巨大优势，那么又失去了什么呢？**Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小**。
1. **Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程**。也就是说编译器所做的工作正好与我们创建泛型函数的步骤相反，编译器**寻找所有泛型代码被调用的位置并针对具体类型生成代码**。
1. 让我们看看一个使用标准库中 Option 枚举的例子。
    ```rust
    let integer = Some(5);
    let float = Some(5.0);
    ```
    当 Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一种对应 i32 另一种对应 f64。为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，接着将泛型定义替换为这两个具体的定义。编译器生成的单态化版本的代码看起来像这样：
    ```rust
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
    ```
    我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在**使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因**。

### Trait

## Exercises

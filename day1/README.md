### Day1: Afternoon

#### Ownership

> 前提知识：
>
> 堆和栈作为不同的内存区块，栈用于存放基本类型的值（比如int），而堆可以用来存放不固定长度的数据，即引用对象所指向的数据。
>
> 栈中的内存管理非常简单，数据量小而且出栈和入栈即可；而堆中内存被对象所引用，需要考虑到这块内存被多少个对象引用，什么时候回收，回收早了会导致引用出错，回收太晚可能导致内存过多而溢出，而且回收的时候只能回收一次，已回收的内存可能会被再被分配其他数据，再次回收会导致数据紊乱。
>
> 通常，内存回收指的是堆中内存的回收

所有权是rust中独特的设计，用于管理内存，区别于java的自动GC和c的手动内存管理。

所有权的核心理念是，保证数据的唯一所有者，配合生命周期自动安全地回收内存。假设变量a拥有一个String类型的数据（数据初始化过程)，变量b想使用这个数据，a需要把所有权移交给b，之后a再使用的话会报如下错误

```rust
let a = String::from("Hello World!");
let b = a; // 此时，所有权移交到了b
// print!("{a}"); // Exception: borrow of moved values: `a` value borrowed here after more
```

如果不想把所有权让渡过去的话，可以配合引用，让b可以访问a所指向的数据

```rust
let a = String::from("Hello World!");
let b = &a;
print!("{a} {b}"); // Hello World! Hello World!
```

在函数中使用时，对象是作为参数传入函数，这个过程可以理解为所有权对象将所有权移交给了参数变量

```rust
let a = String::from("Hello World!");
sub(a);
// print!("After sub: {a}"); // Exception: borrow of moved values: `a` value borrowed here after more

fn sub(text: String) {
   print!("In sub: {text}");  // In sub: Hello World!
}

```

如果想把所有权还给a，那么需要在函数把入参返回来

```rust
let mut a = String::from("Hello World!"); // 此处加上mut修饰，因为下面一行需要对a重新赋值
a = sub(a);
print!("After sub: {a}"); // After sub: Hello World!

fn sub(text: String) {
   print!("In sub: {text}");  // In sub: Hello World!
   text
}
```

这种方式不够优雅，上面变量赋值的时候我们提到了引用，这里同样可以使用引用这种方式把数据“借用"给函数sub

```rust
let a = String::from("Hello World!");
sub(&a);
print!("After sub: {a}"); // After sub: Hello World!

fn sub(text: &String) {
   print!("In sub: {text}");  // In sub: Hello World!
}
```

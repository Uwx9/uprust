// module_path!()
mod testmod {
    pub fn foo()
    {
        println!(module_path!());
        assert!(module_path!().ends_with("testmod"));
    }
}
fn main() {
    testmod::foo();
}

#[cfg(test)]
mod tests {

    // cloumn!()宏计算调用column宏时的列数,即column中字符c所在的列,从1开始算而不是0
    #[test]
    fn test_column()
    {
        let a = ("i am ab", column!());
        let b = ("i bm bcd", column!());
        let c = ("i cm cdef", column!());
        let d = ("i dm defgh", column!());
        assert!(a.1 == 29);
        assert!(b.1 == 30);
        assert!(c.1 == 31);
        assert!(d.1 == 32);

    }
    

    // compile_error!(), 该宏会导致编译期间就报错 
    // 暂时不知道如何让这个测试通过编译期报错
    #[test]
    fn test_compile_error()
    {
        macro_rules! give_me_foo_or_bar {
            (foo) => {};
            (bar) => {};
            ($x: ident) => {
                compile_error!("you should give me foo or bar, so compile_error");
            };
        }

        give_me_foo_or_bar!(bar);
        // give_me_foo_or_bar!(x);
    }

    // concat!() 字符串拼接, 这个concat宏很强大, 不同类型的都可以拼
    #[test]
    fn test_concat()
    {
        // macro_rules! concat {
        //      ($($e:expr),* $(,)?) => { ... };
        // }
        let s = concat!("test", 10, 'a', true, 0.99);
        assert!(s == "test10atrue0.99");
      }

    // dbg!() 打印同时返回表达式的值
    #[test]
    fn test_dbg()
    {
        fn factorial(n: u32) -> u32
        {
            if n <= 1 {
                dbg!(1)
            } else {
                dbg!(n * factorial(n - 1))
            }
        }

        let res = factorial(4);
        assert_eq!(res, 24);
    }

    // debug_assert!() 类似assert!(), 也可以自定义panic消息
    #[test]
    fn test_debug_assert()
    {
        debug_assert!(!false);
        let a = 1;
        let b = 2;
        debug_assert!(a + b == 3, "{a} + {b} = 3");
    }

    // debug_assert_eq!() 类似assert_eq!()  加 -C debug-assertions 参数 = 强制 release 也生效
    #[test]
    fn test_debug_assert_eq()
    {
        let a = vec![1, 0];
        let b = vec![1, 0];
        debug_assert_eq!(a, b);
    }

    // debug_assert_ne!() 加 -C debug-assertions 参数 = 强制 release 也生效
    #[test]
    fn test_debug_assert_ne()
    {
        let a = vec![1, 2, 3];
        let b = vec![1, 7, 9];
        debug_assert_ne!(a, b);
    }

    // env!() 编译时展开环境变量生成&'static str。
    // 如果你想获取其值, 可以通过std::env::var获取
    // 如果环境变量没有定义则会产生编译错误, 除非使用option_env!
    #[test]
    fn test_env()
    {
        // 这里的XDG_SEAT就是我机器的环境变量, 可以用printenv查看
        let env_val = env!("XDG_SEAT");
        println!("\ntest_env: path is {env_val}\n");
    }

    // eprint!(), eprintln!()宏, 和print!(), println!()功能类似, 但是它输出信息到标准错误流。
    // 一般仅仅用于输出错误的场景, 输出失败会产生panic
    #[test]
    fn test_eprint()
    {
        eprintln!("这是个eprintln!调用, 标准错误流");
        println!("这是个println!调用, 标准输出流");
    }

    // file!() 返回当前文件名的宏
    #[test]
    fn test_file()
    {
        let file = file!();
        println!("current file is {file}");
    }

    // format!()
    #[test]
    fn test_format()
    {
        let fname = "main";
        let message = format!("{fname} has {} lines", 3);
        println!("{message}");
    }

    // format_args!()
    #[test]
    fn test_format_args()
    {
        let s = std::fmt::format(format_args!("hello {}", "world"));
        assert_eq!(s, format!("hello {}", "world"));
    }

    // include!() 解析文件为表达式
    #[test]
    fn test_include()
    {
        let s = include!("test.in");
        assert_eq!("991", s);
    }

    // include_bytes!() 和include类似，不同是文件是文件内容会被解释为bytes数组。
    // 生成的表达式为&static' [u8,N]
    #[test]
    fn test_include_bytes()
    {
        let s = include_bytes!("test.in");
        assert_eq!(b"\"99\".to_owned() + \"1\"\n", s);
    }

    // include_str!() 包含utf-8编码的文件转换为字符串
    #[test]
    fn test_include_str()
    {
        let s = include_str!("test.in");
        assert_eq!("\"99\".to_owned() + \"1\"\n", s);
    }

    // line!() 返回当前调用行
    #[test]
    fn test_line()
    {
        let line = line!();
        println!("调用行{line}");
    }

    // matches!() 返回两个表达式是否匹配
    #[test]
    fn test_match()
    {
        let foo = 'f';
        assert!(matches!(foo, 'A'..='Z' | 'a' ..='z'));
        let bar = Some(1);
        assert!(matches!(bar, Some(x) if x >= 1));
    }

    // option_env!() 编译器获取环境变量为字符串
    // 和env不同在于, 环境变量不存在的时候返回None
    #[test] 
    fn test_option_env()
    {
        let key = option_env!("HOME");
        match key {
            Some(_x) => println!("the key is {key:?}"),
            None => println!("dont have this env"),
        }
    }

    // stringify!() 将括号内的表达式转为字符串
    #[test]
    fn test_stringify()
    {
        let s = stringify!(hello, bro);
        println!("s is {s}");
        assert_eq!(s, "hello, bro");
    }

    // todo!() 用于定义一段未实现的代码, 运行未实现代码会产生panic 
    #[test]
    #[should_panic = "todo"]
    fn test_todo()
    {
        // 运行todo!会panic
        todo!("this is todo");
    }

    // unimplemented!() 表示未实现的代码
    #[test]
    #[should_panic = "unimplemented"]
    fn tet_unimplemented()
    {
        // 运行implemented!会panic
        unimplemented!("this is unimplemented");
    }

    // unreachable!() 执行这个宏将会panic
    #[test]
    #[should_panic = "unreachable"]
    fn test_unreachable()
    {
        // 运行unreachable!会panic
        unreachable!("this is unreachable")
    }


    // vec 创建一个包含参数的向量


    // write!() writeln!() 它们都是把格式化字符串写入一个缓冲区(buffer)
    #[test]
    fn test_write() {
        use std::fmt::Write;
        let mut buf = String::new();

        // 写入 3 次，都不换行
        write!(&mut buf, "hello").unwrap();
        write!(&mut buf, " ").unwrap();
        write!(&mut buf, "rust").unwrap();

        // 结果连在一起
        assert_eq!(buf, "hello rust");
    }

    #[test]
    fn test_writeln() {
        use std::fmt::Write;
        let mut buf = String::new();

        // 每次写入都会自动加 \n（换行）
        writeln!(&mut buf, "hello").unwrap();
        writeln!(&mut buf, "rust").unwrap();

        // 结果是：
        // hello\n
        // rust\n
        assert_eq!(buf, "hello\nrust\n");
    }

}


// 
// 
// 学习Box
//
//

#![feature(allocator_api)]

// 实现Allocator trait即实现内存分配, Global 是真正的内存分配器, 我们自己的结构体用于计数
// Ordering::Relaxed 是操作原子类型的最宽松规则,枚举类型
// Layout 的实例描述了特定的内存布局, 您将 Layout 作为输入分配给分配器
use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
struct CounterAllocator {
    allocations: AtomicUsize,
    deallocations: AtomicUsize,
}

impl CounterAllocator {
    fn new() -> Self
    {
        Self::default()
    }

    fn states(&self) -> (usize, usize)
    {
        (
            self.allocations.load(Ordering::Relaxed),
            self.deallocations.load(Ordering::Relaxed)
        )
    }
}

unsafe impl Allocator for CounterAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>
    {
        // 让Global全局分配器帮助分配内存
        let ptr = Global.allocate(layout)?;

        // 分配次数+1
        self.allocations.fetch_add(1, Ordering::Relaxed);
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)
    {
        unsafe {
            Global.deallocate(ptr, layout);
        }
        
        // 回收次数+1
        self.deallocations.fetch_add(1, Ordering::Relaxed);
    }
}


// 自动解引用测试
use std::ops::{Deref, DerefMut};
use std::fmt;

// 包装类型T到新的类型Wrapper
#[derive(Debug, Clone, PartialEq)]
struct MyWrapper<T>(T);

impl<T> MyWrapper<T> {
    fn new(val: T) -> Self
    {
        Self(val)
    }
}

impl<T> Deref for MyWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T> DerefMut for MyWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.0
    }
}

// 为包装类型实现新的trait
impl<T: fmt::Display> fmt::Display for MyWrapper<T>
{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
   {
       write!(f, "Wrappter[{}]", self.0)
   }
}

fn main()
{
    println!("just test");
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试1：Box + 自定义分配器（分配/释放计数）
    #[test]
    fn test_box_custom_allocator() {
        let allocator = CounterAllocator::new();
        
        {
            // 创建 Box → 分配+1
            let mut boxed_value = Box::new_in(42, &allocator);
            assert_eq!(*boxed_value, 42);
            
            *boxed_value = 100;
            assert_eq!(*boxed_value, 100);
            // 离开作用域 → 释放+1
        }

        let (alloc, dealloc) = allocator.states();
        assert_eq!(alloc, 1);    // 必须分配 1 次
        assert_eq!(dealloc, 1);  // 必须释放 1 次
        println!("✅ 分配器测试通过：分配={alloc}, 释放={dealloc}");
    }

    // 测试2：MyWrapper 自动解引用功能
    #[test]
    fn test_my_wrapper_deref() {
        // 测试整数
        let mut wrapped_num = MyWrapper::new(42);
        *wrapped_num += 10;
        assert_eq!(*wrapped_num, 52);

        // 测试字符串
        let mut wrapped_str = MyWrapper::new(String::from("hello"));
        wrapped_str.push_str(" world!");
        assert_eq!(*wrapped_str, "hello world!");

        // 测试Vec方法自动解引用
        let wrapped_vec = MyWrapper::new(vec![1, 2, 3]);
        assert_eq!(wrapped_vec.len(), 3);
        assert_eq!(*wrapped_vec, vec![1,2,3]);
        
        println!("✅ MyWrapper 自动解引用测试通过");
    }
}

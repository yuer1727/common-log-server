

use threadpool::ThreadPool;
use std::collections::HashMap;
use std::marker::{Sync, Send};



lazy_static! {
  pub static ref THREAD_POOL_LIST: HashMap<&'static str, Box<SimpleThreadPool>> = {
        let mut m : HashMap<&'static str, Box<SimpleThreadPool>> = HashMap::new();
        m.insert("default", Box::new(SimpleThreadPool::new(10)));
        m.insert("log", Box::new(SimpleThreadPool::new(10)));
        m
  };
}


pub fn getThreadPoolWithoutNone(pool_name: &str) -> &Box<SimpleThreadPool> {
    let pool = match getThreadPool(pool_name){
        None => THREAD_POOL_LIST.get("default").unwrap(),
        Some(pool) => pool,
    };
    return pool;
}

pub fn getThreadPool(pool_name: &str) -> Option<&Box<SimpleThreadPool>> {
    match THREAD_POOL_LIST.get(pool_name){
        None => THREAD_POOL_LIST.get("default"),
        Some(pool) => Some(pool),
    }
}


pub struct SimpleThreadPool{
    pool: ThreadPool,
    n_workers: usize,
}

unsafe impl Sync for SimpleThreadPool{}
unsafe impl Send for SimpleThreadPool{}

impl SimpleThreadPool{
    fn new(n_workers: usize) -> SimpleThreadPool{
        let pool = ThreadPool::new(n_workers);
        SimpleThreadPool{
            pool,
            n_workers,
        }
    }

    pub fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static {
        //job
        //闭包的逃逸性:根据一个闭包是否会逃逸到创建该闭包的词法作用域之外, 可以将闭包分为非逃逸闭包和逃逸闭包.
        //这二者最根本的区别在于, 逃逸闭包必须复制或移动环境变量. 这是很显然的, 如果闭包在词法作用域之外使用, 而其如果以引用的方式获取环境变量, 有可能引起悬垂指针问题.
        //逃逸闭包的类型声明中, 需要加一个静态生命周期参数'static.
        self.pool.execute(job);
    }
}





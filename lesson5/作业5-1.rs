1 列出3个常用的宏，

//定义pallet模块
#[frame_support::pallet]

//定义pallet模块使用的类型及参数
 #[pallet::config]

//定义模块的事件
 #[pallet::event]

//定义错误信息
#[pallet::error]

//定义数据存储对象
 #[pallet::storage]

//定义模块里的可调用函数，每一个外部交易都会触发一个可调用函数
 #[pallet::call]

2 列出3个常用的存储数据结构

//存单值数据类型
#[pallet::storage]
#[pallet::getter(fn something)]
pub type Something<T> = StorageValue<_,u32>;
pub type MyString<T> = StorageValue<_,Vec<u8>>;   

//简单映射类型
#[pallet::storage]
#[pallet::getter(fn my_map)]
pub type MyMap<T> = StorageMap<
    _,
    Blake2_128Concat,     //哈希算法，密码学安全
    u8,                             //key
    Vec<u8>,                  //value
>；

//双键映射类型  使用两个key来索引value，用于快速删除key1对应的任意记录，也可以遍历key1对应的所有记录.
#[pallet::storage]
#[pallet::getter(fn my_double_map)]
pub type MyDoubleMap<T> = StorageDoubleMap<
    _,
    Blake2_128Concat,     //key1的哈希算法，密码学安全
    T::AccountId,              //key1
    Blake_128Concat,       //key2的哈希算法，密码学安全
    u32,                           //key2 
    Vec<u8>,                  //value
>；



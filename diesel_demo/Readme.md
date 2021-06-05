## diesel 的教程
cargo new --lib diesel_demo
cd diesel_demo
### 安装命令行工具
cargo install diesel_cli
值得注意的是，目前支持postgres mysql sqlite等三种数据库，使用命令行工具的时候需要你安装这个三个数据库的客户端  
如果你一个都没装，会有类似如下的报错：

```text
note: ld: library not found for -lmysqlclient
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```
你也可追加上--no-default-features，来去掉这部分内容，从而通过编译。  
如果安装了其中一个数据库的客户端，那么可以这样：

```shell
cargo install diesel_cli --no-default-features --features postgres
```
如果安装全部特性，那么这样
```shell
 sudo apt install sqlite3 libsqlite3-0 libsqlite3-dev libpq-dev libmysqlclient-dev
```
再执行安装命令。
要装那些东西也可以直接看官网   
https://github.com/diesel-rs/diesel/tree/master/diesel_cli  
英文不行的话，直接看  
https://learnku.com/docs/rust-irwa/setting-up-orm-and-database/10341
###环境配置
这一步讲一下diesel_cli配置一些环境变量来连接上数据库的方法  
为了不污染我们的环境，所以diesel_cli的环境变量是可以这样设置的：
```shell
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```
对，在当前目录下建一个.env的文件，   
现在我们使用sqlite来简化内容
```shell
echo DATABASE_URL="examples.db" > .env
echo BACKEND="sqlite" >> .env
```
现在就可以使用diesel_cli来完成剩下的内容

    diesel setup

这个时候就会有提示信息告诉我们，数据库已经建好了（如果不存在这个数据库的话）
现在使用migration来管理我们的表

    diesel migration generate create_weids

这个时候就会发现，在当前目录下已经创建了migrations文件夹，并生成了两个sql文件，点进去，还是空的
我们在up.sql中填充创建表的DDL，在down.sql中填充删除表的DDL
```sqlite
--up.sql
CREATE TABLE weids (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  addr TEXT NOT NULL,
  chain_id INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```
```sqlite
--down.sql
DROP TABLE weids;
```

执行命令

    diesel migration run

从提示信息中可以看到，执行了我们的sql，来让我们看看down.sql是不是正确的。  
执行

    diesel migration redo

可以看到rolling back的信息

### 代码部分
sql已经足够了，让我们开始写代码！  
可以看到schema.rs中已经自动生成了schema内容，使用的是table!宏。
我们在lib中加入
```rust
pub mod schema;
pub mod models;
```
新建models文件，  
```rust
#[derive(Queryable, PartialEq, Debug)]
pub struct Weid {
    pub id: i32,
    pub chain_id: i32,
    pub addr: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
}
#[derive(Insertable)]
#[table_name = "weids"]
pub struct NewWeid {
    pub chain_id: i32,
    pub addr: String,
}

```
我们通过#[derive(Insertable)]与#[derive(Queryable, PartialEq, Debug)]，让该结构体具备可插入数据库，或从数据库查询的特性。  

在models创建一个方法来插入数据：
```rust
pub fn insert_default_values(conn: &SqliteConnection) -> QueryResult<usize> {
    use schema::weids::dsl::*;

    insert_into(weids).default_values().execute(conn)
}
```
开始交互！！！
创建`/bin/show_weids.rs`，
我们引入包，并添加main函数
```rust
extern crate diesel_demo;
extern crate diesel;
fn main() {}
```

创建一个函数用来建立连接：  
直接拷贝自 Diesel 的Examples，作用是根据环境变量中的DATABASE_URL连接sqlite数据库。
```rust
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```
创建一条数据：
```rust
create_weid(&sqlite_conn, 1, "34be11396f3a91c5Ab5A1220e756C6300FB2b20a");
```
创建数据的方法：
```rust
pub fn create_weid(conn: &SqliteConnection, c_id: i32, address: &str) -> usize {


    let new_weid = NewWeid {chain_id: c_id, addr: address.to_string()};

    diesel::insert_into(weids::table)
        .values(&new_weid)
        .execute(conn)
        .expect("Error saving new weid")
}
```  
加载数据，通过load函数： 

```rust
let results = weids.load::<Weid>(&sqlite_conn)
        .expect("Error loading weids");
``` 

来看看查出来的数据： 
```rust
println!("Displaying {} weids", results.len());
for weid in results{
println!("did:weid:{}:{}", weid.chain_id, weid.addr);
}
```

https://diesel.rs/guides/getting-started
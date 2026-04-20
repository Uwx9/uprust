use std::fs;

// 读文件
fn file2str(filename: String) -> Result<String, std::io::Error>
{
    let file = fs::File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;   // 成功返回读取的字节数
    Ok(s)
}

// 返回值等同于 std::io::Result<vec<u8>>
fn file2vec(filename: String) -> Result<vec<u8>, std::io::Error> {
    let file = fs::File::open(filename)?;
    let mut v = Vec::new();
    file.read_to_end(&mut v)?;  // 成功返回读取的字节数
    Ok(v)
}

fn main() {
    println!("Hello, world!");
}

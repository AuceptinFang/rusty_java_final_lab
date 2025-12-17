use tracing::{debug, error, info};
use crate::score;

pub fn menu(){
    println!("============成绩管理系统============");
    println!("1. 从CSV文件导入成绩到数据库");
    println!("2. 按学号查询某学生的成绩");
    println!("3. 显示总评成绩前N名");
    println!("4. 退出系统");
    println!("请输入选项(1-4)");
}

pub async fn main_loop(){
    dotenvy::dotenv().ok();

    let mut db ;
    match score::db::Db::new().await {
        Ok(d) => {
            db = d;
            info!("数据库连接成功");
        }
        Err(e) => {
            println!("数据库错误: {}", e);
            std::process::exit(0);
        }
    };

    loop{
        menu();
        let option = readnum();
        match option {
            1 =>{
                debug!("开始导入csv文件");
                match db.import_to_db().await{
                    Ok(()) => debug!("导入成功"),
                    Err(e) => error!("出现非预期错误 {}",e)
                }
            }
            2 =>{
                println!("请输入学号");
                let id = readline();
                match db.search(&*id).await{
                    Ok(()) =>{}
                    Err(e) => error!("{}",e)
                };
            }
            3 => {
                println!("请输入要查询的数量");
                let num = readnum();
                match db.show(num).await{
                    Ok(()) =>{}
                    Err(e) => error!("{}",e)
                }
            }
            4 => {
                println!("程序正常退出");
                std::process::exit(0);
            }
            _ =>{
                println!("输入不合法");
            }
        }
    }
}

pub fn readline() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap_or(0);
    line.trim().to_string()
}

pub fn readnum() -> i32{
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap_or(0);
    num.trim().parse::<i32>().unwrap_or(0)
}


fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score: Option<&i32> = scores.get(&team_name);
//     println!("{:?}", score);
// }

// fn main() {
//     use std::collections::HashMap;

//     let teams_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), 50),
//     ];

//     let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

//     println!("{:?}", teams_map)
// }

// fn main() {
//     use std::collections::HashMap;

//     // 创建一个HashMap，用于存储宝石种类和对应的数量
//     let mut my_gems = HashMap::new();

//     // 将宝石类型和对应的数量写入表中
//     my_gems.insert("红宝石", 1);
//     my_gems.insert("蓝宝石", 2);
//     my_gems.insert("河边捡的误以为是宝石的破石头", 18);
//     println!("{:?}", my_gems);
// }

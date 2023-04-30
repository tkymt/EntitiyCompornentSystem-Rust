use rand;

fn main() {
    
    // idは配列の要素数を表す
    let mut id = 0;

    // 配列をやめて、ベクタで宣言する
    let mut pos_ary: Vec<(i32, i32)> = vec![];

    // 効果を確かめるために、10回 create_entity を呼び出す
    for i in 0..10 {
        // ランダムな値を取得する
        let (x, y) = rand::random();
        create_entity(&mut id, &mut pos_ary,);
        let t = id -1;
        set_data(&t, &mut pos_ary, (x, y));
    }



    for i in 0..id  {
        println!("{}: {:?}", i, pos_ary[i])
    }
}

// idを増やして、配列に要素を追加する
fn create_entity(id: &mut usize, ary: &mut Vec<(i32, i32)>){
    *id += 1;
    ary.push((0, 0));
}

// idを増やして、配列に指定された値の要素を追加する
fn create_entity_pos(id: &mut usize, ary: &mut Vec<(i32, i32)>, data: (i32, i32)) {
    *id += 1;
    ary.push(data);
}

// 配列の指定されたidの要素に、指定された値を代入する
fn set_data (id: &usize, ary: &mut Vec<(i32, i32)>, data: (i32, i32)) {
    ary[*id] = data;
}

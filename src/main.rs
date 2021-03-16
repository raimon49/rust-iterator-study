fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }

    sum
}

use std::fmt::Debug;
fn dump<T, U>(t: T)
    where T: IntoIterator<Item=U>,
          U: Debug
{
    println!("Dump via 'dump' function:");
    for u in t {
        println!("{:?}", u);
    }
}

fn main() {
    triangle(4);

    {
        println!("There is:");
        let v = vec!["antimony", "arsenic", "alumium", "selenium"];
        for element in &v {
            println!("{}", element);
        }
        // 上記のループは以下と同等
        let mut iterator = (&v).into_iter();
        while let Some(element) = iterator.next() {
            // Some(element)が返されたらループボディ部を実行するがNoneが返されたら終了する
            println!("{}", element);
        }
    }

    {
        let v = vec![4, 20, 12, 8, 6];
        let mut iterator = v.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.next(), None); // 最後の要素Noneを返したあとでさらにnext()を呼んだ時に何を返すかをIteratorトレイトは規定していないが、多くの実装では再度Noneを返す

        dump(v);

        use std::ffi::OsStr;
        use std::path::Path;

        let path = Path::new("C:/Users/Jimb/Downloads/Fedra.iso");
        let mut iterator = path.iter();
        assert_eq!(iterator.next(), Some(OsStr::new("C:")));
        assert_eq!(iterator.next(), Some(OsStr::new("Users")));
        assert_eq!(iterator.next(), Some(OsStr::new("Jimb")));
        assert_eq!(iterator.next(), Some(OsStr::new("Downloads")));
        assert_eq!(iterator.next(), Some(OsStr::new("Fedra.iso")));
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.next(), None);

        dump(path);
    }

    {
        // HashSetでなくイテレータの順序が保証されるBTreeSetを使う
        use std::collections::BTreeSet;
        let mut favorites = BTreeSet::new();
        favorites.insert("Lucy in the Sky With Diamonds".to_string());
        favorites.insert("Libebestramue No. 3".to_string());

        let mut it = favorites.into_iter();
        assert_eq!(it.next(), Some("Libebestramue No. 3".to_string()));
        assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        // ここで呼ぶと「value used here after move」でコンパイルエラーとなる
        // これはfavorites.into_iter()の呼び出しが所有権ごと返すイテレータであるため
        // dump(favorites);
    }

    {
        use std::iter::FromIterator;

        // 多くのコレクションはdrain()を実装しており、指定範囲の可変参照を借用したイテレータを返す
        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));

        assert_eq!(outer, "Eh");
        assert_eq!(inner, "art");
    }
}

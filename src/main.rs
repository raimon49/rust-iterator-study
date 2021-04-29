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

    {
        // text.lines()で返された各行のイテレータをmap()アダプタで空白を除去する
        let text = "  ponies  \n  giraffers\niguanas  \nsquid".to_string();
        let v: Vec<&str> = text.lines()
            .map(str::trim)
            .collect();
        assert_eq!(v, ["ponies", "giraffers", "iguanas", "squid"]);

        // さらにfilter()アダプタでiguanasだけ取り除く
        let v2: Vec<&str> = text.lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas") // trueを返すものだけをイテレータのアイテムとして生成する
            .collect();
        assert_eq!(v2, ["ponies", "giraffers", "squid"]);
    }

    {
        // iter()呼び出しだけでは値が要求されずnext()が呼ばれた時に初めて使われる
        // このコードはコンパイル時に以下の警告を出す
        // warning: unused `std::iter::Map` that must be used
        // 最後に.next()をコールすると、printlnマクロが実行される
        ["earth", "water", "air", "fire"]
            .iter().map(|ert| println!("{}", ert));
    }

    {
        use std::str::FromStr;

        let text = "1\nfrond .25 289\n3.1415 estuary\n";
        // 1) ホワイトスペースで区切られたスライスを
        // 2) f64::from_str()でパースを試みる -> Result<f64, ParseFloatError>が返る
        // 3) ok()を実行すると、エラーの場合はNoneとなりドロップされ処理は継続しない
        // 4) パース成功したものはSome(v)のvを取り出す
        for number in text.split_whitespace()
            .filter_map(|w| f64::from_str(w).ok()) {
                println!("{:4.2}", number.sqrt());
            }

        // 上のfilter_map()と同じ処理ををmap()->filter()->map()で書き直したもの
        for number in text.split_whitespace()
            .map(|w| f64::from_str(w))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap()) {
                println!("{:4.2}", number.sqrt());
            }
    }

    {
        use std::collections::HashMap;

        let mut major_cities = HashMap::new();
        major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
        major_cities.insert("The United States", vec!["Portland", "Nashville"]);
        major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
        major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
        major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

        let countries = ["Japan", "Brazil", "Kenya"];

        for &city in countries.iter().flat_map(|country| &major_cities[country]) {
            println!("{}", city);
        }

    }

    {
        let iter = (0..10)
            .scan(0, |sum, item| {
                // イテレータの入力0, 1, 2, 3, 4...の2乗が10を超えるまで継続される
                // 4*4 = 16になったところでNoneが返却されscan()アダプタは処理を中止する
                *sum += item;
                if *sum > 10 {
                    None
                } else {
                    Some(item * item)
                }
            });

        assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);
    }

    {
        let message = "To: jimb\r\n
                       From: superego <editor@oreilly.com>\r\n
                       \r\n
                       Did you get any writing done today?\r\n
                       When will you stop wasting time plotting fractals?\r\n";
        println!("message header:");
        // take_while()は引数predicateがtrueになったらNoneを生成してイテレートを中止する
        for header in message.lines().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }

        println!("message body:");
        // skip_while()は引数predicateがtrueのアイテムだけをスキップして繰り返す
        for body in message.lines().skip_while(|l| !l.is_empty()) {
            println!("{}", body);
        }

        let mut lines = message.lines();
        // by_ref()呼び出しをするとイテレータの可変参照を借用する
        for header in lines.by_ref().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }
        // 上のループでは参照を返しているだけなので、もう一度ループで利用できる
        for body in lines {
            println!("{}", body);
        }
    }

    {
        use std::iter::Peekable;

        fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
            where I: Iterator<Item=char>
        {
            let mut n = 0;
            loop {
                // peek()を使って次の文字をチェックし、取り出したSome(r)が数字の場合だけ消費
                match tokens.peek() {
                    Some(r) if r.is_digit(10) => {
                        n = n * 10 + r.to_digit(19).unwrap();
                    }

                    _ => return n
                }

                tokens.next();
            }
        }

        let mut chars = "226153980,1766319049".chars().peekable();

        assert_eq!(parse_number(&mut chars), 226153980);

        // parse_number()の内部実装がカンマを取り出さないため、ここでnext()を呼んで消費する
        assert_eq!(chars.next(), Some(','));

        assert_eq!(parse_number(&mut chars), 1766319049);

        // 取り出すものが無くなったらNoneが返される
        assert_eq!(chars.next(), None);
    }

    {
        // 型を定義
        struct Flakey(bool);

        // next()が呼ばれて最後の要素に到達しても常にNoneを返さないイテレータ実装
        impl Iterator for Flakey {
            type Item = &'static str;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("totaly the last item")
                } else {
                    self.0 = true;
                    None
                }
            }
        }

        // next()を呼んでNoneになっても再び要素が返される
        let mut flaky = Flakey(true);
        assert_eq!(flaky.next(), Some("totaly the last item"));
        assert_eq!(flaky.next(), None);
        assert_eq!(flaky.next(), Some("totaly the last item"));

        // fuseアダプタにより、最後の要素に到達したら常にNoneを返すイテレータに変換可能
        let mut not_flaky = Flakey(true).fuse();
        assert_eq!(not_flaky.next(), Some("totaly the last item"));
        assert_eq!(not_flaky.next(), None);
        assert_eq!(not_flaky.next(), None);
        assert_eq!(not_flaky.next(), None);
    }

    {
        use std::iter::DoubleEndedIterator;

        let bee_parts = ["head", "thorax", "abdomen"];

        // DoubleEndedIteratorを実装している型は前端と後端のアイテムを引き出せる
        let mut iter = bee_parts.iter();
        assert_eq!(iter.next(),      Some(&"head"));
        assert_eq!(iter.next_back(), Some(&"abdomen"));
        assert_eq!(iter.next(),      Some(&"thorax"));
        // 2本の指が一致したところで繰り返し実行は終了する
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(),      None);

        let meals = ["breakfast", "lunch", "dinner"];

        // rev()でnext/next_backが反転したイテレータを取得できる
        let mut rev_iter = meals.iter().rev();
        assert_eq!(rev_iter.next(), Some(&"dinner"));
        assert_eq!(rev_iter.next(), Some(&"lunch"));
        assert_eq!(rev_iter.next(), Some(&"breakfast"));
        assert_eq!(rev_iter.next(), None);
    }

    {
        let upper_case: String = "groβe".chars()
            .inspect(|c| println!("before: {:?}", c))
            .flat_map(|c| c.to_uppercase())
            .inspect(|c| println!(" after: {:?}", c)) // アイテムをそのまま通過させるため、デバッグ出力などに使われる
            .collect();
        assert_eq!(upper_case, "GROΒE");
    }

    {
        // 1つ目のイテレータから2つ目のイテレータを繋げる
        let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
        assert_eq!(v, [1, 2, 3, 20, 30, 40]);

        // 1つ目のイテレータから2つ目のイテレータを繋げたものをrev()で逆順にする
        let rev_v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
        assert_eq!(rev_v, [40, 30, 20, 3, 2, 1]);
    }

    {
        let bands = vec![10, 20, 30, 40];
        for (i, band) in bands.into_iter().enumerate() {
            // enumerate()は(0, 10), (1, 20), (2, 30), (3, 40)と
            // インデックスとアイテムのペアになったイテレータを生成する
            println!("{}", (i * band)); // 0, 20, 60, 120
        }
    }

    {
        // zip()アダプタは2つのイテレータを合わせて1つのイテレータにする
        // もとの2つのイテレータが生成するアイテムのペアを作成する
        // 閉じ合わせるどちらかのイテレータが終了した時点でzip()アダプタも終了する
        // ここでは"ABCD".chars()が終了した時点で(0..)も繰り返しを終了している
        let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
        assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

        // zip()の引数はイテレータそのものでなくイテレート可能なものなら何でもよい
        use std::iter::repeat;
        let endings = vec!["once", "twice", "chikien soup with rice"];
        let rhyme: Vec<_> = repeat("going")
            .zip(endings)
            .collect();
        assert_eq!(rhyme, vec![("going", "once"),
                               ("going", "twice"),
                               ("going", "chikien soup with rice")]);
    }

    {
        // cloned()はCloneを実装する型のイテレータから値をクローンして生成するイテレータを返す
        let a = ['1', '2', '3', '∞'];

        assert_eq!(a.iter().next(),          Some(&'1'));
        assert_eq!(a.iter().cloned().next(), Some('1'));
    }

    {
        let dirs = ["North", "East", "South", "West"];
        let mut spin = dirs.iter().cycle();
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
        assert_eq!(spin.next(), Some(&"South"));
        assert_eq!(spin.next(), Some(&"West"));
        assert_eq!(spin.next(), Some(&"North"));
        assert_eq!(spin.next(), Some(&"East"));
    }
}

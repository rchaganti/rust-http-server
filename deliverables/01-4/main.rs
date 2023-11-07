#[derive(Debug)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Hindi,
  Telugu,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);

  let g : Greeting = Greeting { lang: Lang::Hindi, message: String::from("नमस्ते WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Telugu, message: String::from("నమస్తే WasmEdge!") };
  v.push(g);

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }
}

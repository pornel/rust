// error-pattern: copying a noncopyable value

struct foo {
  let i: int;
  drop {}
}

fn foo(i:int) -> foo {
    foo {
        i: i
    }
}

fn main() { let x <- foo(10); let y = x; log(error, x); }

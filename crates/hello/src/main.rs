#[trait_variant::make(Foo: Send)]
pub trait LocalFoo {
    async fn foo(&self);
}

pub struct Bar;

impl LocalFoo for Bar {
    async fn foo(&self) {
        println!("Hello, world!");
    }
}

#[tokio::main]
async fn main() {
    let bar = Bar {};
    bar.foo().await;
}

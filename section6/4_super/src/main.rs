use test_super::sub_mod::call_super;
fn main() {
    call_super()
}

fn test1() {
    println!("Hello, world!");
}

mod test_super {
    use super::test1;

    pub fn sub_function() {
        test1()
    }

    pub mod sub_mod {
        pub fn call_super() {
            super::sub_function()
        }
    }
}

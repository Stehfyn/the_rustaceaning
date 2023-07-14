pub mod client;

pub mod network;

pub mod outermost;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    //use super::*; //without this, client::connect needs to be ::client::connect
    //or super::client::connect
    //best:, use
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
        assert_eq!(4, 4);
    }
}

// Summary of Module rules in regard to files:
// - If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs
// - If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs
// - These rules apply recursively (i.e.):
//    - If a module named foo has a submodule named bar and foes not have submodules, you should have the following files
//      in your src directory:
//    |
//    |____foo
//        |
//        |__bar.rs (contains declarations in 'foo::bar')
//        |
//        |__mod.rs (contains declarations in 'foo', including 'mod bar')

mod my {
    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn function() {
        println!("called `my::function()`");
    }

    // private function can be accessed from other functions in the same module.
    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my::function();

    // public items, including those inside nested modules,
    // can be accessed from outside the parant module.
    my::indirect_access();
    my::nested::function();


    // private items cannot be directly accessed

    // my::private_function();
    // //=> error: function `private_function` is private

    // my::nested::private_function();
    // //=> error: function `private_function` is private


    // items of private module can not be accessed (even if they are public)

    // my::private_nested::function();
    // //=> error: module `private_nested` is private
}

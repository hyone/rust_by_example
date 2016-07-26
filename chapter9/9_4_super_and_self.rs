fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n>");

        // The `self` keyword refers to the current module scope - in this case `my`
        self::function();
        // calling `self::function()` and calling `function()` directly both give the same result
        function();

        // refer `my::cool::function` via `self`
        self::cool::function();

        // The `super` keyword refers to the parent scope
        super::function();

        // refer from global scope.
        ::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case, the crate scope is the outermost scope.
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}

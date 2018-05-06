// module system
//     * hierarchically split code in logical units(modules)
//     * manage visibility(public/private) between them
// module
//     * functions
//     * structs
//     * traits
//     * impl blocks
//     * other modules

// By default, the items in a module have private visibility. Only the 
// public items of a module can be accessed from outside the module.

mod my_mod {
    // default to private visibility
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }
    
    // override default visibility
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // private items can be accessed in the same module
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // nested module
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // only visible within the given path which must be a parent or 
        // ancestor module
        pub(in my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // only visible within the current module
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();

    // my_mod::nested::public_function_in_my_mod();
}

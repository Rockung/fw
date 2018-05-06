// super, self
//    * remove ambiguity when accessing items
//    * prevent unnecessary hardcoding of paths

fn function() {
    println!("called 'function()'");
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
        print!("called `my::indirect_call()`, that\n> ");
       
        self::function();
        function();

        self::cool::function();

        // outside the 'my' module
        super::function();

        {
            // in the *crate* scope, the outermost scope
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
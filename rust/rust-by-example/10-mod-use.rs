// the 'use' declaration can be used to bind a full path to a new name

use deeply::nested::function as other_function;

fn function() {
    println!("called 'function()'");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called 'deeply::nested::function()'");
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        // shadow the outer one
        use deeply::nested::function; // as function
        function();

        // 'use' bindings have a local scope
        println!("Leaving block");
    }

    function();
}
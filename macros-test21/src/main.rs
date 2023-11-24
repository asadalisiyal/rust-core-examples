// Define a macro named 'my_macro'
macro_rules! my_macro {
    // This pattern matches when the macro is invoked with no arguments
    () => {
        println!("Hello from my_macro!");
    };
    
    // This pattern matches when the macro is invoked with a single expression
    ($x:expr) => {
        println!("Hello, you provided the expression: {}", $x);
    };
    
    // This pattern matches when the macro is invoked with multiple expressions
    ($($x:expr),*) => {
        println!("Hello, you provided the following expressions: {:?}", [$($x),*]);
    };
}

fn main() {
    my_macro!();                    // Invoking the macro with no arguments
    my_macro!(42);                  // Invoking the macro with a single expression
    my_macro!(1, 2, 3);             // Invoking the macro with multiple expressions
}


use proc_macro;

#[proc_macro]
pub fn my_procedural_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens and generate new code
    let output = quote::quote! {
        fn generated_function() {
            println!("This code was generated by my_procedural_macro!");
        }
    };
    
    // Return the generated code as a TokenStream
    output.into()
}

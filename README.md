# rust-odyssey
## Variables and Mutability
- Rust is statically typed, means you will have to assign type upon variable declaration. 
- By default variables are immutable: once a value is bound to a name, you can't change it. 
- To define a value that can be changed you make it a mutable like this: `let mut x: u32 = 5` 
- You define a constant by `const` keyword, like in JS, and they are always immutable and can't be made mutable. 
- Constant can't be set to the result of a value that could only be computed at runtime. 
- Shadowing: it is process of declaring a new variable with the same name as previous variables. Therefore the first variables is shadow by the second one, and the second variable is what the compiler will see when you use the variable until the scope end. 
- To shadow a variable you have to re-declare it, means you use `let` keyword. 
- N.B: when we use `mut` we are re-assigning a value to our variables, but when we shadow (use `let`) we are re-creating the variable. 
- Another difference between shadowing and assigning, with shadowing we can change the type of variable. 
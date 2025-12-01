# Rust Doc Comment Format
This is intended for contributes and users alike.

Each field in the doc comment should be used only if it applicable to the block of code, if not - leave it out.

## The format for Rust doc comments is as follows:

````
/// Short functional description. What the relevant block of code does.
/// 
/// # Example
/// ```
/// let a = add(4, 7)?;
/// assert_eq!(a, 13);
/// ```
/// 
/// # Arguments
/// The `add` function takes in two arguments:
/// * `num1: i32` — The first number to add.
/// * `num2: i32` — The second number to add.
/// 
/// # Panics
/// The `add` function panics if `num2` is equal to `11`.
/// 
/// # Errors
/// The possible errors that can be returned are:
/// * `error_type1` if ...
/// * `error_type2` if ...
/// 
/// # Returns
/// The `add` function returns a Result\<i32, Box\<dyn std::error::Error\>\>.
/// If `Ok` is returned, the unwrapped value is the result of the mathematical operation `num1 + num2`.
///
/// # Notes
/// Any other notes, usage considerations or guidelines, anything important that wasn't included in the previous sections.
pub fn add(num1: i32, num2: i32) -> Result\<i32, Box\<dyn std::error::Error\>\> {...}
````

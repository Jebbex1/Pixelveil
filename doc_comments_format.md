# Doc Comment Format

This document is intended for contributes and users alike.
This project uses multiple languages (Rust, Python) so each language has it's own doc comment format.
The doc comments for the Python wrappers should match the ones for the core Rust crate, just with the adjusted types and keywords.

Each field in the doc comment should be used only if it applicable to the block of code, if not - leave it out.

The fields that are included in the doc comments (if applicable) are:

1. Short functional description. What the relevant block of code does.
2. A functional copy-paste example.
3. Description of each argument and its type, and default value if there is one.
4. If the function panics, on what conditions does it panic? (Rust only)
5. What errors can be returned/raised (language dependant) and on what conditions?
6. If there isn't an error, what does the return value represent?
7. Additional notes and usage considerations if there are any.

Note that, if a Rust function panics, the condition that causes it should be included in the errors section of the matching Python wrapper doc comment.

## Rust Doc Comment Format Example

Formatted in the standard Rust doc comment format.

````rust
/// Add two numbers together
/// 
/// # Example
/// ```rust
/// let a = add(4, 7)?;
/// assert_eq!(a, 13);
/// ```
/// 
/// # Arguments
/// This function takes in two arguments:
/// * `num1: i32` — The first number to add.
/// * `num2: i32` — The second number to add.
/// 
/// # Panics
/// This function panics if `num2` is equal to `11`.
/// 
/// # Errors
/// The possible errors that can be returned are:
/// * `error_type1` if ...
/// * `error_type2` if ...
/// 
/// # Returns
/// This function returns a Result\<i32, Box\<dyn std::error::Error\>\>.
/// If `Ok` is returned, the unwrapped value is the result of the mathematical operation `num1 + num2`.
///
/// # Notes
/// Any other notes, usage considerations or guidelines, anything important that wasn't included in the previous sections.
pub fn add(num1: i32, num2: i32) -> Result\<i32, Box\<dyn std::error::Error\>\> {...}
````

### Python Doc Comment Format

Formatted in the Google-Style docstring format.

````python
def add(num1: int, num2: int) -> int:
    """Add two numbers together

    Note that ... (if there are notes)
    
    Example:
        ```
        a = add(4, 7)
        assert a == 13
        ```
    
    Args:
        num1 (int): The first number.
        num2 (int):  The second number.
    
    Raises:
        PanicException: If `num2` is equal to `11`.
        ValueError: If ...
    
    Returns:
        int: The result of the mathematical operation `num1 + num2`.
    """
    ...
````

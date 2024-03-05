fn main() {
    // Rust provides two floating-point types: f32 (32-bit) and f64 (64-bit, default). f64 is
    // preferred for its balance of speed and precision on modern CPUs. Both types are signed.
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    /*
     * Rust supports basic math operations: addition, subtraction, multiplication, division
     * (truncating towards zero for integers), and remainder. Each operation here evaluates to a
     * single value, assigned to a variable.
     */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // In Rust, the Boolean type (`bool`) has two possible values: `true` and `false`, each taking
    // up one byte.
    let t = true;

    let f: bool = false; // with explicit type annotation
}

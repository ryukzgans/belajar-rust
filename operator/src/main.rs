fn main() {
    // Operator Aritmatika (+, -, *, /, %)
    let (num1, num2): (i8, i8) = (12, 4);

    let value_addition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_addition);

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);

    let value_mut = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mut);

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);

    let value_mod = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_mod);

    // Operator Perbandingan (==, !=, >, <, >=, <=)
    let number_a = 12;
    let number_b = 24;

    let res_one = number_a == number_b;
    println!("{number_a} == {number_b} = {res_one}");

    let res_two = number_a != number_b;
    println!("{number_a} != {number_b} = {res_two}");

    let res_three = number_a > number_b;
    println!("{number_a} > {number_b} = {res_three}");

    let res_four = number_a < number_b;
    println!("{number_a} < {number_b} = {res_four}");

    let res_five = number_a >= number_b;
    println!("{number_a} >= {number_b} = {res_five}");

    let res_six = number_a <= number_b;
    println!("{number_a} <= {number_b} = {res_six}");

    // Operator Negasi (-, !)
    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);

    println!("{res_one} | {res_two}");

    // Operator Logika (&&, ||, !)
    let (bool_left, bool_right) = (false, true);
    println!("AND result \t: {}", bool_left);
    println!("OR result \t: {}", bool_right);

    // Operator reference dan dereference (*, &, &mut)

    // Operator bitwise
    /*
        &   = bitwise AND
        |   = bitwise OR
        ^   = bitwise XOR
        !   = bitwise NOT
        <<  = left shift
        >>  = right shift
    */
}

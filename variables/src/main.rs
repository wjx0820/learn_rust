fn main() {
   let x = 5;
   println!("x has the value {}", x);
   // shadowing
   // preserve immutability
   // can change the type
   let x = "six";
   println!("x has the value {}", x);

   // const variables cannot use the mut keyword
   // has to type explicitly
   // cannot be returned
   const MAX_POINTS: u32 = 100_000;

   // scalar and compound types
   // scalar types: Intergers
   let a = 98_222; // Decimal
   let b = 0xff;  // Hex
   let c = 0o77;  // Octal
   let d = 0b1111_0000; // Binary
   let e = b'A';  // Byte(u8 only)
   let f: u8 = 255; // 256 will overflow

   // scalar types: Floating point
   let g = 2.0;
   let h:f32 = 3.14;

   // scalar types: Boolean
   let i = true;
   let j: bool = false;

   // scalar types: Character
   let k = 'z';
   let l = 'ℤ';
   let heart_eyed_cat = '😻';

   // Coumpound Types
   let tup = ("Let's Get Rusty!", 1, 2.0);
   // destructuring
   let (title, integer, float) = tup;
   // dot notation
   let integer = tup.1;
   // array is fixed size
   let status_codes = [200, 302, 404];
   let not_found = status_codes[2];

   let byte = [0; 8];

   // function
   let sum = my_function(11, 22);
   println!("The sum is: {}", sum);

   // Control Flow
   let condition = true;
   let number = if condition { 1 } else { 0 };

   // loops
   let mut counter = 0;
   let result = loop {
      counter += 1;
      if counter == 10 {
         break counter;
      }
   };
   println!("The result is {}", result);

   // while loop
   let mut number = 3;
   
   while number != 0 {
      println!("{}!", number);
      number -= 1;
   }

   println!("exited while loop");

   // for loop
   let a = [10, 20, 30, 40, 50];

   for element in a.iter() {
      println!("the value is: {}", element);
   }

   for number in 1..10 {
      println!("{}", number);
   }

   // line comment

   /*
      block comment 
   */
}

// function use snake_case
fn my_function(x: i32, y:i32) -> i32 {
   println!("The value of x is: {}", x);
   println!("The value of y is: {}", y);

   // let sum = x + y;

   // // return sum;
   // // the last expression is implicitly returned, and omit the semicolon
   // sum

   x + y
}
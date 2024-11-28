// Traits
// -> Methods  :   The “self” and “Self” Keywords
//  "self" means the value on which the "double" method will be applied, whichever it will be, and "Self" means the type of "self"
// //--- All valid format
// fn double(self) -> Self {
// fn double(self: Self) -> Self {
// fn double(self: i32) -> Self {
// fn double(self) -> i32 {
// fn double(self: Self) -> i32 {
// fn double(self: i32) -> i32 {

fn main() {
    /*
         Let’s see another example.
     We want to be able to write the expression "foobarbaz".letters_count('a') that
    counts how many characters are in the string, and therefore returns 2. We can do it in
    this way
         */
    trait LettersCount {
        fn letters_count(&self, ch: char) -> usize;
    }
    impl LettersCount for str {
        fn letters_count(&self, ch: char) -> usize {
            let mut count = 0;
            for c in self.chars() {
                if c == ch {
                    count += 1;
                }
            }
            count
        }
    }
    print!("{} ", "".letters_count('a'));
    print!("{} ", "ddd".letters_count('a'));
    print!("{} ", "ddd".letters_count('d'));
    print!("{} ", "foobarbaz".letters_count('a'));

    /*
         Notice that if we had opted for a functional style, we would get a much shorter
    program. In fact, the whole body of the function can be equivalently replaced by the
    following line.
     self.chars().filter(|c| *c == ch).count()
     */
}

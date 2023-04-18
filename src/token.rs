use std::char;



struct Token {
    kind: String,
    value: String,
}

// Begin by accepting an input string, then two steps
fn tokenizer(input: &str) -> Vec<Token> {
    // append newline to the program
    input += "\n";
    // variable for tracking out position in the code, like a cursor
    let mut current: i32 = 0;

    // Vector to append the tokens to 
    let mut tokens: Vec<[&Token]> = Vec::new();

    // We begin by creating a loop where we set up out 'current'
    //variable to be incremented as much as we want inside the loop
    //
    // We do this because we may want to incrmement 'current' many times a 
    // single loop because our tokens can be any length.

    while current < input.as_bytes().len() {
        let ch = input.as_bytes()[current];

        // The first thing we check, is to look for open parenthesis. This will later
        // be used s 'CallExpressions' but for now we only care about the character.
        // 
        // We check to see if we have an open parenthesis:
        if ch == '(' {
        // If we do, we append a new token to our vector of kind "paren"
        // and set the value to an open parenthesis.
                tokens.push(Token {
             kind: String::from("paren"), 
             value: String::from('('),
                });
        // increment current to maintain our cursor position
        current += 1;
        // and we continue to the next cycle
        continue;
            }
    // Now we check for closing parenthesis. Exact same steps. check, store token
    // increment current and continue.
        if ch == ')' {
            tokens.push(Token {
            kind: String::from("paren"),
            value: String::from(')'),
        });
            current += 1;
            continue;
    }
        // Now we must check for whitespace. This is interesting because we care that it
    // exists to separate characters, but it isn't important for us to store as a token.
    // we would only throw it out later, so we will check for existance and if it exists, we
    // will simply 'continue'
    if ch == ' ' {
        current += 1;
        continue;
    }
    // our next type of token is a number. This is different because it could be any 
    // number of characters, and we ant to capture th eentire sequence of characters
    // and store it as one token.
    if ch.is_numeric() {

            // We need to create a string that we can append the characters to
            let mut value: String = String::new();

            // Then we are going to loop through each character in the sequence until
            // we encounter a character that isn't a number, pushing each one that 
            // is to our 'value' string and incrementing 'current'
            while ch.is_numeric() {
                value.push(ch);
                current += 1;
                ch = String::from(input.as_bytes()[current]);
            }
            // And we append our 'number' token to the tokens vector

            tokens.push(Token {
                kind: "number",
                value: value,
            });
        continue;
        }
        // The last type of token will be a "name" token. This is a sequence of letters
        // instead of numbers, that are the names of function calls in our lisp syntax.
        // (add 2 4)

        // Name token

        if ch.is_ascii() {
        let mut value: String = String::new();
            // if we find letters, we push them to new string and store it in value variable

            while ch.is_ascii() {
                value.push(ch);
                current += 1;
                ch = String::from(input.as_bytes()[current]);
            }
            // and append that value as a token with the type 'name' and continuing
            tokens.push(Token {
                kind: "name",
                value: value,
            })
            continue;
                    }
        break;
        }
    return tokens;

}



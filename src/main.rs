// Port of https://github.com/hazbo/the-super-tiny-compiler, which gets all credit for all
//explanations, comments, etc. Still work in progress

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: String,
    value: String,
}

// Begin by accepting an input string, then two steps
pub fn tokenizer(mut input: String) -> Vec<Token> {
    // append newline to the program
    input.push('\n');
    // variable for tracking out position in the code, like a cursor
    let mut current: usize = 0;

    // Vector to append the tokens to
    let mut tokens = Vec::new();

    // We begin by creating a loop where we set up out 'current'
    //variable to be incremented as much as we want inside the loop
    //
    // We do this because we may want to incrmement 'current' many times a
    // single loop because our tokens can be any length.

    while current < input.as_bytes().len() as usize {
        let vals: Vec<char> = input.chars().collect();
        let mut ch: String = String::from(vals[current]);

        // The first thing we check, is to look for open parenthesis. This will later
        // be used s 'CallExpressions' but for now we only care about the character.
        //
        // We check to see if we have an open parenthesis:
        if ch == "(" {
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
        if ch == ")" {
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
        if ch.as_str() == " " {
            current += 1;
            continue;
        }
        // our next type of token is a number. This is different because it could be any
        // number of characters, and we ant to capture the entire sequence of characters
        // and store it as one token.
        if ch.parse::<f64>().is_ok() {
            // We need to create a string that we can append the characters to
            let mut value: String = String::new();

            // Then we are going to loop through each character in the sequence until
            // we encounter a character that isn't a number, pushing each one that
            // is to our 'value' string and incrementing 'current'
            while ch.parse::<f64>().is_ok() {
                value += ch.as_str();
                current += 1;
                ch = String::from(vals[current]);
            }
            // And we append our 'number' token to the tokens vector

            tokens.push(Token {
                kind: String::from("number"),
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
                value += ch.as_str();
                current += 1;
                ch = String::from(vals[current]);
            }
            // and append that value as a token with the type 'name' and continuing
            tokens.push(Token {
                kind: String::from("name"),
                value: value,
            });
        }
        break;
    }
    return tokens;
}

// We define our struct "Node". In the struct are the lifetimes of the references
// in the arguments and context fields are tied to the lifetime of the Node instance itself,
// indicated by the use of &'a [Node<'a>].. This allows us to have a flexible number of
//elements in these fields, without needing to allocate a vector on the heap
#[derive(Debug, PartialEq)]

struct Node<'a> {
    kind: String,
    value: String,
    name: String,
    callee: *mut Node<'a>,
    expression: *mut Node<'a>,
    body: Vec<Node<'a>>,
    params: Vec<Node<'a>>,
    arguments: &'a [Node<'a>],
    context: &'a [Node<'a>],
}

// "ast" (abstract syntax tree) will simply be an alias for "Node". As node contains many
// parameters it will end up being referenced a lot
type Ast = Node<'a>;

// Now we define our "parser" function that accepts a vector of Tokens
fn parser(tokens: Vec<&Token>) -> ast {
    //create a counter much as we did before in tokenizer
    let mut pc: usize = 0;

    let mut pt: Vec<&Token> = Vec::new();
    pt = tokens;

    // Create our abstract syntax tree, with a root of type "Program" node
    // TODO FINISH
}

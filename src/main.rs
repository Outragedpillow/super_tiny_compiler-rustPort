// Port of https://github.com/hazbo/the-super-tiny-compiler, which gets all credit for all
//explanations, comments, etc. Still work in progress
use std::collections::HashMap;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(String),
    Value(String),
    String(String),
    Open,
    Close,
}

// Begin by accepting an input string, which we need to "tokenize" before its parsed
pub fn tokenizer(input: String) -> Result<Vec<Token>, String> {
// This is done by splitting up the characters into groups or "Tokens", this makes it
// possible to parse and begin to build our Abstract Syntax Tree.
    // First we create a Vector to append the tokens from our input to,
    // to be returned by our tokenizer function in the form of a Result type

    let mut tokens: Vec<Token> = Vec::new();
    // Instead of incrementing a counter a pointer, we will use Rust's powerful tools such as
    // while let, and match statements to traverse the input string

        let mut vals: Peekable<Chars> = input.clone().chars().peekable();
        while let Some(ch) = vals.next() {
            // While let is
            match ch {
                // The order doesn't matter, but we first check for parenthesis.
                // They will be used later in 'CallExpressions'
                // but for now we only care about the characters/numbers
               '(' => tokens.push(Token::Open),
               // Match statements in rust act much like switch statments in other languages
               ')' => tokens.push(Token::Close),
                '0'...'9' => {
                    let mut num = String::new();
                    num.push(ch);
                    while let Some(&'0'...'9') = ch.peek() {
                        num.push(ch.next().unwrap());
                    }
                    tokens.push(Token::Number(num));
                },
                'a'...'z' => {
                    let mut letters = String::new();
                    letter.push(ch);
                    while let Some(&'a'...'z') = ch.peek() {
                        letters.push(ch.next().unwrap());
                    }
                    tokens.push(Token::String(letters));
                },
                '"' => {
                    let mut value = String::new();
                    while match ch.peek() {
                        Some(&'"') | None() => false,
                        _ => true
                    } {
                        value.push(ch.next().unwrap());
                    }
                    tokens.push(Token::Value(value));
                    ch.next().unwrap();
                },
                (_) => return Err(format!("Character not recognized"))
            };
        }
            return Ok(tokens);
}

// We define our struct "Node". In the struct are the lifetimes of the references
// in the arguments and context fields are tied to the lifetime of the Node instance itself,
// indicated by the use of &'a [Node<'a>].. This allows us to have a flexible number of
//elements in these fields, without needing to allocate a vector on the heap
#[derive(Debug, PartialEq)]
struct Node<'a> {
    kind: Option<String>,
    value: Option<String>,
    name: Option<String>,
    callee: Option<*mut Node<'a>>,
    expression: Option<*mut Node<'a>>,
    body: Option<Vec<Node<'a>>>,
    params: Option<Vec<Node<'a>>>,
    arguments: Option<&'a [Node<'a>]>,
    context: Option<&'a [Node<'a>]>,
}

// "ast" (abstract syntax tree) will simply be an alias for "Node". As node contains many
// parameters it will end up being referenced a lot
type Ast<'a> = Node<'a>;
// Now we define our "parser" function that accepts a vector of Tokens
fn parser<'a>(tokens: Vec<&Token>, &mut pc: usize, &mut pt: Vec<&Token>) -> Ast<'a> {
    //create a counter much as we did before in tokenizer
    pt = tokens;

    // Create our abstract syntax tree, with a root of type "Program" node
    let ast: Node = Ast {
        kind: Option::from(String::from("Program")),
        value: None,
        name: None,
        callee: None,
        expression: None,
        body: Option::from(Vec::new()),
        params: None,
        arguments: None,
        context: None,
    };
   while pc < pt.len() {
      ast.body.push(walk(pc, pt));
   }
    return ast;
}
// Instead of a 'while' loop to access our values, this function will do it recursively
fn walk(&mut pc: usize, &mut pt: Vec<&Token>) -> Result<Node<'static>, Error> {
    // Inside the walk funciton we start by grabbing the 'current' token
    let token = pt[pc];

    // We can split each type of token off into a diff code path,
    // beginning with 'number' tokens.
    if token.parse::<f64>().is_ok() {
        pc += 1;
    }
    // Next we look for CallExpressions. (open parenthesis)
    if token.kind == "paren" && token.value == "(" {
        // increment 'current' to skip the parenthesis since we don't care
        // about it in the AST
        pc += 1;
        token = pt[pc];

        // We create a base node with the type 'CallExpression', and were going
        // to set the name as the current token's value since the next token
        // after the open parenthesis is the name of the function
        let n = Node {
            kind: Option::from(String::from("CallExpression")),
            value: None,
            name: Option::from(token.value),
            params: Option::from(Vec::new()),
            callee: None,
            expression: None,
            body: None,
            arguments: None,
            context: None,
        };
        pc += 1;
        token = pt[pc];
        // And now we want to loop through each token that will be the `params` of
        // our `CallExpression` until we encounter a closing parenthesis.
        //
        // Now this is where recursion comes in. Instead of trying to parse a
        // potentially infinitely nested set of nodes we're going to rely on
        // recursion to resolve things.
        //
        // To explain this, let's take our Lisp code. You can see that the
        // parameters of the `add` are a number and a nested `CallExpression` that
        // includes its own numbers.
        //
        //   (add 2 (subtract 4 2))
        //
        // You'll also notice that in our tokens array we have multiple closing
        // parenthesis.
        //
        //   [
        //     { type: 'paren',  value: '('        },
        //     { type: 'name',   value: 'add'      },
        //     { type: 'number', value: '2'        },
        //     { type: 'paren',  value: '('        },
        //     { type: 'name',   value: 'subtract' },
        //     { type: 'number', value: '4'        },
        //     { type: 'number', value: '2'        },
        //     { type: 'paren',  value: ')'        }, <<< Closing parenthesis
        //     { type: 'paren',  value: ')'        }  <<< Closing parenthesis
        //   ]
        //
        // We're going to rely on the nested `walk` function to increment our
        // `current` variable past any nested `CallExpressions`.

        // So we create a `while` loop that will continue until it encounters a
        // token with a `type` of `'paren'` and a `value` of a closing
        // parenthesis.
        while token.kind != "paren" || (token.kind == "paren" && token.value != ")") {
            // we call the 'walk' function which will return a 'node' and we'll push it into out 'node.params'.
            n.params.push(walk(pc, pt));
            token = pt[pc];
        }
        // Finally we will increment 'current' one last time to skip the closing
        //parenthesis
        pc += 1;
        return Ok(n);
    }

    let result = Node { kind: Option::from(String::from("NumberLiteral")), value: Option::from(token.value), name: None, callee: None, expression: None, body: None, params: None, arguments: None, context: None, };
    return match result {
        Ok(result) => result,
        Err(e) => panic!("{}", e)
    }
}
// The visitor will consist of a string and a function associated with it.
// For each instance variable we defined earlier, we will be able to access a function
// via the value of the string stored as a key in the visitor hashmap
type Visitor<'a> = HashMap<&'a str, fn(&Node<'a>, Node<'a>)>;

fn traverser(a: Ast, v: Visitor) {

    // We call "traverseNode" with our ast with no "parent" because the top level of the
    // AST doesn't have one
    traverseNode(node(a), p, v);
}
// a traverseArray function, that will allow us to iterate over a slice and
// call the next function that we will define: "traverseNode
fn traverseArray(a: Vec<Node>, p: Node, v: Visitor) {
    for child in a {
       traverseNode(child, *p, *v);
    }
}
fn traverseNode(n: Node, p: Node, v: Visitor) -> () {
    // we iterate over the visitor we pass to the traverseNode function, and the value
    // will be the function. We call it with the node and its parent
    for (key, value) in &v {
        if k == n.kind {
            value(&n, *p)
        }
    }
    // Next we split things up by the current node type
    match n.kind {
        // We start at top level "Program". Since program nodes have a prop named
        // 'body' that has a Vector of nodes, we will call 'traverseArray" to traverse
        // down into them. traverseArray will in turn call "traverseNode" so we are
        Some(String::from("Program")) => {
            if let Some(body) = n.body {
                traverseArray(body, &n: Node, &v: Visitor)
            }
        }
        Some(String::from("CallExpression")) => {
                if let Some(params) = n.params {
                    traverseArray(params, &n: Node, &v: Visitor)
                }
            }
        Some(String::from("NumberLiteral")) => {
                return ();
        }
        None() => { return () }
    }
}
type Visitor<'a> = HashMap<&'a str, fn(&Node<'a>, Node<'a>)>;

fn traverser(a: Ast, v: Visitor) {

    // We call "traverseNode" with our ast with no "parent" because the top level of the
    // AST doesn't have one
    traverseNode(node(a), p, v);
}
// a traverseArray function, that will allow us to iterate over a slice and
// call the next function that we will define: "traverseNode
fn traverseArray(a: Vec<Node>, p: Node, v: Visitor) {
    for child in a {
        traverseNode(child, *p, *v);
    }
}
fn traverseNode(n: Node, p: Node, v: Visitor) {
    // we iterate over the visitor we pass to the traverseNode function, and the value
    // will be the function. We call it with the node and its parent
    for (key, value) in &v {
        if k == n.kind {
            value(&n, *p)
        }
    }
    // Next we split things up by the current node type
    match n.kind {
        // We start at top level "Program". Since program nodes have a prop named
        // 'body' that has a Vector of nodes, we will call 'traverseArray" to traverse
        // down into them. traverseArray will in turn call "traverseNode" so we are

        n.kind => traverseArray(n.body, n: Node, v: Visitor),
    }
}
/**
 * Next up, the transformer. Our transformer is going to take the AST that we
 * have built and pass it to our traverser function with a visitor and will
 * create a new ast.
 *
 * ----------------------------------------------------------------------------
 *   Original AST                     |   Transformed AST
 * ----------------------------------------------------------------------------
 *   {                                |   {
 *     type: 'Program',               |     type: 'Program',
 *     body: [{                       |     body: [{
 *       type: 'CallExpression',      |       type: 'ExpressionStatement',
 *       name: 'add',                 |       expression: {
 *       params: [{                   |         type: 'CallExpression',
 *         type: 'NumberLiteral',     |         callee: {
 *         value: '2'                 |           type: 'Identifier',
 *       }, {                         |           name: 'add'
 *         type: 'CallExpression',    |         },
 *         name: 'subtract',          |         arguments: [{
 *         params: [{                 |           type: 'NumberLiteral',
 *           type: 'NumberLiteral',   |           value: '2'
 *           value: '4'               |         }, {
 *         }, {                       |           type: 'CallExpression',
 *           type: 'NumberLiteral',   |           callee: {
 *           value: '2'               |             type: 'Identifier',
 *         }]                         |             name: 'subtract'
 *       }]                           |           },
 *     }]                             |           arguments: [{
 *   }                                |             type: 'NumberLiteral',
 *                                    |             value: '4'
 * ---------------------------------- |           }, {
 *                                    |             type: 'NumberLiteral',
 *                                    |             value: '2'
 *                                    |           }]
 *  (sorry the other one is longer.)  |         }]
 *                                    |       }
 *                                    |     }]
 *                                    |   }
 * ----------------------------------------------------------------------------
 */
// Transformer function accepts the lisp AST
fn transformer(a: Ast) -> &Ast {
    // Create a new Ast with a program node
    let nast: Ast = Ast {
        kind: Option::from(String::from("Program")),
        value: None,
        name: None,
        callee: None,
        expression: None,
        body: Option::from(Vec::new()),
        params: None,
        arguments: None,
        context: None,
    };
    if let Some(mut context) = a.context {
       context = nast.body.copy();
    }
    // Define a sample function to be called by the visitor
  fn visit_node_a(node: &Node, parent: Node) {
        parent.context.push( Node {
            kind: Option::from(String::from("NumberLiteral")),
            value: None,
            name: NoneM,
            callee: None,
            expression: None,
            body: None,
            params: None,
            arguments: None,
            context: None,
        })
    }
// Create the visitor
    let mut visitor: Visitor = HashMap::new();
    visitor.insert("node_a", visit_node_a);
   traverser(a: Ast, visit_node_a({

    }
    return Ast;
}


 fn main() {
    println!("way more complicated than the Go port....")

 }

based on https://www.minizinc.org/doc-2.5.5/en/spec.html#full-grammar

reduced just to what is needed for specifying a base-type parameter, but with no assigned value:

    % A MiniZinc model
    <model> ::= [ <item> ";" ... ]
    % Items
    <item>  ::= <var-decl-item>
    % Variable declaration items
    <var-decl-item> ::= <ti-expr-and-id>
    
    <ti-expr-and-id> ::= <ti-expr> ":" <ident>
    <ti-expr> ::= <base-ti-expr>
    <base-ti-expr> ::= <base-ti-expr-tail>
    <base-ti-expr-tail> ::= <base-type>
    % Identifiers
    <ident> ::= [A-Za-z][A-Za-z0-9_]* | ’[^’\xa\xd\x0]*’
    
    <base-type> ::= "bool"
                  | "int"
                  | "float"
                  | "string"


inline definitions with no variation:

    % A MiniZinc model
    <model> ::= [ <ti-expr-and-id> ";" ... ]
    <ti-expr-and-id> ::= <base-type> ":" <ident>
    <ident> ::= [A-Za-z][A-Za-z0-9_]* | ’[^’\xa\xd\x0]*’
    <base-type> ::= "bool"
    | "int"
    | "float"
    | "string"

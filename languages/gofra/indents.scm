; Gofra blocks are keyword delimited, so every block that opens with a keyword
; and closes with `end` opens an indentation range.

(function_declaration) @indent
(function_declaration "end" @outdent)

(lambda_declaration) @indent
(lambda_declaration "end" @outdent)

(struct_declaration) @indent
(struct_declaration "end" @outdent)

(union_declaration) @indent
(union_declaration "end" @outdent)

(while_statement) @indent
(while_statement "end" @outdent)

(for_statement) @indent
(for_statement "end" @outdent)

; `if` is postfix - the condition is in front of the keyword, so the body
; starts after the `if` token.
(if_statement) @indent
(if_statement "if" @start)
(if_statement "end" @outdent)

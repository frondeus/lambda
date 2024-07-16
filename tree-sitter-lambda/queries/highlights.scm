(comment) @comment.line

(bool) @constant.builtin

":" @operator
"=" @operator
";" @operator

"let" @keyword.storage.type

(def) @function
(let key: (ident) @variable)
(def arg: (ident) @variable.parameter)

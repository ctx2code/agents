# Agent Rules

## Python Code Guidelines

+ Add proper type annotations to function's parameters and return type.
+ Do not use `Any` type annotation.
+ Use `typing.cast` only when it's theorically unable to infer.

+ Put the import statements as near to their usage site as possible.
+ For import statements at the file scope, only import module base.
+ For those imported and only used once, only import module base.

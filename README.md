> [!WARNING]
> This project is working on progress.

# schematch
Declarative schema checking commands

# Usage

```txt
Declarative schema checking commands

Usage: schematch <SCHEMA> [FILE]

Arguments:
  <SCHEMA>  The schema to check against. ex. "id:integer email:string name:string"
  [FILE]    The file to check. If not provided, stdin will be used

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Example

### Valid input case

```terminal
$ cat data.txt
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ cat data.txt | schematch "id:integer email:string name:string"
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ echo $?
0 # this means `schematch` command was successful
```


### Invalid input case

```terminal
$ cat data.txt
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh


$ cat data.txt | schematch "id:integer email:string name:string"
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh

$ echo $?
1 # this means `schematch` command was unsuccessful
```

schematch never modify the input, it just checks the input against the schema and returns 0 if the input is valid and 1 if the input is invalid.


## Schema EBNF

```ebnf
schema ::= term ( ' ' term )*
term ::= name ':' type
name ::= [a-zA-Z_][a-zA-Z0-9_]*
type ::= 'integer' | 'string' | 'float' | 'boolean' | 'null'
```

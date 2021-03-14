[Home](README.md)

# Grammar specification

## Numbers

```
<number>    ::= <int>,[".",<uint>],["e",<int>]
<int>       ::= [<sign>],<uint>
<uint>      ::= {<digit>}
<sign>      ::= "+"|"-"
<digit>     ::= "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"
```

## Characters

```
<latin_char>    ::=  <lower_latin_char> | <upper_latin_char>
<lower_latin_char>    ::= "a"|...|"z"
<upper_latin_char>    ::= "A"|...|"Z"
<space>         ::= " "
<div>           ::= <space>,[{<space>}]
<end>           ::= <NEW_LINE>
<string>        ::= <quote>,{char},<quote>
<quote>         ::= "'"
<char>          ::= ^<quote>
```

## Names

```
<id>        ::= <lower_latin_char>,[{<latin_char>}]
<anonymous> ::= "_"
```

## General

```
<module>    ::= [{<import>,<end>}],[{<block>,<end>}]
<import>    ::= [<modifier>],<div>,"mod",<div>,<mod_path>
<mod_path>  ::= <quote>,<OS_PATH>,<quote>
<modifier>  ::= "pub"
<block>     ::= <control>|<definition>|<call>
```

## Controls

```
<control>   ::= <inline_if>|<if>|<loop>
<inline_if> ::= <id>,<div>,<assignment>,<div>,"if",<div>,<expression>,
                <div>,"else",<expression>
<if>        ::= "if",<div>,<expression>,":",<end>,<block>,<end>,
                [{"elif",<div>,<expression>,":",<end>,<block>,<end>}],
                ["else:",<end>,<block>]
<loop>      ::= "loop:",<end>,<block>,"jmp"
```

[Home](README.md)
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
<import>    ::= [<modifier>],"mod",<mod_path>
<mod_path>  ::= <quote>,<OS_PATH>,<quote>
<modifier>  ::= "pub"
<mut>       ::= "!"
<block>     ::= <control>|<definition>
```

## Controls

```
<control>   ::= <inline_if>|<if>|<loop>|<while>
<inline_if> ::= <id>,<assignment>,"if",<expression>,"else",<expression>
<if>        ::= "if",<expression>,":",<end>,<block>,<end>,
                [{"elif",<expression>,":",<end>,<block>,<end>}],
                ["else:",<end>,<block>]
<loop>      ::= "loop:",<end>,<block>
<while>     ::= "while",<expression>,":",<end>,<block>
```

## Definition

```
<definition>    ::= [<lvalue>,<assignment>],<rvalue>
<lvalue>        ::= [<modfifier>],["("],([<var>]|{<var>,","}),[")"]
<var>           ::= [<mut>],<id>,[":",<type>]
<rvalue>        ::= <value>|<domain>|<fun>
<value>         ::= <id>
                    |<number>
                    |<string>
                    |<bool>
                    |("[",<values>,"]")
                    |(["("],<values>,[")"])
                    |("{",([<pair>]|{<pair>,","}),"}")
<bool>          ::= "true"|"false"
<values>        ::= [<value>]|{<value>,","}
<pair>          ::= <value>,":",<value>
<fun>           ::= <signature>,":",<end>,<block>
<signature>     ::= "(",[<arg>|{",",<arg>}],")",["->",<type>]
<arg>           ::= [<mut>],<id>,[":",<type>],["=",<const>]
<const>         ::= <number>|<string>|<bool>
```

## Types

```
<type>          ::= "Str"
                    |"Bool"
                    |"Int"
                    |"Float"
                    |"Dyn"
                    |("[",<type>,"]")
                    |("(",<type>,{",",<type>},")")
                    |("{",<type>,":",<type>,"}")
                    |("{",<value>,":",<type>,{",",<value>,":",<type>},"}")
                    |<domain>
                    |(["("],[<type>|{<type>,","}],[")"],"->",<type>)
<domain>        ::= (["<"],<type>,[">])|("<",<domain_values>,">")
<domain_values> ::= (<type>|<id>),{",",(<type>|<id>)}
```

[Home](README.md)
[Home](README.md)

# Variables

Nana follows the phylosophy *"Everything is data"*, so any declaration in Nana is data, in other words, it is a some value of a variable, even functions. Also there is no `null` (as in Java) or `None` (as in Python), or some another **null**-value. All variables must be initialized by non-null value at the time of declaration. Every declaration of a variable follows a scheme below:

```
[pub] variable_name[?][: Type] = value
```

As shown in the scheme, we can omit the access modifier (*private* by default), the mutation modifier (*constant* by default) and the data type (in this case Nana inferences the most appropriate type). 

## Mutation modifier

All variables are immitable by default, except for variables marked as mutable `?`. It means that the next code won't be executed:  

```
a = 1
a = 2 # Error! Attempt to reassign a value for a.

b? = 3
b = 4 # OK! The b is mutable.
```

**Notice** that the content of abstract data types can still be changed:

```
list1 = [1, 2, 3, 4]
list1 = list1 + [5] # Error! Attempt to reassign a reference to a new in-memory allocation.
list2 = [1, 2, 3, 4]
list2.add(5) # OK! The reference was not changed.
```

[Home](README.md)

---
title: 2.12 Core Symbols
---

------



##### Core

````
   compile        format     funcall    load    macro-function   
   macroexpand    read       write      version set-macro-character
   get-macro-character
````

##### Sequences

````
   elt       length    sv-list    foldl     foldr
   findl-if  findr-if  length     positionl positionr
````

##### Lists

````
   assoc     dropl     dropr    last        append    
   mapc      mapcar    mapl     maplist
````

##### Predicates

````
   charp     consp     doublep  exceptionp  fixnump
   floatp    functionp listp    namespacep  not
   null      sequencep streamp  stringp     symbolp
   vectorp   zerop     fboundp  boundp      uninternedp
   keywordp  numberp
````

##### Exceptions

````
   error     error-unless       error-when  errorp-if 
   print-except  warn
````

##### Strings

````
   schar     string   string-append  string=  substr
````

##### Symbols

````
   symbol-value symbol-name symbol-ns
````

##### Vectors

````
   vector-type svref
````

##### Fixnums

````
   1+          1-   truncate floor ceiling
   mod         rem  round    ash
````

##### Special operators

````
   defconst  defmacro defun          if       lambda
````

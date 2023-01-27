---
title: 2.10 Core Streams
---

#### **Streams**

<hr>

*core* streams are primarily distinguished from *mu* streams by accepting *stream-designator* arguments that map () to *mu:std-out* and *t* to *mu:std-in*. The *core* *read* function has been enhanced to allow error handling on end of stream conditions, similar to the Common Lisp *read* function.

 

**make-string-stream**  *direction string* ***=>*** *stream*

<hr>

<div class="list">
<span class="dfn">direction</span> : a <span class="dfn">keyword symbol</span> :input | :output</br>
<span class="dfn">string</span> : a <span class="dfn">string</span></br>
</div>



*make-string-stream* creates a string stream with the indicated direction. The stream is initialized from *string*.



**get-output-string-stream** *output-string-stream* ***=>*** *string*

<hr>

<div class="list">
<span class="dfn">string-stream</span> : an output <span class="dfn">string stream</span></br>
</div>



*get-output-string-stream* gets the stream contents, resets the stream to empty, and returns the contents.



**eofp** *stream-designator* ***=>*** *boolean*

<hr>

<div class="list">
<span class="dfn">stream-designator</span> : an input <span class="dfn">string stream designator</span></br>
</div>




*eofp* tests the stream for the end of stream condition.



**open-file** *direction path* ***=>*** *stream*

<hr>

<div class="list">
<span class="dfn">direction</span> : a <span class="dfn">keyword symbol</span> :input | :output</br>
<span class="dfn">path</span> : a file system path <span class="dfn">string</span></br>
</div>

*open-file* opens a file stream for reading or writing, depending on *direction*.



**close** *stream-designator* ***=>*** *boolean*

<hr>

<div class="list">
<span class="dfn">stream-designator</span> : an <span class="dfn">stream designator</span></br>
</div>



*close* closes a stream. *close* returns () on an already closed stream, otherwise ***t***.



**write-char** *char stream-designator*  ***=>*** *char* | ()</br>
**write-byte** *byte stream-designator*  ***=>*** *byte* | ()

<hr>

<div class="list">
<span class="dfn">char</span> : a <span class="dfn">character</span> object</br>
<span class="dfn">byte</span> : a small <span class="dfn">fixnum</span> in the range of [0..255]</br>
<span class="dfn">stream-designator</span> : an output <span class="dfn">stream designator</span></br>
</div>



*write-char* writes *char* to an output stream.

*write-byte* writes *byte* to an output stream.



**write**  *form* stream-designator escape*  ***=>*** t | ()

<hr>
<div class="list">
<span class="dfn">form</span> : an <span class="dfn">value</span></br>
<span class="dfn">stream-designator</span> : an output <span class="dfn">stream designator</span></br>
<span class="dfn">escape</span> : a <span class="dfn">generalized boolean</span></br>
</div>



*write* converts *form* to a *string* and writes it to *stream-designator*. *write* includes escape characters (" for strings, #\ for chars, etc) if *escape* is true. *write* with a true *escape* is suitable for subsequent *read*s for objects that have a printable representation. Other types will be printed in *broket notation*, which are not generally readable.



**terpri**  *stream-designator*  ***=>*** t | ()

<hr>

<div class="list">
<span class="dfn">stream-designator</span> : an output <span class="dfn">stream designator</span></br>
</div>

*terpri* outputs the system's best guess at an end of line character.



**read-char**  *stream-designator*  ***=>*** *char* | ()</br>
**read-byte** *stream-designator error-eofp eof-value*  ***=>*** *fixnum* | ()

<hr>
<div class="list">
<span class="dfn">stream-designator</span> : an input <span class="dfn">stream designator</span></br>
<span class="dfn">error-eofp</span> : a <span class="dfn">generalized boolean</span></br>
<span class="dfn">eof-value</span> : a <span class="dfn">value</span></br>
</div>

*read-char* returns the next *char* from the input stream . If at end of stream and *error-eofp* is true, an error is raised, otherwise *eof-value* is returned.

*read-byte* returns the next *fixnum* from the input stream . If at end of stream and *error-eofp* is true, an error is raised, otherwise *eof-value* is returned.



**unread-char** *char stream-designator*  ***=>*** *char* | ()</br>
**unread-byte** *byte stream-designator*  ***=>*** *byte* | ()

<hr>

<div class="list">
<span class="dfn">char</span> : a <span class="dfn">character</span> object</br>
<span class="dfn">byte</span> : a small <span class="dfn">fixnum</span> in the range of [0..255]</br>
<span class="dfn">stream-designator</span> : an input <span class="dfn">stream designator</span></br>
</div>
*unread-char* pushes back *char* onto the input stream.

*unread-byte* pushes back *byte* onto the input stream. 

One level of pushback is supported, attempts to push more, or attempts to read a non-pushed stream raises an error.



**read**  *stream-designator eof-error eof-value*  ***=>*** *object*

<hr>

<div class="list">
<span class="dfn">stream-designator</span> : an input <span class="dfn">stream designator</span></br>
<span class="dfn">error-eofp</span> : a <span class="dfn">boolean</span> object</br>
<span class="dfn">eof-value</span> : an <span class="dfn">object</span></br>
</div>



*read* returns the next object from the input stream . If at end of stream and *error-eofp* is true, an error is raised, otherwise *eof-value* is returned.
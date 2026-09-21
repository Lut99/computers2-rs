# CAS1 - Assembly language
This file defines the CAS1 assembly language for the Andor computer series.


## 1. File structure
### 1.1. Directives
Every file is a list of _directives_, which represents some kind of information necessary for an A1
program to run.
The directives are supposed to be interpreted in-order; by default, each will be evaluated in turn
in order to represent execution.

### 1.2. Directive types
There are three types of directives:
1. Instructions;
2. Data; and
3. Labels.

The first represents evaluatable code.
The second represents arbitrary data embedded in the final file.
The third marks points in the file for reference.

### 1.3. Directive sizes
Every directive has a _byte size_ associated with it. This is the size that the compiled
counterpart of the directive has in the output binary.
Labels are markers in the stream of directives that can be used to jump around in the file to
specific instructions or data.
For instructions and data, the byte size depends on the contents.
Labels always have 0 byte size, as they are not represented in the final binary file.


## 2. Instructions

# CAS1 - Assembly language
_Version 1.0.0_

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

Note that the actual directive sizes are not necessary to program CAS1. Instead, use labels to
jump.


## 2. Instructions
### 2.1. General shape
Instructions in CAS1 are always headed by an instructions identifier, and zero or more arguments.
Every arguments consists of either:
1. A literal; or
2. A register.

Literals are unsigned numbers in base 2 or base 16, or signed or unsigned integer- or
floating-point numbers in base 10., base 10 or base 16.
Registers are numbers or identifiers prefixed by a hashtag. The exact possible registers are
defined by the specific target processor.

### 2.2. Instruction list
The following instructions are supported in CAS1.
An "address integer" is an unsigned or signed integer who's bitwidth is determined by the specific
processor that's targeted.

**2.2.1. Mathmatical instructions**  
1. `uaddX #<dst> #<lhs> #<rhs>` Adds the X rightmost bits of register `<lhs>` to the X rightmost
   bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X can be 8,
   16, 32 or 64. The bits are interpreted as an unsigned integer.
2. `usubX #<dst> #<lhs> #<rhs>` Subtracts the X rightmost bits of register `<rhs>` from the X
   rightmost bits of register `<lhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as an unsigned integer.
3. `umulX #<dst> #<lhs> #<rhs>` Multiplies the X rightmost bits of register `<lhs>` with the X
   rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as an unsigned integer.
4. `udivX #<dst> #<lhs> #<rhs>` Divides the X rightmost bits of register `<lhs>` by the X rightmost
   bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X can be 8,
   16, 32 or 64. The bits are interpreted as an unsigned integer.
5. `umodX #<dst> #<lhs> #<rhs>` Takes the modulo of the X rightmost bits of register `<lhs>` by the
   X rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as an unsigned integer.
6. `saddX #<dst> #<lhs> #<rhs>` Adds the X rightmost bits of register `<lhs>` to the X rightmost
   bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X can be 8,
   16, 32 or 64. The bits are interpreted as a signed integer.
7. `ssubX #<dst> #<lhs> #<rhs>` Subtracts the X rightmost bits of register `<rhs>` from the X
   rightmost bits of register `<lhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as a signed integer.
8. `smulX #<dst> #<lhs> #<rhs>` Multiplies the X rightmost bits of register `<lhs>` with the X
   rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as a signed integer.
9. `sdivX #<dst> #<lhs> #<rhs>` Takes the modulo of the X rightmost bits of register `<lhs>` by the
   X rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
   can be 8, 16, 32 or 64. The bits are interpreted as a signed integer.
10. `smodX #<dst> #<lhs> #<rhs>` Takes the modulo of the X rightmost bits of register `<lhs>` by
    the X rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of
    `<dst>`. X can be 8, 16, 32 or 64. The bits are interpreted as a signed integer.
11. `faddX #<dst> #<lhs> #<rhs>` Adds the X rightmost bits of register `<rhs>` to the X rightmost
    bits of register `<lhs>` and stores the result in the X rightmost bits of `<dst>`. X can be 32
    or 64. The bits are interpreted as an IEEE754 floating-point.
12. `fsubX #<dst> #<lhs> #<rhs>` Subtracts the X rightmost bits of register `<rhs>` from the X
    rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
    can be 32 or 64. The bits are interpreted as an IEEE754 floating-point. 
13. `fmulX #<dst> #<lhs> #<rhs>` Multiplies the X rightmost bits of register `<lhs>` to the X
    rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
    can be 32 or 64. The bits are interpreted as an IEEE754 floating-point. 
14. `fdivX #<dst> #<lhs> #<rhs>` Divides the X rightmost bits of register `<lhs>` by the X
    rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of `<dst>`. X
    can be 32 or 64. The bits are interpreted as an IEEE754 floating-point. 
15. `fmodX #<dst> #<lhs> #<rhs>` Takes the modulo of the X rightmost bits of register `<lhs>` with
    the X rightmost bits of register `<rhs>` and stores the result in the X rightmost bits of
    `<dst>`. X can be 32 or 64. The bits are interpreted as an IEEE754 floating-point.
16. `land #<dst> #<lhs> <rhs>` Takes the logical and-operation between `<lhs>` and `<rhs>`, storing
    the result in `<dst>`.
17. `lor #<dst> #<lhs> <rhs>` Takes the logical or-operation between `<lhs>` and `<rhs>`, storing
    the result in `<dst>`.
18. `lnot #<dst> #<src>` Takes the logical not-operation of `<src>` and stores it in `<dst>`.

**2.2.2. Bitwise operations**  
1. `shl #<dst> #<lhs> #<rhs>` Shifts the bits in register `<lhs>` by `<rhs>` places to the left,
   filling the right end with 0's. The result is stored in `<dst>`.
2. `shr #<dst> #<lhs> #<rhs>` Shifts the bits in register `<lhs>` by `<rhs>` places to the right,
   filling the left end with 0's. The result is stored in `<dst>`.
3. `band #<dst> #<lhs> #<rhs>` Takes the bitwise and-operation between `<lhs>` and `<rhs>`,
   storing the result in `<dst>`.
3. `bor #<dst> #<lhs> #<rhs>` Takes the bitwise or-operation between `<lhs>` and `<rhs>`,
   storing the result in `<dst>`.
4. `bxor #<dst> #<lhs> #<rhs>` Takes the bitwise xor-operation between `<lhs>` and `<rhs>`,
   storing the result in `<dst>`.
5. `bnot #<dst> #<src>` Takes the bitwise not-operation of `<src>`, storing the result in `<dst>`.

**2.2.3. Comparison operations**
1. `eq #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if `<lhs>` equals `<rhs>`. Else, writes 0.
2. `ne #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if `<lhs>` _does not_ equal `<rhs>`. Else, writes
   0.
3. `ultX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
   smaller than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as unsigned integers. X can be 8, 16, 32 or 64.
4. `ugtX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
   greater than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as unsigned integers. X can be 8, 16, 32 or 64.
5. `uleX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are smaller
   than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as unsigned integers. X can be 8, 16, 32 or 64.
6. `ugeX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are greater
   than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as unsigned integers. X can be 8, 16, 32 or 64.
7. `sltX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
   smaller than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as signed integers. X can be 8, 16, 32 or 64.
8. `sgtX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
   greater than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as signed integers. X can be 8, 16, 32 or 64.
9. `sleX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are smaller
   than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
   interpreted as signed integers. X can be 8, 16, 32 or 64.
10. `sgeX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are greater
    than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
    interpreted as signed integers. X can be 8, 16, 32 or 64.
11. `fltX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
    smaller than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
    interpreted as IEE754 floating-point numbers. X can be 32 or 64.
12. `fgtX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are strictly
    greater than the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
    interpreted as IEE754 floating-point numbers. X can be 32 or 64.
13. `fleX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are smaller
    than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
    interpreted as IEE754 floating-point numbers. X can be 32 or 64.
14. `fgeX #<dst> #<lhs> #<rhs>` Writes 1 to `<dst>` if the rightmost X bits of `<lhs>` are greater
    than or equal to the rightmost X bits of `<rhs>`. Else, writes 0. `<lhs>` and `<rhs>` are
    interpreted as IEE754 floating-point numbers. X can be 32 or 64.

**2.2.4. Literal operations**
1. `constX #<dst> <lit>` Stores literal `<lit>` in the X rightmost bits of register `<dst>`. The
   literal may not exceed X bits in size. X can be 8, 16, 32 or 64.

**2.2.5. Memory operations**  
Memory addresses have a processor-specific size. The address is read in the relevant rightmost bits
of a register.
1. `load8 #<dst> #<src>` Loads the byte at the address in `#<src>` at the 8 rightmost bits in
   `<dst>`.
2. `load16 #<dst> #<src>` Loads the byte at the address in `#<src>` _and_ its subsequent byte at
   the 16 rightmost bits in `<dst>`.
3. `load32 #<dst> #<src>` Loads the byte at the address in `#<src>` _and_ its subsequent three
   bytes at the 32 rightmost bits in `<dst>`.
4. `load64 #<dst> #<src>` Loads the byte at the address in `#<src>` _and_ its subsequent seven
   bytes at the 64 rightmost bits in `<dst>`.
5. `store8 #<dst> #<src>` Stores the 8 rightmost bits of `<src>` at the byte at the address in
   `#<dst>`.
6. `store16 #<dst> #<src>` Stores the 16 rightmost bits of `<src>` at the byte at the address in
   `#<dst>` and its subsequent byte.
7. `store32 #<dst> #<src>` Stores the 32 rightmost bits of `<src>` at the byte at the address in
   `#<dst>` and its subsequent three bytes.
8. `store64 #<dst> #<src>` Stores the 64 rightmost bits of `<src>` at the byte at the address in
   `#<dst>` and its subsequent seven bytes.
9. `memsize <dst>` Loads the total available system memory size in register `<dst>`. The size of
   unsigned integer has the same size as the address width.

**2.2.6. Control flow operations**
1. `goto <dst>` Changes the program counter by the offset defined in register `<dst>`. The contents
   of `<dst>` are interpreted as a signed address integer.
2. `branch <dst> <cond>` Changes the program counter by the offset defined in register `<dst>` _if_
   the contents of `<cond>` evaluate to a boolean true. Else, the program continues. The contents
   of `<dst>` are interpreted as a signed address integer.

**2.2.7. System operations**  
1. `cpuid <dst>` Loads the current CPU identifier, as a 64-bit unsigned integer, in register
   `<dst>`. The identifier itself _may_ have further semantic meaning, but this is
   processor-specific.
2. `time <dst>` Loads the current unix timestamp, as a 64-bit unsigned integer, in register
   `<dst>`.
3. `exit` Terminates the program, shutting down the computer.


## 3. Data
Data is loaded by giving arbitrary pieces of data to the assembler.
These are then placed in the program at that location.
This can be done by giving an arbitrary string of literals. Every literal is represented by the
smallest number of bytes capable of representing it.


## 4. Labels
Labels mark the current spot in the program.
When used outside of instructions, labels create an association with a specific address.
When used as argument to an instruction (in literal position), they are referring to the associated
address.
This allows for jumps to "named" and automatically computed addresses.


## 5. Booting
The CPU will start executing the program at address 0, i.e., it will load the program in memory.
Use relative references only, unless when the specific CPU makes guarantees itself.
No guarantees are made about the absolute position of the code.

Also take note that the data embedded is a "raw" program. Depending on its contents, expect the CPU
to either crash or display unexpected behaviour if it is ever executed.

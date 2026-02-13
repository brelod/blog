# Table of Content

# Introduction
The challenge is writing simple version a dynamic linker (ld.so) which is able to provide a runtime environment for python3.
The tool is the Rust programming language and the Linux kernel. For keeping the code simple the Rust standerd library is not used.

The purpose of this project is not to give you quick answers how something needs to be implemented. It's rather about giving you
questions which I have faced with while I was solving the challenge. As a result you will find many steps back and forth through out
the chapters. If you are interested on the topic I encourage you to avoid jumping immediatelly to the answers section and to find
your own answers before you read mines. 

I am aiming to write an article which is able to provide you questions and walk you through a logical but winding road but at the
same time it can be used as a reference to go back to over and over again and which one can learn from.

As a result the code snippets contain many bugs. This is a key part of the game and these bugs will be identified and
analised throughout the chapters. If you can not find a resolution for a bug, please let me and we can build a chapter 
about it together. You can report such a bug or create a pull request on the git repository.

<div style="page-break-before:always">&nbsp;</div>
<p></p>

Assembly:
- Elf sections
- Static libraries
- Dynamic libraries
- Executables

Rust:
- no-std executable

**Questions**:
1. Standalone executable
    - Where does a program start?
    - How does the memory look like on startup?
2. Handover control
    - How to create static library?
    - How to create shared library?
    - How to link against static library?
    - How to link against shared library?
    - Where does a library get loaded? (pic)
    - Where does an executable get loaded? (pie)
    - How to find out the address of the mappings?
    - How should the stack look like on handover? (argc + stack alignment)
3. Relocate self
    - How to identify the symbol of a relocation?
    - How to work without using relocations?
4. Load libraries
    - In what order should we load the libraries?
    - How to initialize a library? (should we do it before the relocation?)
5. Relocate libraries + executable
    - What kind of relocations are there?
    - How to find out the relocations of a binary / library
6. Lazy bindings
    - What is the difference between PLT / GOT relative relocations?
    - What are the benefits and drawbacks of PLT? (cache, security)
    - What effect does it have on program start?
    - What is the purpose of read-only relocations?
7. Infrastucture (dlopen, etc)
    - How can we open / close dynamic libraries at runtime?
    - 

**Answers**:
- aux
- symbols
- relocations
- PIC / PIE
- GOT / PLT
- Object file
- Shared object
- Static / Dynamic linker
- Static / Dynamic linking

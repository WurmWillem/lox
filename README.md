<H1> Rox: A Refreshingly Simple Programming Language </H1>

<H2>Introducing Rox 🦀</H2>
This is my implementation of Lox, a programming language so simple that even your little sister can program in it. As of right now the language (if you can even call it that) is extremely bare-bones, but it is improving rapidly. The goal of Lox is not to create a fully-functioning language for commercial use; it is simply the hobby project of a mediocre Rust programmer.

<H2>Why should you use Rox?</H2>

-  🚀 Blazingly fast and written in Rust  
- 😃 Because you are a masochist
- 🥰 You want to support me

<H2>Features</H2>

- Math with proper precedence rules
  ```
  3 * (2 + 3)
  output: 15
  
  1 + 6 / 3
  output: 3
  
  ```
- Logic operators that can be chained
  ```
  5 == 2 + 3
  output: true

  1 < 3
  output: true

  (3 + 2 * 3 == 3 * (2 + 1)) == (8 >= 4)
  output: true
  ```

<H2>How do I use Rox?</H2>

- Download the source code and either
  - Download the source code and execute it with the command cargo run "expression" as such 
  ```
  cargo run "2 + 1"
  output: 3
  ```
  - Or create a file called "file.lox" in the same directory as the project and execute the command cargo run without additional arguments
  ```
  cargo run
  output: 3
  ```
- Ask me for a binary and I'll send you an executable for your platform
<H2>Roadmap</H2>

- Make it an actually functioning language with variables and control flow 💀
- Add enums
- Remove null and replace it with Option<T> enum
  - All my homies hate null
import { createFile, type Config } from "./generate";

const config: Config = {
  baseName: "Expr",
  types: [
    {
      name: "Unary",
      parts: [
        { name: "op", type: "Token" },
        { name: "expr", type: "Box<Expr>" },
      ],
    },
    {
      name: "Binary",
      parts: [
        { name: "left", type: "Box<Expr>" },
        { name: "op", type: "Token" },
        { name: "right", type: "Box<Expr>" },
      ],
    },
    {
      name: "Grouping",
      parts: [{ name: "expr", type: "Box<Expr>" }],
    },
    {
      name: "Literal",
      parts: [{ name: "value", type: "Token" }],
    },
  ],
};

const outPath = "../src/ast/expr.rs";
const targetComment = "// Custom implementations";

const imports = `use crate::lexer::token::Token;
use crate::lexer::types::TokenType;`;

createFile(config, outPath, imports, targetComment);

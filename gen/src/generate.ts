import { readFileSync, writeFileSync } from "fs";

interface Part {
  name: string;
  type: string;
}

interface Type {
  name: string;
  parts: Part[];
}

export interface Config {
  baseName: string;
  types: Type[];
}

const createEnumDefinition = (config: Config): string => {
  let definition = `#[derive(Debug)]
pub enum ${config.baseName} {\n`;

  definition += config.types
    .map((type) => {
      const partTypes = type.parts.map((p) => p.type);
      return `\t${type.name}(${partTypes.join(", ")}),\n`;
    })
    .join("");

  definition += "}\n";

  return definition;
};

const createVisitorDefinition = (config: Config): string => {
  let def = "pub trait Visitor<R> {\n";

  def += config.types
    .map((ty) => {
      const functionName = `fn visit_${ty.name.toLowerCase()}_${config.baseName.toLowerCase()}`;
      const argDefs = ty.parts.map((p) => `${p.name}: &${p.type}`);

      return `\t${functionName}(&self, ${argDefs.join(", ")}) -> R;`;
    })
    .join("\n");

  def += "\n}\n";

  return def;
};

const createVisitorImpl = (config: Config): string => {
  const createMatchArm = (type: Type) => {
    const functionName = `visit_${type.name.toLowerCase()}_${config.baseName.toLowerCase()}`;
    const argNames = type.parts.map((p) => p.name);

    return `\t\t\t${config.baseName}::${type.name}(${argNames.join(
      ", "
    )}) => visitor.${functionName}(${argNames.join(", ")}),`;
  };

  return `impl Expr {
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
${config.types.map(createMatchArm).join("\n")}
        }
    }
}\n`;
};

export const createFile = (
  config: Config,
  outPath: string,
  imports: string,
  targetCommentContent: string
): void => {
  let existingContent = readFileSync(outPath, "utf-8");

  let keepIndex = existingContent.indexOf(targetCommentContent);
  if (keepIndex === -1) {
    keepIndex = existingContent.length;
  }

  let content = `${imports}

${createEnumDefinition(config)}
${createVisitorDefinition(config)}
${createVisitorImpl(config)}

${existingContent.slice(keepIndex)}
`;
  writeFileSync(outPath, content);
};

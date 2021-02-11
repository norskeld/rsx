use crate::loader::Script;

pub fn load_scripts() -> Vec<Script> {
  vec![
    Script {
      id: 1,
      script: "build",
      command: "tsc",
    },
    Script {
      id: 2,
      script: "build:watch",
      command: "tsc -w",
    },
    Script {
      id: 3,
      script: "style",
      command: "npm run style:format && npm run style:lint",
    },
    Script {
      id: 4,
      script: "style:format",
      command: "prettier --write \"src/**/*.ts\"",
    },
    Script {
      id: 5,
      script: "style:lint",
      command: "eslint src --ext .js,.ts --fix",
    },
    Script {
      id: 6,
      script: "install:g",
      command: "npm run build && npm i -g",
    },
  ]
}

export type ProgrammingLanguageID =
  | "rust"
  | "java"
  | "javascript"
  | "go"
  | "python"
  | "ruby";

export interface ProgrammingLanguage {
  id: ProgrammingLanguageID;
  name: string;
}

export interface GetProgrammingLanguagesResponse {
  items: ProgrammingLanguage[];
}

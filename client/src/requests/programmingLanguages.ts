import type { GetProgrammingLanguagesResponse } from "../models/programmingLanguages";

export async function getProgrammingLanguages(): Promise<GetProgrammingLanguagesResponse> {
  try {
    const res = await fetch(
      "http://localhost:5000/api/v1/programming-languages",
    );
    const data: GetProgrammingLanguagesResponse = await res.json();

    return data;
  } catch (error) {
    console.log(error);

    throw new Error(
      `Failed to fetch programming languages: ${(error as Error).message}`,
    );
  }
}

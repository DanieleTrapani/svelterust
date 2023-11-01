import type { PageLoad } from "./$types";

export const load = async () => {
  return {
    todos: await fetch("http://localhost:3000/").then((data) => data.json()),
  };
};

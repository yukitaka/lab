import { extract } from "$std/encoding/front_matter.ts";
import { join } from "$std/path/posix.ts";

const DIRECTORY = "./contents";

export interface Content {
  slug: string;
  title: string;
  publishedAt: Date;
  content: string;
}

export async function getContents(): Promise<Content[]> {
  const files = Deno.readDir(DIRECTORY);
  const promises = [];
  for await (const file of files) {
    const slug = file.name.replace(".md", "");
    promises.push(getContent(slug));
  }
  const contents = await Promise.all(promises) as Content[];
  contents.sort((a, b) => b.publishedAt.getTime() - a.publishedAt.getTime());

  return contents;
}

export async function getContent(slug: string): Promise<Content | null> {
  const text = await Deno.readTextFile(join(DIRECTORY, `${slug}.md`));
  const { attrs, body } = extract(text);
  return {
    slug,
    title: attrs.title,
    publishedAt: new Date(attrs.published_at),
    content: body
  };
}

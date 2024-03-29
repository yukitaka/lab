import { Handlers, PageProps } from "$fresh/server.ts";
import { getContent, getContents, Content } from "@/utils/contents.ts";

export const handler: Handlers<Content[]> = {
  async GET(_req, ctx) {
    const contents = await getContents();
    return ctx.render(contents);
  },
};

export default function ContentIndexPage(props: PageProps<Content[]>) {
  const contents = props.data;
  return (
    <main class="max-w-screen-md px-4 pt-16 mx-auto">
      <h1 class="text-5x1 font-bold">Content</h1>
      <div class="mt-8">
        {contents.map((content) => <ContentCard content={content} />)}
      </div>
    </main>
  );
}

function ContentCard(props: { content: Content }) {
  const { content } = props;
  return (
    <div class="py-8 border(t gray-200)">
      <a class="sm:col-span-2" href={`/${content.slug}`}>
        <h3 class="text(3xl gray-900) font-bold">
          {content.title}
        </h3>
        <time class="text-gray-500">
          {new Date(content.publishedAt).toLocaleDateString("en-us", {
            year: "numeric",
            month: "long",
            day: "numeric",
          })}
        </time>
      </a>
    </div>
  );
}

import { useRouteData } from "solid-start";
import { redirect } from "solid-start/server";
import { getUser } from "~/db/session";

export function routeData() {
  return createServerData$(async (_, { request }) => {
    const user = await getUser(request);

    if (!user) {
      throw redirect("/login");
    }

    return user;
  });
}

export default function Home() {
  const user = useRouteData<typeof routeData>();

  return (
    <main class="w-full p-4 space-y-2">
      <h1 class="font-bold text-3xl">Hello {user()?.username}</h1>
    </main>
  );
}

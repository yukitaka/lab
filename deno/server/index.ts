import {serve} from "https://deno.land/std/http/server.ts";

const port = 8080;

const handler = (req: Request): Response => {
  const body = `Hello World\n${req.headers.get("user-agent") ?? "Unknown"}`;

  return new Response(body, {status: 200});
};

await serve(handler, {port});

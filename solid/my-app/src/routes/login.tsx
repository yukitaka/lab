import { createServerAction$ } from "solid-start/server";

export default function Login() {
  const [loggingIn, { Form }] = createServerAction$(async (form: FormData) => {
  });

  return (
    <main>
      <h1>Login</h1>
      <Form>
      </Form>
    </main>
  )
}

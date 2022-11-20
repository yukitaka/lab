import { useParams, useRouteData } from "solid-start";
import { FormError } from "solid-start/data";
import { createServerAction$, createServerData$ } from "solid-start/server";
import { getUser } from "~/db/session";

function validateUsername(username: unknown) {
  if (typeof username !== "string" || username.length < 3) {
    return `Usernames must be at least 3 characters long`;
  }
}

function validatePassword(password: unknown) {
  if (typeof password !== "string" || password.length < 6) {
    return `Passwords must be at least 6 characters long`;
  }
}

export function routeData() {
  return createServerData$(async (_, { request }) => {
    if (await getUser(request)) {
      throw redirect("/");
    }
    return {};
  });
}

export default function Login() {
  const data = useRouteData<typeof routeData>();
  const params = useParams();

  const [loggingIn, { Form }] = createServerAction$(async (form: FormData) => {
    const loginType = form.get("loginType");
    const username = form.get("username");
    const password = form.get("password");

    const fields = { loginType };
    const fieldErrors = {
      username: validateUsername(username),
      password: validatePassword(password)
    };

    if (Object.values(fieldErrors).some(Boolean)) {
      throw new FormError("Fields invalid", { fieldErrors, fields });
    }
  });

  return (
    <main>
      <h1>Login</h1>
      <Form>
        <input type="hidden" name="redirectTo" value={params.redirectTo ?? "/"} />
        <fieldset>
          <legend>Login or Register?</legend>
          <label>
            <input type="radio" name="loginType" value="login" checked={true} /> Login
          </label>
          <label>
            <input type="radio" name="loginType" value="register" /> Register
          </label>
        </fieldset>
        <div>
          <label for="username-input">Username</label>
          <input name="username" placeholder="name" />
        </div>
        <Show when={loggingIn.error?.fieldErrors?.username}>
          <p role="alert">{loggingIn.error.fieldErrors.username}</p>
        </Show>
        <div>
          <label for="password-input">Password</label>
          <input name="password" type="password" placeholder="passw0rd" />
        </div>
        <Show when={loggingIn.error?.fieldErrors?.password}>
          <p role="alert">{loggingIn.error.fieldErrors.password}</p>
        </Show>
        <Show when={loggingIn.error}>
          <p role="alert" id="error-message">
            {loggingIn.error.message}
          </p>
        </Show>
        <button type="submit">{data() ? "Login" : ""}</button>
      </Form>
    </main>
  )
}

import { useState } from "react";
import { PageShell } from "~/components/PageShell";
import { Field, FieldGroup, FieldLabel } from "~/components/ui/field";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    console.log("[login] handleSubmit fired");
    debugger;

    try {
      const res = await fetch("http://127.0.0.1:3000/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ username, password }),
      });

      console.log("[login] response:", res.status, res.statusText);
    } catch (err) {
      console.error("[login] fetch error", err);
    }
  }

  return (
    <PageShell>
      <main className="flex justify-center items-center w-full">
        <Card className="w-96 bg-zinc-900">
          <CardHeader>
            <CardTitle>Login to RustyMine</CardTitle>
            <CardDescription>Enter your username and password to login</CardDescription>
          </CardHeader>
          <CardContent>
            {/* IMPORTANT: no action="", no method="" */}
            <form onSubmit={handleSubmit}>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="username">Username</FieldLabel>
                  <Input
                    id="username"
                    name="username"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    required
                  />
                </Field>
                <Field>
                  <FieldLabel htmlFor="password">Password</FieldLabel>
                  <Input
                    id="password"
                    name="password"
                    type="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    required
                  />
                </Field>
              </FieldGroup>

              <CardFooter className="mt-4 px-0">
                <Button
                  type="submit"
                  className="w-full"
                  onClick={() => console.log("[login] button clicked")}
                >
                  Login
                </Button>
              </CardFooter>
            </form>
          </CardContent>
        </Card>
      </main>
    </PageShell>
  );
}

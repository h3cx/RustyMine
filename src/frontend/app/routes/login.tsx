import { useState } from "react"
import { useNavigate } from "react-router"

import { PageShell } from "~/components/PageShell"
import { Field, FieldGroup, FieldLabel } from "~/components/ui/field"
import { Input } from "~/components/ui/input"
import { Button } from "~/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card"
import { loginUser } from "~/lib/api"

export default function Login() {
  const navigate = useNavigate()
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")
  const [error, setError] = useState<string | null>(null)
  const [isSubmitting, setIsSubmitting] = useState(false)

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setError(null)
    setIsSubmitting(true)

    try {
      await loginUser({ username, password })
      navigate("/dashboard")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Unable to log in"
      setError(message)
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <PageShell>
      <main className="flex w-full flex-1 items-center justify-center px-4">
        <Card className="w-full max-w-md border-border/50 bg-zinc-900">
          <CardHeader>
            <CardTitle className="text-2xl">Login to RustyMine</CardTitle>
            <CardDescription>
              Enter your username and password to access the dashboard
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form className="space-y-4" onSubmit={handleSubmit}>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="username">Username</FieldLabel>
                  <Input
                    id="username"
                    name="username"
                    autoComplete="username"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    disabled={isSubmitting}
                    required
                  />
                </Field>
                <Field>
                  <FieldLabel htmlFor="password">Password</FieldLabel>
                  <Input
                    id="password"
                    name="password"
                    type="password"
                    autoComplete="current-password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    disabled={isSubmitting}
                    required
                  />
                </Field>
              </FieldGroup>

              {error && (
                <p className="text-sm text-destructive" role="alert">
                  {error}
                </p>
              )}

              <CardFooter className="px-0">
                <Button
                  type="submit"
                  className="w-full"
                  disabled={isSubmitting}
                >
                  {isSubmitting ? "Signing in..." : "Login"}
                </Button>
              </CardFooter>
            </form>
          </CardContent>
        </Card>
      </main>
    </PageShell>
  )
}

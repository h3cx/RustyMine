import { redirect, useLoaderData } from "react-router"

import { PageShell } from "~/components/PageShell"
import type { Route } from "./+types/dashboard"
import { SidebarProvider, SidebarTrigger } from "~/components/ui/sidebar"
import { AppSidebar } from "~/components/sidebar"
import { fetchCurrentUser } from "~/lib/api"

export function meta(_: Route.MetaArgs) {
  return [{ title: "RustyMine Dashboard" }]
}

export async function loader({ request }: Route.LoaderArgs) {
  const user = await fetchCurrentUser(request)

  if (!user) {
    throw redirect("/login")
  }

  return { user }
}

export default function Dashboard() {
  const { user } = useLoaderData<typeof loader>()

  return (
    <PageShell>
      <SidebarProvider>
        <AppSidebar />
        <main className="flex-1 p-6">
          <div className="flex items-center gap-3">
            <SidebarTrigger />
            <h1 className="text-2xl font-semibold text-foreground">
              Hello {user.username}
            </h1>
          </div>
        </main>
      </SidebarProvider>
    </PageShell>
  )
}

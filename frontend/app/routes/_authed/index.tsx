// app/routes/index.tsx
import * as fs from 'node:fs'
import { createFileRoute, useRouter } from '@tanstack/react-router'
import { createServerFn } from '@tanstack/start'

const filePath = 'count.txt'

export const Route = createFileRoute('/_authed/')({
  component: Home,
  loader: () => {
    return { count: 0 }
  },
})

function Home() {
  const router = useRouter()
  const state = Route.useLoaderData()

  console.log('xxx', state)

  return <div>XXXXXXXXXX</div>
}

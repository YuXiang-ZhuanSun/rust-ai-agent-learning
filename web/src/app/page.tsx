import Link from "next/link";

export default function RootPage() {
  return (
    <main className="flex min-h-screen items-center justify-center p-6">
      <Link
        href="/en/"
        className="rounded-lg bg-zinc-900 px-5 py-3 text-sm font-medium text-white transition-colors hover:bg-zinc-700"
      >
        Open AI Agent Learning
      </Link>
    </main>
  );
}

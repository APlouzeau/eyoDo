import Link from "next/dist/client/link";

export default function Home() {
    return (
        <div className="flex min-h-screen flex-col items-center justify-between p-24">
            Coucou
            <Link href="/taches/en-cours">En cours</Link>
        </div>
    );
}

import Link from "next/link";

export default function Sidebar() {
    return (
        <aside className="min-h-full flex flex-col w-64 bg-gray-200 p-8 gap-4">
            <Link href="/taches/en-cours">En cours</Link>
            <Link href="/taches/terminees">Terminées</Link>
        </aside>
    );
}

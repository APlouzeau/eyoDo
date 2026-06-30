import TaskTable from "../components/TaskTable";

export default function EnCours() {
    return (
        <div className="flex min-h-screen w-full flex-col items-center">
            <h2 className="text-2xl font-bold mb-4 text-center pt-8">En cours</h2>
            <div className="w-full gap-4 p-8">
                <div className="flex justify-end">
                    <button className="flex bg-gray-800 text-white px-4 py-2 rounded mb-4 font-semibold">
                        Ajouter une tâche
                    </button>
                </div>
                <TaskTable />
            </div>
        </div>
    );
}

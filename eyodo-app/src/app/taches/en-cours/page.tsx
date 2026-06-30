import TaskTable from "../components/TaskTable";

export default function EnCours() {
    return (
        <div className="flex min-h-screen w-full flex-col items-center">
            <h2 className="text-2xl font-bold mb-4 text-center pt-8">En cours</h2>
            <div className="w-full gap-4 p-8">
                <TaskTable />
            </div>
        </div>
    );
}

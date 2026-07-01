import { CreateTaskDto } from "../types/Task";

interface AddTaskFormProps {
    onSubmit: (task: CreateTaskDto) => void;
    onClose: () => void;
}

export default function AddTaskForm({ onSubmit, onClose }: AddTaskFormProps) {
    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const data = new FormData(e.currentTarget);
        const task: CreateTaskDto = {
            title: data.get("taskName") as string,
            description: data.get("taskDescription") as string,
            dateToFinish: data.get("dateToFinish") as string,
            assignedTo: data.get("assignedTo") as string,
        };
        onSubmit(task);
    };

    return (
        <div className="bg-white p-6 rounded shadow-lg w-96">
            <h2 className="text-xl font-bold mb-4">Ajouter une tâche</h2>
            <form onSubmit={handleSubmit}>
                <div className="mb-4">
                    <label htmlFor="taskName" className="block text-sm font-medium text-gray-700">
                        Titre
                    </label>
                    <input
                        type="text"
                        id="taskName"
                        name="taskName"
                        className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    />
                </div>
                <div className="mb-4">
                    <label htmlFor="taskDescription" className="block text-sm font-medium text-gray-700">
                        Description
                    </label>
                    <textarea
                        id="taskDescription"
                        name="taskDescription"
                        rows={3}
                        className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    ></textarea>
                </div>
                <div className="flex mb-4 w-full gap-4 align-items-center">
                    <div className="mb-4 w-full">
                        <label htmlFor="taskAssignedTo" className="text-sm font-medium text-gray-700">
                            Assigné à
                        </label>
                        <select
                            name="assignedTo"
                            className="border border-gray-300 rounded-md shadow-sm p-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm w-full"
                        >
                            <option value="user1">Utilisateur 1</option>
                            <option value="user2">Utilisateur 2</option>
                            <option value="user3">Utilisateur 3</option>
                        </select>
                    </div>
                    <div className="mb-4 w-full">
                        <label htmlFor="dateToFinish" className="text-sm font-medium text-gray-700">
                            Date de fin
                        </label>
                        <input
                            type="date"
                            id="dateToFinish"
                            name="dateToFinish"
                            className="w-full border border-gray-300 rounded-md shadow-sm p-2 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                        />
                    </div>
                </div>
                <div className="flex justify-around">
                    <button
                        type="button"
                        onClick={onClose}
                        className="bg-red-300 text-gray-700 px-4 py-2 rounded hover:bg-red-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                    >
                        Annuler
                    </button>
                    <button
                        type="submit"
                        className="bg-indigo-600 text-white px-4 py-2 rounded hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                        Ajouter
                    </button>
                </div>
            </form>
        </div>
    );
}

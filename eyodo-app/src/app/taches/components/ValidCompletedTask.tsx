interface ValidCompletedTaskProps {
    id: string;
    onSubmit: (query: { id: string }) => void;
    onClose: () => void;
}

export default function ValidCompletedTask({ onSubmit, onClose, id }: ValidCompletedTaskProps) {
    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        onSubmit({ id });
    };

    return (
        <div className="bg-white p-6 rounded shadow-lg w-96 h-48">
            <form onSubmit={handleSubmit} className="flex flex-col justify-around h-full">
                <p className=" text-center">Cette tâche est-elle terminée ?</p>
                <div className="flex justify-around">
                    <button
                        type="button"
                        onClick={onClose}
                        className="bg-red-300 text-gray-700 px-4 py-2 rounded hover:bg-red-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                    >
                        Non
                    </button>
                    <button
                        type="submit"
                        className="bg-indigo-600 text-white px-4 py-2 rounded hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                        Oui
                    </button>
                </div>
            </form>
        </div>
    );
}

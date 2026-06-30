export default function TaskTable() {
    return (
        <>
            <table className=" table-auto w-full border-collapse border border-gray-300">
                <thead>
                    <tr>
                        <th className="border border-gray-300 px-4 py-2">Titre</th>
                        <th className="border border-gray-300 px-4 py-2">Description</th>
                        <th className="border border-gray-300 px-4 py-2">Date</th>
                        <th className="border border-gray-300 px-4 py-2">Status</th>
                        <th className="border border-gray-300 px-4 py-2">Attribution</th>
                        <th className="border border-gray-300 px-4 py-2">Commentaires</th>
                        <th className="border border-gray-300 px-4 py-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td className="border border-gray-300 px-4 py-2">Task Title</td>
                        <td className="border border-gray-300 px-4 py-2">Task Description</td>
                        <td className="border border-gray-300 px-4 py-2">2023-01-01</td>
                        <td className="border border-gray-300 px-4 py-2">In Progress</td>
                        <td className="border border-gray-300 px-4 py-2">No comments</td>
                        <td className="border border-gray-300 px-4 py-2">John Doe</td>
                        <td className=" flex border border-gray-300 px-4 py-2">
                            <button className="bg-blue-500 text-white px-2 py-1 rounded mr-2">Edit</button>
                            <button className="bg-red-500 text-white px-2 py-1 rounded">Delete</button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </>
    );
}

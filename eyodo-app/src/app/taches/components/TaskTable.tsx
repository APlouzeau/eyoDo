"use client";
import { useState } from "react";
import { useTasks } from "../hooks/useTasks";
import { useCompleteTask } from "../hooks/useCompleteTask";
import Modal from "@/app/components/Modal";
import ValidCompletedTask from "./ValidCompletedTask";
import { filteredTask } from "../types/Task";
import Image from "next/image";

export default function TaskTable({ filter }: filteredTask) {
    const { data: tasks, isLoading } = useTasks(filter);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [selectedId, setSelectedId] = useState<number | null>(null);
    const { mutate } = useCompleteTask();
    if (isLoading || !tasks) {
        return <div>Loading...</div>;
    }

    console.log("tasks:", tasks);
    return (
        <>
            <table className=" table-auto w-full border-collapse border border-gray-300">
                <thead>
                    <tr>
                        <th className="border border-gray-300 px-4 py-2">Status</th>
                        <th className="border border-gray-300 px-4 py-2">Titre</th>
                        <th className="border border-gray-300 px-4 py-2">Description</th>
                        <th className="border border-gray-300 px-4 py-2">Date</th>
                        <th className="border border-gray-300 px-4 py-2">Attribution</th>
                        <th className="border border-gray-300 px-4 py-2">Commentaires</th>
                        <th className="border border-gray-300 px-4 py-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {(tasks ?? []).map((task) => (
                        <tr key={task.id}>
                            <td className="border border-gray-300 px-4 py-2">
                                {filter !== "completed" && (
                                    <input
                                        type="checkbox"
                                        onChange={() => {
                                            setIsModalOpen(true);
                                            setSelectedId(task.id);
                                        }}
                                    />
                                )}
                                {filter === "completed" && (
                                    <Image
                                        src="/checkbox.png"
                                        alt="Completed"
                                        width={12}
                                        height={12}
                                        className="w-4 h-4 m-auto"
                                    />
                                )}
                            </td>
                            <td className="border border-gray-300 px-4 py-2">{task.title}</td>
                            <td className="border border-gray-300 px-4 py-2">{task.description}</td>
                            <td className="border border-gray-300 px-4 py-2">{task.dueDate}</td>
                            <td className="border border-gray-300 px-4 py-2">{task.ownerName ?? task.creatorName}</td>
                            <td className="border border-gray-300 px-4 py-2">{task.comments ?? ""}</td>
                            <td className=" flex border border-gray-300 px-4 py-2">
                                <button className="bg-blue-500 text-white px-2 py-1 rounded mr-2">Edit</button>
                                <button className="bg-red-500 text-white px-2 py-1 rounded">Delete</button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)}>
                {selectedId && (
                    <ValidCompletedTask
                        id={selectedId}
                        onClose={() => setIsModalOpen(false)}
                        onSubmit={(data) => {
                            mutate(data.id, { onSuccess: () => setIsModalOpen(false) });
                        }}
                    />
                )}
            </Modal>
        </>
    );
}

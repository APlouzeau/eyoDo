"use client";
import Modal from "@/app/components/Modal";
import AddTaskForm from "../components/AddTaskForm";
import TaskTable from "../components/TaskTable";
import { useState } from "react";
import { useCreateTask } from "../hooks/useCreateTask";

export default function Terminees() {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const { mutate } = useCreateTask();

    return (
        <div className="flex min-h-screen w-full flex-col items-center">
            <h2 className="text-2xl font-bold mb-4 text-center pt-8">Terminées</h2>
            <div className="w-full gap-4 p-8">
                <div className="flex justify-end">
                    <button
                        className="flex bg-gray-800 text-white px-4 py-2 rounded mb-4 font-semibold"
                        onClick={() => setIsModalOpen(true)}
                    >
                        Ajouter une tâche
                    </button>
                </div>
                <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)}>
                    <AddTaskForm
                        onSubmit={(data) => {
                            mutate(data, { onSuccess: () => setIsModalOpen(false) });
                        }}
                        onClose={() => setIsModalOpen(false)}
                    />
                </Modal>
                <TaskTable filter="completed" />
            </div>
        </div>
    );
}

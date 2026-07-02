export type Task = {
    id: string;
    title: string;
    description: string;
    createdAt: string;
    dueDate?: string;
    status: "en-cours" | "terminée";
    assignedTo: string;
    comments: string[];
    completedAt?: string;
};

export type CreateTaskDto = {
    title: string;
    description: string;
    dueDate?: string;
    assignedTo: string;
};

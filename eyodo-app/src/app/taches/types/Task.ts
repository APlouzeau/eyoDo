export type Task = {
    id: string;
    title: string;
    description: string;
    createdAt: string;
    dueDate?: string;
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

export type filteredTask = {
    filter: "completed" | "in-progress";
};

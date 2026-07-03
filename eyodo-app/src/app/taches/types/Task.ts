export type Task = {
    id: number;
    title: string;
    description: string;
    dueDate?: string;
    completedAt?: string;
    createdAt: string;
    creatorId: number;
    ownerUserId?: number;
    ownerGroupId?: number;
};

export type CreateTaskDto = {
    title: string;
    description: string;
    dueDate?: string;
    creatorId: number;
    ownerUserId?: number;
    ownerGroupId?: number;
};

export type filteredTask = {
    filter: "completed" | "in-progress";
};

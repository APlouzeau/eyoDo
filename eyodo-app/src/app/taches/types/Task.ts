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
    comments?: string[];
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

export type TaskResponse = {
    id: number;
    title: string;
    description: string;
    dueDate?: string;
    completedAt?: string;
    createdAt: string;
    creatorId: number;
    creatorName: string;
    ownerUserId?: number;
    ownerName?: string;
    ownerGroupId?: number;
    ownerGroupName?: string;
    comments?: string[];
};

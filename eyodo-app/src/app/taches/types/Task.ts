export type Task = {
    id: string;
    title: string;
    description: string;
    createdDate: string;
    dateToFinish?: string;
    status: "en-cours" | "terminée";
    assignedTo: string;
    comments: string[];
};

export type CreateTaskDto = {
    title: string;
    description: string;
    dateToFinish?: string;
    assignedTo: string;
};

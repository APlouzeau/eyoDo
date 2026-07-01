import { CreateTaskDto, Task } from "./types/Task";

export async function getTasks(): Promise<Task[]> {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + "/taches");
    return res.json();
}

export async function createTask(data: CreateTaskDto): Promise<Task> {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + "/taches", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    return res.json();
}

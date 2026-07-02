import { CreateTaskDto, Task } from "./types/Task";
import { filteredTask } from "./types/Task";

export async function getTasks(filter: filteredTask["filter"]): Promise<Task[]> {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + `/tasks?filter=${filter}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
    });
    return res.json();
}

export async function createTask(data: CreateTaskDto): Promise<Task> {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + "/tasks", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    return res.json();
}

export async function completeTask(id: string): Promise<Task> {
    console.log("Completing task with ID:", id);
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + `/tasks/${id}/complete`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ id }), // Send the ID in the request body
    });
    return res.json();
}

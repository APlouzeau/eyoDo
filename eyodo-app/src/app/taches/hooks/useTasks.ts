import { useQuery } from "@tanstack/react-query";
import { getTasks } from "../api";
import { filteredTask } from "../types/Task";

export function useTasks(filter: filteredTask["filter"]) {
    return useQuery({
        queryKey: ["tasks", filter],
        queryFn: () => getTasks(filter),
    });
}

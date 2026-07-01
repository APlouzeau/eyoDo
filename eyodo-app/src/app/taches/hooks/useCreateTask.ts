import { useMutation, useQueryClient } from "@tanstack/react-query";
import { createTask } from "../api";

export function useCreateTask() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: createTask,
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ["tasks"] });
        },
    });
}

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { completeTask } from "../api";

export function useCompleteTask() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: completeTask,
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ["tasks"] });
        },
    });
}

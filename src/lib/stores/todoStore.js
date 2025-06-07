import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

// Create a writable store for todos
export const todos = writable([]); // Initialize with an empty array

// Implement loadTodos function
export async function loadTodos() {
    try {
        const fetchedTodos = await invoke('get_todos');
        todos.set(fetchedTodos);
    } catch (error) {
        console.error("Failed to load todos:", error);
        // Optionally, set an error state in another store or display a notification
    }
}

// Implement addTodo function
export async function addTodo(taskText) {
    if (!taskText || taskText.trim() === "") {
        console.error("Task cannot be empty");
        return; // Or throw an error
    }
    try {
        await invoke('add_todo', { task: taskText });
        await loadTodos(); // Reload todos to reflect the addition
    } catch (error) {
        console.error("Failed to add todo:", error);
    }
}

// Implement updateTodoStatus function
export async function updateTodoStatus(id, completed) {
    try {
        await invoke('update_todo_status', { id, completed });
        // Optimistic update or reload:
        // For a smoother experience, update the local store directly
        todos.update(currentTodos =>
            currentTodos.map(todo =>
                todo.id === id ? { ...todo, completed } : todo
            )
        );
        // Alternatively, uncomment the line below to reload all todos from backend
        // await loadTodos();
    } catch (error) {
        console.error("Failed to update todo status:", error);
        // Optionally, revert optimistic update here
    }
}

// Implement deleteTodo function
export async function deleteTodo(id) {
    try {
        await invoke('delete_todo', { id });
        // Optimistic update or reload:
        // For a smoother experience, update the local store directly
        todos.update(currentTodos => currentTodos.filter(todo => todo.id !== id));
        // Alternatively, uncomment the line below to reload all todos from backend
        // await loadTodos();
    } catch (error) {
        console.error("Failed to delete todo:", error);
    }
}

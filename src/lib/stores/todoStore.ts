import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Define the Todo interface
export interface Todo {
  id: number; // Assuming id from backend is i64, which maps to number in JS/TS
  task: string;
  completed: boolean;
  dueDate: string | null;
}

// Create a writable store for todos with the Todo interface
export const todos: Writable<Todo[]> = writable([]);

// Implement loadTodos function with types
export async function loadTodos(): Promise<void> {
  try {
    const fetchedTodos = await invoke<Todo[]>('get_todos');
    todos.set(fetchedTodos);
  } catch (error) {
    console.error("Failed to load todos:", error);
    // Optionally, set an error state or display a notification
  }
}

// Implement addTodo function with types
export async function addTodo(taskText: string, dueDate: string | null): Promise<void> {
  if (!taskText || taskText.trim() === "") {
    console.error("Task cannot be empty");
    return;
  }
  try {
    // Log parameters as received by this store function
    console.log(`[todoStore.addTodo] Preparing to invoke 'add_todo'. Task: "${taskText}", DueDate: ${dueDate}`);

    // Wrap payload in 'args' to match Rust struct argument
    await invoke('add_todo', { args: { task: taskText, due_date: dueDate } });
    await loadTodos();
  } catch (error) {
    console.error("Failed to add todo:", error);
  }
}

// Implement updateTodoStatus function with types
export async function updateTodoStatus(id: number, completed: boolean): Promise<void> {
  try {
    await invoke('update_todo_status', { id, completed });
    todos.update(currentTodos =>
      currentTodos.map(todo =>
        todo.id === id ? { ...todo, completed } : todo
      )
    );
  } catch (error) {
    console.error("Failed to update todo status:", error);
    // Optionally, revert optimistic update here
  }
}

// Implement deleteTodo function with types
export async function deleteTodo(id: number): Promise<void> {
  try {
    await invoke('delete_todo', { id });
    todos.update(currentTodos => currentTodos.filter(todo => todo.id !== id));
  } catch (error) {
    console.error("Failed to delete todo:", error);
  }
}

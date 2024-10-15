import { invoke } from "@tauri-apps/api/core";

/**
 * Adds a todo to the backend.
 */
export async function addTodo(title: string) {
    console.log("Adding todo...", title);
    const todo = await invoke("add_todo", {
        name: title,
    });
}

/**
 * Fetches the list of todos from the backend and updates the UI.
 */
export async function getTodos() {
    return await invoke("get_todos") as { id: number, name: string, done: boolean }[];
}
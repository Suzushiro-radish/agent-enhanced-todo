import { invoke } from "@tauri-apps/api/core";

// TODOの型定義
export type Todo = {
    id: number,
    name: string,
    done: boolean
};

/**
 * TODOを追加する
 */
export async function addTodo(name: string): Promise<void> {
    console.log("Adding todo...", name);
    await invoke('add_todo', { name });

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

/**
 * すべてのTODOを取得する
 */
export async function getTodos(): Promise<Todo[]> {
    return await invoke<Todo[]>('get_todos');
}

/**
 * TODOの完了状態を更新する
 */
export async function updateTodoStatus(id: number, done: boolean): Promise<void> {
    await invoke('update_todo_status', { id, done });

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

/**
 * TODOを削除する
 */
export async function deleteTodo(id: number): Promise<void> {
    await invoke('delete_todo', { id });

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

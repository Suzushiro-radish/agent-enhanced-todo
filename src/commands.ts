import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Database from "@tauri-apps/plugin-sql";

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
    // データベース接続を保持する変数
    const db = await Database.load('sqlite:todos.db');
    console.log("Adding todo...", name);
    await db.execute("INSERT INTO todos (name, done) VALUES (?, ?)", [name, 0]);

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

/**
 * すべてのTODOを取得する
 */
export async function getTodos(): Promise<Todo[]> {
    // データベース接続を保持する変数
    const db = await Database.load('sqlite:todos.db');
    return await db.select<Todo[]>("SELECT id, name, done FROM todos ORDER BY id");
}

/**
 * TODOの完了状態を更新する
 */
export async function updateTodoStatus(id: number, done: boolean): Promise<void> {
    // データベース接続を保持する変数
    const db = await Database.load('sqlite:todos.db');
    await db.execute("UPDATE todos SET done = ? WHERE id = ?", [done ? 1 : 0, id]);

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

/**
 * TODOを削除する
 */
export async function deleteTodo(id: number): Promise<void> {
    // データベース接続を保持する変数
    const db = await Database.load('sqlite:todos.db');
    await db.execute("DELETE FROM todos WHERE id = ?", [id]);

    // イベントを発行してUIを更新
    window.dispatchEvent(new CustomEvent('todo-updated'));
}

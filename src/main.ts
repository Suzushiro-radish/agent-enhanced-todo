import { invoke } from "@tauri-apps/api/core";

/**
 * Adds a todo to the backend.
 */
async function addTodo(title: string) {
  console.log("Adding todo...", title);
  const todo = await invoke("add_todo", {
    name: title,
  });
  console.log(todo);
}

/**
 * Fetches the list of todos from the backend and updates the UI.
 */
async function updateTodo() {
  const todos = await invoke("get_todos");
  const todoList = document.querySelector("#todo-list");
  if (todoList) {
    todoList.innerHTML = "";
    (todos as { id: number, name: string, done: boolean }[]).forEach((todo) => {
      todoList.appendChild(document.createElement("li")).textContent = todo.name;
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#todo-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    const formData = new FormData(e.target as HTMLFormElement);
    const todoName = formData.get("todo-name") as string;
    addTodo(todoName);
  });
  document.querySelector("#update-todo")?.addEventListener("click", (e) => {
    e.preventDefault();
    updateTodo();
  });
});
